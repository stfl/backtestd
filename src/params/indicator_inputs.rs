use super::to_param_string::ToParamString;
use crate::database::indicator::IndicatorInput;
use crate::database::indicator_inputs_explicit::IndicatorInputsExplicit;
use crate::database::IndicatorInputs;
use bigdecimal::BigDecimal;

impl IndicatorInputs {
    pub fn to_param_string_vec(&self) -> Vec<String> {
        self.clone()
            .into_iter()
            .map(|input| input.to_param_string())
            .collect()
    }
}

impl ToParamString for IndicatorInput {
    fn to_param_string(&self) -> String {
        format!(
            "_double{idx}={input:.2}||{start:.2}||{step:.2}||{stop:.2}||{optimize}",
            idx = self.index,
            input = self.input.as_ref().unwrap_or(&BigDecimal::from(0)), // TODO creating a BigDecimal in case of None is kinda useless
            start = self.start.as_ref().unwrap_or(&BigDecimal::from(0)),
            stop = self.stop.as_ref().unwrap_or(&BigDecimal::from(0)),
            step = self.step.as_ref().unwrap_or(&BigDecimal::from(0)),
            optimize = match self
                .start
                .as_ref()
                .and(self.step.as_ref())
                .and(self.step.as_ref())
            {
                Some(_) => "Y",
                None => "N",
            }
        )
    }
}
