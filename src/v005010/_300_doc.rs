use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 300 - Reservation (Booking Request) (Ocean)
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Reservation (Booking Request) (Ocean) Transaction Set (300) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by shippers or their agents to request space and equipment from an ocean carrier for the transportation of cargo.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0100 | ST | Transaction Set Header | M | 1
/// 0200 | B1 | Beginning Segment for Booking or Pickup/Delivery | M | 1
/// 0250 | G61 | Contact | O | 3
/// 0300 | Y6 | Authentication | O | 2
/// 0400 | Y7 | Cargo Booking Priority | O | 1
/// 0500 | Y1 | Space Reservation Request | M | 1
/// LOOP ID - 0600 | 10
/// 0600 -> 0600 | Y2 | Container Details | M | 1
/// 0600 -> 0650 | W09 | Equipment and Temperature | O | 1
/// 0690 | N9 | Extended Reference Information | O | 100
/// 0700 | R2A | Route Information with Preference | O | 25
/// LOOP ID - 0800 | 10
/// 0800 -> 0800 | N1 | Party Identification | M | 1
/// 0800 -> 0900 | N2 | Additional Name Information | O | 1
/// 0800 -> 1000 | N3 | Party Location | O | 2
/// 0800 -> 1100 | N4 | Geographic Location | O | 1
/// 0800 -> 1200 | G61 | Contact | O | 3
/// LOOP ID - 1300 | 20
/// 1300 -> 1300 | R4 | Port or Terminal | M | 1
/// 1300 -> 1400 | DTM | Date/Time Reference | O | 15
/// 1500 | W09 | Equipment and Temperature | O | 1
/// 1600 | H3 | Special Handling Instructions | O | 6
/// 1700 | EA | Equipment Attributes | O | 5
/// LOOP ID - 0100 (detail) | 999
/// 0100 -> 0100 | LX | Assigned Number | M | 1
/// 0100 -> 0200 | N7 | Equipment Details | O | 1
/// 0100 -> 0210 | W09 | Equipment and Temperature | O | 1
/// 0100 -> 0300 | DTM | Date/Time Reference | O | 1
/// 0100 -> 0400 | L0 | Line Item - Quantity and Weight | O | 1
/// 0100 -> 0500 | L5 | Description, Marks and Numbers | O | 1
/// 0100 -> 0600 | L4 | Measurement | O | 1
/// 0100 -> 0650 | L1 | Rate and Charges | O | 1
/// 0100 -> LOOP ID - 0700 | 10
/// 0100 -> 0700 -> 0700 | H1 | Hazardous Material | M | 1
/// 0100 -> 0700 -> 0800 | H2 | Additional Hazardous Material Description | O | 10
/// 0100 -> LOOP ID - 0810 | 100
/// 0100 -> 0810 -> 0810 | LH1 | Hazardous Identification Information | M | 1
/// 0100 -> 0810 -> 0820 | LH2 | Hazardous Classification Information | O | 4
/// 0100 -> 0810 -> 0830 | LH3 | Hazardous Material Shipping Name | O | 10
/// 0100 -> 0810 -> 0840 | LFH | Freeform Hazardous Material Information | O | 25
/// 0100 -> 0810 -> 0850 | LEP | EPA Required Data | O | 3
/// 0100 -> 0810 -> 0860 | LH4 | Canadian Dangerous Requirements | O | 1
/// 0100 -> 0810 -> 0870 | LHT | Transborder Hazardous Requirements | O | 3
/// 0100 -> 0810 -> 0880 | LHR | Hazardous Material Identifying Reference Numbers | O | 5
/// 0100 -> 0810 -> 0890 | PER | Administrative Communications Contact | O | 5
/// 0900 | V1 | Vessel Identification | O | 2
/// 1000 | V9 | Event Detail | O | 10
/// 1100 | K1 | Remarks | O | 2
/// SE | SE | Transaction Set Trailer | M | 1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _300 {
    pub st: ST,
    pub b1: B1,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub g61: Vec<G61>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub y6: Vec<Y6>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y7: Option<Y7>,
    pub y1: Y1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "Y2")]
    pub loop_y2: Vec<_300LoopY2>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r2a: Vec<R2A>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_300LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "R4")]
    pub loop_r4: Vec<_300LoopR4>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w09: Option<W09>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub h3: Vec<H3>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ea: Vec<EA>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_300LoopLx>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub v1: Vec<V1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub v9: Vec<V9>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub k1: Vec<K1>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _300LoopY2 {
    pub y2: Y2,
    pub w09: Option<W09>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _300LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g61: Vec<G61>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _300LoopR4 {
    pub r4: R4,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _300LoopLx {
    pub lx: LX,
    pub n7: Option<N7>,
    pub w09: Option<W09>,
    pub dtm: Option<DTM>,
    pub l0: Option<L0>,
    pub l5: Option<L5>,
    pub l4: Option<L4>,
    pub l1: Option<L1>,
    #[x12(loop_trigger = "H1")]
    pub loop_h1: Vec<_300LoopLxH1>,
    #[x12(loop_trigger = "LH1")]
    pub loop_lh1: Vec<_300LoopLxLh1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _300LoopLxH1 {
    pub h1: H1,
    pub h2: Vec<H2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _300LoopLxLh1 {
    pub lh1: LH1,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Vec<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Vec<LHT>,
    pub lhr: Vec<LHR>,
    pub per: Vec<PER>,
}
