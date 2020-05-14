use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{ensure, Context, Result};
use bigdecimal::BigDecimal;
use chrono::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::indicator_set::*;
use super::{
    terminal_params::*, to_param_string::ToParamString, to_terminal_config::ToTerminalConfig,
};

// input from the API
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct RunParams {
    pub name: String,
    pub indi_set: IndicatorSet,
    pub date: (DateTime<Utc>, DateTime<Utc>),
    pub backtest_model: BacktestModel,
    pub optimize: OptimizeMode,
    pub optimize_crit: OptimizeCrit,
    pub visual: bool,
    // symbols : &[],
    pub symbols: Vec<String>,
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

impl RunParams {
    pub fn from_file(file: &str) -> Result<Self> {
        let json_file = File::open(Path::new(file))?;
        Ok(serde_json::from_reader(json_file)?)
    }

    // pub fn to_file(&self, file: &str) -> Result<()> {
    //     let json_file = File::create(Path::new(file))?;
    //     Ok(serde_json::ser::to_writer_pretty(json_file, self)?)
    // }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.symbols.iter()
    }
}

impl ToParamString for RunParams {
    fn to_param_string(&self) -> String {
        let mut string = self.indi_set.to_param_string();
        for (i, symbol) in self.symbols.iter().enumerate() {
            string.push_str(&format!(
                "Expert_symbol{idx}={symbol}\n",
                symbol = symbol,
                idx = i,
            ));
        }
        // string.push_str(&format!("Expert_Symbols={}", self.symbols.join(" ")));
        debug!("Params config for terminal:\n{}", string);
        string
    }
}

impl ToTerminalConfig for RunParams {
    fn to_terminal_config(&self) -> String {
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
            opti_crit = self.optimize_crit as u8
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
}

// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
// pub struct RunParamsFile {
//     pub name: String,
//     pub indi_set: IndicatorSetFile,
//     // pub date: (String, String),
//     pub date: (DateTime<Utc>, DateTime<Utc>),
//     pub backtest_model: BacktestModel,
//     pub optimize: OptimizeMode,
//     pub optimize_crit: OptimizeCrit,
//     pub visual: bool,
//     pub symbols: Vec<String>,
// }

// // impl TryFrom<RunParamsFile> for RunParams {
// impl From<RunParamsFile> for RunParams {
//     // type Error = anyhow::Error;

//     // fn try_from(s: RunParamsFile) -> Result<Self, Self::Error> {
//     fn from(s: RunParamsFile) -> Self {
//         // Ok(
//         RunParams {
//             name: s.name,
//             indi_set: s.indi_set.into(),
//             /* date: (
//              *     DateTime::parse_from_rfc3339(&s.date.0)?.into(),
//              *     DateTime::parse_from_rfc3339(&s.date.1)?.into(),
//              * ), */
//             date: s.date,
//             backtest_model: s.backtest_model,
//             optimize: s.optimize,
//             optimize_crit: s.optimize_crit,
//             visual: s.visual,
//             symbols: s.symbols,
//         }
//         // )
//     }
// }

pub fn get_reports_dir(common: &CommonParams, run: &RunParams) -> Result<PathBuf> {
    ensure!(
        common.reports.is_relative(),
        "Reports path needs to be relative"
    );
    Ok(common.workdir.join(&common.reports)) //.join(&run.name).with_extension("xml")))
}

pub fn get_reports_path(common: &CommonParams, run: &RunParams) -> Result<PathBuf> {
    let reports_path = get_reports_dir(&common, &run)?
        .join(&run.name)
        .with_extension("xml");
    Ok(reports_path)
}

pub fn to_terminal_config(common: &CommonParams, run: &RunParams) -> String {
    // ensure!(
    //     !common.reports.is_absolute(),
    //     "reports path must be relative"
    // );
    // generate the reports path for the terminal.ini with windows-style "\"
    let reports_path_relative = common
        .reports
        .join(&run.name)
        .with_extension("xml")
        // .join("reports.xml")
        .iter()
        .filter_map(|s| s.to_str())
        .collect::<Vec<&str>>()
        .join("\\");
    format!(
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
        common = common.to_terminal_config(),
        run = run.to_terminal_config(),
        symb = run
            .symbols
            .iter()
            .max_by(|x, y| x.cmp(y))
            // FIXME take the alphanumerical. This causes the bar times to be correct
            // this is a very vage assumtion and needs to be double and tripple checked in the EA
            .context("sorting symbols failed")
            .unwrap(),
        report = reports_path_relative
    )
}
