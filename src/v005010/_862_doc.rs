use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 862 - Shipping Schedule
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Shipping Schedule Transaction Set (862) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used to convey precise shipping schedule requirements to a supplier.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _862 {
    pub st: ST,
    pub bss: BSS,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_862LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lin: Vec<_862LoopLin>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _862LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _862LoopLin {
    pub lin: LIN,
    pub uit: Vec<UIT>,
    pub r#ref: Vec<REF>,
    pub shp: Vec<SHP>,
    pub loop_fst: Vec<_862LoopFst>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _862LoopFst {
    pub fst: FST,
    pub sdp: Vec<SDP>,
    pub r#ref: Vec<REF>,
}
