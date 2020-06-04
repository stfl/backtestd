use csv;
use csv::Writer;

use super::ResultRow;
use crate::params::indicator_set::IndicatorSet;
use anyhow::{Context, Result};

use std::path::{Path, PathBuf};

pub fn write_result_csv(path: &Path, indi_set: IndicatorSet, results: Vec<ResultRow>) -> Result<()> {
    let mut wtr = Writer::from_path(path);
    unimplemented!();
}
