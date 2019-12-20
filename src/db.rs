use super::schema::*;
use super::params::*;
use chrono::prelude::*;
use chrono::DateTime;
use diesel::data_types::Cents;
use serde_json::Value;

/* #[derive(Queryable, Insertable, Identifiable, AsChangeset)]
 * #[table_name="indicators"]
 * pub struct IndicatorDb {
 *     id: i32,
 *     parent_id: Option<i32>,
 *     name: String,
 *     inputs: Vec<Vec<f32>>,
 *     shift: i16,
 * } */

/* #[derive(Queryable, Identifiable, Insertable, AsChangeset)]
 * #[table_name="indicators"]
 * // #[belongs_to(IndicatorDb, foreign_key = "parent_id")]
 * pub struct IndicatorDb {
 *     id: i32,
 *     // parent_id: Option<i32>,
 *     name: String,
 *     // inputs: Vec<Vec<f32>>,
 *     value0: Option<f32>,
 *     range0: Option<Vec<f32>>,
 *     value1: Option<f32>,
 *     range1: Option<Vec<f32>>,
 *     value2: Option<f32>,
 *     range2: Option<Vec<f32>>,
 *     value3: Option<f32>,
 *     range3: Option<Vec<f32>>,
 *     value4: Option<f32>,
 *     range4: Option<Vec<f32>>,
 *     value5: Option<f32>,
 *     range5: Option<Vec<f32>>,
 *     value6: Option<f32>,
 *     range6: Option<Vec<f32>>,
 *     value7: Option<f32>,
 *     range7: Option<Vec<f32>>,
 *     value8: Option<f32>,
 *     range8: Option<Vec<f32>>,
 *     value9: Option<f32>,
 *     range9: Option<Vec<f32>>,
 *     value10: Option<f32>,
 *     range10: Option<Vec<f32>>,
 *     value11: Option<f32>,
 *     range11: Option<Vec<f32>>,
 *     value12: Option<f32>,
 *     range12: Option<Vec<f32>>,
 *     value13: Option<f32>,
 *     range13: Option<Vec<f32>>,
 *     value14: Option<f32>,
 *     range14: Option<Vec<f32>>,
 *     value15: Option<f32>,
 *     range15: Option<Vec<f32>>,
 *     value16: Option<f32>,
 *     range16: Option<Vec<f32>>,
 *     value17: Option<f32>,
 *     range17: Option<Vec<f32>>,
 *     value18: Option<f32>,
 *     range18: Option<Vec<f32>>,
 *     value19: Option<f32>,
 *     range19: Option<Vec<f32>>,
 *     shift: i16,
 * } */

/* #[derive(Queryable, Insertable, Associations, AsChangeset)]
 * // #[derive(Identifiable)]
 * #[table_name="indicator_samples"]
 * #[belongs_to(Indicator)]
 * pub struct IndicatorSampleDb {
 *     // id: i32,
 *     indicator_id: i32,
 *     // inputs: Vec<f32>,
 *     value0: Option<f32>,
 *     value1: Option<f32>,
 *     value2: Option<f32>,
 *     value3: Option<f32>,
 *     value4: Option<f32>,
 *     value5: Option<f32>,
 *     value6: Option<f32>,
 *     value7: Option<f32>,
 *     value8: Option<f32>,
 *     value9: Option<f32>,
 *     value10: Option<f32>,
 *     value11: Option<f32>,
 *     value12: Option<f32>,
 *     value13: Option<f32>,
 *     value14: Option<f32>,
 *     value15: Option<f32>,
 *     value16: Option<f32>,
 *     value17: Option<f32>,
 *     value19: Option<f32>,
 *     shift: i16,
 * }
 *  */
#[derive(Queryable, Insertable, Associations)]
#[table_name="indicator_sets"]
#[belongs_to(Run)]
#[belongs_to(Indicator)]
pub struct IndicatorSetDb {
    run_id: i32,
    indicator_id: i32,
    indi_type: String,
}

/* #[derive(Queryable, Insertable, Associations)]
 * // #[table_name="indicator_sample_sets"]
 * #[belongs_to(Sample)]
 * #[belongs_to(IndicatorSampleDb, foreign_key="indi_sample_id")]
 * pub struct IndicatorSampleSet {
 *     sample_id: i32,
 *     indi_sample_id: i32,
 *     indi_type: String,
 * }
 *  */
#[derive(Queryable, Insertable, Associations, AsChangeset)]
// #[derive(Identifiable)]
// #[table_name="results"]
pub struct Run {
    // id: i32,
    timestamp: DateTime<Utc>,
    result: f32,
    profit: f32,
    trades: i32,
    input_params: Option<Value>,
}

#[derive(Queryable, Insertable, Associations)]
// #[derive(Identifiable)]
// #[table_name="sample_results"]
#[belongs_to(Run, foreign_key="run_id")]
pub struct Sample {
    // id: i32,
    run_id: i32,
    result: f32,
    profit: f32,
    trades: i32,
    params: Vec<f32>,
}

/* #[derive(Queryable, Insertable, Identifiable, Associations, AsChangeset)]
 * #[table_name="run_params"]
 * #[belongs_to(RunDb, foreign_key="result_id")]
 * pub struct RunParamsDb {
 *     id: i32,
 *     result_id: i32,
 *     name: String,
 *     date_from: NaiveDate,
 *     date_to: NaiveDate,
 *     backtest_model: u8,
 *     optimize: u8,
 *     optimize_crit: u8,
 *     visual: bool,
 *     symbols: Vec<String>,
 * } */
