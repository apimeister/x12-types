use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 888 - Item Maintenance
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Item Maintenance Transaction Set (888) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to transmit the item maintenance actions (additions, changes and deletions) and the related product specification data for grocery and other items.
///
/// Heading: ST, BGN, the N1 party loop, N9, G61, NTE, G93, G62, LDT and the LM code loop.
/// Detail LOOP 0300 (G53): G53, G62, NTE, then LOOP 0310 (LX) — the line-item loop carrying
///   the full pricing/description segment set and its N1 (0311), G55 consumer-unit (0312)
///   and LM (0313) sub-loops.
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _888 {
    pub st: ST,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bgn: Option<BGN>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_888LoopN1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub g61: Vec<G61>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub g93: Vec<G93>,
    pub g62: G62,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ldt: Vec<LDT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_888LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "G53")]
    pub loop_g53: Vec<_888LoopG53>,
    pub se: SE,
}

/// Heading party loop (N1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _888LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}

/// Reusable code-source loop (LM + LQ), used at the heading and line-item levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _888LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Detail maintenance loop (G53).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _888LoopG53 {
    pub g53: G53,
    pub g62: Vec<G62>,
    pub nte: Vec<NTE>,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_888LoopLx>,
}

/// Line-item loop (LX) nested in the G53 loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _888LoopLx {
    pub lx: LX,
    pub g39: Option<G39>,
    pub g69: Vec<G69>,
    pub qty: Vec<QTY>,
    pub lin: Option<LIN>,
    pub pid: Vec<PID>,
    pub pkg: Vec<PKG>,
    pub g23: Vec<G23>,
    pub g62: Vec<G62>,
    pub g36: Option<G36>,
    pub g26: Vec<G26>,
    pub g43: Vec<G43>,
    pub g24: Vec<G24>,
    pub g40: Vec<G40>,
    pub g93: Vec<G93>,
    pub g22: Vec<G22>,
    pub g46: Vec<G46>,
    pub h1: Vec<H1>,
    pub g54: Vec<G54>,
    pub n9: Vec<N9>,
    pub uit: Vec<UIT>,
    pub mea: Vec<MEA>,
    pub td1: Option<TD1>,
    pub td4: Vec<TD4>,
    pub ldt: Option<LDT>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_888LoopLxN1>,
    #[x12(loop_trigger = "G55")]
    pub loop_g55: Vec<_888LoopG55>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_888LoopLm>,
}

/// Party loop (N1) nested in the line-item loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _888LoopLxN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub pal: Vec<PAL>,
    pub g93: Vec<G93>,
    pub qty: Option<QTY>,
}

/// Consumer-unit characteristics loop (G55) nested in the line-item loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _888LoopG55 {
    pub g55: G55,
    pub g69: Vec<G69>,
    pub qty: Option<QTY>,
    pub lin: Option<LIN>,
    pub pid: Vec<PID>,
    pub h1: Vec<H1>,
    pub r#ref: Vec<REF>,
    pub pkg: Vec<PKG>,
    pub mea: Vec<MEA>,
    pub td1: Option<TD1>,
    pub td4: Vec<TD4>,
    pub sln: Vec<SLN>,
    pub ldt: Option<LDT>,
}
