use crate::database::indicator_set::IndicatorSet;
use crate::database::run::Run;
use crate::database::schema::results;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[primary_key(result_id)]
#[belongs_to(Run, foreign_key = "run_id")]
#[table_name = "results"]
pub struct RunResult {
    pub result_id: i64,
    pub run_id: i64,
    pub result: f32,
    pub profit: f32,
    pub expected_payoff: f32,
    pub profit_factor: f32,
    pub recovery_factor: f32,
    pub sharpe_ratio: f32,
    pub custom_result: f32,
    pub equity_drawdown: f32,
    pub trades: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "results"]
pub struct NewRunResult {
    pub run_id: i64,
    pub result: f32,
    pub profit: f32,
    pub expected_payoff: f32,
    pub profit_factor: f32,
    pub recovery_factor: f32,
    pub sharpe_ratio: f32,
    pub custom_result: f32,
    pub equity_drawdown: f32,
    pub trades: i32,
}
