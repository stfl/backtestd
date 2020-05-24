use super::signal_class::SignalClass;
use super::to_param_string::ToParamString;
use anyhow::{ensure, Context, Result};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct Indicator {
    pub name: String,
    pub filename: Option<String>,
    pub class: SignalClass,
    pub inputs: Vec<Vec<BigDecimal>>,
    pub buffers: Option<Vec<u8>>,
    pub params: Option<Vec<BigDecimal>>,
    pub shift: u8,
}

impl Indicator {
    pub fn to_param_string_vec(&self) -> Vec<String> {
        use super::signal_class::SignalClass;
        let mut res = vec![
            format!(
                "Indicator={}",
                match self.class {
                    SignalClass::Preset => &self.name,
                    // _ => self.filename.unwrap_or(self.name)
                    _ => match &self.filename {
                        Some(filename) => &filename,
                        _ => &self.name,
                    },
                }
            ),
            format!("SignalClass={}", self.class as u8),
            "Shift=".to_string() + &self.shift.to_string(),
        ];
        res.extend(
            self.inputs
                .iter()
                .enumerate()
                .map(|(i, input)| format!("input{}={}", i, input_param_str(input))),
        );
        if let Some(buffers) = &self.buffers {
            res.extend(
                buffers
                    .iter()
                    .enumerate()
                    .map(|(i, buf)| format!("buffer{}={}", i, buf)),
            );
        }
        if let Some(params) = &self.params {
            res.extend(
                params
                    .iter()
                    .enumerate()
                    .map(|(i, param)| format!("param{}={:.2}", i, param)),
            );
        }
        res
    }

    pub fn from_file(file: &str) -> Result<Self> {
        let json_file = File::open(Path::new(file))?;
        Ok(serde_json::from_reader(json_file)?)
    }

    pub fn to_file(&self, file: &str) -> Result<()> {
        let json_file = File::create(Path::new(file))?;
        Ok(serde_json::ser::to_writer_pretty(json_file, self)?)
    }

    pub fn count_inputs_crossed(&self) -> u64 {
        let lengths = self.count_input_length();
        if lengths.len() == 0 {
            return 0u64;
        }
        lengths.iter().fold(1u64, |prod, x| prod * x)
    }

    pub fn count_input_length(&self) -> Vec<u64> {
        use bigdecimal::ToPrimitive;
        self.inputs
            .iter()
            // .filter(|i| i.len() == 3 || i.len() == 4)
            .map(|i| match i.len() {
                3 => (((i[1].to_f32().unwrap() - &i[0].to_f32().unwrap()) + 1f32)
                    / &i[2].to_f32().unwrap())
                    .floor() as u64,
                4 => (((i[2].to_f32().unwrap() - &i[1].to_f32().unwrap()) + 1f32)
                    / &i[3].to_f32().unwrap())
                    .floor() as u64,
                _ => 1u64,
            })
            .collect()
    }

    pub fn slice_longest_input(&self) -> Option<Vec<Self>> {
        use bigdecimal::ToPrimitive;
        use std::cmp::Ordering;

        let index_of_max: Option<usize> = self
            .count_input_length()
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index);

        if let Some(i) = index_of_max {
            println!("longest input: {:?}", self.inputs[i]);
            let mut new_indis = vec![self.clone(), self.clone()];
            let start_idx = 0;
            if self.inputs[i].len() == 4 {
                let start_idx = 1;
            }
            let start = &self.inputs[i][start_idx];
            let stop = &self.inputs[i][start_idx + 1];
            let step = &self.inputs[i][start_idx + 2];

            let new_start = start + (stop - start) / 2;
            new_indis[0].inputs[i][start_idx + 1] = &new_start - step;
            new_indis[1].inputs[i][start_idx] = new_start;
            return Some(new_indis);
        }

        None
    }

    // fn parse_result_set(&self, result_params: &mut VecDeque<f32>) -> Self {
    //     Indicator {
    //         name: self.name.clone(),
    //         shift: self.shift,
    //         inputs: self
    //             .inputs
    //             .clone()
    //             .into_iter()
    //             .map(|inp| {
    //                 if (3..=4).contains(&inp.len()) {
    //                     vec![result_params
    //                         .pop_front()
    //                         .expect("no more params found in result")]
    //                 // TODO we MUST have a value here otherwise something went wrong with the test run
    //                 // TODO assert value is in range
    //                 } else {
    //                     inp
    //                 }
    //             })
    //             .collect(),
    //     }
    // }
}

fn input_param_str(input: &Vec<BigDecimal>) -> String {
    match input.len() {
        1 => format!("{:.2}||0||0||0||N", input[0]),
        3 => format!("0||{:.2}||{:.2}||{:.2}||Y", input[0], input[2], input[1]),
        4 => format!(
            "{:.2}||{:.2}||{:.2}||{:.2}||Y",
            input[0], input[1], input[3], input[2]
        ),
        e => panic!("wrong length of indicator params input: {}", e),
        // e => Err(anyhow!("wrong length of indicator params input: {}", e)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::params::signal_class::SignalClass::*;
    use crate::params::vec_to_bigdecimal;
    use crate::params::vec_vec_to_bigdecimal;
    use glob::glob;
    use std::path::Path;

    #[test]
    fn indi_to_param_vec() {
        let mut indi = Indicator {
            name: "ama".to_string(),
            filename: None,
            shift: 0,
            inputs: Vec::new(),
            buffers: None,
            params: None,
            class: Preset,
        };

        assert_eq!(
            indi.to_param_string_vec(),
            vec!["Indicator=ama", "SignalClass=0", "Shift=0"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );

        {
            // if a filename is set
            // if class Preset, name needs to be filled
            let mut indi = indi.clone();
            indi.filename = Some("ama.ex5".to_string());
            assert_eq!(
                indi.to_param_string_vec(),
                vec!["Indicator=ama", "SignalClass=0", "Shift=0"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            );

            indi.class = ZeroLineCross;
            assert_eq!(
                indi.to_param_string_vec(),
                vec!["Indicator=ama.ex5", "SignalClass=1", "Shift=0"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            );

            // Panics
            // indi.filename = None;
            // assert_eq!(
            //     indi.to_param_string_vec(),
            //     vec!["Indicator=ama.ex5", "SignalClass=ZeroLineCross", "Shift=0"]
            //         .iter()
            //         .map(|s| s.to_string())
            //         .collect::<Vec<String>>()
            // );
        }

        {
            let mut indi = indi.clone();
            indi.buffers = Some(vec![1u8]);
            assert_eq!(
                indi.to_param_string_vec(),
                vec!["Indicator=ama", "SignalClass=0", "Shift=0", "buffer0=1"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            );
        }

        {
            let mut indi = indi.clone();
            indi.params = Some(vec_to_bigdecimal(vec![1.]));
            assert_eq!(
                indi.to_param_string_vec(),
                vec!["Indicator=ama", "SignalClass=0", "Shift=0", "param0=1.00",]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            );

            indi.params = Some(vec_to_bigdecimal(vec![1., 4.55]));
            assert_eq!(
                indi.to_param_string_vec(),
                vec![
                    "Indicator=ama",
                    "SignalClass=0",
                    "Shift=0",
                    "param0=1.00",
                    "param1=4.55",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
            );
        }

        indi.shift = 7;
        assert_eq!(
            indi.to_param_string_vec(),
            vec!["Indicator=ama", "SignalClass=0", "Shift=7",]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );

        indi.inputs.push(vec_to_bigdecimal(vec![3.]));
        assert_eq!(
            indi.to_param_string_vec(),
            vec![
                "Indicator=ama",
                "SignalClass=0",
                "Shift=7",
                "input0=3.00||0||0||0||N",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        );

        indi.inputs.push(vec_to_bigdecimal(vec![4.]));
        assert_eq!(
            indi.to_param_string_vec(),
            vec![
                "Indicator=ama",
                "SignalClass=0",
                "Shift=7",
                "input0=3.00||0||0||0||N",
                "input1=4.00||0||0||0||N",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        );

        indi.inputs.push(vec_to_bigdecimal(vec![10., 200., 0.5]));
        assert_eq!(
            indi.to_param_string_vec(),
            vec![
                "Indicator=ama",
                "SignalClass=0",
                "Shift=7",
                "input0=3.00||0||0||0||N",
                "input1=4.00||0||0||0||N",
                "input2=0||10.00||0.50||200.00||Y",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        );

        indi.inputs
            .push(vec_to_bigdecimal(vec![15., 10., 20., 0.5]));
        assert_eq!(
            indi.to_param_string_vec(),
            vec![
                "Indicator=ama",
                "SignalClass=0",
                "Shift=7",
                "input0=3.00||0||0||0||N",
                "input1=4.00||0||0||0||N",
                "input2=0||10.00||0.50||200.00||Y",
                "input3=15.00||10.00||0.50||20.00||Y",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        );

        indi.class = ZeroLineCross;
        assert_eq!(
            indi.to_param_string_vec(),
            vec![
                "Indicator=ama",
                "SignalClass=1",
                "Shift=7",
                "input0=3.00||0||0||0||N",
                "input1=4.00||0||0||0||N",
                "input2=0||10.00||0.50||200.00||Y",
                "input3=15.00||10.00||0.50||20.00||Y",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        );
    }

    #[test]
    fn load_indicators_test() {
        for entry in glob("config/indicator/*/*").unwrap().filter_map(Result::ok) {
            let indi = serde_any::from_file::<Indicator, _>(entry);
            // println!("{:#?}", indi);
            assert!(indi.is_ok());
        }
    }

    #[test]
    fn count_crossed_test() {
        let mut indi = Indicator {
            name: "ama".to_string(),
            filename: None,
            shift: 0,
            inputs: Vec::new(),
            buffers: None,
            params: None,
            class: Preset,
        };

        assert_eq!(indi.count_inputs_crossed(), 0);

        indi.inputs = vec_vec_to_bigdecimal(vec![vec![1.]]);
        assert_eq!(indi.count_inputs_crossed(), 1);

        // indi.inputs.push(vec_to_bigdecimal(vec![]))
        indi.inputs.push(vec_to_bigdecimal(vec![11., 15., 1.]));
        assert_eq!(indi.count_inputs_crossed(), 5);

        indi.inputs
            .push(vec_to_bigdecimal(vec![15., 11., 20., 0.5]));
        assert_eq!(indi.count_inputs_crossed(), 100);

        indi.slice_longest_input();
    }

    #[test]
    fn slice_indicator_test() {
        let mut indi = Indicator {
            name: "ama".to_string(),
            filename: None,
            shift: 0,
            inputs: Vec::new(),
            buffers: None,
            params: None,
            class: Preset,
        };

        assert!(indi.slice_longest_input().is_none());

        indi.inputs = vec_vec_to_bigdecimal(vec![vec![10., 20., 1.]]);
        let mut indis = vec![indi.clone(), indi.clone()];
        indis[0].inputs = vec_vec_to_bigdecimal(vec![vec![10., 14., 1.]]);
        indis[1].inputs = vec_vec_to_bigdecimal(vec![vec![15., 20., 1.]]);
        assert_eq!(indi.slice_longest_input(), Some(indis));

        indi.inputs = vec_vec_to_bigdecimal(vec![vec![10., 20., 1.], vec![10., 20., 0.5]]);
        let mut indis = vec![indi.clone(), indi.clone()];
        indis[0].inputs = vec_vec_to_bigdecimal(vec![vec![10., 20., 1.], vec![10., 14.5, 0.5]]);
        indis[1].inputs = vec_vec_to_bigdecimal(vec![vec![10., 20., 1.], vec![15., 20., 0.5]]);
        assert_eq!(indi.slice_longest_input(), Some(indis));

        indi.inputs = vec_vec_to_bigdecimal(vec![vec![10., 20., 1.], vec![20., 10., -0.5]]);
        let mut indis = vec![indi.clone(), indi.clone()];
        indis[0].inputs = vec_vec_to_bigdecimal(vec![vec![10., 20., 1.], vec![20., 15.5, -0.5]]);
        indis[1].inputs = vec_vec_to_bigdecimal(vec![vec![10., 20., 1.], vec![15., 10., -0.5]]);
        assert_eq!(indi.slice_longest_input(), Some(indis));
    }
}
