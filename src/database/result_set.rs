use crate::database::indicator_set::IndicatorSet;
use crate::database::run::Run;
use crate::database::result::*;
use crate::database::indicator_inputs_explicit::*;

use crate::database::schema::result_sets;

#[derive(Queryable, Associations, Insertable, Identifiable, Debug)]
#[primary_key(result_id, inputs_id)]
#[belongs_to(RunResult, foreign_key = "result_id")]
#[belongs_to(IndicatorInputsExplicit, foreign_key = "inputs_id")]
#[table_name = "result_sets"]
pub struct ResultSet {
        pub result_id: i64,
        pub inputs_id: i64,
}
