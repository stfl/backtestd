use super::ResultRow;
use anyhow::{Context, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::path::{Path, PathBuf};

pub fn _read_results_xml(results_file: PathBuf) -> Result<Vec<ResultRow>> {
    debug!("reading results from {:?}", results_file);
    let mut report_reader = Reader::from_file(results_file.as_path())?;
    report_reader.trim_text(true);
    let mut count = 0;
    let mut buf = Vec::new();
    let mut rows = Vec::new(); // may be larger as well
    let mut txts = Vec::<String>::new();
    let mut txt: Option<String> = None;

    loop {
        match report_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.local_name() {
                    b"Row" => {
                        count += 1;
                        txts.clear(); // delete the values but keep the capacity
                                      // state = State::Row;
                    }
                    // b"Data" => state = State::Data,
                    _ => (),
                }
            }
            Ok(Event::End(ref e)) => match e.local_name() {
                b"Row" => {
                    if count > 1 {
                        // ignore the header row
                        rows.push(ResultRow {
                            pass: txts[0]
                                .parse()
                                .context(format!("Parsing Numeric 0 failed {:?}", txts[0]))?,
                            result: txts[1].parse().context("Parsing Numeric 1 failed")?,
                            profit: txts[2].parse().context("Parsing Numeric 2 failed")?,
                            expected_payoff: txts[3].parse().context("Parsing Numeric 3 failed")?,
                            profit_factor: txts[4].parse().context("Parsing Numeric 4 failed")?,
                            recovery_factor: txts[5].parse().context("Parsing Numeric 5 failed")?,
                            sharpe_ratio: txts[6].parse().context("Parsing Numeric 6 failed")?,
                            custom: txts[7].parse().context("Parsing Numeric 7 failed")?,
                            equity_dd: txts[8].parse().context("Parsing Numeric 8 failed")?,
                            trades: txts[9].parse().context("Parsing Numeric 9 failed")?,
                            params: txts[10..]
                                .iter()
                                .map(|s| s.parse().expect("Parsing Numeric 10 input failed"))
                                .collect(),
                        });
                    }
                }
                b"Data" => {
                    // state = State::Row;
                    if let Some(t) = &txt {
                        txts.push(t.into());
                    } else {
                        txts.push("".into());
                    }
                    txt = None;
                }
                _ => (),
            },
            Ok(Event::Text(e)) => txt = Some(e.unescape_and_decode(&report_reader)?),
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

pub fn read_results_xml_to_csv(xml_file: &Path, csv_file: &Path) -> Result<i32> {
    let mut report_reader = Reader::from_file(xml_file)?;
    report_reader.trim_text(true);
    let mut csv_writer = csv::Writer::from_path(csv_file)?;

    let mut count = 0;
    let mut buf = Vec::new();
    let mut txts = Vec::<String>::new();
    let mut txt: Option<String> = None;

    loop {
        match report_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.local_name() {
                    b"Row" => {
                        count += 1;
                        txts.clear(); // delete the values but keep the capacity
                                      // state = State::Row;
                    }
                    // b"Data" => state = State::Data,
                    _ => (),
                }
            }
            Ok(Event::End(ref e)) => match e.local_name() {
                b"Row" => {
                    csv_writer.write_record(&txts)?;
                    // state = State::None;
                }
                b"Data" => {
                    // state = State::Row;
                    if let Some(t) = &txt {
                        txts.push(t.into());
                    } else {
                        txts.push("".into());
                    }
                    txt = None;
                }
                _ => (),
            },
            Ok(Event::Text(e)) => txt = Some(e.unescape_and_decode(&report_reader)?),
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }

    debug!(
        "read {} result rows\nfrom {:?}\ninto {:?}",
        count, xml_file, csv_file
    );
    Ok(count)
}

#[cfg(test)]
mod xml_test {
    use super::*;
    use std::fs;
    use test;

    #[test]
    fn read_results_xml_test() {
        let rows = _read_results_xml(PathBuf::from("tests/multicurrency.xml")).unwrap();
        assert_eq!(rows.len(), 663);
    }

    #[test]
    fn read_results_xml_to_csv_test() {
        let _ = fs::remove_file("/tmp/bt_run.csv");
        let rows = read_results_xml_to_csv(
            &Path::new("tests/bt_run.xml"),
            &Path::new("/tmp/bt_run.csv"),
        )
        .unwrap();
        assert_eq!(rows, 5714);
        assert!(Path::new("/tmp/bt_run.csv").exists());
    }

    #[bench]
    fn bench_read_results_xml(b: &mut test::Bencher) {
        b.iter(|| {
            let rows = _read_results_xml(PathBuf::from("tests/multicurrency.xml")).unwrap();
            assert_eq!(rows.len(), 663)
        });
    }
}
