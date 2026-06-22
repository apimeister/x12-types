use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 210 - Motor Carrier Freight Details and Invoice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Motor Carrier Freight Details and Invoice Transaction Set (210) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by motor carriers to provide detailed freight and invoice information to a paying party.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210 {
    pub st: ST,
    pub b3: B3,
    pub c2: Option<C2>,
    pub c3: Option<C3>,
    pub itd: Vec<ITD>,
    pub n9: Vec<N9>,
    pub g62: Vec<G62>,
    pub r3: Vec<R3>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_210LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lx: Vec<_210LoopLx>,
    pub l3: L3,
    pub se: SE,
}

/// Loop N1 - Party Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

/// Loop LX - Assigned Number
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopLx {
    pub lx: LX,
    pub n9: Vec<N9>,
    pub pod: Vec<POD>,
    pub loop_l0: Vec<_210LoopL0>,
}

/// Loop L0 - Line Item - Quantity and Weight
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopL0 {
    pub l0: L0,
    pub l1: Vec<L1>,
    pub l5: Vec<L5>,
    pub l7: Vec<L7>,
    pub mea: Vec<MEA>,
}
