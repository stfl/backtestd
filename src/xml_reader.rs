#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![allow(dead_code)]

use quick_xml::events::Event;
use quick_xml::Reader;
use std::path::Path;
// use std::string::String::from_utf8_lossy;
use std::borrow::Cow;

use anyhow::{Context, Result};

use serde_derive;

use std::{fs, io, path::PathBuf};

#[derive(Debug, Deserialize, PartialEq)]
struct Row {
    pass: u32,
    result: f32,
    profit: f32,
    expected_payoff: f32,
    profit_factor: f32,
    recovery_factor: f32,
    sharpe_ratio: f32,
    custom_result: f32,
    equity_dd: f32,
    trades: u32,
    input_params: Vec<f32>,
}

impl Row {
    pub fn from_vec(vec: &Vec<String>) -> Row {
        println!("{:?}", vec);
        let mut iter = vec.iter();
        let row = Row {
            pass: iter.next().unwrap().parse().unwrap(),
            result: iter.next().unwrap().parse().unwrap(),
            profit: iter.next().unwrap().parse().unwrap(),
            expected_payoff: iter.next().unwrap().parse().unwrap(),
            profit_factor: iter.next().unwrap().parse().unwrap(),
            recovery_factor: iter.next().unwrap().parse().unwrap(),
            sharpe_ratio: iter.next().unwrap().parse().unwrap(),
            custom_result: iter.next().unwrap().parse().unwrap(),
            equity_dd: iter.next().unwrap().parse().unwrap(),
            trades: iter.next().unwrap().parse().unwrap(),
            input_params: iter.map(|s| s.parse().unwrap()).collect(),
        };
        row
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ResultRow {
    profit: f32,
    result: f32,
    trades: u32,
    input_params: Vec<f32>,
}

impl ResultRow {
    pub fn try_from_vec(vec: &Vec<String>) -> Result<ResultRow> {
        let row = ResultRow {
            profit: vec[2].parse().context("Parsing Numeric failed")?,
            result: vec[7].parse().context("Parsing Numeric failed")?,
            trades: vec[9].parse().context("Parsing Numeric failed")?,
            input_params: vec[10..].iter().map(|s| s.parse().unwrap()).collect(),
            // TODO identify the input_params_set and store the hash or sth
            // generate a reproducable hash xxHash?
        };
        Ok(row)
    }
}

pub fn read_results_xml(results_file: PathBuf) -> Result<Vec<ResultRow>> {
    let mut report_reader = Reader::from_file(results_file.as_path())?;
    report_reader.trim_text(true);
    let mut count = 0;
    let mut buf = Vec::new();
    let mut txt = Vec::new();
    let mut rows = Vec::new(); // may be larger as well
    loop {
        match report_reader
            .read_event(&mut buf)
            .context("Failed to decode XML")?
        {
            Event::Start(ref e) => {
                match std::str::from_utf8(e.local_name()).context("Failed to decode XML")? {
                    "Row" => {
                        count += 1;
                        txt.clear(); // delete the values but keep the capacity
                    }
                    _ => (),
                }
            }
            Event::End(ref e) => {
                match std::str::from_utf8(e.local_name()).context("Failed to decode XML")? {
                    "Row" => {
                        if count > 1 {
                            // ignore the header row
                            rows.push(ResultRow::try_from_vec(&txt)?);
                        }
                    }
                    _ => (),
                }
            }
            Event::Text(e) => txt.push(
                e.unescape_and_decode(&report_reader)
                    .context("Failed to decode XML")?,
            ),
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }

    ensure!(
        count - 1 == rows.len(),
        "something went wrong with the row count"
    );
    info!(
        "read {} rows from {:?}",
        count - 1,
        results_file.file_name().unwrap()
    );
    Ok(rows)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_results_xml_test() {
        let rows = read_results_xml(PathBuf::from("tests/report_AUDCAD.xml")).unwrap();
        assert_eq!(rows.len(), 176)
    }
}
