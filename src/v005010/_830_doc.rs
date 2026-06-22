use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 830 - Planning Schedule with Release Capability
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Planning Schedule with Release Capability Transaction Set (830) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide forecasting/material release information.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830 {
    pub st: ST,
    pub bfr: BFR,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_830LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lin: Vec<_830LoopLin>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830LoopLin {
    pub lin: LIN,
    pub uit: Vec<UIT>,
    pub pid: Vec<PID>,
    pub r#ref: Vec<REF>,
    pub shp: Vec<SHP>,
    pub loop_fst: Vec<_830LoopFst>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830LoopFst {
    pub fst: FST,
    pub sdp: Vec<SDP>,
    pub r#ref: Vec<REF>,
}
