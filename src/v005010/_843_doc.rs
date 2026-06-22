use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 843 - Response to Request for Quotation
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Response to Request for Quotation Transaction Set (843) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set provides potential sellers with the ability to provide price, availability, and other terms to potential buyers.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843 {
    pub st: ST,
    pub bqr: BQR,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_843LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_po1: Vec<_843LoopPo1>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopPo1 {
    pub po1: PO1,
    pub pid: Vec<PID>,
    pub ctp: Vec<CTP>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
}
