use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 309 - U.S. Customs Manifest
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the U.S. Customs Manifest Transaction Set (309) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by carriers, terminal operators, port authorities, or service centers to provide U.S. Customs with manifest data on cargo arriving in or departing from the U.S. on oceangoing vessels, railroad trains, or other types of conveyances. The transaction set can be also used by carriers to provide terminal operators, port authorities, or service centers with manifest data on cargo arriving at their facilities via the conveyances mentioned above.
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1
/// 0020 | M10 | Manifest Identifying Information | M | 1
/// LOOP ID - P4 | 20
/// P4 -> 0040 | P4 | U.S. Port Information | M | 1
/// P4 -> LOOP ID - LX | 9999 |
/// P4 -> LX -> 0060 | LX | Assigned Number | M | 1
/// P4 -> LX -> 0070 | M13 | Manifest Amendment Details | O | 1
/// P4 -> LX -> 0080 | M11 | Manifest Bill of Lading Details | O | 1
/// P4 -> LX -> 0085 | N9 | Reference Identification | O | 999
/// P4 -> LX -> LOOP ID - N1 | 5 |   |
/// P4 -> LX -> N1 -> 0100 | N1 | Name | O | 1
/// P4 -> LX -> N1 -> 0110 | N3 | Address Information | O | 2
/// P4 -> LX -> N1 -> 0120 | N4 | Geographic Location | O | 1
/// P4 -> LX -> N1 -> 0123 | DTM | Date/Time Reference | O | 1
/// P4 -> LX -> N1 -> 0125 | PER | Administrative Communications Contact | O | 1
/// P4 -> LX -> LOOP ID - M12 | 1 |   |
/// P4 -> LX -> M12 -> 0130 | M12 | In-bond Identifying Information | O | 1
/// P4 -> LX -> M12 -> 0135 | P5 | Port Information | O | 5
/// P4 -> LX -> LOOP ID - VID | 999 |   |
/// P4 -> LX -> VID -> 0150 | VID | Conveyance Identification | O | 1
/// P4 -> LX -> VID -> 0155 | VC | Motor Vehicle Control | O | 21
/// P4 -> LX -> VID -> LOOP ID - N10 | 999 |   |   |
/// P4 -> LX -> VID -> N10 -> 0160 | N10 | Quantity and Description | O | 1
/// P4 -> LX -> VID -> N10 -> LOOP ID - H1 | 10 |   |   |   |
/// P4 -> LX -> VID -> N10 -> H1 -> 0165 | H1 | Hazardous Material | O | 1
/// P4 -> LX -> VID -> N10 -> H1 -> 0166 | H2 | Additional Hazardous Material Description | O | 99
/// 0200 | SE | Transaction Set Trailer | M | 1
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _309 {
    pub st: ST,
    pub m10: M10,
    #[x12(loop_trigger = "P4")]
    pub loop_p4: Vec<_309LoopP4>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _309LoopP4 {
    pub p4: P4,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_309LoopLX>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _309LoopLX {
    pub lx: LX,
    pub m13: Option<M13>,
    pub m11: Option<M11>,
    pub n9: Vec<N9>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_309LoopN1>,
    #[x12(loop_trigger = "M12")]
    pub loop_m12: Vec<_309LoopM12>,
    #[x12(loop_trigger = "VID")]
    pub loop_vid: Vec<_309LoopVID>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _309LoopN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dtm: Option<DTM>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _309LoopM12 {
    pub m12: Option<M12>,
    pub r4: Vec<R4>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _309LoopVID {
    pub vid: Option<VID>,
    pub m7: Vec<M7>,
    #[x12(loop_trigger = "N10")]
    pub loop_n10: Vec<_309LoopN10>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _309LoopN10 {
    pub n10: Option<N10>,
    pub vc: Vec<VC>,
    #[x12(loop_trigger = "H1")]
    pub loop_h1: Vec<_309LoopH1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _309LoopH1 {
    pub h1: Option<H1>,
    pub h2: Vec<H2>,
}
