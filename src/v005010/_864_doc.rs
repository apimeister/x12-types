use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 864 - Text Message
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Text Message Transaction Set (864) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to transmit text messages of an informational, contractual, administrative, or promotional nature.
///
/// Heading: ST, BMG, DTM, N1 loop (N1, N2, N3, N4, REF, PER).
/// Detail LOOP MIT: MIT, N1 loop (N1, N2, N3, N4, REF, PER), MSG.
/// Summary: SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _864 {
    pub st: ST,
    pub bmg: BMG,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_864LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "MIT")]
    pub loop_mit: Vec<_864LoopMit>,
    pub se: SE,
}

/// Reusable party loop, used at the heading and MIT levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _864LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _864LoopMit {
    pub mit: MIT,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_864LoopN1>,
    pub msg: Vec<MSG>,
}
