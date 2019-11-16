// #![warn(rust_2018_idioms)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate anyhow;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod backtest_runner;
mod params;
mod xml_reader;

use backtest_runner::*;
use params::*;

// use std::future::Future;

use std::path::{Path, PathBuf};

// use tokio::io::AsyncWriteExt;

// #[tokio::main]
fn main() {
    pretty_env_logger::init();

    let workdir = Path::new(
        r#"C:\Users\stele\AppData\Roaming\MetaQuotes\Terminal\D0E8209F77C8CF37AD8BF550E51FF075"#,
    );
    let common = CommonParams {
        params_file: "expert_params.set".to_string(),
        terminal_exe: PathBuf::from(r"C:\Program Files\MetaTrader 5\terminal64.exe"),
        workdir: workdir.to_path_buf(),
        reports: PathBuf::from("reports"),
        expert: r"BacktestExpert\nnfx-ea\nnfx-ea.ex5".to_string(),
        period: "D1".to_string(),
        login: "5152".to_string(),
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
        name: "test1".to_string(),
        indi_set: IndicatorSet {
            confirm: Some(Indicator {
                name: "asctrend".to_string(),
                indi_type: IndicatorType::ZeroLineCross,
                inputs: vec![vec![1., 20., 1.]],
                shift: 0,
            }),
            confirm2: None,
            confirm3: None,
            exit: None,
            cont: None,
            baseline: None,
            volume: Some(Indicator {
                name: "wae".to_string(),
                indi_type: IndicatorType::TwoLinesCross,
                inputs: vec![
                    vec![20.],
                    vec![40.],
                    vec![20.],
                    vec![2.],
                    vec![150.],
                    vec![400.],
                    vec![15.],
                    vec![150.],
                    vec![2.],
                ],
                shift: 0,
            }),
        },
        date: ("2017.08.01".to_string(), "2019.08.20".to_string()),
        backtest_model: BacktestModel::OpenPrice,
        optimize: OptimizeMode::Complete,
        optimize_crit: OptimizeCrit::Custom,
        visual: false,
        symbols: vec!["EURUSD".to_string(), "AUDCAD".into()],
    };
    let runner = BacktestRunner::new(run.clone(), common.clone());
    runner.run_backtest().unwrap();
}
