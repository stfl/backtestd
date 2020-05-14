use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use anyhow::{ensure, Context, Result};

use super::indicator::Indicator;
use super::indi_func::IndiFunc;

use super::to_param_string::ToParamString;

use derive_more::{From, IntoIterator, Index, Deref, DerefMut, IndexMut};
use std::collections::HashMap;

#[derive(IntoIterator, Debug, PartialEq, Clone, Deserialize, Serialize, Deref)]
pub struct IndicatorSet(HashMap<IndiFunc, Indicator>);

impl ToParamString for IndicatorSet {
    fn to_param_string(&self) -> String {
        use super::signal_class::SignalClass::*;

        self.clone()
            .into_iter()
            .map(|(func, indi)| {
                // the hashmap <func, indicator>
                indi.to_param_string_vec()
                    .iter()
                    .map(|indi_param| func.to_string().to_owned() + indi_param)
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n") // join the strings for all Indicators
    }
}

// #[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
// pub struct IndicatorSet {
//     pub confirm: Option<Indicator>,
//     pub confirm2: Option<Indicator>,
//     pub confirm3: Option<Indicator>,
//     pub exit: Option<Indicator>,
//     pub cont: Option<Indicator>,
//     pub baseline: Option<Indicator>,
//     pub volume: Option<Indicator>,
// }

// impl IndicatorSet {
//     fn to_params_config(&self) -> Result<String> {
//         let mut string = String::new();
//         match &self.confirm {
//             Some(i) => string.push_str(&i.to_params_config("Confirm")?),
//             _ => (),
//         }
//         match &self.confirm2 {
//             Some(i) => string.push_str(&i.to_params_config("Confirm2")?),
//             _ => (),
//         }
//         match &self.confirm3 {
//             Some(i) => string.push_str(&i.to_params_config("Confirm3")?),
//             _ => (),
//         }
//         match &self.cont {
//             Some(i) => string.push_str(&i.to_params_config("Continue")?),
//             _ => (),
//         }
//         match &self.exit {
//             Some(i) => string.push_str(&i.to_params_config("Exit")?),
//             _ => (),
//         }
//         match &self.baseline {
//             Some(i) => string.push_str(&i.to_params_config("Baseline")?),
//             _ => (),
//         }
//         match &self.volume {
//             Some(i) => string.push_str(&i.to_params_config("Volume")?),
//             _ => (),
//         }

//         Ok(string)
//     }

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

// TODO impl Iterator
// create a indi_list Vec<&Indicator>
// return indi_list.iter();


#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;
    use crate::params::signal_class::SignalClass::*;


}