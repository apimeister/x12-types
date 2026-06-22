use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 322 - Terminal Operations and Intermodal Ramp Activity
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Terminal Operations and Intermodal Ramp Activity Transaction Set (322) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide all the information necessary for a terminal operation, port authority or intermodal ramp to communicate terminal and intermodal ramp activities (e.g., "ingates" and "outgates") to authorized parties to a shipment.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1 |   |
/// 0015 | ZC1 | Beginning Segment For Data Correction Or Change | O | 1 |   |
/// 0016 | Q5 | Status Details | M | 1 |   |
/// LOOP ID - N7 | 1000
/// N7 -> 0020 | N7 | Equipment Details | M | 1 |   |
/// N7 -> 0030 | V4 | Cargo Location Reference | O | 1 |   |
/// N7 -> 0040 | DTM | Date/Time Reference | O | 2 |   |
/// N7 -> 0050 | M7 | Seal Numbers | O | 5 |   |
/// N7 -> 0060 | W09 | Equipment and Temperature | O | 1 |   |
/// N7 -> 0070 | W2 | Equipment Identification | O | 1 |   |
/// N7 -> 0080 | NA | Cross-Reference Equipment | O | 30 |   |
/// N7 -> 0085 | GR5 | Loading Details | O | 10 |   |
/// N7 -> 0100 | Y7 | Priority | O | 1 |   |
/// N7 -> 0110 | V1 | Vessel Identification | O | 1 |   |
/// N7 -> LOOP ID - R4 | 20 |
/// N7 -> R4 -> 0120 | R4 | Port or Terminal | M | 1 |   |
/// N7 -> R4 -> 0130 | DTM | Date/Time Reference | O | 15 |   |
/// N7 -> 0140 | H3 | Special Handling Instructions | O | 6 |   |
/// N7 -> LOOP ID - N1 | 10 |
/// N7 -> N1 -> 0150 | N1 | Name | O | 1 |   |
/// N7 -> N1 -> 0153 | N3 | Address Information | O | 2 |   |
/// N7 -> N1 -> 0156 | N4 | Geographic Location | O | 1 |   |
/// N7 -> 0160 | K1 | Remarks | O | 2 |   |
/// N7 -> 0170 | N9 | Reference Identification | O | 10 |   |
/// N7 -> LOOP ID - L0 | 999 |
/// N7 -> L0 -> 0180 | L0 | Line Item - Quantity and Weight | O | 1 |   |
/// N7 -> L0 -> 0190 | L5 | Description, Marks and Numbers | O | 1 |   |
/// N7 -> L0 -> 0200 | H1 | Hazardous Material | O | 3 |   |
/// N7 -> 0210 | L3 | Total Weight and Charges | O | 2 |   |
/// 0220 | SE | Transaction Set Trailer | M | 1 |   |
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _322 {
    pub st: ST,
    pub zc1: Option<ZC1>,
    pub q5: Q5,
    #[x12(loop_trigger = "N7")]
    pub loop_n7: Vec<_322LoopN7>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _322LoopN7 {
    pub n7: N7,
    pub v4: Option<V4>,
    pub dtm: Vec<DTM>,
    pub m7: Vec<M7>,
    pub w09: Option<W09>,
    pub w2: Option<W2>,
    pub na: Vec<NA>,
    pub gr5: Vec<GR5>,
    pub y7: Option<Y7>,
    pub v1: Option<V1>,
    #[x12(loop_trigger = "R4")]
    pub loop_r4: Vec<_322LoopR4>,
    pub h3: Vec<H3>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_322LoopN1>,
    pub k1: Vec<K1>,
    pub n9: Vec<N9>,
    #[x12(loop_trigger = "L0")]
    pub loop_l0: Vec<_322LoopL0>,
    pub l3: Vec<L3>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _322LoopR4 {
    r4: R4,
    #[serde(default)]
    dtm: Vec<DTM>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _322LoopN1 {
    n1: N1,
    n3: Vec<N3>,
    n4: Option<N4>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _322LoopL0 {
    l0: Option<L0>,
    l5: Option<L5>,
    h1: Vec<H1>,
}
