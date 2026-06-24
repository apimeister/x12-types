use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 843 - Response to Request for Quotation
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Response to Request for Quotation Transaction Set (843) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide a potential buyer with price, delivery schedule, and other terms in response to a request for quotation.
///
/// Heading: ST, BQR, CUR, INC, REF, PER, TAX, FOB, CTP, PAM, CSH, SAC loop, ITD, DIS, DTM,
///   LIN, PID, MEA, PWK, PKG, TD1/TD5/TD3/TD4, MAN, CTB, CPR, PCT, N9 loop, N1 loop,
///   AMT loop, ADV loop, LM loop, LDT loop.
/// Detail LOOP PO1: PO1, LIN, CUR, PO3, CTP, PAM, MEA, PID loop, PWK, PKG, PO4, REF, PER,
///   SAC loop, IT8, CSH, ITD, DIS, INC, TAX, FOB, SDQ, DTM, FST, TD1/TD5/TD3/TD4, MAN, MTX,
///   PCT, CTB, SPI, LM loop, QTY loop, SCH loop, CST loop, SLN loop, PD loop, LDT loop,
///   N9 loop, N1 loop, AMT loop.
/// Summary: CTT, AMT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843 {
    pub st: ST,
    pub bqr: BQR,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inc: Option<INC>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tax: Vec<TAX>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fob: Vec<FOB>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ctp: Vec<CTP>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pam: Vec<PAM>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub csh: Vec<CSH>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_843LoopSac>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub itd: Vec<ITD>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dis: Vec<DIS>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lin: Vec<LIN>,
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ctb: Vec<CTB>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cpr: Vec<CPR>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pct: Vec<PCT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_843LoopN9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_843LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "AMT")]
    pub loop_amt: Vec<_843LoopAmt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "ADV")]
    pub loop_adv: Vec<_843LoopAdv>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_843LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LDT")]
    pub loop_ldt: Vec<_843LoopLdt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "PO1")]
    pub loop_po1: Vec<_843LoopPo1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub amt: Vec<AMT>,
    pub se: SE,
}

/// Reusable service/charge loop (SAC + CUR).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopSac {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

/// Reusable lead-time loop (LDT + QTY/MTX + LM).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopLdt {
    pub ldt: LDT,
    pub qty: Vec<QTY>,
    pub mtx: Vec<MTX>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_843LoopLm>,
}

/// Reusable code-source loop (LM + LQ).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Reusable extended-reference loop (N9 + DTM/MTX/PWK/EFI).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopN9 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub pwk: Vec<PWK>,
    pub efi: Vec<EFI>,
}

/// Reusable monetary-amount loop (AMT + PCT).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopAmt {
    pub amt: AMT,
    pub pct: Vec<PCT>,
}

/// Reusable quantity loop (QTY + SI).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopQty {
    pub qty: QTY,
    pub si: Vec<SI>,
}

/// Reusable advertising loop (ADV + DTM/MTX).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopAdv {
    pub adv: ADV,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
}

/// Reusable cost-analysis loop (CST + PID/CUR/DTM).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopCst {
    pub cst: CST,
    pub pid: Vec<PID>,
    pub cur: Option<CUR>,
    pub dtm: Vec<DTM>,
}

/// Reusable pricing-data loop (PD + PDD).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopPd {
    pub pd: PD,
    pub pdd: Vec<PDD>,
}

/// Party loop, used at the heading and SLN levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub si: Vec<SI>,
    pub fob: Option<FOB>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub pkg: Vec<PKG>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopPo1 {
    pub po1: PO1,
    pub lin: Vec<LIN>,
    pub cur: Option<CUR>,
    pub po3: Vec<PO3>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub mea: Vec<MEA>,
    #[x12(loop_trigger = "PID")]
    pub loop_pid: Vec<_843LoopPid>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub po4: Vec<PO4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_843LoopSac>,
    pub it8: Option<IT8>,
    pub csh: Vec<CSH>,
    pub itd: Vec<ITD>,
    pub dis: Vec<DIS>,
    pub inc: Option<INC>,
    pub tax: Vec<TAX>,
    pub fob: Vec<FOB>,
    pub sdq: Vec<SDQ>,
    pub dtm: Vec<DTM>,
    pub fst: Vec<FST>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub man: Vec<MAN>,
    pub mtx: Vec<MTX>,
    pub pct: Vec<PCT>,
    pub ctb: Vec<CTB>,
    pub spi: Vec<SPI>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_843LoopLm>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_843LoopQty>,
    #[x12(loop_trigger = "SCH")]
    pub loop_sch: Vec<_843LoopSch>,
    #[x12(loop_trigger = "CST")]
    pub loop_cst: Vec<_843LoopCst>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_843LoopSln>,
    #[x12(loop_trigger = "PD")]
    pub loop_pd: Vec<_843LoopPd>,
    #[x12(loop_trigger = "LDT")]
    pub loop_ldt: Vec<_843LoopLdt>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_843LoopN9>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_843LoopPo1N1>,
    #[x12(loop_trigger = "AMT")]
    pub loop_amt: Vec<_843LoopAmt>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopPid {
    pub pid: PID,
    pub mea: Vec<MEA>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopSch {
    pub sch: SCH,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub r#ref: Vec<REF>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopSln {
    pub sln: SLN,
    pub mtx: Vec<MTX>,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub po3: Vec<PO3>,
    pub mea: Vec<MEA>,
    pub adv: Vec<ADV>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_843LoopQty>,
    #[x12(loop_trigger = "CST")]
    pub loop_cst: Vec<_843LoopCst>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_843LoopN9>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_843LoopN1>,
}

/// Party loop within a PO1 line item, carrying its own lead-time sub-loop.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopPo1N1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub qty: Vec<QTY>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub si: Vec<SI>,
    pub dtm: Option<DTM>,
    pub fob: Option<FOB>,
    pub sch: Vec<SCH>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub pkg: Vec<PKG>,
    #[x12(loop_trigger = "LDT")]
    pub loop_ldt: Vec<_843LoopN1Ldt>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _843LoopN1Ldt {
    pub ldt: LDT,
    pub man: Vec<MAN>,
    pub qty: Vec<QTY>,
    pub mtx: Vec<MTX>,
}
