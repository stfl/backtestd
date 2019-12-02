// #![warn(rust_2018_idioms)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate anyhow;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

mod backtest_runner;
mod params;
mod signal_generator;
mod xml_reader;

use backtest_runner::*;
use params::*;
use signal_generator::*;

// use std::future::Future;

use std::path::{Path, PathBuf};

// use tokio::io::AsyncWriteExt;

// #[tokio::main]
fn main() {
    pretty_env_logger::init();

    //     .version(crate_version!())
    //     .about("Runs backtests of given indicator sets and other things")
    //     .author(crate_authors!())
    //     .arg(
    //         Arg::with_name("CONFIG")
    //             .short("c")
    //             .long("config")
    //             .value_name("FILE")
    //             .help("config file")
    //             .takes_value(true)
    //             .required(true)
    //             // .default_value("config/config.yaml")
    //     )
    //     .arg(
    //         Arg::with_name("WORKDIR")
    //             .short("w")
    //             .long("workdir")
    //             .value_name("DIR")
    //             .help("MT5 Terminal workdir")
    //             .takes_value(true)
    //     )
    //     .subcommand(
    //         SubCommand::with_name("run")
    //             .about("run a backtest")
    //             .arg(Arg::with_name("INPUT").help("").required(true)),
    //     )
    //     .subcommand(
    //         SubCommand::with_name("gen")
    //             .about("generate an indicator signal")
    //             .arg(
    //                 Arg::with_name("INPUT")
    //                     .help("yaml file the specifies the signal params")
    //                     .required(true),
    //             ),
    //     )
    //     .get_matches();

    let matches = clap_app!(BacktestRunner =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Runs backtests of given indicator sets and other things")
        (@arg CONFIG: -c --config +takes_value "Config file")
        (@arg WORKDIR: -w --workdir +takes_value "overwrite workdir path")
        (@subcommand run =>
            (about: "run a backtest")
            (@arg INPUT: +required "yaml file the specifies the run params")
        )
        (@subcommand gen =>
            (about: "generate an indicator signal")
            (@arg INPUT: +required "yaml file the specifies signal params")
            (@arg HEADER: -h --("header-out-dir") "signal header source output dir")
            (@arg INDI: -i --("indi-out-dir") "indicator params output dir")
        )
    )
    .get_matches();

    let config_file = matches.value_of("CONFIG").unwrap_or("config/config.yaml");
    let mut config: CommonParams =
        serde_any::from_file(config_file).expect("reading config file failed");

    if let Some(w) = matches.value_of("WORKDIR") {
        config.workdir = PathBuf::from(w);
    }
    debug!("config: {:#?}", config);

    // -------------
    // Generate Signals App
    // -------------
    if let Some(matches) = matches.subcommand_matches("gen") {
        let input_file = matches.value_of("INPUT").unwrap();
        info!("Generate Signal from: {}", input_file);
        let signal_params: SignalParams =
            serde_any::from_file(input_file).expect("reading signal params for generation failed");
        debug!("SignalParams {:#?}", signal_params);

        generate_signal(
            &signal_params,
            &config.workdir.join("MQL5/Include/IndiSignals"),
        )
        .expect("generating signal failed");

        // let indi_config_dir = Path::new(matches.value_of("HEADER").unwrap_or("config/indicator"));
        generate_signal_includes(&config.workdir.join("MQL5/Include/IndiSignals"))
            .expect("generating signal includes failed");

        let indi_config_dir = Path::new(matches.value_of("INDI").unwrap_or("config/indicator"));
        let indi = &Indicator::from(&signal_params);
        debug!("geneaterd indi input {:#?}", indi);
        serde_any::to_file(
            indi_config_dir.join(format!("{}.yaml", signal_params.name)),
            indi,
        )
        .expect("writing signal input config failed");
    }

    // -------------
    // Run Backtest App
    // -------------
    if let Some(matches) = matches.subcommand_matches("run") {
        let input_file = matches.value_of("INPUT").unwrap();
        info!("Running backtest from: {}", input_file);
        let run: RunParams = serde_any::from_file::<RunParamsFile, _>(input_file)
            .expect("reading RunParamsFile failed")
            .into();
        let runner = BacktestRunner::new(run, config.clone());
        let _ = runner.run_backtest().expect("running backtest failed");
    }

    // let common = CommonParams {
    //     params_file: "expert_params.set".to_string(),
    //     terminal_exe: PathBuf::from(r"C:\Program Files\MetaTrader 5\terminal64.exe"),
    //     workdir: workdir.to_path_buf(),
    //     reports: PathBuf::from("reports"),
    //     expert: r"BacktestExpert\nnfx-ea\nnfx-ea.ex5".to_string(),
    //     period: "D1".to_string(),
    //     login: "5152".to_string(),
    //     use_remote: true,
    //     use_local: true,
    //     replace_report: true,
    //     shutdown_terminal: true,
    //     deposit: 10000,
    //     currency: "USD".to_string(),
    //     leverage: 100,
    //     execution_mode: 0,
    // };

    // let run = RunParams {
    //     name: "asctrend_wae".to_string(),
    //     indi_set: IndicatorSet {
    //         confirm: Some(Indicator {
    //             name: "asctrend".to_string(),
    //             inputs: vec![vec![1., 20., 1.]],
    //             shift: 0,
    //         }),
    //         confirm2: None,
    //         confirm3: None,
    //         exit: None,
    //         cont: None,
    //         baseline: None,
    //         volume: Some(
    //             serde_any::from_file(
    //                 common
    //                     .workdir
    //                     .join("wae.yaml")
    //                     .as_os_str()
    //                     .to_str()
    //                     .unwrap(),
    //             )
    //             .unwrap(),
    //         ),
    //     },
    //     date: ("2017.08.01".to_string(), "2019.08.20".to_string()),
    //     backtest_model: BacktestModel::OpenPrice,
    //     optimize: OptimizeMode::Complete,
    //     optimize_crit: OptimizeCrit::Custom,
    //     visual: false,
    //     symbols: vec!["EURUSD".to_string(), "AUDCAD".into()],
    // };

    // println!(
    //     "writing to {:?}",
    //     common
    //         .workdir
    //         .join("run.json")
    //         .as_os_str()
    //         .to_str()
    //         .unwrap()
    // );

    /* serde_any::to_file(
     *     common
     *         .workdir
     *         .join("run.yaml")
     *         .as_os_str()
     *         .to_str()
     *         .unwrap(),
     *     &run,
     * )
     * .unwrap();
     * serde_any::to_file(
     *     common
     *         .workdir
     *         .join("wae.yaml")
     *         .as_os_str()
     *         .to_str()
     *         .unwrap(),
     *     &run.indi_set.volume,
     * )
     * .unwrap(); */

    /* common.to_file(common.workdir.join("common.json").as_os_str().to_str().unwrap()).expect("can't write file");
     * run.to_file(common.workdir.join("run.json").as_os_str().to_str().unwrap()).expect("can't write file"); */
    // let runner = BacktestRunner::new(run.clone(), common.clone());
    // runner.run_backtest().unwrap();
}
