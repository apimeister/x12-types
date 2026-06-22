use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 943 - Warehouse Stock Transfer Shipment Advice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Warehouse Stock Transfer Shipment Advice Transaction Set (943) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a depositor or an agent to advise a receiving location that a stock transfer shipment has been made.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _943 {
    pub st: ST,
    pub w06: W06,
    pub n9: Vec<N9>,
    pub g62: Vec<G62>,
    pub nte: Vec<NTE>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_943LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lx: Vec<_943LoopLx>,
    pub w14: Option<W14>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _943LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _943LoopLx {
    pub lx: LX,
    pub n9: Vec<N9>,
    pub g62: Vec<G62>,
    pub loop_w07: Vec<_943LoopW07>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _943LoopW07 {
    pub w07: W07,
    pub n9: Vec<N9>,
    pub g69: Vec<G69>,
}
