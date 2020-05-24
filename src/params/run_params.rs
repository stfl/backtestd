use chrono::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

use super::*;
use indicator_set::IndicatorSet;

// input from the API
// TODO derive Default -> requires date to impl Default
// this may be done when changing date to (NaivaDate, NaiveDate)
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RunParams {
    pub name: String,
    pub indi_set: IndicatorSet,
    pub date: (DateTime<Utc>, DateTime<Utc>),
    pub backtest_model: BacktestModel,
    pub optimize: OptimizeMode,
    pub optimize_crit: OptimizeCrit,
    pub visual: bool,
    pub symbols: Vec<String>,
    pub store_results: StoreResults,
}

impl ToParamString for RunParams {
    fn to_param_string(&self) -> String {
        let mut string = self.to_param_string_vec().join("\n");
        // string.push_str(&format!("Expert_Symbols={}", self.symbols.join(" ")));
        debug!("Params config for terminal:\n{}", string);
        return string;
    }
}

impl RunParams {
    pub fn to_param_string_vec(&self) -> Vec<String> {
        let mut strings = self.indi_set.to_param_string_vec();
        strings.extend(
            self.symbols.iter().enumerate().map(|(i, symbol)| {
                format!("Expert_symbol{idx}={symbol}", symbol = symbol, idx = i)
            }),
        );
        strings.push(format!(
            "Expert_Store_Results={}",
            store_res = self.store_results as u8
        ));
        strings
        // for (i, symbol) in self.symbols.iter().enumerate() {
        //     string.push_str(&format!(
        //         "Expert_symbol{idx}={symbol}\n",
        //         symbol = symbol,
        //         idx = i,
        //     ));
        // }
        // string.push_str(&format!("Expert_Symbols={}", self.symbols.join(" ")));
        // debug!("Params config for terminal:\n{}", string);
        // return string;
    }

    pub fn to_config(&self) -> String {
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
            opti_crit = self.optimize_crit as u8,
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

#[cfg(test)]
mod test {
    use super::indi_func::IndiFunc;
    use super::indi_func::IndiFunc::*;
    use super::indicator::Indicator;
    use super::indicator_set::IndicatorSet;
    use super::signal_class::SignalClass::*;
    use super::*;
    use std::collections::HashMap;
    use std::path::Path;

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
            indi_set: [
                (
                    Confirm,
                    Indicator {
                        name: "ma".to_string(),
                        filename: None,
                        shift: 0,
                        inputs: vec_vec_to_bigdecimal(vec![vec![1.], vec![1., 100., 3.]]),
                        buffers: None,
                        params: None,
                        class: Preset,
                    },
                ),
                (
                    Confirm2,
                    Indicator {
                        name: "ma2".to_string(),
                        filename: None,
                        inputs: vec_vec_to_bigdecimal(vec![vec![1.], vec![10., 200., 5.]]),
                        shift: 1,
                        buffers: None,
                        params: None,
                        class: Preset,
                    },
                ),
                (
                    Exit,
                    Indicator {
                        name: "exitor".to_string(),
                        filename: None,
                        inputs: vec_vec_to_bigdecimal(vec![vec![14., 100., 3.], vec![1., 30., 2.]]),
                        shift: 2,
                        buffers: None,
                        params: None,
                        class: Preset,
                    },
                ),
                (
                    Baseline,
                    Indicator {
                        name: "Ichy".to_string(),
                        filename: None,
                        inputs: vec_vec_to_bigdecimal(vec![vec![41.], vec![10.]]),
                        shift: 0,
                        buffers: None,
                        params: None,
                        class: Preset,
                    },
                ),
                (
                    Volume,
                    Indicator {
                        name: "WAE".to_string(),
                        filename: None,
                        inputs: vec_vec_to_bigdecimal(vec![vec![7.], vec![222.]]),
                        shift: 0,
                        buffers: None,
                        params: None,
                        class: Preset,
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<IndiFunc, Indicator>>()
            .into(),
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
            store_results: StoreResults::None,
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
Report=reports\test.xml"
        );
    }
}
