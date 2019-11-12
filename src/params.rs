#![allow(dead_code)]
#![allow(unused_variables)]

use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use serde_repr::{Deserialize_repr, Serialize_repr};

const FOREX_PAIRS: &'static [&'static str] = &[
    "EURUSD", "GBPUSD", "USDCHF", "USDJPY", "USDCAD", "AUDUSD", "EURCHF", "EURJPY", "EURGBP",
    "EURCAD", "GBPCHF", "GBPJPY", "AUDJPY", "AUDNZD", "AUDCAD", "AUDCHF", "CHFJPY", "EURAUD",
    "EURNZD", "CADCHF", "GBPAUD", "GBPCAD", "GBPNZD", "NZDCAD", "NZDCHF", "NZDJPY", "NZDUSD",
    "CADJPY",
];

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Indicator {
    name: String,
    indi_type: IndicatorType,
    inputs: Vec<Vec<f32>>,
    shift: u8,
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
                "{use_case}_Double{idx}=\
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
}

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IndicatorType {
    ZeroLineCross = 0,
    TwoLineCross = 1,
    OnChart = 2,
}

impl Default for IndicatorType {
    fn default() -> Self {
        IndicatorType::ZeroLineCross
    }
}

fn input_param_str(input: &Vec<f32>) -> Result<String> {
    match input.len() {
        1 => Ok(format!("{:.2}||0||0||0||N", input[0] as f32)),
        3 => Ok(format!(
            "0||{:.2}||{:.2}||{:.2}||Y",
            input[0] as f32, input[1] as f32, input[2] as f32
        )),
        e => Err(anyhow!("wrong length of input: {}", e)),
    }
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndicatorSet {
    confirm: Option<Indicator>,
    confirm2: Option<Indicator>,
    confirm3: Option<Indicator>,
    exit: Option<Indicator>,
    cont: Option<Indicator>,
    baseline: Option<Indicator>,
    volume: Option<Indicator>,
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
}

// input from the API
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunParams {
    pub name: String,
    pub indi_set: IndicatorSet,
    pub date: (String, String),
    pub backtest_model: BacktestModel,
    optimize: OptimizeMode,
    optimize_crit: OptimizeCrit,
    visual: bool,
    // symbols : &[],
    pub symbols: Vec<String>,
}

impl RunParams {
    pub fn to_params_config(&self) -> Result<String> {
        return Ok(self.indi_set.to_params_config()?);
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
            from_date = self.date.0,
            to_date = self.date.1,
            model = self.backtest_model as u8,
            opti = self.optimize as u8,
            opti_crit = self.optimize_crit as u8
        )
    }

    pub fn new() -> Self {
        RunParams {
            name: "backtest".to_string(),
            indi_set: IndicatorSet::default(),
            date: ("2017.08.01".to_string(), "2019.08.20".to_string()),
            backtest_model: BacktestModel::default(),
            optimize: OptimizeMode::default(),
            optimize_crit: OptimizeCrit::default(),
            visual: false,
            symbols: FOREX_PAIRS.iter().map(|s| s.to_string()).collect(),
            // to_vec().to_string(),
            // symbols_iter : symbols.iter()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.symbols.iter()
    }
}

/* impl Iterator for RunParams {
 *     type Item = String;
 *
 *     fn next(&mut self) -> Option<String> {
 *         let symbols_iter = self.symbols.iter();
 *         symbols_iter.next()
 *     }
 * } */

// terminal execution specific configuration
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonParams {
    pub params_file: String,
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
    pub fn new(workdir: &Path) -> Self {
        CommonParams {
            params_file: "expert_params.set".to_string(),
            terminal_exe: PathBuf::from(r"C:/Program Files/MetaTrader 5/terminal64.exe"),
            workdir: workdir.to_path_buf(),
            reports: workdir.join("reports"),
            // expert : "nnfx-ea/nnfx-ea.ex5".to_string(),
            expert: "expert/expert.ex5".to_string(),
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
            // run_params : run,
        }
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

pub fn to_terminal_config(common: &CommonParams, run: &RunParams, symbol: &String) -> String {
    let mut reports_path = get_reports_dir(common, run).join(symbol);
    reports_path.set_extension("xml");
    let reports_path = reports_path.as_os_str();
    format!(
        "[Tester]
{common}
{run}
Symbol={symb}
Report={report}",
        common = common.to_config(),
        run = run.to_config(),
        symb = symbol,
        report = reports_path.to_string_lossy()
    )
}

fn get_reports_dir(common: &CommonParams, run: &RunParams) -> PathBuf {
    common.reports.join(&run.name)
}

pub fn get_reports_path(common: &CommonParams, run: &RunParams, symbol: &String) -> PathBuf {
    let mut reports_path = get_reports_dir(&common, &run).join(symbol);
    reports_path.set_extension("xml");
    reports_path
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
            indi_type: IndicatorType::OnChart,
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
Confirm_Double0=3.00||0||0||0||N
Confirm_Shift=7
"
        );

        indi.inputs.push(vec![4.]);
        assert_eq!(
            indi.to_params_config("Confirm").unwrap(),
            "Confirm_Indicator=ama
Confirm_Double0=3.00||0||0||0||N
Confirm_Double1=4.00||0||0||0||N
Confirm_Shift=7
"
        );

        indi.inputs.push(vec![10., 200., 0.5]);
        assert_eq!(
            indi.to_params_config("Baseline").unwrap(),
            "Baseline_Indicator=ama
Baseline_Double0=3.00||0||0||0||N
Baseline_Double1=4.00||0||0||0||N
Baseline_Double2=0||10.00||200.00||0.50||Y
Baseline_Shift=7
"
        );

        indi.inputs.push(vec![10., 0.5]);
        assert!(indi.to_params_config("Baseline").is_err());
    }

    #[test]
    fn terminal_config_params_path_test() {
        let term_params = CommonParams {
            workdir: PathBuf::from(r"C:/workdir"),
            params_file: "test.set".to_string(),
            ..Default::default()
        };
        assert_eq!(
            term_params.params_path().as_path(),
            Path::new("C:/workdir/MQL5/Profiles/Tester/test.set")
        );

        let term_params = CommonParams::new(Path::new(
            "C:/Users/stele/AppData/Roaming/MetaQuotes/Terminal/D0E8209F77C8CF37AD8BF550E51FF075",
        ));
        assert_eq!(term_params.params_path().as_path(),
                    Path::new("C:/Users/stele/AppData/Roaming/MetaQuotes/Terminal/D0E8209F77C8CF37AD8BF550E51FF075/MQL5/Profiles/Tester/expert_params.set")
         );
    }

    #[test]
    fn reports_dir_test() {
        let common = CommonParams::new(Path::new("C:/workdir"));
        let mut run = RunParams::new();
        run.name = "test".to_string();
        assert_eq!(
            get_reports_dir(&common, &run).as_path(),
            PathBuf::from("C:/workdir/reports/").join("test")
        );

        let mut reports_path = get_reports_dir(&common, &run).join("USDCHF");
        reports_path.set_extension("xml");
        let reports_path = reports_path.as_os_str();

        assert_eq!(
            reports_path.to_string_lossy(),
            "C:/workdir/reports/test/USDCHF.xml"
        );

        assert_eq!(
            (*get_reports_path(&common, &run, &"USDCHF".to_string())).to_str(),
            Some("C:/workdir/reports/test/USDCHF.xml")
        );
    }

    #[test]
    fn run_iter_test() {
        let mut run = RunParams::new();
        run.symbols = vec!["USDCHF", "USDJPY", "USDCAD"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut sym_iter = run.iter();
        assert_eq!(sym_iter.next().unwrap(), "USDCHF");
        assert_eq!(sym_iter.next().unwrap(), "USDJPY");
    }

    #[test]
    fn to_terminal_config_test() {
        let common = CommonParams::new(Path::new("C:/workdir"));
        let mut run = RunParams::new();
        run.symbols = vec!["USDCHF", "USDJPY", "USDCAD"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        run.name = "test".to_string();
        let mut sym_iter = run.iter();

        assert_eq!(
            to_terminal_config(&common, &run, sym_iter.next().unwrap()),
            "[Tester]

Expert=expert/expert.ex5
ExpertParameters=expert_params.set
Period=D1
Login=
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
Model=2
Optimization=1
OptimizationCriterion=6
Symbol=USDCHF
Report=C:/workdir/reports/test/USDCHF.xml"
        );
    }

    #[test]
    fn json_test() {
        let workdir = Path::new("C:/workdir");
        let term_params = CommonParams {
            params_file: "expert_params.set".to_string(),
            terminal_exe: PathBuf::from(r"C:/terminal64.exe"),
            workdir: workdir.to_path_buf(),
            reports: workdir.join("reports"),
            expert: "expert/expert.ex5".to_string(),
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
                       "terminal_exe":"C:/terminal64.exe",
                       "workdir":"C:/workdir",
                       "reports":"C:/workdir/reports",
                       "expert":"expert/expert.ex5",
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
                    indi_type: IndicatorType::ZeroLineCross,
                    inputs: vec![vec![1.], vec![1., 100., 3.]],
                    shift: 0,
                }),
                confirm2: Some(Indicator {
                    name: "ma2".to_string(),
                    indi_type: IndicatorType::ZeroLineCross,
                    inputs: vec![vec![1.], vec![10., 200., 5.]],
                    shift: 1,
                }),
                confirm3: None,
                exit: Some(Indicator {
                    name: "exitor".to_string(),
                    indi_type: IndicatorType::TwoLineCross,
                    inputs: vec![vec![14., 100., 3.], vec![1., 30., 2.]],
                    shift: 2,
                }),
                cont: None,
                baseline: Some(Indicator {
                    name: "Ichy".to_string(),
                    indi_type: IndicatorType::OnChart,
                    inputs: vec![vec![41.], vec![10.]],
                    shift: 0,
                }),
                volume: Some(Indicator {
                    name: "WAE".to_string(),
                    indi_type: IndicatorType::ZeroLineCross,
                    inputs: vec![vec![7.], vec![222.]],
                    shift: 0,
                }),
            },
            date: ("2017.08.01".to_string(), "2019.08.20".to_string()),
            backtest_model: BacktestModel::EveryTick,
            optimize: OptimizeMode::Complete,
            optimize_crit: OptimizeCrit::Custom,
            visual: false,
            symbols: vec!["EURUSD".to_string(), "AUDCAD".into()],
        };

        let run_string = r#"{"name":"bt_run_name",
            "indi_set":{"confirm":{"name":"ma","indi_type":0,"inputs":[[1.0],[1.0,100.0,3.0]],"shift":0},
            "confirm2":{"name":"ma2","indi_type":0,"inputs":[[1.0],[10.0,200.0,5.0]],"shift":1},
            "confirm3":null,
            "exit":{"name":"exitor","indi_type":1,"inputs":[[14.0,100.0,3.0],[1.0,30.0,2.0]],"shift":2},
            "cont":null,
            "baseline":{"name":"Ichy","indi_type":2,"inputs":[[41.0],[10.0]],"shift":0},
            "volume":{"name":"WAE","indi_type":0,"inputs":[[7.0],[222.0]],"shift":0}},
            "date":["2017.08.01","2019.08.20"],
            "backtest_model":0, "optimize":1,"optimize_crit":6,"visual":false,
            "symbols":["EURUSD","AUDCAD"]}"#;

        assert_eq!(run, serde_json::from_str(run_string).unwrap());
        // println!("{}", serde_json::to_string(&run).unwrap());
    }
}
