use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::indi_func::IndiFunc;
use super::indicator::Indicator;

use super::to_param_string::ToParamString;

use derive_more::{Constructor, Deref, DerefMut, From, Index, IndexMut, IntoIterator};
use std::collections::HashMap;

#[derive(
    Constructor,
    IntoIterator,
    Debug,
    PartialEq,
    Clone,
    Deserialize,
    Serialize,
    Deref,
    From,
    DerefMut,
)]
pub struct IndicatorSet(HashMap<IndiFunc, Indicator>);

impl ToParamString for IndicatorSet {
    fn to_param_string(&self) -> String {
        self.to_param_string_vec().join("\n")
    }
}

impl IndicatorSet {
    pub fn to_param_string_vec(&self) -> Vec<String> {
        use super::signal_class::SignalClass::*;

        self.clone()
            .into_iter()
            .map(|(func, indi)| {
                // the hashmap <func, indicator>
                indi.to_param_string_vec()
                    .iter()
                    .map(|indi_param| func.to_string().to_owned() + "_" + indi_param)
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect()
    }
}

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::params::indi_func::IndiFunc::*;
    use crate::params::signal_class::SignalClass::*;
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn to_param_string_test() {
        let mut set: IndicatorSet = [(
            Confirm,
            Indicator {
                name: "ama".to_string(),
                filename: None,
                shift: 0,
                inputs: Vec::new(),
                buffers: None,
                params: None,
                class: Preset,
            },
        )]
        .iter()
        .cloned()
        .collect::<HashMap<IndiFunc, Indicator>>()
        .into();

        assert_eq!(
            vec![
                "Confirm_Indicator=ama",
                "Confirm_SignalClass=0",
                "Confirm_Shift=0"
            ],
            set.to_param_string_vec()
        );

        set.insert(
            Baseline,
            Indicator {
                name: "ma".to_string(),
                filename: None,
                shift: 0,
                inputs: Vec::new(),
                buffers: None,
                params: None,
                class: Preset,
            },
        );
        assert_eq!(
            vec![
                "Confirm_Indicator=ama",
                "Confirm_SignalClass=0",
                "Confirm_Shift=0",
                "Baseline_Indicator=ma",
                "Baseline_SignalClass=0",
                "Baseline_Shift=0"
            ]
            .sort(),
            set.to_param_string_vec().sort()
        );
    }
}
