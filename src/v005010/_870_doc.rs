use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 870 - Order Status Report
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Order Status Report Transaction Set (870) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a seller to report on the current status of an order, group of orders, or selected line items.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870 {
    pub st: ST,
    pub bsr: BSR,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_870LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_hl: Vec<_870LoopHl>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870LoopHl {
    pub hl: HL,
    pub prf: Vec<PRF>,
    pub po1: Vec<PO1>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
}
