use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 880 - Grocery Products Invoice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Grocery Products Invoice Transaction Set (880) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used by a supplier to bill for grocery products supplied to a retail or wholesale grocery operation.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _880 {
    pub st: ST,
    pub g01: G01,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_880LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lin: Vec<_880LoopLin>,
    pub tds: Option<TDS>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _880LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _880LoopLin {
    pub lin: LIN,
    pub g69: Vec<G69>,
    pub qty: Vec<QTY>,
    pub amt: Vec<AMT>,
    pub r#ref: Vec<REF>,
}
