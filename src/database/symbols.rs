// table! {
//     session_symbol_sets (symbols_set_id) {
//         symbols_set_id -> Int4,
//     }
// }

// table! {
//     session_symbols (symbols_set_id) {
//         symbols_set_id -> Int4,
//         symbol -> Varchar,
//     }
// }

use crate::database::schema::{set_symbols, symbol_sets};
use diesel::prelude::*;
use std::borrow::Cow;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[primary_key(symbol_set_id)]
pub struct SymbolSet {
    pub symbol_set_id: i32,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[primary_key(symbol_set_id, symbol)]
pub struct SetSymbol<'a> {
    pub symbol_set_id: i32,
    pub symbol: Cow<'a, str>,
}

pub fn store_default_forex_symbols(conn: &PgConnection) -> QueryResult<SymbolSet> {
    use crate::database::schema::symbol_sets::dsl::*;
    diesel::insert_into(symbol_sets)
        .default_values()
        .get_result(conn)

    // FIXME this just creates a set to use as a reference in run_sessions
}
