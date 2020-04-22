// use super::super::params::Indicator;
use super::schema::*;
use crate::database::indicator::*;
// use super::*;

use bigdecimal::BigDecimal;
// use diesel::prelude::*;

#[derive(Queryable, Associations, Identifiable, Debug, Clone)]
#[primary_key(inputs_id)]
#[table_name = "indicator_inputs_explicit"]
#[belongs_to(Indicator, foreign_key = "indicator_id")]
pub struct IndicatorInputsExplicit {
    pub inputs_id: i64,
    pub indicator_id: i32,
    pub input0: Option<BigDecimal>,
    pub input1: Option<BigDecimal>,
    pub input2: Option<BigDecimal>,
    pub input3: Option<BigDecimal>,
    pub input4: Option<BigDecimal>,
    pub input5: Option<BigDecimal>,
    pub input6: Option<BigDecimal>,
    pub input7: Option<BigDecimal>,
    pub input8: Option<BigDecimal>,
    pub input9: Option<BigDecimal>,
    pub input10: Option<BigDecimal>,
    pub input11: Option<BigDecimal>,
    pub input12: Option<BigDecimal>,
    pub input13: Option<BigDecimal>,
    pub input14: Option<BigDecimal>,
}

#[derive(Insertable, Debug)]
#[table_name = "indicator_inputs_explicit"]
pub struct NewIndicatorInputsExplicit {
    pub indicator_id: i32,
    pub input0: Option<BigDecimal>,
    pub input1: Option<BigDecimal>,
    pub input2: Option<BigDecimal>,
    pub input3: Option<BigDecimal>,
    pub input4: Option<BigDecimal>,
    pub input5: Option<BigDecimal>,
    pub input6: Option<BigDecimal>,
    pub input7: Option<BigDecimal>,
    pub input8: Option<BigDecimal>,
    pub input9: Option<BigDecimal>,
    pub input10: Option<BigDecimal>,
    pub input11: Option<BigDecimal>,
    pub input12: Option<BigDecimal>,
    pub input13: Option<BigDecimal>,
    pub input14: Option<BigDecimal>,
}
