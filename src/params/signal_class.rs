use serde::{Deserialize, Serialize};

// FIXME define values same as in MQL
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum SignalClass {
   Preset = 0,
   ZeroLineCross,
   TwoLinesCross,
   TwoLinesTwoLevelsCross,
   TwoLevelsCross,
   PriceCross,
   PriceCrossInverted,
   Semaphore,
   TwoLinesColorChange,
   ColorChange,
   BothLinesTwoLevelsCross,
   BothLinesLevelCross,
   SaturationLevels,
   SaturationLines,
   BothLinesSaturationLevels,
   SlopeChange,
   TwoLinesSlopeChange,
}