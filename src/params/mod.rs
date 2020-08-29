use anyhow::{ensure, Context, Result};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::path::PathBuf;

pub mod common_params;
pub mod indi_func;
pub mod indicator;
pub mod indicator_set;
pub mod indicator_set_files;
pub mod run_params;
pub mod run_params_file;
pub mod signal_class;
pub mod to_param_string;

pub use common_params::CommonParams;
pub use indi_func::IndiFunc;
pub use indicator_set::IndicatorSet;
pub use run_params::RunParams;
pub use run_params_file::RunParamsFile;
pub use signal_class::SignalClass;
pub use to_param_string::ToParamString;

// const FOREX_PAIRS: &'static [&'static str] = &[
//     "EURUSD", "GBPUSD", "USDCHF", "USDJPY", "USDCAD", "AUDUSD", "EURCHF", "EURJPY", "EURGBP",
//     "EURCAD", "GBPCHF", "GBPJPY", "AUDJPY", "AUDNZD", "AUDCAD", "AUDCHF", "CHFJPY", "EURAUD",
//     "EURNZD", "CADCHF", "GBPAUD", "GBPCAD", "GBPNZD", "NZDCAD", "NZDCHF", "NZDJPY", "NZDUSD",
//     "CADJPY",
// ];

pub fn to_terminal_config(common: &CommonParams, run: &RunParams) -> Result<String> {
    ensure!(
        !common.reports.is_absolute(),
        "reports path must be relative"
    );
    // generate the reports path for the terminal.ini with windows-style "\"
    let reports_path_relative = common
        .reports
        .join(
            run.name.clone()
                + "_"
                + &run
                    .symbols
                    .iter()
                    .max_by(|x, y| x.cmp(y))
                    .expect("get highest symbol failed"),
        )
        .with_extension("xml")
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

pub fn get_reports_dir(common: &CommonParams) -> Result<PathBuf> {
    ensure!(
        common.reports.is_relative(),
        "Reports path needs to be relative"
    );
    Ok(common.workdir.join(&common.reports)) //.join(&run.name).with_extension("xml")))
}

pub fn get_reports_path(common: &CommonParams, run: &RunParams) -> Result<PathBuf> {
    let reports_path = get_reports_dir(&common)?
        .join(
            run.name.clone()
                + "_"
                + &run
                    .symbols
                    .iter()
                    .max_by(|x, y| x.cmp(y))
                    .expect("get highest symbol failed"),
        )
        .with_extension("xml");
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

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StoreResults {
    None = 0, // optimization disabled
    SideChanges = 1, // "Slow complete algorithm"
              // Buffers = 2,     // "Fast genetic based algorithm"
              // Results = 3,     // "All symbols selected in Market Watch"
}

impl Default for StoreResults {
    fn default() -> Self {
        StoreResults::None
    }
}

pub fn _vec_to_bigdecimal(vec: Vec<f32>) -> Vec<BigDecimal> {
    vec.iter().map(|v| (*v).into()).collect()
}

pub fn _vec_vec_to_bigdecimal(vec: Vec<Vec<f32>>) -> Vec<Vec<BigDecimal>> {
    vec.iter().map(|v| _vec_to_bigdecimal(v.to_vec())).collect()
}

#[cfg(test)]
mod test {
    use super::indi_func::IndiFunc;
    use super::indi_func::IndiFunc::*;
    use super::indicator::Indicator;
    use super::indicator_set::IndicatorSet;
    use super::signal_class::SignalClass::*;
    use super::*;
    use chrono::DateTime;
    use std::collections::HashMap;
    use std::path::Path;
    use std::str::FromStr;

    #[test]
    fn to_bigdecimal_test() {
        assert_eq!(
            _vec_to_bigdecimal(vec![3.]),
            vec![BigDecimal::from_str("3.").unwrap()]
        );
        assert_eq!(_vec_to_bigdecimal(vec![3.]), vec![BigDecimal::from(3.)]);
        assert_eq!(_vec_to_bigdecimal(vec![3.]), vec![(3.).into()]);
        assert_eq!(
            _vec_to_bigdecimal(vec![3., 5.8, 60f32]),
            vec![
                BigDecimal::from(3.),
                BigDecimal::from(5.8),
                BigDecimal::from(60)
            ]
        );
        assert_eq!(
            _vec_vec_to_bigdecimal(vec![vec![1.], vec![10., 200., 5.]]),
            vec![
                vec![BigDecimal::from(1.)],
                vec![
                    BigDecimal::from(10.),
                    BigDecimal::from(200.),
                    BigDecimal::from(5.)
                ]
            ]
        );
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
            indi_set: IndicatorSet::new(HashMap::new()),
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
            store_results: StoreResults::None,
        };

        assert_eq!(
            get_reports_dir(&common).unwrap().as_path(),
            PathBuf::from(r"C:/workdir/reports/")
        );

        assert_eq!(
            (*get_reports_path(&common, &run).unwrap()).to_str(),
            Some(r"C:/workdir/reports/test_USDCHF.xml")
        );

        common.workdir = PathBuf::from(r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5");
        assert_eq!(
            (*get_reports_path(&common, &run).unwrap()).to_str(),
            Some(r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5/reports/test_USDCHF.xml")
        );

        common.reports = PathBuf::from(r"reports/inner");
        assert_eq!(
            (*get_reports_path(&common, &run).unwrap()).to_str(),
            Some(
                r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5/reports/inner/test_USDCHF.xml"
            )
        );

        // TODO paths are not platform agnostic
        // reports need configured correctly for the platform!
        // common.reports = PathBuf::from(r"reports\inner");
        // assert_eq!(
        //     (*get_reports_path(&common, &run).unwrap()).to_str(),
        //     Some(r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5/reports/inner/test/reports.xml")
        // );
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
            symbols: vec!["EURUSD".to_string(), "AUDCAD".into()],
            store_results: StoreResults::SideChanges,
        };

        let run_string = r#"{
                "name": "bt_run_name",
                "indi_set": {
                    "Confirm": {
                        "name": "ma",
                        "class": "Preset",
                        "inputs": [[1.0], [1.0, 100.0, 3.0]],
                        "shift": 0
                    },
                    "Confirm2": {
                        "name": "ma2",
                        "class": "Preset",
                        "inputs": [[1.0], [10.0, 200.0, 5.0]],
                        "shift": 1
                    },
                    "Exit": {
                        "name": "exitor",
                        "class": "Preset",
                        "inputs": [
                            [14.0, 100.0, 3.0],
                            [1.0, 30.0, 2.0]
                        ],
                        "shift": 2
                    },
                    "Baseline": {
                        "name": "Ichy",
                        "inputs": [[41.0], [10.0]],
                        "shift": 0,
                        "class": "Preset"
                    },
                    "Volume": {
                        "name": "WAE",
                        "inputs": [[7.0], [222.0]],
                        "shift": 0,
                        "class": "Preset"
                    }
                },
                "date": ["2017-08-01T00:00:00-00:00", "2019-08-20T00:00:00-00:00"],
                "backtest_model": 0,
                "optimize": 1,
                "optimize_crit": 6,
                "store_results": 1,
                "visual": false,
                "symbols": ["EURUSD", "AUDCAD"]
            }"#;

        let des: RunParams = serde_json::from_str(run_string).unwrap();
        for (f, i) in run.indi_set.iter() {
            assert_eq!(des.indi_set.get(f), Some(i));
        }
        assert_eq!(run, des);

        let _ = serde_any::to_file("/tmp/confirm.yaml", &run.indi_set.get(&Confirm));
        let _ = serde_any::to_file("/tmp/confirm2.yaml", &run.indi_set.get(&Confirm2));
        let _ = serde_any::to_file("/tmp/baseline.yaml", &run.indi_set.get(&Baseline));
        let _ = serde_any::to_file("/tmp/exit.yaml", &run.indi_set.get(&Exit));
        let _ = serde_any::to_file("/tmp/volume.yaml", &run.indi_set.get(&Volume));

        assert_eq!(
            serde_any::from_file::<Indicator, _>("/tmp/confirm.yaml").unwrap(),
            *run.indi_set.get(&Confirm).unwrap()
        );

        let indi_set: HashMap<IndiFunc, PathBuf> = [
            (Confirm, "/tmp/confirm.yaml".into()),
            (Confirm2, "/tmp/confirm2.yaml".into()),
            (Exit, "/tmp/exit.yaml".into()),
            (Baseline, "/tmp/baseline.yaml".into()),
            (Volume, "/tmp/volume.yaml".into()),
        ]
        .iter()
        .cloned()
        .collect();

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
            store_results: run_cl.store_results,
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
