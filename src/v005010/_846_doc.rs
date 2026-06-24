use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 846 - Inventory Inquiry/Advice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Inventory Inquiry/Advice Transaction Set (846) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to advise a trading partner of inventory positions, and to support a variety of standard business transactions associated with inventory levels.
///
/// Heading: ST, BIA, CUR, DTM, REF, PER, MEA, N1 loop, LM loop.
/// Detail LOOP LIN: LIN, PID, MEA, PKG, DTM, CTP, CUR, SAC, REF, PER, SDQ, MAN, UIT, CS, DD,
///   G53, PCT, LDT, LM loop, SLN loop (SLN, PID, MEA, PKG, MAN loop), QTY loop (QTY, UIT,
///   MEA, LDT, DTM, SCH loop, LM loop, LS/REF loop (REF, DTM, N1, LM loop)/LE), N1 loop.
/// Summary: CTT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846 {
    pub st: ST,
    pub bia: BIA,
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_846LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_846LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LIN")]
    pub loop_lin: Vec<_846LoopLin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

/// Reusable code-source loop (LM + LQ), used at the heading, LIN, QTY and REF levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopLin {
    pub lin: LIN,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pkg: Vec<PKG>,
    pub dtm: Vec<DTM>,
    pub ctp: Vec<CTP>,
    pub cur: Option<CUR>,
    pub sac: Vec<SAC>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub sdq: Vec<SDQ>,
    pub man: Option<MAN>,
    pub uit: Vec<UIT>,
    pub cs: Option<CS>,
    pub dd: Vec<DD>,
    pub g53: Option<G53>,
    pub pct: Vec<PCT>,
    pub ldt: Vec<LDT>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_846LoopLm>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_846LoopSln>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_846LoopQty>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_846LoopLinN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopSln {
    pub sln: SLN,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pkg: Vec<PKG>,
    #[x12(loop_trigger = "MAN")]
    pub loop_man: Vec<_846LoopMan>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopMan {
    pub man: MAN,
    pub mea: Vec<MEA>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopQty {
    pub qty: QTY,
    pub uit: Vec<UIT>,
    pub mea: Vec<MEA>,
    pub ldt: Vec<LDT>,
    pub dtm: Vec<DTM>,
    #[x12(loop_trigger = "SCH")]
    pub loop_sch: Vec<_846LoopSch>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_846LoopLm>,
    pub ls: Option<LS>,
    #[x12(loop_trigger = "REF")]
    pub loop_ref: Vec<_846LoopQtyRef>,
    pub le: Option<LE>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopSch {
    pub sch: SCH,
    pub mea: Vec<MEA>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopQtyRef {
    pub r#ref: REF,
    pub dtm: Vec<DTM>,
    pub n1: Option<N1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_846LoopLm>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopLinN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}
