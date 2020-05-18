use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// FIXME define values same as in MQL
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum SignalClass {
    Preset = 0,
    ZeroLineCross = 1,
    TwoLinesCross = 2,
    TwoLinesTwoLevelsCross = 3,
    TwoLevelsCross = 4,
    PriceCross = 5,
    PriceCrossInverted = 6,
    Semaphore = 7,
    TwoLinesColorChange = 8,
    ColorChange = 9,
    BothLinesTwoLevelsCross = 10,
    BothLinesLevelCross = 11,
    SaturationLevels = 12,
    SaturationLines = 13,
    BothLinesSaturationLevels = 14,
    SlopeChange = 15,
    TwoLinesSlopeChange = 16,
}

impl Default for SignalClass {
    fn default() -> Self {
        SignalClass::Preset
    }
}
