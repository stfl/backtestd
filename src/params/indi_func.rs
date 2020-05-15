use derive_more::Display;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize, Display)]
pub enum IndiFunc {
    Confirm,
    Confirm2,
    Confirm3,
    Baseline,
    Volume,
    Continue,
    Exit,
}
