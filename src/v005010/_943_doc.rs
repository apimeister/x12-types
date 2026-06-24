use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 943 - Warehouse Stock Transfer Shipment Advice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Warehouse Stock Transfer Shipment Advice Transaction Set (943) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a depositor or an agent to advise a receiving location that a stock transfer shipment has been made.
///
/// Heading: ST, W06, N1 loop (N1, N2, N3, N4, PER), N9, G61, G62, NTE, W27, W28, W10.
/// Detail LOOP W04: W04, G69, N9, W20.
/// Summary: W03, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _943 {
    pub st: ST,
    pub w06: W06,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_943LoopN1>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g61: Vec<G61>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    pub w27: W27,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w28: Option<W28>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w10: Option<W10>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "W04")]
    pub loop_w04: Vec<_943LoopW04>,
    pub w03: W03,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _943LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _943LoopW04 {
    pub w04: W04,
    pub g69: Vec<G69>,
    pub n9: Vec<N9>,
    pub w20: Vec<W20>,
}
