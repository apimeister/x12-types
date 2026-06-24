use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 830 - Planning Schedule with Release Capability
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Planning Schedule with Release Capability Transaction Set (830) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide forecasting/material release information.
///
/// Heading: ST, BFR, XPO, CUR, REF, PER, TAX, FOB, CTP, SAC, CSH, ITD, DTM, PID, MEA, PWK,
///   PKG, TD1, TD5, TD3, TD4, MAN, N1 loop, LM loop.
/// Detail LOOP LIN: LIN, UIT, DTM, CUR, PO3, CTP, PID, MEA, PWK, PKG, PO4, PRS, REF, PER,
///   SAC, ITD, TAX, FOB, LDT, QTY, ATH, TD1, TD5, TD3, TD4, MAN, DD, SLN loop, N1 loop,
///   LM loop, FST loop (FST, QTY, SDQ, LM loop), SDP loop (SDP, FST), SHP loop (SHP, REF).
/// Summary: CTT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830 {
    pub st: ST,
    pub bfr: BFR,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub xpo: Vec<XPO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tax: Vec<TAX>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fob: Option<FOB>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ctp: Vec<CTP>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sac: Vec<SAC>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csh: Option<CSH>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub itd: Vec<ITD>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pid: Vec<PID>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mea: Vec<MEA>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pwk: Vec<PWK>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pkg: Vec<PKG>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub td1: Vec<TD1>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub td5: Vec<TD5>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub td3: Vec<TD3>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub td4: Vec<TD4>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub man: Vec<MAN>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_830LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_830LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LIN")]
    pub loop_lin: Vec<_830LoopLin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub fob: Option<FOB>,
}

/// Reusable code-source loop (LM + LQ), used at the heading, LIN and FST levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830LoopLin {
    pub lin: LIN,
    pub uit: Option<UIT>,
    pub dtm: Vec<DTM>,
    pub cur: Option<CUR>,
    pub po3: Vec<PO3>,
    pub ctp: Vec<CTP>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub po4: Option<PO4>,
    pub prs: Option<PRS>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub sac: Vec<SAC>,
    pub itd: Vec<ITD>,
    pub tax: Vec<TAX>,
    pub fob: Option<FOB>,
    pub ldt: Vec<LDT>,
    pub qty: Vec<QTY>,
    pub ath: Vec<ATH>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub man: Vec<MAN>,
    pub dd: Vec<DD>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_830LoopSln>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_830LoopN1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_830LoopLm>,
    #[x12(loop_trigger = "FST")]
    pub loop_fst: Vec<_830LoopFst>,
    #[x12(loop_trigger = "SDP")]
    pub loop_sdp: Vec<_830LoopSdp>,
    #[x12(loop_trigger = "SHP")]
    pub loop_shp: Vec<_830LoopShp>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830LoopSln {
    pub sln: SLN,
    pub pid: Vec<PID>,
    pub nm1: Vec<NM1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830LoopFst {
    pub fst: FST,
    pub qty: Vec<QTY>,
    pub sdq: Vec<SDQ>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_830LoopLm>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830LoopSdp {
    pub sdp: SDP,
    pub fst: Vec<FST>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _830LoopShp {
    pub shp: SHP,
    pub r#ref: Vec<REF>,
}
