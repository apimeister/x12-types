use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 275 - Patient Information
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Patient Information Transaction Set (275) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to convey patient information between providers, payers, and other interested parties.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _275 {
    pub st: ST,
    pub bgn: BGN,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_nm1: Vec<_275LoopNm1>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _275LoopNm1 {
    pub nm1: NM1,
    pub r#ref: Vec<REF>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub dmg: Option<DMG>,
    pub ins: Option<INS>,
    pub dtp: Vec<DTP>,
    pub pwk: Vec<PWK>,
}
