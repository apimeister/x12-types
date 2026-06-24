use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 870 - Order Status Report
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Order Status Report Transaction Set (870) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a seller to report on the current status of an order, group of orders, or selected line items.
///
/// Heading: ST, BSR, TD3, TD4, TD5, DTM, REF loop (REF, DTM), N1 loop (N1, N2, N3, N4, REF,
///   PER, PWK), LM loop (LM, LQ).
/// Detail LOOP HL: HL, PRF, ISR loop (ISR, PID, QTY, PER, DTM, CS), REF loop, N1 loop,
///   LM loop, PO1 loop (PO1, CUR, SLN, PO3, PID, MEA, PKG, ISR loop (ISR, PID, QTY, DTM,
///   N1, carrier, REF, SAC, LM loop), LX loop (LX, REF, N1, DTM, LM loop)).
/// Summary: CTT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870 {
    pub st: ST,
    pub bsr: BSR,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub td3: Option<TD3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub td4: Option<TD4>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub td5: Option<TD5>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "REF")]
    pub loop_ref: Vec<_870LoopRef>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_870LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_870LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "HL")]
    pub loop_hl: Vec<_870LoopHl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

/// Reusable reference loop (REF + DTM), used at the heading and HL levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870LoopRef {
    pub r#ref: REF,
    pub dtm: Vec<DTM>,
}

/// Reusable party loop, used at the heading and HL levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub pwk: Option<PWK>,
}

/// Reusable code-source loop (LM + LQ).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870LoopHl {
    pub hl: HL,
    pub prf: Option<PRF>,
    #[x12(loop_trigger = "ISR")]
    pub loop_isr: Vec<_870LoopHlIsr>,
    #[x12(loop_trigger = "REF")]
    pub loop_ref: Vec<_870LoopRef>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_870LoopN1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_870LoopLm>,
    #[x12(loop_trigger = "PO1")]
    pub loop_po1: Vec<_870LoopPo1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870LoopHlIsr {
    pub isr: ISR,
    pub pid: Vec<PID>,
    pub qty: Vec<QTY>,
    pub per: Vec<PER>,
    pub dtm: Vec<DTM>,
    pub cs: Vec<CS>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870LoopPo1 {
    pub po1: PO1,
    pub cur: Option<CUR>,
    pub sln: Vec<SLN>,
    pub po3: Option<PO3>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pkg: Vec<PKG>,
    #[x12(loop_trigger = "ISR")]
    pub loop_isr: Vec<_870LoopPo1Isr>,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_870LoopPo1Lx>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870LoopPo1Isr {
    pub isr: ISR,
    pub pid: Vec<PID>,
    pub qty: Vec<QTY>,
    pub dtm: Vec<DTM>,
    pub n1: Option<N1>,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub td1: Option<TD1>,
    pub td5: Option<TD5>,
    pub td3: Option<TD3>,
    pub td4: Option<TD4>,
    pub r#ref: Vec<REF>,
    pub sac: Vec<SAC>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_870LoopLm>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _870LoopPo1Lx {
    pub lx: LX,
    pub r#ref: Vec<REF>,
    pub n1: Option<N1>,
    pub dtm: Vec<DTM>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_870LoopLm>,
}
