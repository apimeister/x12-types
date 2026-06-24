use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 880 - Grocery Products Invoice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Grocery Products Invoice Transaction Set (880) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used by a supplier to bill for grocery products supplied to a retail or wholesale grocery operation.
///
/// Heading: ST, G01, N9, G61, G62, NTE, CAD, G23, G25, N1 loop (N1, N2, N3, N4),
///   G72 allowance loop (G72, G73).
/// Detail LOOP G17: G17, G69, G19, G20, N9, G23, G25, G72 allowance loop (G72, G73).
/// Detail LOOP ENT: ENT, N2, N3, N4, N9, REF loop (REF, QTY, AMT, G72, G17 sub-loop (G17, G19)).
/// Summary: G31, G33, SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _880 {
    pub st: ST,
    pub g01: G01,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g61: Vec<G61>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cad: Vec<CAD>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g23: Vec<G23>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g25: Option<G25>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_880LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "G72")]
    pub loop_g72: Vec<_880LoopG72>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "G17")]
    pub loop_g17: Vec<_880LoopG17>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "ENT")]
    pub loop_ent: Vec<_880LoopEnt>,
    pub g31: G31,
    pub g33: G33,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _880LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}

/// Reusable allowance/charge loop (G72 + G73), used at the heading and line-item levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _880LoopG72 {
    pub g72: G72,
    pub g73: Vec<G73>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _880LoopG17 {
    pub g17: G17,
    pub g69: Vec<G69>,
    pub g19: Vec<G19>,
    pub g20: Option<G20>,
    pub n9: Vec<N9>,
    pub g23: Vec<G23>,
    pub g25: Option<G25>,
    #[x12(loop_trigger = "G72")]
    pub loop_g72: Vec<_880LoopG72>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _880LoopEnt {
    pub ent: ENT,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub n9: Vec<N9>,
    #[x12(loop_trigger = "REF")]
    pub loop_ref: Vec<_880LoopEntRef>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _880LoopEntRef {
    pub r#ref: REF,
    pub qty: Option<QTY>,
    pub amt: Vec<AMT>,
    pub g72: Option<G72>,
    #[x12(loop_trigger = "G17")]
    pub loop_g17: Vec<_880LoopEntG17>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _880LoopEntG17 {
    pub g17: G17,
    pub g19: Vec<G19>,
}
