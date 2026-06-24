use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 867 - Product Transfer and Resale Report
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Product Transfer and Resale Report Transaction Set (867) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to report the movement of product to, from, or between locations, including product transfers and resales reported by a distributor or reseller to a manufacturer or supplier. It conveys the parties, dates, sales-item detail, quantities, monetary amounts, and supporting reference and accounting data for each product transfer or resale event.
///
/// Heading:
/// 0100 ST, 0200 BPT, 0400 CUR, 0500 DTM, 0600 REF, 0700 PER, 0750 MEA, 0780 PSA,
/// LOOP N1 (0800 N1, 0900 N2, 1000 N3, 1100 N4, 1200 REF, LOOP PER [1300 PER, 1350 REF]),
/// LOOP LM (1400 LM, 1500 LQ, 1600 LCD).
///
/// Detail:
/// LOOP PTD (0100 PTD, 0200 DTM, 0300 REF, 0350 PRF, 0400 PER, 0450 MAN, 0475 LCD,
/// 0490 LQ, 0495 MEA, LOOP N1 [0500 N1 .. 1000 PER], LOOP SII [1050 SII, 1070 N9,
/// LOOP QTY [1100 QTY .. 2500 LDT, LOOP LM [2600 LM, 2700 LQ], LOOP LX [2800 LX, 2900 REF,
/// 3000 DTM, 3100 N1, LOOP LM [3200 LM, 3300 LQ], LOOP FA1 [3400 FA1, 3500 FA2]]]]).
///
/// Summary:
/// LOOP CTT (0100 CTT, 0200 AMT, 0210 ITA), 0300 SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867 {
    pub st: ST,
    pub bpt: BPT,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mea: Vec<MEA>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub psa: Vec<PSA>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_867LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_867LoopLmHeading>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "PTD")]
    pub loop_ptd: Vec<_867LoopPtd>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "CTT")]
    pub loop_ctt: Vec<_867LoopCtt>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    #[x12(loop_trigger = "PER")]
    pub loop_per: Vec<_867LoopPer>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopPer {
    pub per: PER,
    pub r#ref: Vec<REF>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopLmHeading {
    pub lm: LM,
    pub lq: Vec<LQ>,
    pub lcd: Vec<LCD>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopPtd {
    pub ptd: PTD,
    pub dtm: Vec<DTM>,
    pub r#ref: Vec<REF>,
    pub prf: Option<PRF>,
    pub per: Vec<PER>,
    pub man: Option<MAN>,
    pub lcd: Vec<LCD>,
    pub lq: Vec<LQ>,
    pub mea: Vec<MEA>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_867LoopPtdN1>,
    #[x12(loop_trigger = "SII")]
    pub loop_sii: Vec<_867LoopSii>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopPtdN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopSii {
    pub sii: SII,
    pub n9: Option<N9>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_867LoopQty>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopQty {
    pub qty: QTY,
    pub lin: Option<LIN>,
    pub po3: Vec<PO3>,
    pub po4: Option<PO4>,
    pub uit: Vec<UIT>,
    pub amt: Vec<AMT>,
    pub ita: Vec<ITA>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dtm: Vec<DTM>,
    pub cur: Option<CUR>,
    pub dd: Vec<DD>,
    pub ldt: Option<LDT>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_867LoopLm>,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_867LoopLx>,
}

/// Reusable code-source loop (LM + LQ), used in the QTY and LX loops.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopLx {
    pub lx: LX,
    pub r#ref: Vec<REF>,
    pub dtm: Option<DTM>,
    pub n1: Option<N1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_867LoopLm>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_867LoopFa1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _867LoopCtt {
    pub ctt: CTT,
    pub amt: Vec<AMT>,
    pub ita: Vec<ITA>,
}
