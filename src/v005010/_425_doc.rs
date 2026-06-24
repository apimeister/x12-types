use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 425 - Rail Waybill Request
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Rail Waybill Request Transaction Set (425) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used to request a copy of a revenue or movement waybill from the origin or previous rail carrier when the requesting carrier cannot locate the original transmission.
///
/// Heading: ST.
/// Detail LOOP ZT: ZT waybill request information with optional F9 origin and D9 destination
///   stations.
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _425 {
    pub st: ST,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "ZT")]
    pub loop_zt: Vec<_425LoopZt>,
    pub se: SE,
}

/// Waybill request loop (ZT) with origin (F9) and destination (D9) stations.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _425LoopZt {
    pub zt: ZT,
    pub f9: Option<F9>,
    pub d9: Option<D9>,
}
