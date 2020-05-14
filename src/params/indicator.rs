use super::to_param_string::ToParamString;
use crate::database::IndiFunc;
use crate::database::Indicator;
use crate::database::SignalClass;

impl Indicator {
    pub fn to_param_string(&self, func: Option<IndiFunc>) -> String {
        format!(
            "{func}_Indicator={name}
{func}_SignalClass={class}
{func}_Shift={shift}
", // indicator specific config
            func = func.unwrap_or(self.indi.func).to_string(),
            name = self.indi.indicator_name,
            class = self.indi.class.unwrap_or(SignalClass::Preset).to_string().to_owned(),
            shift = self.indi.shift
        ) + &self
            .inputs
            .to_param_string_vec() // convert the inputs into ["_double0_1.00||2.00||...", ..]
            .iter()
            .map(|input_str| func.unwrap_or(self.indi.func).to_string() + input_str) // prepend to all of them the function
            .collect::<Vec<String>>()
            .join("\n") // make a single string
    }
}

//     // maybe implement io::Write instead?
//     pub fn to_params_config<'a>(&self, use_case: &'a str) -> Result<String> {
//         let mut string: String = format!(
//             "{use_case}_Indicator={name}\n",
//             use_case = use_case,
//             name = self.name
//         );
//         for (i, inp) in self.inputs.iter().enumerate() {
//             string.push_str(&format!(
//                 // TODO remove _Double
//                 "{use_case}_double{idx}=\
//                  {input_value}\n",
//                 use_case = use_case,
//                 input_value = input_param_str(inp)?,
//                 idx = i,
//             ));
//         }
//         if self.shift > 0 {
//             string.push_str(&format!("{}_Shift={}\n", use_case, self.shift));
//         }

//         Ok(string)
//     }

// pub fn to_file(&self, file: &str) -> Result<()> {
//     let json_file = File::create(Path::new(file))?;
//     Ok(serde_json::ser::to_writer_pretty(json_file, self)?)
// }

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

// legacy_indicator::put: &Vec<BigDecimal>) -> Result<String> {
//     match input.len() {
//         1 => Ok(format!("{:.2}||0||0||0||N", input[0])),
//         3 => Ok(format!(
//             "0||{:.2}||{:.2}||{:.2}||Y",
//             input[0], input[2], input[1]
//         )),
//         4 => Ok(format!(
//             "{:.2}||{:.2}||{:.2}||{:.2}||Y",
//             input[0], input[1], input[3], input[2]
//         )),
//         e => Err(anyhow!("wrong length of indicator params input: {}", e)),
//     }
// }
