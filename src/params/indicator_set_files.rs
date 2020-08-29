use super::indi_func::IndiFunc;
use super::indicator::Indicator;
use super::indicator_set::IndicatorSet;
use std::collections::HashMap;
use std::path::PathBuf;

impl From<HashMap<IndiFunc, PathBuf>> for IndicatorSet {
    fn from(s: HashMap<IndiFunc, PathBuf>) -> Self {
        s.into_iter()
            .map(|(func, indi_file)| {
                (
                    func,
                    serde_any::from_file::<Indicator, _>(indi_file).unwrap(),
                )
            })
            .collect::<HashMap<IndiFunc, Indicator>>()
            .into()
    }
}
