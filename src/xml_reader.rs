use std::{
    fs, io,
    path::{Path, PathBuf},
};
// use std::string::String::from_utf8_lossy;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

use super::params::*;

use anyhow::{Context, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_derive;

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BacktestResult {
    // indi_set: IndicatorSet,
    params: Vec<String>,
    profit: f32,
    result: f32,
    trades: u32,
}

// pub struct ResultMap<IndicatorSet, BacktestResult>

pub fn read_results_xml(
    input_indi_set: &IndicatorSet,
    results_file: PathBuf,
) -> Result<Vec<BacktestResult>> {
    debug!("reading results from {:?}", results_file);
    let mut report_reader = Reader::from_file(results_file.as_path())?;
    report_reader.trim_text(true);
    let mut count = 0;
    let mut buf = Vec::new();
    let mut txt = Vec::<String>::new();
    let mut rows = Vec::new(); // may be larger as well
                               // let mut results: HashMap<IndicatorSet, BacktestResult>::new();
    loop {
        match report_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.local_name() {
                    b"Row" => {
                        count += 1;
                        txt.clear(); // delete the values but keep the capacity
                    }
                    _ => (),
                }
            }
            Ok(Event::End(ref e)) => {
                match e.local_name() {
                    b"Row" => {
                        if count > 1 {
                            // ignore the header row
                            rows.push(BacktestResult {
                                /* indi_set: input_indi_set.parse_result_set(
                                 *     txt[10..]
                                 *         .iter()
                                 *         .map(|s| s.parse().expect("Parsing Numeric input failed"))
                                 *         .collect(),
                                 * ), */
                                params: txt[10..]
                                    .iter()
                                    .map(|s| s.parse().expect("Parsing Numeric input failed"))
                                    .collect(),
                                profit: txt[2].parse().context("Parsing Numeric failed")?,
                                result: txt[7].parse().context("Parsing Numeric failed")?,
                                trades: txt[9].parse().context("Parsing Numeric failed")?,
                            })
                        }
                    }
                    _ => (),
                }
            }
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&report_reader)?),
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }

    ensure!(
        count - 1 == rows.len(),
        "something went wrong with the row count"
    );
    if rows.len() == 0 {
        warn!(
            "read {} rows from {:?}",
            count - 1,
            results_file.file_name().unwrap())
    } else {
        info!(
            "read {} rows from {:?}",
            count - 1,
            results_file.file_name().unwrap()
        );
    }
    Ok(rows)
}

#[cfg(test)]
mod xml_test {
    use super::*;
    use test;

    #[test]
    fn read_results_xml_test() {
        let indi_set = IndicatorSet {
            confirm: Some(Indicator {
                name: "Ash".to_owned(),
                shift: 0u8,
                inputs: vec![
                    vec![14., 100., 3.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                ],
            }),
            ..Default::default()
        };

        let rows = read_results_xml(&indi_set, PathBuf::from("tests/multicurrency.xml")).unwrap();
        assert_eq!(rows.len(), 663);
    }

    #[test]
    #[should_panic]
    #[ignore]
    // the output format has changed to directly return a Vec of the given params.
    // no casting into IndicatorSet
    fn xml_results_not_enough_params() {
        let indi_set = IndicatorSet {
            confirm: Some(Indicator {
                name: "Ash".to_owned(),
                shift: 0u8,
                inputs: vec![
                    vec![14., 100., 3.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                ],
            }),
            ..Default::default()
        };
        // we are expacting more params in result than there are given

        // let result = std::panic::catch_unwind(|| {
        read_results_xml(&indi_set, PathBuf::from("tests/multicurrency.xml")).unwrap();
        // });
        // assert!(result.is_err());

        // entered a random indi_set
        /* let rows = read_results_xml(&indi_set, PathBuf::from("tests/report_AUDCAD.xml")).unwrap();
         * assert_eq!(rows.len(), 176); */
    }

    #[bench]
    fn bench_read_results_xml(b: &mut test::Bencher) {
        let indi_set = IndicatorSet {
            confirm: Some(Indicator {
                name: "Wae".to_owned(),
                shift: 0u8,
                inputs: vec![
                    vec![14., 100., 3.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                ],
            }),
            ..Default::default()
        };

        b.iter(|| {
            let rows =
                read_results_xml(&indi_set, PathBuf::from("tests/multicurrency.xml")).unwrap();
            assert_eq!(rows.len(), 663)
        });
    }
}
