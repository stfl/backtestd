#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context, Result};
use chrono::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use serde_repr::{Deserialize_repr, Serialize_repr};

const FOREX_PAIRS: &'static [&'static str] = &[
    "EURUSD", "GBPUSD", "USDCHF", "USDJPY", "USDCAD", "AUDUSD", "EURCHF", "EURJPY", "EURGBP",
    "EURCAD", "GBPCHF", "GBPJPY", "AUDJPY", "AUDNZD", "AUDCAD", "AUDCHF", "CHFJPY", "EURAUD",
    "EURNZD", "CADCHF", "GBPAUD", "GBPCAD", "GBPNZD", "NZDCAD", "NZDCHF", "NZDJPY", "NZDUSD",
    "CADJPY",
];

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct Indicator {
    pub name: String,
    pub inputs: Vec<Vec<f32>>,
    pub shift: u8,
}

impl Indicator {
    // maybe implement io::Write instead?
    pub fn to_params_config<'a>(&self, use_case: &'a str) -> Result<String> {
        let mut string: String = format!(
            "{use_case}_Indicator={name}\n",
            use_case = use_case,
            name = self.name
        );
        for (i, inp) in self.inputs.iter().enumerate() {
            string.push_str(&format!(
                // TODO remove _Double
                "{use_case}_double{idx}=\
                 {input_value}\n",
                use_case = use_case,
                input_value = input_param_str(inp)?,
                idx = i,
            ));
        }
        if self.shift > 0 {
            string.push_str(&format!("{}_Shift={}\n", use_case, self.shift));
        }

        Ok(string)
    }

    pub fn from_file(file: &str) -> Result<Self> {
        let json_file = File::open(Path::new(file))?;
        Ok(serde_json::from_reader(json_file)?)
    }

    pub fn to_file(&self, file: &str) -> Result<()> {
        let json_file = File::create(Path::new(file))?;
        Ok(serde_json::ser::to_writer_pretty(json_file, self)?)
    }

    fn parse_result_set(&self, result_params: &mut VecDeque<f32>) -> Self {
        Indicator {
            name: self.name.clone(),
            shift: self.shift,
            inputs: self
                .inputs
                .clone()
                .into_iter()
                .map(|inp| {
                    if (3..=4).contains(&inp.len()) {
                        vec![result_params
                            .pop_front()
                            .expect("no more params found in result")]
                    // TODO we MUST have a value here otherwise something went wrong with the test run
                    // TODO assert value is in range
                    } else {
                        inp
                    }
                })
                .collect(),
        }
    }
}

fn input_param_str(input: &Vec<f32>) -> Result<String> {
    match input.len() {
        1 => Ok(format!("{:.2}||0||0||0||N", input[0] as f32)),
        3 => Ok(format!(
            "0||{:.2}||{:.2}||{:.2}||Y",
            input[0] as f32, input[2] as f32, input[1] as f32
        )),
        e => Err(anyhow!("wrong length of indicator params input: {}", e)),
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct IndicatorSet {
    pub confirm: Option<Indicator>,
    pub confirm2: Option<Indicator>,
    pub confirm3: Option<Indicator>,
    pub exit: Option<Indicator>,
    pub cont: Option<Indicator>,
    pub baseline: Option<Indicator>,
    pub volume: Option<Indicator>,
}

impl IndicatorSet {
    fn to_params_config(&self) -> Result<String> {
        let mut string = String::new();
        match &self.confirm {
            Some(i) => string.push_str(&i.to_params_config("Confirm")?),
            _ => (),
        }
        match &self.confirm2 {
            Some(i) => string.push_str(&i.to_params_config("Confirm2")?),
            _ => (),
        }
        match &self.confirm3 {
            Some(i) => string.push_str(&i.to_params_config("Confirm3")?),
            _ => (),
        }
        match &self.cont {
            Some(i) => string.push_str(&i.to_params_config("Continue")?),
            _ => (),
        }
        match &self.exit {
            Some(i) => string.push_str(&i.to_params_config("Exit")?),
            _ => (),
        }
        match &self.baseline {
            Some(i) => string.push_str(&i.to_params_config("Baseline")?),
            _ => (),
        }
        match &self.volume {
            Some(i) => string.push_str(&i.to_params_config("Volume")?),
            _ => (),
        }

        Ok(string)
    }

    pub fn parse_result_set(&self, mut result_params: VecDeque<f32>) -> IndicatorSet {
        IndicatorSet {
            confirm: self
                .confirm
                .as_ref()
                .and_then(|i| Some(i.parse_result_set(&mut result_params))),
            confirm2: self
                .confirm2
                .as_ref()
                .and_then(|i| Some(i.parse_result_set(&mut result_params))),
            confirm3: self
                .confirm3
                .as_ref()
                .and_then(|i| Some(i.parse_result_set(&mut result_params))),
            exit: self
                .exit
                .as_ref()
                .and_then(|i| Some(i.parse_result_set(&mut result_params))),
            cont: self
                .cont
                .as_ref()
                .and_then(|i| Some(i.parse_result_set(&mut result_params))),
            baseline: self
                .baseline
                .as_ref()
                .and_then(|i| Some(i.parse_result_set(&mut result_params))),
            volume: self
                .volume
                .as_ref()
                .and_then(|i| Some(i.parse_result_set(&mut result_params))),
        }
    }
}

// TODO impl Iterator
// create a indi_list Vec<&Indicator>
// return indi_list.iter();

// input from the API
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RunParams {
    pub name: String,
    pub indi_set: IndicatorSet,
    pub date: (DateTime<Utc>, DateTime<Utc>),
    pub backtest_model: BacktestModel,
    pub optimize: OptimizeMode,
    pub optimize_crit: OptimizeCrit,
    pub visual: bool,
    // symbols : &[],
    pub symbols: Vec<String>,
}

impl RunParams {
    pub fn to_params_config(&self) -> Result<String> {
        let mut string = self.indi_set.to_params_config()?;
        for (i, symbol) in self.symbols.iter().enumerate() {
            string.push_str(&format!(
                "Expert_symbol{idx}={symbol}\n",
                symbol = symbol,
                idx = i,
            ));
        }
        // string.push_str(&format!("Expert_Symbols={}", self.symbols.join(" ")));
        debug!("Params config for terminal:\n{}", string);
        return Ok(string);
    }

    fn to_config(&self) -> String {
        format!(
            "
Visual={visual}
FromDate={from_date}
ToDate={to_date}
Model={model}
Optimization={opti}
OptimizationCriterion={opti_crit}",
            visual = self.visual as i32,
            from_date = DateTime::format(&self.date.0, "%Y.%m.%d"),
            to_date = DateTime::format(&self.date.1, "%Y.%m.%d"),
            model = self.backtest_model as u8,
            opti = self.optimize as u8,
            opti_crit = self.optimize_crit as u8
        )
    }

    // TODO remove new() function which sets too many defaults
    /* pub fn new() -> Self {
     *     RunParams {
     *         name: "backtest".to_string(),
     *         indi_set: IndicatorSet::default(),
     *         date: (
     *             DateTime::parse_from_rfc3339("2017-08-01").unwrap().into(),
     *             DateTime::parse_from_rfc3339("2019-08-20").unwrap().into(),
     *         ),
     *         backtest_model: BacktestModel::default(),
     *         optimize: OptimizeMode::default(),
     *         optimize_crit: OptimizeCrit::default(),
     *         visual: false,
     *         symbols: FOREX_PAIRS.iter().map(|s| s.to_string()).collect(),
     *         // to_vec().to_string(),
     *         // symbols_iter : symbols.iter()
     *     }
     * } */

    pub fn from_file(file: &str) -> Result<Self> {
        let json_file = File::open(Path::new(file))?;
        Ok(serde_json::from_reader(json_file)?)
    }

    pub fn to_file(&self, file: &str) -> Result<()> {
        let json_file = File::create(Path::new(file))?;
        Ok(serde_json::ser::to_writer_pretty(json_file, self)?)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.symbols.iter()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RunParamsFile {
    pub name: String,
    pub indi_set: IndicatorSetFile,
    // pub date: (String, String),
    pub date: (DateTime<Utc>, DateTime<Utc>),
    pub backtest_model: BacktestModel,
    pub optimize: OptimizeMode,
    pub optimize_crit: OptimizeCrit,
    pub visual: bool,
    pub symbols: Vec<String>,
}

// impl TryFrom<RunParamsFile> for RunParams {
impl From<RunParamsFile> for RunParams {
    // type Error = anyhow::Error;

    // fn try_from(s: RunParamsFile) -> Result<Self, Self::Error> {
    fn from(s: RunParamsFile) -> Self {
        // Ok(
        RunParams {
            name: s.name,
            indi_set: s.indi_set.into(),
            /* date: (
             *     DateTime::parse_from_rfc3339(&s.date.0)?.into(),
             *     DateTime::parse_from_rfc3339(&s.date.1)?.into(),
             * ), */
            date: s.date,
            backtest_model: s.backtest_model,
            optimize: s.optimize,
            optimize_crit: s.optimize_crit,
            visual: s.visual,
            symbols: s.symbols,
        }
        // )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IndicatorSetFile {
    pub confirm: Option<PathBuf>,
    pub confirm2: Option<PathBuf>,
    pub confirm3: Option<PathBuf>,
    pub exit: Option<PathBuf>,
    pub cont: Option<PathBuf>,
    pub baseline: Option<PathBuf>,
    pub volume: Option<PathBuf>,
}

impl From<IndicatorSetFile> for IndicatorSet {
    fn from(s: IndicatorSetFile) -> Self {
        IndicatorSet {
            confirm: s.confirm.map(|f| serde_any::from_file(f).unwrap()).into(),
            confirm2: s.confirm2.map(|f| serde_any::from_file(f).unwrap()).into(),
            confirm3: s
                .confirm3
                .map({ |f| serde_any::from_file(f).unwrap() })
                .into(),
            exit: s.exit.map({ |f| serde_any::from_file(f).unwrap() }).into(),
            cont: s.cont.map({ |f| serde_any::from_file(f).unwrap() }).into(),
            baseline: s
                .baseline
                .map({ |f| serde_any::from_file(f).unwrap() })
                .into(),
            volume: s
                .volume
                .map({ |f| serde_any::from_file(f).unwrap() })
                .into(),
        }
    }
}

// terminal execution specific configuration
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommonParams {
    pub params_file: String,
    pub wine: bool,
    pub terminal_exe: PathBuf,
    pub workdir: PathBuf,
    pub reports: PathBuf,
    pub expert: String,
    pub period: String,
    pub login: String,
    pub use_remote: bool,
    pub use_local: bool,
    pub replace_report: bool,
    pub shutdown_terminal: bool,
    pub deposit: u32,
    pub currency: String,
    pub leverage: u16,
    pub execution_mode: u8,
    // run_params : RunParams,
}

impl CommonParams {
    /* pub fn new(workdir: &Path) -> Self {
     *     CommonParams {
     *         params_file: "expert_params.set".to_string(),
     *         terminal_exe: PathBuf::from(r"C:\Program Files\MetaTrader 5\terminal64.exe"),
     *         workdir: workdir.to_path_buf(),
     *         reports: PathBuf::from("reports"),
     *         // expert : "nnfx-ea/nnfx-ea.ex5".to_string(),
     *         expert: r"expert\expert.ex5".to_string(),
     *         period: "D1".to_string(),
     *         login: "".to_string(),
     *         use_remote: true,
     *         use_local: true,
     *         replace_report: true,
     *         shutdown_terminal: true,
     *         deposit: 10000,
     *         currency: "USD".to_string(),
     *         leverage: 100,
     *         execution_mode: 0,
     *         // run_params : run,
     *     }
     * } */

    pub fn from_file(file: &str) -> Result<Self> {
        let json_file = File::open(Path::new(file))?;
        Ok(serde_json::from_reader(json_file)?)
    }

    pub fn to_file(&self, file: &str) -> Result<()> {
        let json_file = File::create(Path::new(file))?;
        Ok(serde_json::ser::to_writer_pretty(json_file, self)?)
    }

    pub fn reports_dir(mut self, reports_dir: &str) -> Self {
        self.reports = reports_dir.into();
        self
    }

    pub fn params_path(&self) -> PathBuf {
        let mut params_path = self.workdir.clone();
        params_path.push("MQL5/Profiles/Tester");
        params_path.push(&self.params_file);
        params_path
    }

    pub fn to_config(&self) -> String {
        format!(
            "
Expert={expert}
ExpertParameters={params_file}
Period={period}
Login={login}
UseLocal={use_local}
UseRemote={use_remote}
ReplaceReport={replace_report}
ShutdownTerminal={shutdown_terminal}
Deposit={deposit}
Currency={currency}
Leverage={leverage}
ExecutionMode={exec_mode}",
            expert = self.expert,
            params_file = self.params_file,
            period = self.period,
            login = self.login,
            use_local = self.use_local as i32,
            use_remote = self.use_remote as i32,
            replace_report = self.replace_report as i32,
            shutdown_terminal = self.shutdown_terminal as i32,
            deposit = self.deposit,
            currency = self.currency,
            leverage = self.leverage,
            exec_mode = self.execution_mode
        )
    }
}

pub fn to_terminal_config(common: &CommonParams, run: &RunParams) -> Result<String> {
    ensure!(! common.reports.is_absolute(), "reports path must be relative");
    // generate the reports path for the terminal.ini with windows-style "\"
    let reports_path_relative = common
        .reports
        .join(&run.name)
        .join("reports.xml")
        .iter()
        .filter_map(|s| s.to_str())
        .collect::<Vec<&str>>()
        .join("\\");
    Ok(format!(
        "[Common]
Login={login}
ProxyEnable=0
CertInstall=0
NewsEnable=0
[Tester]
{common}
{run}
Symbol={symb}
Report={report}",
        login = &common.login,
        common = common.to_config(),
        run = run.to_config(),
        symb = run
            .symbols
            .iter()
            .max_by(|x, y| x.cmp(y))
            // TODO take the alphanumerical. This causes the bar times to be correct
            // this is a very vage assumtion and needs to be double and tripple checked in the EA
            .context("sorting symbols failed")?,
        report = reports_path_relative
    ))
}

pub fn get_reports_dir(common: &CommonParams, run: &RunParams) -> Result<PathBuf> {
    ensure!(
        common.reports.is_relative(),
        "Reports path needs to be relative"
    );
    Ok(common.workdir.join(common.reports.join(&run.name)))
}

pub fn get_reports_path(common: &CommonParams, run: &RunParams) -> Result<PathBuf> {
    let reports_path = get_reports_dir(&common, &run)?.join("reports.xml");
    // reports_path.set_extension("xml");
    Ok(reports_path)
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BacktestModel {
    EveryTick = 0,     // "Every tick"
    OneMinuteOHLC = 1, // "1 minute OHLC"
    OpenPrice = 2,     // "Open price only"
    MathCalc = 3,      // "Math calculations"
    EveryTickReal = 4, // "Every tick based on real ticks"
}

// fn f(m: &)

impl Default for BacktestModel {
    fn default() -> Self {
        BacktestModel::OpenPrice
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OptimizeMode {
    Disabled = 0,   // optimization disabled
    Complete = 1,   // "Slow complete algorithm"
    Genetic = 2,    // "Fast genetic based algorithm"
    AllSymbols = 3, // "All symbols selected in Market Watch"
}

impl Default for OptimizeMode {
    fn default() -> Self {
        OptimizeMode::Complete
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OptimizeCrit {
    Balance = 0,         // the maximum balance value,
    BalanceProf = 1,     // the maximum value of product of the balance and profitability,
    BalancePayoff = 2,   // the product of the balance and expected payoff,
    Drawdown = 3,        // the maximum value of the expression (100% - Drawdown)*Balance,
    BalanceRecovery = 4, // the product of the balance and the recovery factor,
    BalanceSharpe = 5,   // the product of the balance and the Sharpe Ratio,
    Custom = 6, // a custom optimization criterion received from the OnTester() function in the Expert Advisor).
}

impl Default for OptimizeCrit {
    fn default() -> Self {
        OptimizeCrit::Custom
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn indicator_config_test() {
        let mut indi = Indicator {
            name: "ama".to_string(),
            shift: 0,
            inputs: Vec::new(),
        };
        assert_eq!(
            indi.to_params_config("Confirm").unwrap(),
            "Confirm_Indicator=ama\n"
        );

        indi.shift = 7;
        assert_eq!(
            indi.to_params_config("Confirm").unwrap(),
            "Confirm_Indicator=ama
Confirm_Shift=7
"
        );

        indi.inputs.push(vec![3.]);
        assert_eq!(
            indi.to_params_config("Confirm").unwrap(),
            "Confirm_Indicator=ama
Confirm_double0=3.00||0||0||0||N
Confirm_Shift=7
"
        );

        indi.inputs.push(vec![4.]);
        assert_eq!(
            indi.to_params_config("Confirm").unwrap(),
            "Confirm_Indicator=ama
Confirm_double0=3.00||0||0||0||N
Confirm_double1=4.00||0||0||0||N
Confirm_Shift=7
"
        );

        indi.inputs.push(vec![10., 200., 0.5]);
        assert_eq!(
            indi.to_params_config("Baseline").unwrap(),
            "Baseline_Indicator=ama
Baseline_double0=3.00||0||0||0||N
Baseline_double1=4.00||0||0||0||N
Baseline_double2=0||10.00||0.50||200.00||Y
Baseline_Shift=7
"
        );

        indi.inputs.push(vec![10., 0.5]);
        assert!(indi.to_params_config("Baseline").is_err());
    }

    /*     #[test]
     *     #[cfg(unix)]
     *     fn terminal_config_params_path_test() {
     *         let term_params = CommonParams {
     *             workdir: PathBuf::from(r"C:/workdir"),
     *             params_file: "test.set".to_string(),
     *             ..Default::default()
     *         };
     *         assert_eq!(
     *             term_params.params_path().as_path(),
     *             Path::new(r"C:/workdir/MQL5/Profiles/Tester/test.set")
     *         );
     *
     *         let term_params = CommonParams::new(Path::new(
     *             r"C:/Users/stele/AppData/Roaming/MetaQuotes/Terminal/D0E8209F77C8CF37AD8BF550E51FF075",
     *         ));
     *         assert_eq!(
     *             term_params.params_path().as_path(),
     *             Path::new(
     *                 r"C:/Users/stele/AppData/Roaming/MetaQuotes/Terminal/D0E8209F77C8CF37AD8BF550E51FF075/MQL5/Profiles/Tester/expert_params.set"
     *             )
     *         );
     *     } */

    #[test]
    #[cfg(unix)]
    fn reports_dir_test() {
        let mut common = CommonParams {
            params_file: "expert_params.set".to_string(),
            wine: false,
            terminal_exe: PathBuf::from(r"C:\terminal64.exe"),
            workdir: PathBuf::from(r"C:/workdir"),
            reports: PathBuf::from("reports"),
            expert: r"expert\expert.ex5".to_string(),
            period: "D1".to_string(),
            login: "1234".to_string(),
            use_remote: true,
            use_local: true,
            replace_report: true,
            shutdown_terminal: true,
            deposit: 10000,
            currency: "USD".to_string(),
            leverage: 100,
            execution_mode: 0,
        };

        let run = RunParams {
            name: "test".to_string(),
            indi_set: IndicatorSet {
                confirm: None,
                confirm2: None,
                confirm3: None,
                exit: None,
                cont: None,
                baseline: None,
                volume: None,
            },
            date: (
                DateTime::parse_from_rfc3339("2017-08-01T00:00:00-00:00")
                    .unwrap()
                    .into(),
                DateTime::parse_from_rfc3339("2019-08-20T00:00:00-00:00")
                    .unwrap()
                    .into(),
            ),
            backtest_model: BacktestModel::EveryTick,
            optimize: OptimizeMode::Complete,
            optimize_crit: OptimizeCrit::Custom,
            visual: false,
            symbols: vec!["USDCHF".to_string()],
        };

        assert_eq!(
            get_reports_dir(&common, &run).unwrap().as_path(),
            PathBuf::from(r"C:/workdir/reports/").join("test")
        );

        let mut reports_path = get_reports_dir(&common, &run).unwrap().join("USDCHF");
        reports_path.set_extension("xml");
        let reports_path = reports_path.as_os_str();

        assert_eq!(
            reports_path.to_string_lossy(),
            r"C:/workdir/reports/test/USDCHF.xml"
        );

        assert_eq!(
            (*get_reports_path(&common, &run).unwrap()).to_str(),
            Some(r"C:/workdir/reports/test/reports.xml")
        );

        common.workdir = PathBuf::from(r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5");
        assert_eq!(
            (*get_reports_path(&common, &run).unwrap()).to_str(),
            Some(r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5/reports/test/reports.xml")
        );

        common.reports = PathBuf::from(r"reports/inner");
        assert_eq!(
            (*get_reports_path(&common, &run).unwrap()).to_str(),
            Some(
                r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5/reports/inner/test/reports.xml"
            )
        );

        // FIXME paths are not platform agnostic
        // reports need configured correctly for the platform!
        // common.reports = PathBuf::from(r"reports\inner");
        // assert_eq!(
        //     (*get_reports_path(&common, &run).unwrap()).to_str(),
        //     Some(r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5/reports/inner/test/reports.xml")
        // );
    }

    /* #[test]
     * fn run_iter_test() {
     *     let mut run = RunParams::new();
     *     run.symbols = vec!["USDCHF", "USDJPY", "USDCAD"]
     *         .iter()
     *         .map(|s| s.to_string())
     *         .collect();
     *     let mut sym_iter = run.iter();
     *     assert_eq!(sym_iter.next().unwrap(), "USDCHF");
     *     assert_eq!(sym_iter.next().unwrap(), "USDJPY");
     * } */

    #[test]
    fn to_terminal_config_test() {
        let common = CommonParams {
            params_file: "expert_params.set".to_string(),
            wine: false,
            terminal_exe: PathBuf::from(r"C:\terminal64.exe"),
            workdir: PathBuf::from(r"C:\workdir"),
            reports: PathBuf::from("reports"),
            expert: r"expert\expert.ex5".to_string(),
            period: "D1".to_string(),
            login: "1234".to_string(),
            use_remote: true,
            use_local: true,
            replace_report: true,
            shutdown_terminal: true,
            deposit: 10000,
            currency: "USD".to_string(),
            leverage: 100,
            execution_mode: 0,
        };

        let run = RunParams {
            name: "test".to_string(),
            indi_set: IndicatorSet {
                confirm: Some(Indicator {
                    name: "ma".to_string(),
                    inputs: vec![vec![1.], vec![1., 100., 3.]],
                    shift: 0,
                }),
                confirm2: Some(Indicator {
                    name: "ma2".to_string(),
                    inputs: vec![vec![1.], vec![10., 200., 5.]],
                    shift: 1,
                }),
                confirm3: None,
                exit: Some(Indicator {
                    name: "exitor".to_string(),
                    inputs: vec![vec![14., 100., 3.], vec![1., 30., 2.]],
                    shift: 2,
                }),
                cont: None,
                baseline: Some(Indicator {
                    name: "Ichy".to_string(),
                    inputs: vec![vec![41.], vec![10.]],
                    shift: 0,
                }),
                volume: Some(Indicator {
                    name: "WAE".to_string(),
                    inputs: vec![vec![7.], vec![222.]],
                    shift: 0,
                }),
            },
            date: (
                DateTime::parse_from_rfc3339("2017-08-01T00:00:00-00:00")
                    .unwrap()
                    .into(),
                DateTime::parse_from_rfc3339("2019-08-20T00:00:00-00:00")
                    .unwrap()
                    .into(),
            ),
            backtest_model: BacktestModel::EveryTick,
            optimize: OptimizeMode::Complete,
            optimize_crit: OptimizeCrit::Custom,
            visual: false,
            symbols: vec!["USDCHF", "AUDCAD", "USDJPY", "USDCAD"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        };

        assert_eq!(
            to_terminal_config(&common, &run).unwrap(),
            r"[Common]
Login=1234
ProxyEnable=0
CertInstall=0
NewsEnable=0
[Tester]

Expert=expert\expert.ex5
ExpertParameters=expert_params.set
Period=D1
Login=1234
UseLocal=1
UseRemote=1
ReplaceReport=1
ShutdownTerminal=1
Deposit=10000
Currency=USD
Leverage=100
ExecutionMode=0

Visual=0
FromDate=2017.08.01
ToDate=2019.08.20
Model=0
Optimization=1
OptimizationCriterion=6
Symbol=USDJPY
Report=reports\test\reports.xml"
        );
    }

    #[test]
    fn json_test() {
        let workdir = Path::new(r"C:\workdir");
        let term_params = CommonParams {
            params_file: "expert_params.set".to_string(),
            wine: false,
            terminal_exe: PathBuf::from(r"C:\terminal64.exe"),
            workdir: workdir.to_path_buf(),
            reports: PathBuf::from("reports"),
            expert: r"expert\expert.ex5".to_string(),
            period: "D1".to_string(),
            login: "".to_string(),
            use_remote: true,
            use_local: true,
            replace_report: true,
            shutdown_terminal: true,
            deposit: 10000,
            currency: "USD".to_string(),
            leverage: 100,
            execution_mode: 0,
        };

        let j = r#"{"params_file":"expert_params.set",
                       "wine":false,
                       "terminal_exe":"C:\\terminal64.exe",
                       "workdir":"C:\\workdir",
                       "reports":"reports",
                       "expert":"expert\\expert.ex5",
                       "period":"D1",
                       "login":"",
                       "use_remote":true,
                       "use_local":true,
                       "replace_report":true,
                       "shutdown_terminal":true,
                       "deposit":10000,
                       "currency":"USD",
                       "leverage":100,
                       "execution_mode":0}
                "#;
        assert_eq!(term_params, serde_json::from_str(j).unwrap());

        let run = RunParams {
            name: "bt_run_name".to_string(),
            indi_set: IndicatorSet {
                confirm: Some(Indicator {
                    name: "ma".to_string(),
                    inputs: vec![vec![1.], vec![1., 100., 3.]],
                    shift: 0,
                }),
                confirm2: Some(Indicator {
                    name: "ma2".to_string(),
                    inputs: vec![vec![1.], vec![10., 200., 5.]],
                    shift: 1,
                }),
                confirm3: None,
                exit: Some(Indicator {
                    name: "exitor".to_string(),
                    inputs: vec![vec![14., 100., 3.], vec![1., 30., 2.]],
                    shift: 2,
                }),
                cont: None,
                baseline: Some(Indicator {
                    name: "Ichy".to_string(),
                    inputs: vec![vec![41.], vec![10.]],
                    shift: 0,
                }),
                volume: Some(Indicator {
                    name: "WAE".to_string(),
                    inputs: vec![vec![7.], vec![222.]],
                    shift: 0,
                }),
            },
            date: (
                DateTime::parse_from_rfc3339("2017-08-01T00:00:00-00:00")
                    .unwrap()
                    .into(),
                DateTime::parse_from_rfc3339("2019-08-20T00:00:00-00:00")
                    .unwrap()
                    .into(),
            ),
            backtest_model: BacktestModel::EveryTick,
            optimize: OptimizeMode::Complete,
            optimize_crit: OptimizeCrit::Custom,
            visual: false,
            symbols: vec!["EURUSD".to_string(), "AUDCAD".into()],
        };

        let run_string = r#"{"name":"bt_run_name",
            "indi_set":{"confirm":{"name":"ma","inputs":[[1.0],[1.0,100.0,3.0]],"shift":0},
            "confirm2":{"name":"ma2","inputs":[[1.0],[10.0,200.0,5.0]],"shift":1},
            "confirm3":null,
            "exit":{"name":"exitor","inputs":[[14.0,100.0,3.0],[1.0,30.0,2.0]],"shift":2},
            "cont":null,
            "baseline":{"name":"Ichy","inputs":[[41.0],[10.0]],"shift":0},
            "volume":{"name":"WAE","inputs":[[7.0],[222.0]],"shift":0}},
            "date":["2017-08-01T00:00:00-00:00","2019-08-20T00:00:00-00:00"],
            "backtest_model":0, "optimize":1,"optimize_crit":6,"visual":false,
            "symbols":["EURUSD","AUDCAD"]}"#;

        assert_eq!(run, serde_json::from_str(run_string).unwrap());

        let _ = serde_any::to_file("/tmp/confirm.yaml", &run.indi_set.confirm);
        let _ = serde_any::to_file("/tmp/confirm2.yaml", &run.indi_set.confirm2);
        let _ = serde_any::to_file("/tmp/baseline.yaml", &run.indi_set.baseline);
        let _ = serde_any::to_file("/tmp/exit.yaml", &run.indi_set.exit);
        let _ = serde_any::to_file("/tmp/volume.yaml", &run.indi_set.volume);

        assert_eq!(
            Some(serde_any::from_file::<Indicator, _>("/tmp/confirm.yaml").unwrap()),
            run.indi_set.confirm
        );

        let indi_set = IndicatorSetFile {
            confirm: Some("/tmp/confirm.yaml".into()),
            confirm2: Some("/tmp/confirm2.yaml".into()),
            confirm3: None,
            exit: Some("/tmp/exit.yaml".into()),
            cont: None,
            baseline: Some("/tmp/baseline.yaml".into()),
            volume: Some("/tmp/volume.yaml".into()),
        };
        assert_eq!(IndicatorSet::from(indi_set.clone()), run.indi_set);

        let run_cl = run.clone();
        let rpf = RunParamsFile {
            name: run_cl.name,
            indi_set: indi_set.into(),
            date: run_cl.date,
            backtest_model: run_cl.backtest_model,
            optimize: run_cl.optimize,
            optimize_crit: run_cl.optimize_crit,
            visual: run_cl.visual,
            symbols: run_cl.symbols,
        };

        let _ = serde_any::to_file("/tmp/run.yaml", &rpf);

        assert_eq!(RunParams::from(rpf), run);
    }

    /* #[test]
     * fn parse_from_results() {
     *     unimplemented!();
     *   // TODO test input list length
     *   // TODO test if output IndicatorSet has 0 range inputs
     *   // TODO test if param resul is in the range of the input range
     * } */
}
