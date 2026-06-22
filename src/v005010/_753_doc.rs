use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 753 - Request for Routing Instructions
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Request for Routing Instructions Transaction Set (753) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used by a shipper to request routing instructions from a carrier or controlling party.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _753 {
    pub st: ST,
    pub g62: Vec<G62>,
    pub n9: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_753LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lx: Vec<_753LoopLx>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _753LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _753LoopLx {
    pub lx: LX,
    pub n7: Vec<N7>,
    pub g62: Vec<G62>,
    pub r#ref: Vec<REF>,
}
