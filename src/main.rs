// #![warn(rust_2018_idioms)]
// #![allow(dead_code)]
// #![allow(unused)]
#![feature(test)]
use std::path::PathBuf;

extern crate lazy_static;
extern crate test;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate actix_web;
extern crate chrono;

use actix_web::{
    error::ErrorInternalServerError, middleware, web, App as ActixApp, Error as ActixError,
    HttpResponse, HttpServer,
};
mod backtest_runner;
use backtest_runner::*;
mod params;
use params::*;
mod results;

// running the multi-currency EA is significantly slower than running on single Symbol
// The overhead to init the backtest is also significant
// If the number of crossed out inputs is "low" then running the multi-currency EA is faster
// ... for OptimizeMode::Complete
const RUN_LIMIT_MULTI_CURRENCY: u64 = 5_000;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // TODO extend_var or sth
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug,debug");
    pretty_env_logger::init();

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
            (@arg SPLIT_YEARS: --splityears +takes_value)
            // (@arg SPLIT_SYMBOLS: --split-symbols)
        )
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
    if let Some(_matches) = matches.subcommand_matches("daemon") {
        return server(config).await;
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

        let runs = run.split_run_into_queue(value_t!(matches, "SPLIT_YEARS", u32).unwrap_or(1));
        // let runs = run.split_run_into_queue();
        // let runs = vec![run];
        backtest_runner::execute_run_queue(&config, &runs).expect("running queue failed");
    }

    Ok(())
}

async fn server(config: CommonParams) -> std::io::Result<()> {
    return HttpServer::new(move || {
            ActixApp::new()
                // enable logger
                .wrap(middleware::Logger::default())
                .data(config.clone())
                .service(web::resource("/run").route(web::post().to(backtest_run)))
        })
        // start http server
        .bind("0.0.0.0:12311")?
        .run()
        .await;
}

async fn backtest_run(
    data: web::Json<RunParams>,
    config: web::Data<CommonParams>,
) -> Result<HttpResponse, ActixError> {
    let run = data.into_inner();
    let config = config.into_inner();
    info!("running backtest with common: {:?}\nrun:{:?}", config, run);

    // let runs = run.split_run_into_queue();
    let runs = vec![run];
    backtest_runner::execute_run_queue(&config, &runs).map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(
        get_csv_filenames_from_queue(&config, &runs), // .map_err(|e| ErrorInternalServerError(e))?,
    ))
}
