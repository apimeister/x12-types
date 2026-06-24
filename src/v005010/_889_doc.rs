use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 889 - Promotion Announcement
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Promotion Announcement Transaction Set (889) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to announce a promotion to trading partners, conveying the promotion's conditions, performance requirements and the products to which it applies.
///
/// Heading: ST, G42, N9, G61, G62, NTE, G43, G23, then the N1 party loop and the G94
///   promotion-conditions loop (G94, G95).
/// Detail LOOP 0300 (LX): LX, G46, G51, with a nested G94 conditions loop (G94 -> G95/G62)
///   and a G45 promotional-product loop (G69/G43/G51/G23/G62/G22/QTY).
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _889 {
    pub st: ST,
    pub g42: G42,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g61: Vec<G61>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g43: Vec<G43>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g23: Option<G23>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_889LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "G94")]
    pub loop_g94: Vec<_889LoopG94>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_889LoopLx>,
    pub se: SE,
}

/// Heading party loop (N1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _889LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g62: Vec<G62>,
}

/// Heading promotion-conditions loop (G94 + G95).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _889LoopG94 {
    pub g94: G94,
    pub g95: Vec<G95>,
}

/// Detail line-item loop (LX).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _889LoopLx {
    pub lx: LX,
    pub g46: Vec<G46>,
    pub g51: Option<G51>,
    #[x12(loop_trigger = "G94")]
    pub loop_g94: Vec<_889LoopLxG94>,
    #[x12(loop_trigger = "G45")]
    pub loop_g45: Vec<_889LoopG45>,
}

/// Conditions loop (G94) nested in the LX loop, with a performance (G95) sub-loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _889LoopLxG94 {
    pub g94: G94,
    #[x12(loop_trigger = "G95")]
    pub loop_g95: Vec<_889LoopG95>,
}

/// Performance-requirements loop (G95 + G62) nested in the LX-level G94 loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _889LoopG95 {
    pub g95: G95,
    pub g62: Vec<G62>,
}

/// Promotional-product loop (G45) nested in the LX loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _889LoopG45 {
    pub g45: G45,
    pub g69: Vec<G69>,
    pub g43: Vec<G43>,
    pub g51: Vec<G51>,
    pub g23: Option<G23>,
    pub g62: Vec<G62>,
    pub g22: Option<G22>,
    pub qty: Vec<QTY>,
}
