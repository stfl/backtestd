use super::*;
use chrono::prelude::*;
use chrono::DateTime;
use indicator_set::IndicatorSet;
use serde::{Deserialize, Serialize};

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
    pub symbols: Vec<String>,
    pub store_results: StoreResults,
}

impl ToParamString for RunParams {
    fn to_param_string(&self) -> String {
        let string = self.to_param_string_vec().join("\n");
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
        strings.push(format!("Expert_Store_Results={}", self.store_results as u8));
        strings.push(format!("Expert_Title={}", self.name));
        strings
    }

    pub fn get_reports_filename(&self) -> PathBuf {
        PathBuf::from(
            self.name.clone()
                + "_"
                + self
                    .symbols
                    .iter()
                    .max_by(|x, y| x.cmp(y))
                    .expect("cannot determine the alpahnumerical first symbol"),
        )
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

    pub fn split_run_into_queue(self) -> Vec<Self> {
        let run = self;
        let optimize = run.optimize;
        let mut runs = match optimize {
            OptimizeMode::Complete => run.split_too_many_runs(),
            _ => vec![run],
        };

        if optimize != OptimizeMode::Genetic {
            // create a vec of new runs with only a single Symbol
            // if we test in Genetic mode use all Symbols
            runs = runs
                .into_iter()
                .flat_map(|r| r.split_per_symbol())
                .collect();
        }

        info!(
            "{} Runs in the queue:\n{}",
            runs.len(),
            runs.clone()
                .into_iter()
                .map(|r| format!("{}: {}", r.name, r.symbols.join(" ")))
                .collect::<Vec<String>>()
                .join("\n")
        );

        runs
    }

    fn split_too_many_runs(self) -> Vec<Self> {
        let runs: Vec<RunParams>;
        let run = self;
        // MT5 forces genetic optimization if there are more than 100M possibilities
        let new_sets = run.clone().indi_set.slice_recursive(100_000_000); // TODO implement slice_recursive on &self to not move indi_set out of run

        if new_sets.len() > 1 {
            runs = new_sets
                .into_iter()
                .enumerate()
                .map(|(i, s)| {
                    let mut r = run.clone();
                    r.indi_set = s;
                    r.name = format!("{}_{}", r.name, i);
                    r
                })
                .collect::<Vec<RunParams>>();
        } else {
            runs = vec![run];
        }
        runs
    }

    fn split_per_symbol(self) -> Vec<Self> {
        let r = self;
        if r.indi_set.count_inputs_crossed() > crate::RUN_LIMIT_MULTI_CURRENCY {
            r.symbols
                .iter()
                .map(|s| {
                    let mut rr = r.clone();
                    rr.symbols = vec![s.into()];
                    debug!("creating a separate run for {}", s);
                    // keep the same name -> all Symbols are stored to the same sqlite db as individual table
                    // rr.name = format!("{}_{}", r.name, s);
                    rr
                })
                .collect::<Vec<RunParams>>()
        } else {
            vec![r]
        }
    }

    pub fn _new_test(num: usize) -> Self {
        RunParams {
            name: "test".to_string(),
            date: (
                DateTime::parse_from_rfc3339("2017-01-01T00:00:00-00:00")
                    .unwrap()
                    .into(),
                DateTime::parse_from_rfc3339("2019-01-01T00:00:00-00:00")
                    .unwrap()
                    .into(),
            ),
            backtest_model: BacktestModel::EveryTick,
            optimize: OptimizeMode::Complete,
            optimize_crit: OptimizeCrit::Custom,
            visual: false,
            symbols: vec!["USDCHF", "NZDAUD"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            store_results: StoreResults::None,
            indi_set: IndicatorSet::_new_test(num),
        }
    }
}

#[cfg(test)]
mod test {
    use super::indi_func::IndiFunc;
    use super::indi_func::IndiFunc::*;
    use super::indicator::Indicator;
    use super::signal_class::SignalClass::*;
    use super::*;
    use crate::params::_vec_vec_to_bigdecimal;
    use std::collections::HashMap;

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
                        inputs: _vec_vec_to_bigdecimal(vec![vec![1.], vec![1., 100., 3.]]),
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
                        inputs: _vec_vec_to_bigdecimal(vec![vec![1.], vec![10., 200., 5.]]),
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
                        inputs: _vec_vec_to_bigdecimal(vec![
                            vec![14., 100., 3.],
                            vec![1., 30., 2.],
                        ]),
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
                        inputs: _vec_vec_to_bigdecimal(vec![vec![41.], vec![10.]]),
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
                        inputs: _vec_vec_to_bigdecimal(vec![vec![7.], vec![222.]]),
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
Report=reports\test_USDJPY.xml"
        );
    }
}
