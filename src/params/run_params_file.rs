use chrono::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::indi_func::IndiFunc;
use super::indicator_set::IndicatorSet;
use super::run_params::RunParams;
use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RunParamsFile {
    pub name: String,
    pub indi_set: HashMap<IndiFunc, PathBuf>,
    // pub date: (String, String),
    pub date: (DateTime<Utc>, DateTime<Utc>),
    pub backtest_model: BacktestModel,
    pub optimize: OptimizeMode,
    pub optimize_crit: OptimizeCrit,
    pub visual: bool,
    pub symbols: Vec<String>,
    pub store_results: StoreResults,
}

// impl TryFrom<RunParamsFile> for RunParams {
impl From<RunParamsFile> for RunParams {
    // type Error = anyhow::Error;

    // fn try_from(s: RunParamsFile) -> Result<Self, Self::Error> {
    fn from(s: RunParamsFile) -> Self {
        // Ok(
        RunParams {
            name: s.name,
            indi_set: s.indi_set.into(),
            /* date: (
             *     DateTime::parse_from_rfc3339(&s.date.0)?.into(),
             *     DateTime::parse_from_rfc3339(&s.date.1)?.into(),
             * ), */
            date: s.date,
            backtest_model: s.backtest_model,
            optimize: s.optimize,
            optimize_crit: s.optimize_crit,
            visual: s.visual,
            symbols: s.symbols,
            store_results: s.store_results,
        }
        // )
    }
}
