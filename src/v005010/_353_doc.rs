use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 353 - Customs Events Advisory Details
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Customs Events Advisory Details Transaction Set (353) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set enables carriers to communicate customs-related events to authorities, including cargo movements, conveyance arrivals/departures, and custodial liability transfers.
///
/// Heading: ST, M10, P4.
/// Detail LOOP M15: M15 customs events advisory details with K1 remarks.
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _353 {
    pub st: ST,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m10: Option<M10>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p4: Option<P4>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "M15")]
    pub loop_m15: Vec<_353LoopM15>,
    pub se: SE,
}

/// Customs events advisory loop (M15 + K1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _353LoopM15 {
    pub m15: M15,
    pub k1: Vec<K1>,
}
