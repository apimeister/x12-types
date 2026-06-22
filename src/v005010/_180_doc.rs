use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 180 - Return Merchandise Authorization and Notification
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Return Merchandise Authorization and Notification Transaction Set (180) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to request, authorize, or provide notification of returned products.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _180 {
    pub st: ST,
    pub bgn: BGN,
    pub dtm: Vec<DTM>,
    pub r#ref: Vec<REF>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_180LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lin: Vec<_180LoopLin>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _180LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _180LoopLin {
    pub lin: LIN,
    pub pid: Vec<PID>,
    pub qty: Vec<QTY>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
}
