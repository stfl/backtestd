use super::to_param_string::ToParamString;
use serde::{Deserialize, Serialize};
use anyhow::{ensure, Context, Result};
use bigdecimal::BigDecimal;
use super::signal_class::SignalClass;

use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct Indicator {
    pub name: String,
    pub inputs: Vec<Vec<BigDecimal>>,
    pub shift: u8,
    pub buffers: Option<Vec<u8>>,
    pub params: Option<Vec<BigDecimal>>,
    pub class: SignalClass,
}

impl Indicator {
    pub fn to_param_string_vec(&self) -> Vec<String> {
        let mut res = vec!["Indicator=".to_string() + &self.name,
             format!("SignalClass={:?}", self.class),
             "Shift=".to_string() + &self.shift.to_string(),
        ];
        res.extend(self
            .inputs
            .iter()
            .enumerate()
            .map(|(i, input)| format!("double{}={}", i, input_param_str(input))));
        res
    }
}

impl Indicator {
    // maybe implement io::Write instead?
    // pub fn to_params_config<'a>(&self, use_case: &'a str) -> Result<String> {
    //     let mut string: String = format!(
    //         "{use_case}_Indicator={name}\n",
    //         use_case = use_case,
    //         name = self.name
    //     );
    //     for (i, inp) in self.inputs.iter().enumerate() {
    //         string.push_str(&format!(
    //             // TODO remove _Double
    //             "{use_case}_double{idx}=\
    //              {input_value}\n",
    //             use_case = use_case,
    //             input_value = input_param_str(inp)?,
    //             idx = i,
    //         ));
    //     }
    //     if self.shift > 0 {
    //         string.push_str(&format!("{}_Shift={}\n", use_case, self.shift));
    //     }

    //     Ok(string)
    // }

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
        3 => format!(
            "0||{:.2}||{:.2}||{:.2}||Y",
            input[0], input[2], input[1]
        ),
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
    use std::path::Path;
    use crate::params::signal_class::SignalClass::*;

    #[test]
    fn indi_to_param_vec() {
        let mut indi = Indicator {
            name: "ama".to_string(),
            shift: 0,
            inputs: Vec::new(),
            buffers: None,
            params: None,
            class: Preset,
        };

        assert_eq!(indi.to_param_string_vec(),
                   vec!["Indicator=ama",
                        "SignalClass=Preset",
                        "Shift=0"].iter().map(|s| s.to_string()).collect::<Vec<String>>())
    }
}