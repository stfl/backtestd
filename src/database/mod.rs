// TODO make non pub and export here

pub mod expert_inputs;
pub mod indicator;
pub mod indicator_inputs_explicit;
pub mod indicator_set;
pub mod result;
pub mod result_set;
pub mod run;
pub mod run_session;
pub mod schema;
pub mod symbols;

pub mod legacy_import;

use crate::params::to_param_string::ToParamString;
use derive_more::{From, IntoIterator, Index, Deref, DerefMut, IndexMut};
use diesel::{Identifiable, PgConnection, QueryResult, Queryable};
use std::collections::HashMap;

pub use indicator::{IndiFunc, IndicatorInput, SignalClass};
pub use indicator_inputs_explicit::IndicatorInputsExplicit;

#[derive(IntoIterator, Debug, PartialEq, Clone, Deserialize, Serialize, Deref)]
pub struct IndicatorSet(HashMap<IndiFunc, Indicator>);

#[derive(IntoIterator, Debug, PartialEq, Clone, Deserialize, Serialize, From, Index, Deref, DerefMut, IndexMut)]
// pub struct IndicatorInputs(HashMap<i16, indicator::IndicatorInput>);
pub struct IndicatorInputs(Vec<IndicatorInput>);

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Deref, Index)]
pub struct Indicator {
    #[deref]
    pub indi: indicator::Indicator,
    #[index]
    pub inputs: IndicatorInputs,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Deref)]
pub struct IndicatorExplicit {
    #[deref]
    pub indi: indicator::Indicator,
    pub inputs: IndicatorInputsExplicit,
}

// #[derive(Debug, PartialEq, Clone, Deserialize)]
// pub struct NewIndicatorExplicit {
//     indi: indicator::Indicator,
//     inputs: indicator_inputs_explicit::IndicatorInputsExplicit,
// }

trait ToDb {
    fn store(&self, conn: &PgConnection) -> QueryResult<usize>;
}

trait NewToDb<T> {
    fn store(&self, conn: &PgConnection) -> QueryResult<T>;
}

// TODO
// trait FromDb {
//     type S;
//     fn load(id: diesel::Id, conn: &PgConnection) -> QueryResult<Self::S>;
// }
