use diesel::prelude::*;
use glob::glob;
use serde_any;

use crate::database::indicator::{store_indicator, store_signal_params};
use crate::database::indicator::{IndiFunc, Indicator};

use crate::params;
use crate::params::legacy_indicator::LegacyIndicator;
use crate::signal_generator;

// TODO this is a really ugly implementation
pub fn load_all_indicators_from_file<'a>(conn: &PgConnection) -> QueryResult<Vec<Indicator>> {
    use super::schema::indicators;
    use IndiFunc::*;

    let mut indis = Vec::<Indicator>::with_capacity(50);
    for entry in glob("config/indicator/*/*").unwrap().filter_map(Result::ok) {
        // println!("loading-indicator-file: {:?}", entry);
        let func = match entry
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
        {
            "confirm" => Confirm,
            "baseline" => Baseline,
            "exit" => Exit,
            "volume" => Volume,
            "continue" => Continue,
            e => panic!("unknown func {:?}", e),
        };
        let indi: LegacyIndicator = serde_any::from_file(entry).unwrap();
        indis.push(store_indicator(conn, &indi, None, func)?);

        // indis.push((func, &indi).into());
        match func {
            Confirm => {
                indis.push(store_indicator(conn, &indi, None, Confirm2)?);
                indis.push(store_indicator(conn, &indi, None, Confirm3)?);
                indis.push(store_indicator(conn, &indi, None, Exit)?);
                indis.push(store_indicator(conn, &indi, None, Continue)?);
            }
            Baseline => {
                indis.push(store_indicator(conn, &indi, None, Confirm)?);
                indis.push(store_indicator(conn, &indi, None, Confirm2)?);
                indis.push(store_indicator(conn, &indi, None, Confirm3)?);
                indis.push(store_indicator(conn, &indi, None, Exit)?);
                indis.push(store_indicator(conn, &indi, None, Continue)?);
            }
            _ => (),
        }
    }

    for entry in glob("config/generate/*/*").unwrap().filter_map(Result::ok) {
        // println!("loading-indicator-file: {:?}", entry);
        let func = match entry
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
        {
            "confirm" => Confirm,
            "baseline" => Baseline,
            "exit" => Exit,
            "volume" => Volume,
            "continue" => Continue,
            e => panic!("unknown func {:?}", e),
        };
        let indi: signal_generator::SignalParams = serde_any::from_file(entry).unwrap();
        indis.push(store_signal_params(conn, &indi, None, func)?);

        // indis.push((func, &indi).into());
        match func {
            Confirm => {
                indis.push(store_signal_params(conn, &indi, None, Confirm2)?);
                indis.push(store_signal_params(conn, &indi, None, Confirm3)?);
                indis.push(store_signal_params(conn, &indi, None, Exit)?);
                indis.push(store_signal_params(conn, &indi, None, Continue)?);
            }
            Baseline => {
                indis.push(store_signal_params(conn, &indi, None, Confirm)?);
                indis.push(store_signal_params(conn, &indi, None, Confirm2)?);
                indis.push(store_signal_params(conn, &indi, None, Confirm3)?);
                indis.push(store_signal_params(conn, &indi, None, Exit)?);
                indis.push(store_signal_params(conn, &indi, None, Continue)?);
            }
            _ => (),
        }
    }

    // diesel::insert_into(indicators::table)
    //     .values(&indis)
    //     .get_results(conn)
    Ok(indis)
}
