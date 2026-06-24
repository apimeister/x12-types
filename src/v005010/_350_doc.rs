use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 350 - Customs Status Information
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Customs Status Information Transaction Set (350) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by Customs to provide carriers and other interested parties with the disposition (release, hold, or other status) of cargo shipments. The P4 (import) and BA1 (export) loops are mutually exclusive.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0100 | ST | Transaction Set Header | M | 1
/// 0200 | M10 | Manifest Identifying Information | O | 1
/// LOOP ID - P4 | 20
/// 0400 | P4 | Port Information | M | 1
/// 0450 | V9 | Event Detail | O | 20
/// 0470 | VID | Conveyance Identification | O | 9999
/// 0500 | K1 | Remarks | O | 4
/// P4 -> LOOP ID - X4 | 9999
/// 0600 | X4 | Customs Release Information | M | 1
/// 0700 | K1 | Remarks | O | 4
/// 0710 | N9 | Extended Reference Information | O | 999
/// 0810 | N7 | Equipment Details | O | 999
/// LOOP ID - BA1 | 999
/// 0850 | BA1 | Export Shipment Identifying Information | M | 1
/// BA1 -> LOOP ID - X4 | 9999
/// 0900 | X4 | Customs Release Information | M | 1
/// 0950 | K1 | Remarks | O | 4
/// 1000 | SE | Transaction Set Trailer | M | 1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _350 {
    pub st: ST,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m10: Option<M10>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "P4")]
    pub loop_p4: Vec<_350LoopP4>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "BA1")]
    pub loop_ba1: Vec<_350LoopBa1>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _350LoopP4 {
    pub p4: P4,
    pub v9: Vec<V9>,
    pub vid: Vec<VID>,
    pub k1: Vec<K1>,
    #[x12(loop_trigger = "X4")]
    pub loop_x4: Vec<_350LoopP4X4>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _350LoopP4X4 {
    pub x4: X4,
    pub k1: Vec<K1>,
    pub n9: Vec<N9>,
    pub n7: Vec<N7>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _350LoopBa1 {
    pub ba1: BA1,
    #[x12(loop_trigger = "X4")]
    pub loop_x4: Vec<_350LoopBa1X4>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _350LoopBa1X4 {
    pub x4: X4,
    pub k1: Vec<K1>,
}
