use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 944 - Warehouse Stock Transfer Receipt Advice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Warehouse Stock Transfer Receipt Advice Transaction Set (944) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a warehouse to advise a depositor that a stock transfer shipment has been received.
///
/// Heading: ST, W17, N1 loop (N1, N2, N3, N4, PER), N9, G61, G62, NTE, W08, W18, G08, TD1.
/// Detail LOOP LX: LX, MAN, PAL, N9, W07 loop (W07, G69, N9, W20, W13 loop (W13, N9)).
/// Summary: W14, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _944 {
    pub st: ST,
    pub w17: W17,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_944LoopN1>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g61: Vec<G61>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w08: Option<W08>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub w18: Vec<W18>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g08: Vec<G08>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub td1: Vec<TD1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_944LoopLx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w14: Option<W14>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _944LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _944LoopLx {
    pub lx: LX,
    pub man: Option<MAN>,
    pub pal: Vec<PAL>,
    pub n9: Vec<N9>,
    #[x12(loop_trigger = "W07")]
    pub loop_w07: Vec<_944LoopW07>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _944LoopW07 {
    pub w07: W07,
    pub g69: Vec<G69>,
    pub n9: Vec<N9>,
    pub w20: Vec<W20>,
    #[x12(loop_trigger = "W13")]
    pub loop_w13: Vec<_944LoopW13>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _944LoopW13 {
    pub w13: W13,
    pub n9: Vec<N9>,
}
