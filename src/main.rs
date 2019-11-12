// #![warn(rust_2018_idioms)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate anyhow;

mod backtest_runner;
mod params;
mod xml_reader;

use backtest_runner::*;
use params::*;

use std::future::Future;
use std::process::Command;

use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let common = CommonParams::from_file("common.json").unwrap();
    let run = RunParams::from_file("run.json").unwrap();
    let runner = BacktestRunner::new(run.clone(), common.clone());

    // backtest_runner::run().await;
    /*     println!("Hello, world!");
     *
     *
     *     async fn start_terminal_t() {
     *         println!("start sleep");
     *         let s = Command::new("sleep")
     *             .arg("10")
     *             .output();
     *         println!("{:?}", s);
     *     }
     *
     *     async {
     *         let f = start_terminal_t();
     *         println!("started, waiting");
     *         f.await;
     *         println!("finished");
     *     };
     *
     *     // start_start();
     *     println!("fin out"); */
}
