use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 862 - Shipping Schedule
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Shipping Schedule Transaction Set (862) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used to convey precise shipping schedule requirements to a supplier.
///
/// Heading: ST, BSS, DTM, N1 loop (N1, N2, N3, N4, REF, PER, FOB).
/// Detail LOOP LIN: LIN, UIT, PKG, PO4, PRS, QTY, REF, PER, SDP, FST loop (FST, DTM, SDQ,
///   JIT loop (JIT, REF, DTM)), SHP loop (SHP, REF), TD1, TD3, TD5.
/// Summary: CTT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _862 {
    pub st: ST,
    pub bss: BSS,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_862LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LIN")]
    pub loop_lin: Vec<_862LoopLin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _862LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub fob: Option<FOB>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _862LoopLin {
    pub lin: LIN,
    pub uit: UIT,
    pub pkg: Vec<PKG>,
    pub po4: Vec<PO4>,
    pub prs: Option<PRS>,
    pub qty: Option<QTY>,
    pub r#ref: Vec<REF>,
    pub per: Option<PER>,
    pub sdp: Option<SDP>,
    #[x12(loop_trigger = "FST")]
    pub loop_fst: Vec<_862LoopFst>,
    #[x12(loop_trigger = "SHP")]
    pub loop_shp: Vec<_862LoopShp>,
    pub td1: Vec<TD1>,
    pub td3: Vec<TD3>,
    pub td5: Vec<TD5>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _862LoopFst {
    pub fst: FST,
    pub dtm: Vec<DTM>,
    pub sdq: Vec<SDQ>,
    #[x12(loop_trigger = "JIT")]
    pub loop_jit: Vec<_862LoopJit>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _862LoopJit {
    pub jit: JIT,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _862LoopShp {
    pub shp: SHP,
    pub r#ref: Vec<REF>,
}
