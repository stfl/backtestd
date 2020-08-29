// #![warn(rust_2018_idioms)]
#![allow(dead_code)]
#![allow(unused)]
#![feature(test)]

// std includes
use std::path::{Path, PathBuf};
// use std::future::Future;
use std::sync::Mutex;

// crates
extern crate test;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
use serde_derive::Serialize;

#[macro_use]
extern crate anyhow;
// use anyhow::{Context, Result};
// use anyhow::Context;

extern crate pretty_env_logger;
// extern crate env_logger;
#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate chrono;

#[macro_use]
extern crate actix_web;
// use actix::prelude::*;
use actix_files as fs;
use actix_web::{
    error::ErrorInternalServerError, middleware, web, App as ActixApp, Error as ActixError,
    HttpRequest, HttpResponse, HttpServer,
};
// use actix_web_actors::ws;

// own mods
mod backtest_runner;
use backtest_runner::*;
mod params;
use params::*;
// mod signal_generator;
// use signal_generator::*;
mod results;

use results::xml_reader::*;

// running the multi-currency EA is significantly slower than running on single Symbol
// The overhead to init the backtest is also significant
// If the number of crossed out inputs is "low" then running the multi-currency EA is faster
// ... for OptimizeMode::Complete
const RUN_LIMIT_MULTI_CURRENCY: u64 = 5_000;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // async fn main() -> Result<(), anyhow::Error> {
    // async fn main() -> std::io::Result<()> {
    // TODO extend_var or sth
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info,debug");
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
            (@arg CLEANUP: -c --cleanup "cleanup files after running the backtest")
        )
        // (@subcommand gen =>
        //     (about: "generate an indicator signal")
        //     (@arg INPUT: +required "yaml file the specifies signal params")
        //     (@arg HEADER: -h --("header-out-dir") +takes_value "signal header source output dir")
        //     (@arg INDI: -i --("indi-out-dir") +takes_value "indicator params output dir")
        // )
        (@subcommand daemon =>
            (about: "start a daemon with a REST API")
        )
    )
    .get_matches();

    let config_file = matches.value_of("CONFIG").unwrap_or("config/config.yaml");
    let mut config: CommonParams = serde_any::from_file(config_file)
        .expect(&format!("reading config file failed: {}", config_file));

    if let Some(w) = matches.value_of("WORKDIR") {
        config.workdir = PathBuf::from(w);
    }
    info!("config: {:?}", config);

    // -------------
    // Daemon App
    // -------------
    if let Some(matches) = matches.subcommand_matches("daemon") {
        return HttpServer::new(move || {
            ActixApp::new()
                // enable logger
                .wrap(middleware::Logger::default())
                .data(config.clone())
                // .data(web::Data::new(Mutex::new(config.clone())))
                .service(web::resource("/run").route(web::post().to(backtest_run)))
        })
        // start http server
        .bind("0.0.0.0:12311")?
        .run()
        .await;
        // returning here..
    }

    // -------------
    // Generate Signals App
    // -------------
    // if let Some(matches) = matches.subcommand_matches("gen") {
    //     let input_file = matches.value_of("INPUT").unwrap();
    //     info!("Generate Signal from: {}", input_file);
    //     let signal_params: SignalParams =
    //         serde_any::from_file(input_file).expect("reading signal params for generation failed");
    //     debug!("SignalParams {:?}", signal_params);

    //     generate_signal(
    //         &signal_params,
    //         &config.workdir.join("MQL5/Include/IndiSignals"),
    //     )
    //     .expect("generate signal failed");
    //     // ?;

    //     // let indi_config_dir = Path::new(matches.value_of("HEADER").unwrap_or("config/indicator"));
    //     generate_signal_includes(&config.workdir.join("MQL5/Include/IndiSignals"))
    //         .expect("generating signal includes failed");

    //     let indi_config_dir = Path::new(matches.value_of("INDI").unwrap_or("config/indicator"));
    //     let indi = &Indicator::from(&signal_params);
    //     debug!("geneaterd indi input {:?}", indi);
    //     std::fs::create_dir_all(indi_config_dir)?;
    //     serde_any::to_file(
    //         indi_config_dir.join(format!("{}.yaml", signal_params.name)),
    //         indi,
    //     )
    //     .expect("writing signal input config failed");
    //     return Ok(());
    // }

    // -------------
    // Run Backtest App
    // -------------
    if let Some(matches) = matches.subcommand_matches("run") {
        let input_file = matches.value_of("INPUT").unwrap();
        info!("Running backtest from: {}", input_file);
        let run: RunParams = serde_any::from_file::<RunParamsFile, _>(input_file)
            .expect("reading RunParamsFile failed")
            .into();

        let runs = run.split_run_into_queue();
        backtest_runner::execute_run_queue(&config, &runs).expect("running queue failed");
    }

    Ok(())
}

// impl
/* impl From<anyhow::Error> for actix_web::Error {
 *     fn from(error: anyhow::Error) -> Self {
 *         error.into()
 *         // actix_web::Error {
 *             // cause:
 *         // }
 *             // ::as_resonse_error(error.description())
 *     }
 * } */

// impl actix_web::ResponseError for anyhow::Error {
// }

async fn backtest_run(
    // data: web::Json<(CommonParams, RunParams)>, //web::Data<Mutex<CommonParams>>)
    data: web::Json<RunParams>,
    config: web::Data<CommonParams>,
) -> Result<HttpResponse, ActixError> {
    // let (config, run) = data.into_inner();
    let run = data.into_inner();
    let config = config.into_inner();
    info!("running backtest with common: {:?}\nrun:{:?}", config, run);

    let runs = run.split_run_into_queue();
    backtest_runner::execute_run_queue(&config, &runs).map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(
        collect_csv_filenames_from_queue(&config, &runs)
            .map_err(|e| ErrorInternalServerError(e))?,
    ))

    // let runner = BacktestRunner::new(run, &config);
    // runner
    //     .prepare_files()
    //     .map_err(|e| ErrorInternalServerError(e))?;
    // runner.run().map_err(|e| ErrorInternalServerError(e))?;
    // Ok(HttpResponse::Ok().json(
    //     runner
    //         .read_results()
    //         .map_err(|e| ErrorInternalServerError(e))?,
    // ))

}

// async fn signal_gen(
//     data: web::Json<(CommonParams, SignalParams)>,
// ) -> Result<HttpResponse, ActixError> {
//     let (config, sig) = data.into_inner();
//     debug!(
//         "generating signal with common: {:#?}\nsignal_params: {:#?}",
//         config, sig
//     );

//     generate_signal(&sig, &config.workdir.join("MQL5/Include/IndiSignals"))
//         .map_err(|e| ErrorInternalServerError(e))?;

//     // let indi_config_dir = Path::new(matches.value_of("HEADER").unwrap_or("config/indicator"));
//     generate_signal_includes(&config.workdir.join("MQL5/Include/IndiSignals"))
//         .map_err(|e| ErrorInternalServerError(e))?;

//     // let indi = &Indicator::from(&sig);
//     // generate_signal(sig, );
//     Ok(HttpResponse::Ok().json(Indicator::from(&sig)))
// }
//
