use chrono::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::indi_func::IndiFunc;
use super::run_params::RunParams;
use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RunParamsFile {
    pub name: String,
    pub indi_set: HashMap<IndiFunc, PathBuf>,
    pub date: (DateTime<Utc>, DateTime<Utc>),
    pub backtest_model: BacktestModel,
    pub optimize: OptimizeMode,
    pub optimize_crit: OptimizeCrit,
    pub visual: bool,
    pub symbols: Vec<String>,
    pub store_results: StoreResults,
}

impl From<RunParamsFile> for RunParams {
    fn from(s: RunParamsFile) -> Self {
        RunParams {
            name: s.name,
            indi_set: s.indi_set.into(),
            date: s.date,
            backtest_model: s.backtest_model,
            optimize: s.optimize,
            optimize_crit: s.optimize_crit,
            visual: s.visual,
            symbols: s.symbols,
            store_results: s.store_results,
        }
    }
}
