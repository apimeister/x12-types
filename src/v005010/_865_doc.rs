use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 865 - Purchase Order Change Acknowledgment/Request - Seller Initiated
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Purchase Order Change Acknowledgment/Request - Seller Initiated Transaction Set (865) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a seller to convey acknowledgment of, or changes to, a purchase order.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865 {
    pub st: ST,
    pub bca: BCA,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_865LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_poc: Vec<_865LoopPoc>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopPoc {
    pub poc: POC,
    pub ctp: Vec<CTP>,
    pub pid: Vec<PID>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
}
