use serde::{Deserialize, Serialize};

use super::indi_func::IndiFunc;
use super::indicator::Indicator;

use super::to_param_string::ToParamString;

use derive_more::{Constructor, Deref, DerefMut, From, IntoIterator};
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
    Default,
)]
pub struct IndicatorSet(HashMap<IndiFunc, Indicator>);

impl ToParamString for IndicatorSet {
    fn to_param_string(&self) -> String {
        self.to_param_string_vec().join("\n")
    }
}

impl IndicatorSet {
    pub fn to_param_string_vec(&self) -> Vec<String> {
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

    pub fn count_inputs_crossed(&self) -> u64 {
        let lengths = self.count_input_length();
        if lengths.len() == 0 {
            return 0u64;
        }
        lengths.iter().fold(1u64, |prod, (_, x)| prod * x)
    }

    pub fn count_input_length(&self) -> HashMap<IndiFunc, u64> {
        self.iter()
            .map(|(f, i)| (f.clone(), i.count_inputs_crossed()))
            .collect()
    }

    pub fn slice_longest_input(&self) -> Option<Vec<Self>> {
        let lengths = self.count_input_length();
        let longest = lengths.iter().max_by(|(_, a), (_, b)| a.cmp(b));

        if let Some((f, l)) = longest {
            debug!("longest Indicator {:?} with {} inputs", f, l);

            // get the longest indicator and slice it
            let mut new_sets = Vec::with_capacity(2);
            for new_indi in self.get(&f).unwrap().slice_longest_input().unwrap() {
                let mut new_set = self.clone(); // clone the set
                let _ = new_set.insert(f.clone(), new_indi);
                new_sets.push(new_set);
            }
            return Some(new_sets);
        }

        error!("longest indicator not found in {:?}", self);
        None
    }

    pub fn slice_recursive(self, target_length: u64) -> Vec<Self> {
        let cnt = self.count_inputs_crossed();
        if cnt < target_length {
            debug!("returning from slicing at length {}", cnt);
            return vec![self];
        }
        self.slice_longest_input()
            .unwrap()
            .into_iter()
            .map(|s| s.slice_recursive(target_length))
            .flatten()
            .collect()
    }

    pub fn _new_test(num: usize) -> Self {
        use IndiFunc::*;
        [
            Confirm, Confirm2, Confirm3, Baseline, Volume, Continue, Exit,
        ]
        .iter()
        .take(num)
        .map(|f| (*f, Indicator::_new_test(*f, 0)))
        .collect::<HashMap<IndiFunc, Indicator>>()
        .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::params::_vec_vec_to_bigdecimal;
    use crate::params::indi_func::IndiFunc::*;
    use crate::params::signal_class::SignalClass::*;
    use std::collections::HashMap;

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

    #[test]
    fn slice_indi_set_test() {
        let set: IndicatorSet = [
            (
                Confirm,
                Indicator {
                    name: "ama".to_string(),
                    filename: None,
                    shift: 0,
                    inputs: _vec_vec_to_bigdecimal(vec![vec![10., 20., 1.]]),
                    buffers: None,
                    params: None,
                    class: Preset,
                },
            ),
            (
                Confirm2,
                Indicator {
                    name: "ama2".to_string(),
                    filename: None,
                    shift: 0,
                    inputs: _vec_vec_to_bigdecimal(vec![vec![10., 20., 0.5]]),
                    buffers: None,
                    params: None,
                    class: Preset,
                },
            ),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<IndiFunc, Indicator>>()
        .into();

        set.slice_longest_input();

        let mut new_set = vec![set.clone(), set.clone()];
        new_set[0].get_mut(&Confirm2).unwrap().inputs =
            _vec_vec_to_bigdecimal(vec![vec![10., 14.5, 0.5]]);
        new_set[1].get_mut(&Confirm2).unwrap().inputs =
            _vec_vec_to_bigdecimal(vec![vec![15., 20., 0.5]]);
        assert_eq!(set.slice_longest_input(), Some(new_set));
    }
}
