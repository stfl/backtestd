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
}
