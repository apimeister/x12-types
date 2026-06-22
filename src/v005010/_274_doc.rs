use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 274 - Healthcare Provider Information
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Healthcare Provider Information Transaction Set (274) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to transmit provider data among interested parties.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274 {
    pub st: ST,
    pub bgn: BGN,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_274LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lx: Vec<_274LoopLx>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274LoopN1 {
    pub n1: N1,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274LoopLx {
    pub lx: LX,
    pub loop_nm1: Vec<_274LoopNm1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274LoopNm1 {
    pub nm1: NM1,
    pub r#ref: Vec<REF>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub prv: Option<PRV>,
    pub dtp: Vec<DTP>,
}
