use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 860 - Purchase Order Change Request - Buyer Initiated
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Purchase Order Change Request - Buyer Initiated Transaction Set (860) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide the information required for the customary and established business and industry practice relative to a purchase order change.
///
/// Heading: ST, BCH, CUR, REF, PER, TAX, FOB, CTP, PAM, SAC loop, ITD, DIS, INC, DTM, LIN,
///   SI, PID, MEA, PWK, PKG, TD1/TD5/TD3/TD4, CTB, MAN, G53, TXI, PCT, LDT loop, AMT loop
///   (with FA1), N9 loop, N1 loop, LM loop, SPI loop (with N1 and CB1), ADV loop.
/// Detail LOOP POC: POC, LIN, SI, CUR, CN1, PO3, CTP, PAM, MEA, PID loop, PWK, PKG, PO4,
///   REF, PER, SAC loop, IT8, CSH, ITD, DIS, INC, TAX, FOB, SDQ, DTM, TD1/TD5/TD3/TD4, MAN,
///   PCT, TC2, CTB, TXI, SPI, MTX, QTY loop, SCH loop, N9 loop, LDT loop, N1 loop, SLN loop,
///   AMT loop, LM loop.
/// Summary: CTT, AMT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860 {
    pub st: ST,
    pub bch: BCH,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tax: Vec<TAX>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fob: Vec<FOB>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ctp: Vec<CTP>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pam: Vec<PAM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_860LoopSac>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub itd: Vec<ITD>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dis: Vec<DIS>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inc: Option<INC>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lin: Vec<LIN>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub si: Vec<SI>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pid: Vec<PID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mea: Vec<MEA>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pwk: Vec<PWK>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pkg: Vec<PKG>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td1: Vec<TD1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td5: Vec<TD5>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td3: Vec<TD3>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td4: Vec<TD4>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ctb: Vec<CTB>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub man: Vec<MAN>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub g53: Vec<G53>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub txi: Vec<TXI>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pct: Vec<PCT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LDT")]
    pub loop_ldt: Vec<_860LoopLdt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "AMT")]
    pub loop_amt: Vec<_860LoopAmt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_860LoopN9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_860LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_860LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "SPI")]
    pub loop_spi: Vec<_860LoopSpi>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "ADV")]
    pub loop_adv: Vec<_860LoopAdv>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "POC")]
    pub loop_poc: Vec<_860LoopPoc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt: Option<AMT>,
    pub se: SE,
}

/// Reusable service/charge loop (SAC + CUR).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopSac {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

/// Reusable lead-time loop (LDT + QTY/MTX/REF), used at the heading and POC levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopLdt {
    pub ldt: LDT,
    pub qty: Vec<QTY>,
    pub mtx: Vec<MTX>,
    pub r#ref: Vec<REF>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_860LoopLm>,
}

/// Reusable code-source loop (LM + LQ).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Reusable financial-accounting loop (FA1 + FA2).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

/// Reusable monetary-amount loop (AMT + REF/DTM/PCT + FA1 loop).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopAmt {
    pub amt: AMT,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub pct: Vec<PCT>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_860LoopFa1>,
}

/// Reusable extended-reference loop (N9 + DTM/MTX/PWK/EFI).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopN9 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub pwk: Vec<PWK>,
    pub efi: Vec<EFI>,
}

/// Reusable code-source quantity loop (QTY + SI).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopQty {
    pub qty: QTY,
    pub si: Vec<SI>,
}

/// Party loop, used at the heading, SPI and SLN levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub in2: Vec<IN2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub nx2: Vec<NX2>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub g61: Option<G61>,
    pub si: Vec<SI>,
    pub fob: Option<FOB>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub pkg: Vec<PKG>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopSpi {
    pub spi: SPI,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_860LoopN1>,
    #[x12(loop_trigger = "CB1")]
    pub loop_cb1: Vec<_860LoopCb1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopCb1 {
    pub cb1: CB1,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub ldt: Option<LDT>,
    pub mtx: Vec<MTX>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopAdv {
    pub adv: ADV,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopPoc {
    pub poc: POC,
    pub lin: Vec<LIN>,
    pub si: Vec<SI>,
    pub cur: Option<CUR>,
    pub cn1: Option<CN1>,
    pub po3: Vec<PO3>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub mea: Vec<MEA>,
    #[x12(loop_trigger = "PID")]
    pub loop_pid: Vec<_860LoopPid>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub po4: Vec<PO4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_860LoopSac>,
    pub it8: Option<IT8>,
    pub csh: Vec<CSH>,
    pub itd: Vec<ITD>,
    pub dis: Vec<DIS>,
    pub inc: Option<INC>,
    pub tax: Vec<TAX>,
    pub fob: Vec<FOB>,
    pub sdq: Vec<SDQ>,
    pub dtm: Vec<DTM>,
    pub td1: Option<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub man: Vec<MAN>,
    pub pct: Vec<PCT>,
    pub tc2: Vec<TC2>,
    pub ctb: Vec<CTB>,
    pub txi: Vec<TXI>,
    pub spi: Vec<SPI>,
    pub mtx: Vec<MTX>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_860LoopQty>,
    #[x12(loop_trigger = "SCH")]
    pub loop_sch: Vec<_860LoopSch>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_860LoopN9>,
    pub ls: Option<LS>,
    #[x12(loop_trigger = "LDT")]
    pub loop_ldt: Vec<_860LoopLdt>,
    pub le: Option<LE>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_860LoopPocN1>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_860LoopSln>,
    #[x12(loop_trigger = "AMT")]
    pub loop_amt: Vec<_860LoopAmt>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_860LoopLm>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopPid {
    pub pid: PID,
    pub mea: Vec<MEA>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopSch {
    pub sch: SCH,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub r#ref: Vec<REF>,
}

/// Party loop within a POC line item, carrying its own lead-time sub-loop.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopPocN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub in2: Vec<IN2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub qty: Vec<QTY>,
    pub nx2: Vec<NX2>,
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
    pub loop_ldt: Vec<_860LoopN1Ldt>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopN1Ldt {
    pub ldt: LDT,
    pub man: Vec<MAN>,
    pub qty: Vec<QTY>,
    pub mtx: Vec<MTX>,
    pub r#ref: Vec<REF>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopSln {
    pub sln: SLN,
    pub mtx: Vec<MTX>,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub po3: Vec<PO3>,
    pub mea: Vec<MEA>,
    pub tc2: Vec<TC2>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_860LoopSac>,
    pub dtm: Vec<DTM>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub po4: Option<PO4>,
    pub tax: Vec<TAX>,
    pub adv: Vec<ADV>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_860LoopQty>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_860LoopN9>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_860LoopN1>,
}
