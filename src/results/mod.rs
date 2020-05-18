pub mod xml_reader;
// pub mod csv_writer;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ResultRow {
    pass: u64,
    result: f64,
    profit: f32,
    expected_payoff: f32,
    profit_factor: f32,
    recovery_factor: f32,
    sharpe_ratio: f32,
    custom: f64,
    equity_dd: f32,
    trades: u32,
    params: Vec<f32>,
}

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub struct BacktestResult {
//     // indi_set: IndicatorSet,
//     params: Vec<String>,
//     profit: f32,
//     result: f32,
//     trades: u32,
// }
