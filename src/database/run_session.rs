// table! {
//     run_sessions (session_id) {
//         session_id -> Int4,
//         start_date -> Date,
//         end_date -> Date,
//         expert_version -> Nullable<Uuid>,
//         symbols_set_id -> Int4,
//     }
// }

use crate::database::schema::run_sessions;
use crate::database::symbols::SymbolSet;
use chrono::prelude::*;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[primary_key(session_id)]
#[belongs_to(SymbolSet)] //, foreign_key = "symbol_set_id")]
pub struct RunSession {
    pub session_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub expert_version: Option<Uuid>,
    pub symbol_set_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "run_sessions"]
pub struct NewRunSession {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub expert_version: Option<Uuid>,
    pub symbol_set_id: i32,
}
