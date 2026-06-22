use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 754 - Routing Instructions
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Routing Instructions Transaction Set (754) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used by a carrier or controlling party to provide routing instructions in response to a request.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _754 {
    pub st: ST,
    pub g62: Vec<G62>,
    pub n9: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_754LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lx: Vec<_754LoopLx>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _754LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _754LoopLx {
    pub lx: LX,
    pub n7: Vec<N7>,
    pub r4: Vec<R4>,
    pub g62: Vec<G62>,
    pub r#ref: Vec<REF>,
}
