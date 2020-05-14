pub use crate::database::{Indicator, IndicatorInputs, IndicatorSet};
// use super::Indicator;

use anyhow::{ensure, Context, Result};

use super::{to_param_string::ToParamString, to_terminal_config::ToTerminalConfig};

impl ToParamString for IndicatorSet {
    fn to_param_string(&self) -> String {
        use crate::database::indicator::SignalClass::*;

        self.clone()
            .into_iter()
            .map(|(func, indi)| {
                // the hashmap <func, indicator>
                indi.to_param_string(Some(func))
            })
            .collect::<Vec<String>>()
            .join("\n") // join the strings for all Indicators
    }
}

// impl IndicatorSet {
// pub fn parse_result_set(&self, mut result_params: VecDeque<f32>) -> IndicatorSet {
//     IndicatorSet {
//         confirm: self
//             .confirm
//             .as_ref()
//             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//         confirm2: self
//             .confirm2
//             .as_ref()
//             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//         confirm3: self
//             .confirm3
//             .as_ref()
//             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//         exit: self
//             .exit
//             .as_ref()
//             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//         cont: self
//             .cont
//             .as_ref()
//             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//         baseline: self
//             .baseline
//             .as_ref()
//             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//         volume: self
//             .volume
//             .as_ref()
//             .and_then(|i| Some(i.parse_result_set(&mut result_params))),
//     }
// }
// }
