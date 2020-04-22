// table! {
//     runs (run_id) {
//         run_id -> Int8,
//         session_id -> Int4,
//         run_date -> Timestamp,
//         indicator_set_id -> Int8,
//     }
// }

use crate::database::indicator_set::IndicatorSet;
use crate::database::run_session::RunSession;
use crate::database::schema::*;
use chrono::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[primary_key(run_id)]
#[belongs_to(RunSession, foreign_key = "session_id")]
#[belongs_to(IndicatorSet, foreign_key = "indicator_set_id")]
pub struct Run {
    pub run_id: i64,
    pub session_id: i32,
    pub run_date: NaiveDateTime,
    pub indicator_set_id: i64,
}

#[derive(Insertable, Debug)]
#[table_name = "runs"]
pub struct NewRun {
    pub session_id: i32,
    pub run_date: NaiveDateTime,
    pub indicator_set_id: i64,
}
