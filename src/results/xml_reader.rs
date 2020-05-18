use std::borrow::Cow;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::params::indicator_set::IndicatorSet;
use crate::params::*;

use super::ResultRow;

use anyhow::{Context, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_derive;

pub fn read_results_xml(
    input_indi_set: &IndicatorSet,
    results_file: PathBuf,
) -> Result<Vec<ResultRow>> {
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
                            rows.push(ResultRow {
                                pass: txt[0].parse().context(format!("Parsing Numeric 0 failed {:?}", txt[0]))?,
                                result: txt[1].parse().context("Parsing Numeric 1 failed")?,
                                profit: txt[2].parse().context("Parsing Numeric 2 failed")?,
                                expected_payoff: txt[3]
                                    .parse()
                                    .context("Parsing Numeric 3 failed")?,
                                profit_factor: txt[4].parse().context("Parsing Numeric 4 failed")?,
                                recovery_factor: txt[5]
                                    .parse()
                                    .context("Parsing Numeric 5 failed")?,
                                sharpe_ratio: txt[6].parse().context("Parsing Numeric 6 failed")?,
                                custom: txt[7].parse().context("Parsing Numeric 7 failed")?,
                                equity_dd: txt[8].parse().context("Parsing Numeric 8 failed")?,
                                trades: txt[9].parse().context("Parsing Numeric 9 failed")?,
                                params: txt[10..]
                                    .iter()
                                    .map(|s| s.parse().expect("Parsing Numeric 10 input failed"))
                                    .collect(),
                            })
                            // rows.push(BacktestResult {
                            //     /* indi_set: input_indi_set.parse_result_set(
                            //      *     txt[10..]
                            //      *         .iter()
                            //      *         .map(|s| s.parse().expect("Parsing Numeric input failed"))
                            //      *         .collect(),
                            //      * ), */
                            //     params: txt[10..]
                            //         .iter()
                            //         .map(|s| s.parse().expect("Parsing Numeric input failed"))
                            //         .collect(),
                            //     profit: txt[2].parse().context("Parsing Numeric failed")?,
                            //     result: txt[7].parse().context("Parsing Numeric failed")?,
                            //     trades: txt[9].parse().context("Parsing Numeric failed")?,
                            // })
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
            results_file.file_name().unwrap()
        )
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
    use crate::params::indi_func::IndiFunc;
    use crate::params::indi_func::IndiFunc::*;
    use crate::params::indicator::Indicator;
    use crate::params::indicator_set::IndicatorSet;
    use crate::params::signal_class::SignalClass::*;
    use crate::params::signal_class::SignalClass::*;
    use crate::params::vec_to_bigdecimal;
    use crate::params::vec_vec_to_bigdecimal;
    use std::collections::{BTreeMap, HashMap};
    use test;

    #[test]
    fn read_results_xml_test() {
        let indi_set: IndicatorSet = [(
            Confirm,
            Indicator {
                name: "Ash".to_owned(),
                filename: None,
                shift: 0u8,
                inputs: vec_vec_to_bigdecimal(vec![
                    vec![14., 100., 3.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                ]),
                buffers: None,
                params: None,
                class: Preset,
            },
        )]
        .iter()
        .cloned()
        .collect::<HashMap<IndiFunc, Indicator>>()
        .into();

        let rows = read_results_xml(&indi_set, PathBuf::from("tests/multicurrency.xml")).unwrap();
        assert_eq!(rows.len(), 663);
    }

    #[test]
    #[should_panic]
    #[ignore]
    // the output format has changed to directly return a Vec of the given params.
    // no casting into IndicatorSet
    fn xml_results_not_enough_params() {
        let indi_set: IndicatorSet = [(
            Confirm,
            Indicator {
                name: "Ash".to_owned(),
                filename: None,
                shift: 0u8,
                inputs: vec_vec_to_bigdecimal(vec![
                    vec![14., 100., 3.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                ]),
                buffers: None,
                params: None,
                class: Preset,
            },
        )]
        .iter()
        .cloned()
        .collect::<HashMap<IndiFunc, Indicator>>()
        .into();
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
        let indi_set: IndicatorSet = [(
            Confirm,
            Indicator {
                name: "Wae".to_owned(),
                filename: None,
                shift: 0u8,
                inputs: vec_vec_to_bigdecimal(vec![
                    vec![14., 100., 3.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                    vec![1., 30., 2.],
                ]),
                buffers: None,
                params: None,
                class: Preset,
            },
        )]
        .iter()
        .cloned()
        .collect::<HashMap<IndiFunc, Indicator>>()
        .into();

        b.iter(|| {
            let rows =
                read_results_xml(&indi_set, PathBuf::from("tests/multicurrency.xml")).unwrap();
            assert_eq!(rows.len(), 663)
        });
    }
}

pub fn read_results_xml_to_csv(
    input_indi_set: &IndicatorSet,
    results_file: &Path,
    csv_file: &Path,
) -> Result<i32> {
    let mut report_reader = Reader::from_file(results_file)?;
    report_reader.trim_text(true);
    let mut csv_writer = csv::Writer::from_path(csv_file)?;

    let mut count = 0;
    let mut buf = Vec::new();
    let mut txt = Vec::<String>::new();

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
            Ok(Event::End(ref e)) => match e.local_name() {
                b"Row" => {
                    csv_writer.write_record(&txt)?;
                }
                _ => (),
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&report_reader)?),
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }

    debug!(
        "read {} result rows\nfrom {:?}\ninto {:?}",
        count, results_file, csv_file
    );
    Ok(count)
}
