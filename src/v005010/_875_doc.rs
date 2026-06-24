use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 875 - Grocery Products Purchase Order
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Grocery Products Purchase Order Transaction Set (875) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to place a purchase order for grocery products.
///
/// Heading: ST, G50, N9, G61, G62, NTE, G66, G23, N1 loop (N1, N2, N3, N4),
///   G72 allowance loop (G72, G73).
/// Detail LOOP G68: G68, G69, G70, N9, G23, G72 loop, N1 loop (N1, QTY, N9), SLN loop
///   (SLN, G72).
/// Summary: G76, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _875 {
    pub st: ST,
    pub g50: G50,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g61: Vec<G61>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g66: Option<G66>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g23: Vec<G23>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_875LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "G72")]
    pub loop_g72: Vec<_875LoopG72>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "G68")]
    pub loop_g68: Vec<_875LoopG68>,
    pub g76: G76,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _875LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}

/// Reusable allowance/charge loop (G72 + G73), used at the heading and line-item levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _875LoopG72 {
    pub g72: G72,
    pub g73: Vec<G73>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _875LoopG68 {
    pub g68: G68,
    pub g69: Vec<G69>,
    pub g70: Vec<G70>,
    pub n9: Vec<N9>,
    pub g23: Vec<G23>,
    #[x12(loop_trigger = "G72")]
    pub loop_g72: Vec<_875LoopG72>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_875LoopG68N1>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_875LoopSln>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _875LoopG68N1 {
    pub n1: N1,
    pub qty: Option<QTY>,
    pub n9: Vec<N9>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _875LoopSln {
    pub sln: SLN,
    pub g72: Vec<G72>,
}
