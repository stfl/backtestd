use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context, Result};
use bigdecimal::BigDecimal;
use chrono::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

pub mod indicator;
pub mod indicator_inputs;
pub mod indicator_set;
pub mod run_params;
pub mod terminal_params;
pub mod to_param_string;
pub mod to_terminal_config;

pub mod legacy_indicator;

const FOREX_PAIRS: &'static [&'static str] = &[
    "EURUSD", "GBPUSD", "USDCHF", "USDJPY", "USDCAD", "AUDUSD", "EURCHF", "EURJPY", "EURGBP",
    "EURCAD", "GBPCHF", "GBPJPY", "AUDJPY", "AUDNZD", "AUDCAD", "AUDCHF", "CHFJPY", "EURAUD",
    "EURNZD", "CADCHF", "GBPAUD", "GBPCAD", "GBPNZD", "NZDCAD", "NZDCHF", "NZDJPY", "NZDUSD",
    "CADJPY",
];

// #[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
// pub struct IndicatorSet {
//     pub confirm: Option<Indicator>,
//     pub confirm2: Option<Indicator>,
//     pub confirm3: Option<Indicator>,
//     pub exit: Option<Indicator>,
//     pub cont: Option<Indicator>,
//     pub baseline: Option<Indicator>,
//     pub volume: Option<Indicator>,
// }

// impl IndicatorSet {
//     fn to_params_config(&self) -> Result<String> {
//         let mut string = String::new();
//         match &self.confirm {
//             Some(i) => string.push_str(&i.to_params_config("Confirm")?),
//             _ => (),
//         }
//         match &self.confirm2 {
//             Some(i) => string.push_str(&i.to_params_config("Confirm2")?),
//             _ => (),
//         }
//         match &self.confirm3 {
//             Some(i) => string.push_str(&i.to_params_config("Confirm3")?),
//             _ => (),
//         }
//         match &self.cont {
//             Some(i) => string.push_str(&i.to_params_config("Continue")?),
//             _ => (),
//         }
//         match &self.exit {
//             Some(i) => string.push_str(&i.to_params_config("Exit")?),
//             _ => (),
//         }
//         match &self.baseline {
//             Some(i) => string.push_str(&i.to_params_config("Baseline")?),
//             _ => (),
//         }
//         match &self.volume {
//             Some(i) => string.push_str(&i.to_params_config("Volume")?),
//             _ => (),
//         }

//         Ok(string)
//     }

//     // pub fn parse_result_set(&self, mut result_params: VecDeque<f32>) -> IndicatorSet {
//     //     IndicatorSet {
//     //         confirm: self
//     //             .confirm
//     //             .as_ref()
//     //             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//     //         confirm2: self
//     //             .confirm2
//     //             .as_ref()
//     //             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//     //         confirm3: self
//     //             .confirm3
//     //             .as_ref()
//     //             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//     //         exit: self
//     //             .exit
//     //             .as_ref()
//     //             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//     //         cont: self
//     //             .cont
//     //             .as_ref()
//     //             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//     //         baseline: self
//     //             .baseline
//     //             .as_ref()
//     //             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//     //         volume: self
//     //             .volume
//     //             .as_ref()
//     //             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//     //     }
//     // }
// }

// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
// pub struct IndicatorSetFile {
//     pub confirm: Option<PathBuf>,
//     pub confirm2: Option<PathBuf>,
//     pub confirm3: Option<PathBuf>,
//     pub exit: Option<PathBuf>,
//     pub cont: Option<PathBuf>,
//     pub baseline: Option<PathBuf>,
//     pub volume: Option<PathBuf>,
// }

// impl From<IndicatorSetFile> for IndicatorSet {
//     fn from(s: IndicatorSetFile) -> Self {
//         IndicatorSet {
//             confirm: s.confirm.map(|f| serde_any::from_file(f).unwrap()).into(),
//             confirm2: s.confirm2.map(|f| serde_any::from_file(f).unwrap()).into(),
//             confirm3: s
//                 .confirm3
//                 .map({ |f| serde_any::from_file(f).unwrap() })
//                 .into(),
//             exit: s.exit.map({ |f| serde_any::from_file(f).unwrap() }).into(),
//             cont: s.cont.map({ |f| serde_any::from_file(f).unwrap() }).into(),
//             baseline: s
//                 .baseline
//                 .map({ |f| serde_any::from_file(f).unwrap() })
//                 .into(),
//             volume: s
//                 .volume
//                 .map({ |f| serde_any::from_file(f).unwrap() })
//                 .into(),
//         }
//     }
// }

pub fn vec_to_bigdecimal(vec: Vec<f32>) -> Vec<BigDecimal> {
    vec.iter().map(|v| (*v).into()).collect()
}

pub fn vec_vec_to_bigdecimal(vec: Vec<Vec<f32>>) -> Vec<Vec<BigDecimal>> {
    vec.iter().map(|v| vec_to_bigdecimal(v.to_vec())).collect()
}

#[cfg(test)]
mod test {
    use super::indicator::*;
    use super::run_params::*;
    // use super::set_indicator::*;
    use super::terminal_params::*;
    use super::*;
    use std::path::Path;

    #[test]
    fn to_bigdecimal_test() {
        assert_eq!(
            vec_to_bigdecimal(vec![3.]),
            vec![BigDecimal::from_str("3.").unwrap()]
        );
        assert_eq!(vec_to_bigdecimal(vec![3.]), vec![BigDecimal::from(3.)]);
        assert_eq!(vec_to_bigdecimal(vec![3.]), vec![(3.).into()]);
        assert_eq!(
            vec_to_bigdecimal(vec![3., 5.8, 60f32]),
            vec![
                BigDecimal::from(3.),
                BigDecimal::from(5.8),
                BigDecimal::from(60)
            ]
        );
        assert_eq!(
            vec_vec_to_bigdecimal(vec![vec![1.], vec![10., 200., 5.]]),
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

    #[test]
    fn indicator_config_test() {
        use crate::database;
        use database::indicator::Indicator as DbIndicator;
        use database::indicator::IndicatorInput;
        use database::IndiFunc::*;
        use database::Indicator;
        use database::IndicatorInputs;
        use database::SignalClass::*;

        let mut indi = Indicator {
            indi: DbIndicator {
                indicator_id: 12,
                parent_id: None,
                child_id: None,
                indicator_name: "ama".to_string(),
                shift: 0,
                func: Confirm,
                class: None,
                filename: None,
                buffers: None,
                config: None,
            },
            inputs: IndicatorInputs::from(vec![]),
        };

        assert_eq!(
            indi.to_param_string(None),
            "Confirm_Indicator=ama
Confirm_SignalClass=Preset
Confirm_Shift=0
"
        );

        indi.indi.shift = 7;
        assert_eq!(
            indi.to_param_string(None),
            "Confirm_Indicator=ama
Confirm_SignalClass=Preset
Confirm_Shift=7
"
        );

        indi.inputs.push(IndicatorInput {
            indicator_id: 12,
            index: 0,
            input: Some(4.into()),
            start: None,
            stop: None,
            step: None,
        });

        assert_eq!(
            indi.to_param_string(None),
            "Confirm_Indicator=ama
Confirm_SignalClass=Preset
Confirm_Shift=7
Confirm_double0=4.00||0.00||0.00||0.00||N"
        );

        //         indi.inputs.push(vec_to_bigdecimal(vec![3.]));
        //         assert_eq!(
        //             indi.to_params_config("Confirm").unwrap(),
        //             "Confirm_Indicator=ama
        // Confirm_double0=3.00||0||0||0||N
        // Confirm_Shift=7
        // "
        //         );

        //         indi.inputs.push(vec_to_bigdecimal(vec![4.]));
        //         assert_eq!(
        //             indi.to_params_config("Confirm").unwrap(),
        //             "Confirm_Indicator=ama
        // Confirm_double0=3.00||0||0||0||N
        // Confirm_double1=4.00||0||0||0||N
        // Confirm_Shift=7
        // "
        //         );

        //         indi.inputs.push(vec_to_bigdecimal(vec![10., 200., 0.5]));
        //         assert_eq!(
        //             indi.to_params_config("Baseline").unwrap(),
        //             "Baseline_Indicator=ama
        // Baseline_double0=3.00||0||0||0||N
        // Baseline_double1=4.00||0||0||0||N
        // Baseline_double2=0||10.00||0.50||200.00||Y
        // Baseline_Shift=7
        // "
        //         );

        //         indi.inputs.push(vec_to_bigdecimal(vec![10., 0.5]));
        //         assert!(indi.to_params_config("Baseline").is_err());
    }

    // FIXME
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

    // #[test]
    // #[cfg(unix)]
    // fn reports_dir_test() {
    //     let mut common = CommonParams {
    //         params_file: "expert_params.set".to_string(),
    //         wine: false,
    //         terminal_exe: PathBuf::from(r"C:\terminal64.exe"),
    //         workdir: PathBuf::from(r"C:/workdir"),
    //         reports: PathBuf::from("reports"),
    //         expert: r"expert\expert.ex5".to_string(),
    //         period: "D1".to_string(),
    //         login: "1234".to_string(),
    //         use_remote: true,
    //         use_local: true,
    //         replace_report: true,
    //         shutdown_terminal: true,
    //         deposit: 10000,
    //         currency: "USD".to_string(),
    //         leverage: 100,
    //         execution_mode: 0,
    //     };

    //     let run = RunParams {
    //         name: "test".to_string(),
    //         indi_set: IndicatorSet {
    //             confirm: None,
    //             confirm2: None,
    //             confirm3: None,
    //             exit: None,
    //             cont: None,
    //             baseline: None,
    //             volume: None,
    //         },
    //         date: (
    //             DateTime::parse_from_rfc3339("2017-08-01T00:00:00-00:00")
    //                 .unwrap()
    //                 .into(),
    //             DateTime::parse_from_rfc3339("2019-08-20T00:00:00-00:00")
    //                 .unwrap()
    //                 .into(),
    //         ),
    //         backtest_model: BacktestModel::EveryTick,
    //         optimize: OptimizeMode::Complete,
    //         optimize_crit: OptimizeCrit::Custom,
    //         visual: false,
    //         symbols: vec!["USDCHF".to_string()],
    //     };

    //     assert_eq!(
    //         get_reports_dir(&common, &run).unwrap().as_path(),
    //         PathBuf::from(r"C:/workdir/reports/")
    //     );

    //     let reports_path = get_reports_dir(&common, &run)
    //         .unwrap()
    //         .join(&run.name)
    //         .with_extension("xml");
    //     // reports_path.set_extension("xml");
    //     let reports_path = reports_path.as_os_str();

    //     assert_eq!(
    //         reports_path.to_string_lossy(),
    //         r"C:/workdir/reports/test.xml"
    //     );

    //     assert_eq!(
    //         (*get_reports_path(&common, &run).unwrap()).to_str(),
    //         Some(r"C:/workdir/reports/test.xml")
    //     );

    //     common.workdir = PathBuf::from(r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5");
    //     assert_eq!(
    //         (*get_reports_path(&common, &run).unwrap()).to_str(),
    //         Some(r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5/reports/test.xml")
    //     );

    //     common.reports = PathBuf::from(r"reports/inner");
    //     assert_eq!(
    //         (*get_reports_path(&common, &run).unwrap()).to_str(),
    //         Some(r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5/reports/inner/test.xml")
    //     );

    //     // FIXME paths are not platform agnostic
    //     // reports need configured correctly for the platform!
    //     // common.reports = PathBuf::from(r"reports\inner");
    //     // assert_eq!(
    //     //     (*get_reports_path(&common, &run).unwrap()).to_str(),
    //     //     Some(r"/home/stefan/.wine/drive_c/Program Files/MetaTrader 5/reports/inner/test/reports.xml")
    //     // );
    // }

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

    // FIXME
    //     #[test]
    //     fn to_terminal_config_test() {
    //         let common = CommonParams {
    //             params_file: "expert_params.set".to_string(),
    //             wine: false,
    //             terminal_exe: PathBuf::from(r"C:\terminal64.exe"),
    //             workdir: PathBuf::from(r"C:\workdir"),
    //             reports: PathBuf::from("reports"),
    //             expert: r"expert\expert.ex5".to_string(),
    //             period: "D1".to_string(),
    //             login: "1234".to_string(),
    //             use_remote: true,
    //             use_local: true,
    //             replace_report: true,
    //             shutdown_terminal: true,
    //             deposit: 10000,
    //             currency: "USD".to_string(),
    //             leverage: 100,
    //             execution_mode: 0,
    //         };

    //         let run = RunParams {
    //             name: "test".to_string(),
    //             indi_set: IndicatorSet {
    //                 confirm: Some(Indicator {
    //                     name: "ma".to_string(),
    //                     inputs: vec_vec_to_bigdecimal(vec![vec![1.], vec![1., 100., 3.]]),
    //                     shift: 0,
    //                 }),
    //                 confirm2: Some(Indicator {
    //                     name: "ma2".to_string(),
    //                     inputs: vec_vec_to_bigdecimal(vec![vec![1.], vec![10., 200., 5.]]),
    //                     shift: 1,
    //                 }),
    //                 confirm3: None,
    //                 exit: Some(Indicator {
    //                     name: "exitor".to_string(),
    //                     inputs: vec_vec_to_bigdecimal(vec![vec![14., 100., 3.], vec![1., 30., 2.]]),
    //                     shift: 2,
    //                 }),
    //                 cont: None,
    //                 baseline: Some(Indicator {
    //                     name: "Ichy".to_string(),
    //                     inputs: vec_vec_to_bigdecimal(vec![vec![41.], vec![10.]]),
    //                     shift: 0,
    //                 }),
    //                 volume: Some(Indicator {
    //                     name: "WAE".to_string(),
    //                     inputs: vec_vec_to_bigdecimal(vec![vec![7.], vec![222.]]),
    //                     shift: 0,
    //                 }),
    //             },
    //             date: (
    //                 DateTime::parse_from_rfc3339("2017-08-01T00:00:00-00:00")
    //                     .unwrap()
    //                     .into(),
    //                 DateTime::parse_from_rfc3339("2019-08-20T00:00:00-00:00")
    //                     .unwrap()
    //                     .into(),
    //             ),
    //             backtest_model: BacktestModel::EveryTick,
    //             optimize: OptimizeMode::Complete,
    //             optimize_crit: OptimizeCrit::Custom,
    //             visual: false,
    //             symbols: vec!["USDCHF", "AUDCAD", "USDJPY", "USDCAD"]
    //                 .iter()
    //                 .map(|s| s.to_string())
    //                 .collect(),
    //         };

    //         assert_eq!(
    //             to_terminal_config(&common, &run).unwrap(),
    //             r"[Common]
    // Login=1234
    // ProxyEnable=0
    // CertInstall=0
    // NewsEnable=0
    // [Tester]

    // Expert=expert\expert.ex5
    // ExpertParameters=expert_params.set
    // Period=D1
    // Login=1234
    // UseLocal=1
    // UseRemote=1
    // ReplaceReport=1
    // ShutdownTerminal=1
    // Deposit=10000
    // Currency=USD
    // Leverage=100
    // ExecutionMode=0

    // Visual=0
    // FromDate=2017.08.01
    // ToDate=2019.08.20
    // Model=0
    // Optimization=1
    // OptimizationCriterion=6
    // Symbol=USDJPY
    // Report=reports\test.xml"
    //         );
    //     }

    // FIXME
    // #[test]
    // fn json_test() {
    //     let workdir = Path::new(r"C:\workdir");
    //     let term_params = CommonParams {
    //         params_file: "expert_params.set".to_string(),
    //         wine: false,
    //         terminal_exe: PathBuf::from(r"C:\terminal64.exe"),
    //         workdir: workdir.to_path_buf(),
    //         reports: PathBuf::from("reports"),
    //         expert: r"expert\expert.ex5".to_string(),
    //         period: "D1".to_string(),
    //         login: "".to_string(),
    //         use_remote: true,
    //         use_local: true,
    //         replace_report: true,
    //         shutdown_terminal: true,
    //         deposit: 10000,
    //         currency: "USD".to_string(),
    //         leverage: 100,
    //         execution_mode: 0,
    //     };

    //     let j = r#"{"params_file":"expert_params.set",
    //                    "wine":false,
    //                    "terminal_exe":"C:\\terminal64.exe",
    //                    "workdir":"C:\\workdir",
    //                    "reports":"reports",
    //                    "expert":"expert\\expert.ex5",
    //                    "period":"D1",
    //                    "login":"",
    //                    "use_remote":true,
    //                    "use_local":true,
    //                    "replace_report":true,
    //                    "shutdown_terminal":true,
    //                    "deposit":10000,
    //                    "currency":"USD",
    //                    "leverage":100,
    //                    "execution_mode":0}
    //             "#;
    //     assert_eq!(term_params, serde_json::from_str(j).unwrap());

    //     let run = RunParams {
    //         name: "bt_run_name".to_string(),
    //         indi_set: IndicatorSet {
    //             confirm: Some(Indicator {
    //                 name: "ma".to_string(),
    //                 inputs: vec_vec_to_bigdecimal(vec![vec![1.], vec![1., 100., 3.]]),
    //                 shift: 0,
    //             }),
    //             confirm2: Some(Indicator {
    //                 name: "ma2".to_string(),
    //                 inputs: vec_vec_to_bigdecimal(vec![vec![1.], vec![10., 200., 5.]]),
    //                 shift: 1,
    //             }),
    //             confirm3: None,
    //             exit: Some(Indicator {
    //                 name: "exitor".to_string(),
    //                 inputs: vec_vec_to_bigdecimal(vec![vec![14., 100., 3.], vec![1., 30., 2.]]),
    //                 shift: 2,
    //             }),
    //             cont: None,
    //             baseline: Some(Indicator {
    //                 name: "Ichy".to_string(),
    //                 inputs: vec_vec_to_bigdecimal(vec![vec![41.], vec![10.]]),
    //                 shift: 0,
    //             }),
    //             volume: Some(Indicator {
    //                 name: "WAE".to_string(),
    //                 inputs: vec_vec_to_bigdecimal(vec![vec![7.], vec![222.]]),
    //                 shift: 0,
    //             }),
    //         },
    //         date: (
    //             DateTime::parse_from_rfc3339("2017-08-01T00:00:00-00:00")
    //                 .unwrap()
    //                 .into(),
    //             DateTime::parse_from_rfc3339("2019-08-20T00:00:00-00:00")
    //                 .unwrap()
    //                 .into(),
    //         ),
    //         backtest_model: BacktestModel::EveryTick,
    //         optimize: OptimizeMode::Complete,
    //         optimize_crit: OptimizeCrit::Custom,
    //         visual: false,
    //         symbols: vec!["EURUSD".to_string(), "AUDCAD".into()],
    //     };

    //     let run_string = r#"{"name":"bt_run_name",
    //         "indi_set":{"confirm":{"name":"ma","inputs":[[1.0],[1.0,100.0,3.0]],"shift":0},
    //         "confirm2":{"name":"ma2","inputs":[[1.0],[10.0,200.0,5.0]],"shift":1},
    //         "confirm3":null,
    //         "exit":{"name":"exitor","inputs":[[14.0,100.0,3.0],[1.0,30.0,2.0]],"shift":2},
    //         "cont":null,
    //         "baseline":{"name":"Ichy","inputs":[[41.0],[10.0]],"shift":0},
    //         "volume":{"name":"WAE","inputs":[[7.0],[222.0]],"shift":0}},
    //         "date":["2017-08-01T00:00:00-00:00","2019-08-20T00:00:00-00:00"],
    //         "backtest_model":0, "optimize":1,"optimize_crit":6,"visual":false,
    //         "symbols":["EURUSD","AUDCAD"]}"#;

    //     assert_eq!(run, serde_json::from_str(run_string).unwrap());

    //     let _ = serde_any::to_file("/tmp/confirm.yaml", &run.indi_set.confirm);
    //     let _ = serde_any::to_file("/tmp/confirm2.yaml", &run.indi_set.confirm2);
    //     let _ = serde_any::to_file("/tmp/baseline.yaml", &run.indi_set.baseline);
    //     let _ = serde_any::to_file("/tmp/exit.yaml", &run.indi_set.exit);
    //     let _ = serde_any::to_file("/tmp/volume.yaml", &run.indi_set.volume);

    //     assert_eq!(
    //         Some(serde_any::from_file::<Indicator, _>("/tmp/confirm.yaml").unwrap()),
    //         run.indi_set.confirm
    //     );

    //     let indi_set = IndicatorSetFile {
    //         confirm: Some("/tmp/confirm.yaml".into()),
    //         confirm2: Some("/tmp/confirm2.yaml".into()),
    //         confirm3: None,
    //         exit: Some("/tmp/exit.yaml".into()),
    //         cont: None,
    //         baseline: Some("/tmp/baseline.yaml".into()),
    //         volume: Some("/tmp/volume.yaml".into()),
    //     };
    //     assert_eq!(IndicatorSet::from(indi_set.clone()), run.indi_set);

    //     let run_cl = run.clone();
    //     let rpf = RunParamsFile {
    //         name: run_cl.name,
    //         indi_set: indi_set.into(),
    //         date: run_cl.date,
    //         backtest_model: run_cl.backtest_model,
    //         optimize: run_cl.optimize,
    //         optimize_crit: run_cl.optimize_crit,
    //         visual: run_cl.visual,
    //         symbols: run_cl.symbols,
    //     };

    //     let _ = serde_any::to_file("/tmp/run.yaml", &rpf);

    //     assert_eq!(RunParams::from(rpf), run);
    // }

    /* #[test]
     * fn parse_from_results() {
     *     unimplemented!();
     *   // TODO test input list length
     *   // TODO test if output IndicatorSet has 0 range inputs
     *   // TODO test if param resul is in the range of the input range
     * } */
}
