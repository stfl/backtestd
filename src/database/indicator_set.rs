// use serde_repr::{Serialize_repr, Deserialize_repr};
// use super::super::params;
use super::schema::*; //{indicator_sets, indicators};
use super::indicator::*;
use diesel::prelude::*;

use crate::params;

// // Custom declaration of indicator_sets to allow derive(DbEnum) for DbIndiFunc
// table! {
//     use diesel::sql_types::*;
//     use super::DbIndiFuncMapping;
//     set_indicators (set_id, indicator_id) {
//         set_id -> Int8,
//         indicator_id -> Int4,
//         func -> DbIndiFuncMapping,
//     }
// }

// joinable!(set_indicators -> indicators (indicator_id));
// joinable!(set_indicators -> indicator_sets (set_id));

#[derive(Queryable, Insertable, Identifiable, Associations, Debug)]
#[primary_key(indicator_set_id, indicator_id)]
#[belongs_to(Indicator, foreign_key = "indicator_id")]
#[belongs_to(IndicatorSet, foreign_key = "indicator_set_id")]
#[table_name = "set_indicators"]
pub struct SetIndicator {
    pub indicator_set_id: i64,
    pub indicator_id: i32, // 1:m
}

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[primary_key(indicator_set_id)]
#[table_name = "indicator_sets"]
pub struct IndicatorSet {
    pub indicator_set_id: i64,
}

// #[derive(Insertable, Debug)]
// #[table_name = "indicator_sets"]
// pub struct DbNewIndicatorSet;

// pub fn load_indicator_set(
//     conn: &PgConnection,
//     indi_set_id: i64,
// ) -> Result<IndicatorSet, diesel::result::Error> {
//     use self::set_indicators::dsl::*;
//     use DbIndiFunc::*;

//     let db_indi_set = set_indicators
//         .filter(indicator_set_id.eq(indi_set_id))
//         .load::<SetIndicator>(conn)? // load the indicator set from DB
//         .iter()
//         .map(|set| (load_indicator(conn, set.indicator_id).unwrap(), set.func)) // load all indicators specified in the Set
//         // FIXME database errors or if the indicator is not found are ignored
//         .collect::<Vec<(Indicator, DbIndiFunc)>>(); // store the Indicator struct together with it's function for the set

//     let mut indi_set: IndicatorSet = Default::default();
//     for indi in db_indi_set {
//         // match DbIndiFunc
//         match indi.1 {
//             Confirm => indi_set.confirm = Some(indi.0), // and assign the Indicator struct
//             Confirm2 => indi_set.confirm2 = Some(indi.0),
//             Confirm3 => indi_set.confirm3 = Some(indi.0),
//             Baseline => indi_set.baseline = Some(indi.0),
//             Volume => indi_set.volume = Some(indi.0),
//             Continue => indi_set.cont = Some(indi.0),
//             Exit => indi_set.exit = Some(indi.0),
//         }
//     }
//     Ok(indi_set)
// }

pub fn find_db_indicator_set(
    conn: &PgConnection,
    indi_set: params::IndicatorSet,
) -> Result<Option<Vec<SetIndicator>>, diesel::result::Error> {
    // TODO
    // for each func in indi_set
    // find_db_indicator()
    // SELECT indicator_id, func from indicator_sets
    // WHERE func ==
    unimplemented!();
}

pub fn store_plain_indicator_set(
    conn: &PgConnection,
    indi_set: &params::IndicatorSet,
) -> QueryResult<Vec<SetIndicator>> {
    let db_indi_set = store_new_db_indicator_set(conn)?;
    // TODO Optional
    // TODO
    // for each func in indi_set
    // find_db_indicator()
    // insert_into
    // if find or insert fails.. -> delete db_indi_set
    unimplemented!();
}

pub fn store_set_indicators(
    conn: &PgConnection,
    set_indis: Vec<SetIndicator>,
) -> QueryResult<Vec<SetIndicator>> {
    use self::set_indicators::dsl::*;
    diesel::insert_into(set_indicators)
        .values(set_indis)
        .get_results(conn)
}

// creates a new indicator_set in the DB (a new row) which gets a ne unique id that can be used for set_indicators
pub fn store_new_db_indicator_set(conn: &PgConnection) -> QueryResult<IndicatorSet> {
    use super::schema::indicator_sets::dsl::*;
    diesel::insert_into(indicator_sets)
        .default_values()
        .get_result(conn)
}

pub fn store_new_indicator_set(
    conn: &PgConnection,
    indis: &Vec<Indicator>,
) -> QueryResult<IndicatorSet> {
    use crate::database::schema::indicator_sets;
    let indi_set = store_new_db_indicator_set(conn)?;
    let set_indis = indis
        .iter()
        .map(|i| SetIndicator {
            indicator_set_id: indi_set.id().to_owned(),
            indicator_id: i.id().to_owned(),
        })
        .collect();

    if let Err(e) = store_set_indicators(conn, set_indis) {
        error!("inserting the indicators for a set failed. deleting the indicator_set record with id {}", indi_set.id());
        let _ = diesel::delete(indicator_sets::table.find(indi_set.id())).execute(conn)?;
        return Err(e);
    }

    Ok(indi_set)
}
