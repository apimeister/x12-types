use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 840 - Request for Quotation
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Request for Quotation Transaction Set (840) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide potential buyers with the ability to solicit price, delivery schedule, and other terms from potential sellers of goods and services.
///
/// Heading: ST, BQT, CUR, INC, REF, PER, TAX, FOB, CTP, PAM, CSH, SAC loop, ITD, DIS, DTM,
///   LIN, PID, MEA, PWK, PKG, TD1/TD5/TD3/TD4, MAN, RRA, CTB, LDT loop, N9 loop, N1 loop,
///   SPI loop (with N1 and CB1), PCT loop, ADV loop, LM loop.
/// Detail LOOP PO1: PO1, LIN, G53, CUR, CN1, PO3, CTP, PAM, CTB, MEA, PID loop, PWK, PKG,
///   PO4, REF, PER, SAC loop, IT8, CSH, ITD, DIS, INC, TAX, FOB, SDQ, DTM, FST, TD1/TD5/TD3/
///   TD4, MAN, RRA, MTX, SPI, LM loop, QTY loop, SCH loop, LDT loop, SLN loop, N9 loop,
///   N1 loop, PCT loop.
/// Summary: CTT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840 {
    pub st: ST,
    pub bqt: BQT,
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
    pub loop_sac: Vec<_840LoopSac>,
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
    pub rra: Vec<RRA>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ctb: Vec<CTB>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LDT")]
    pub loop_ldt: Vec<_840LoopLdt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_840LoopN9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_840LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "SPI")]
    pub loop_spi: Vec<_840LoopSpi>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "PCT")]
    pub loop_pct: Vec<_840LoopPct>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "ADV")]
    pub loop_adv: Vec<_840LoopAdv>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_840LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "PO1")]
    pub loop_po1: Vec<_840LoopPo1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

/// Reusable service/charge loop (SAC + CUR).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopSac {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

/// Reusable lead-time loop (LDT + QTY/MTX + LM).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopLdt {
    pub ldt: LDT,
    pub qty: Vec<QTY>,
    pub mtx: Vec<MTX>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_840LoopLm>,
}

/// Reusable code-source loop (LM + LQ).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Reusable extended-reference loop (N9 + DTM/MTX/PWK/EFI).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopN9 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub pwk: Vec<PWK>,
    pub efi: Vec<EFI>,
}

/// Reusable percent-amount loop (PCT + AMT).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopPct {
    pub pct: PCT,
    pub amt: Vec<AMT>,
}

/// Reusable quantity loop (QTY + SI).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopQty {
    pub qty: QTY,
    pub si: Vec<SI>,
}

/// Reusable advertising loop (ADV + DTM/MTX).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopAdv {
    pub adv: ADV,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
}

/// Party loop, used at the heading, SPI, SLN levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub si: Vec<SI>,
    pub g61: Option<G61>,
    pub fob: Option<FOB>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub pkg: Vec<PKG>,
    pub rra: Vec<RRA>,
    pub mtx: Vec<MTX>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopSpi {
    pub spi: SPI,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_840LoopN1>,
    #[x12(loop_trigger = "CB1")]
    pub loop_cb1: Vec<_840LoopCb1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopCb1 {
    pub cb1: CB1,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub ldt: Option<LDT>,
    pub mtx: Vec<MTX>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopPo1 {
    pub po1: PO1,
    pub lin: Vec<LIN>,
    pub g53: Option<G53>,
    pub cur: Option<CUR>,
    pub cn1: Option<CN1>,
    pub po3: Vec<PO3>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub ctb: Vec<CTB>,
    pub mea: Vec<MEA>,
    #[x12(loop_trigger = "PID")]
    pub loop_pid: Vec<_840LoopPid>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub po4: Vec<PO4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_840LoopSac>,
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
    pub rra: Vec<RRA>,
    pub mtx: Vec<MTX>,
    pub spi: Vec<SPI>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_840LoopLm>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_840LoopQty>,
    #[x12(loop_trigger = "SCH")]
    pub loop_sch: Vec<_840LoopSch>,
    #[x12(loop_trigger = "LDT")]
    pub loop_ldt: Vec<_840LoopLdt>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_840LoopSln>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_840LoopN9>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_840LoopN1>,
    #[x12(loop_trigger = "PCT")]
    pub loop_pct: Vec<_840LoopPct>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopPid {
    pub pid: PID,
    pub mea: Vec<MEA>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopSch {
    pub sch: SCH,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub r#ref: Vec<REF>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _840LoopSln {
    pub sln: SLN,
    pub mtx: Vec<MTX>,
    pub pid: Vec<PID>,
    pub adv: Vec<ADV>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_840LoopQty>,
}
