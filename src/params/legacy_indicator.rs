use anyhow::{ensure, Context, Result};
use bigdecimal::BigDecimal;
use std::fs::File;
use std::path::Path;

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct LegacyIndicator {
    pub name: String,
    pub inputs: Vec<Vec<BigDecimal>>,
    pub shift: u8,
}

impl LegacyIndicator {
    pub fn from_file(file: &str) -> Result<Self> {
        let json_file = File::open(Path::new(file))?;
        Ok(serde_json::from_reader(json_file)?)
    }
}
