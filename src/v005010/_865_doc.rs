use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 865 - Purchase Order Change Acknowledgment/Request - Seller Initiated
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Purchase Order Change Acknowledgment/Request - Seller Initiated Transaction Set (865) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a seller to convey acknowledgment of, or changes to, a purchase order.
///
/// Heading: ST, BCA, CUR, REF, PER, TAX, FOB, CTP, PAM, CSH, SAC loop, ITD, DIS, INC, DTM,
///   LIN, SI, PID, MEA, PWK, PKG, TD1/TD5/TD3/TD4, PCT, MAN, TXI, CTB, G53, LDT loop,
///   N9 loop, N1 loop, AMT loop, ADV loop, LM loop.
/// Detail LOOP POC: POC, LIN, SI, CUR, PO3, CTP, PAM, MEA, PID loop, PWK, PKG, PO4, REF,
///   PER, SAC loop, IT8, CSH, ITD, DIS, INC, TAX, FOB, SDQ, DTM, TD1/TD5/TD3/TD4, TXI, PCT,
///   ACK loop, LM loop, AMT loop, QTY loop, SCH loop, LDT loop, N9 loop, N1 loop, SLN loop,
///   PD loop.
/// Summary: CTT, AMT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865 {
    pub st: ST,
    pub bca: BCA,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csh: Option<CSH>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_865LoopSac>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub itd: Vec<ITD>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dis: Vec<DIS>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inc: Option<INC>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lin: Vec<LIN>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub si: Vec<SI>,
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
    pub pct: Vec<PCT>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub man: Vec<MAN>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub txi: Vec<TXI>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ctb: Vec<CTB>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g53: Vec<G53>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LDT")]
    pub loop_ldt: Vec<_865LoopLdt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_865LoopN9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_865LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "AMT")]
    pub loop_amt: Vec<_865LoopAmt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "ADV")]
    pub loop_adv: Vec<_865LoopAdv>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_865LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "POC")]
    pub loop_poc: Vec<_865LoopPoc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<AMT>,
    pub se: SE,
}

/// Reusable service/charge loop (SAC + CUR).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopSac {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

/// Reusable lead-time loop (LDT + QTY/MTX + LM).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopLdt {
    pub ldt: LDT,
    pub qty: Vec<QTY>,
    pub mtx: Vec<MTX>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_865LoopLm>,
}

/// Reusable code-source loop (LM + LQ).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Reusable extended-reference loop (N9 + DTM/MTX/PWK/EFI).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopN9 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub pwk: Vec<PWK>,
    pub efi: Vec<EFI>,
}

/// Reusable monetary-amount loop (AMT + PCT).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopAmt {
    pub amt: AMT,
    pub pct: Vec<PCT>,
}

/// Reusable quantity loop (QTY + SI).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopQty {
    pub qty: QTY,
    pub si: Vec<SI>,
}

/// Reusable advertising loop (ADV + DTM/MTX).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopAdv {
    pub adv: ADV,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
}

/// Party loop, used at the heading, POC, SLN levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopN1 {
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
    pub loop_ldt: Vec<_865LoopN1Ldt>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopN1Ldt {
    pub ldt: LDT,
    pub man: Vec<MAN>,
    pub qty: Vec<QTY>,
    pub mtx: Vec<MTX>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopPoc {
    pub poc: POC,
    pub lin: Vec<LIN>,
    pub si: Vec<SI>,
    pub cur: Option<CUR>,
    pub po3: Vec<PO3>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub mea: Vec<MEA>,
    #[x12(loop_trigger = "PID")]
    pub loop_pid: Vec<_865LoopPid>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub po4: Vec<PO4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_865LoopSac>,
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
    pub txi: Vec<TXI>,
    pub pct: Vec<PCT>,
    #[x12(loop_trigger = "ACK")]
    pub loop_ack: Vec<_865LoopAck>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_865LoopLm>,
    #[x12(loop_trigger = "AMT")]
    pub loop_amt: Vec<_865LoopAmt>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_865LoopQty>,
    #[x12(loop_trigger = "SCH")]
    pub loop_sch: Vec<_865LoopSch>,
    #[x12(loop_trigger = "LDT")]
    pub loop_ldt: Vec<_865LoopLdt>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_865LoopN9>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_865LoopN1>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_865LoopSln>,
    #[x12(loop_trigger = "PD")]
    pub loop_pd: Vec<_865LoopPd>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopPid {
    pub pid: PID,
    pub mea: Vec<MEA>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopAck {
    pub ack: ACK,
    pub dtm: Vec<DTM>,
    pub man: Vec<MAN>,
    pub spi: Vec<SPI>,
    pub mtx: Vec<MTX>,
    pub ctb: Vec<CTB>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopSch {
    pub sch: SCH,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub r#ref: Vec<REF>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopSln {
    pub sln: SLN,
    pub mtx: Vec<MTX>,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub po3: Vec<PO3>,
    pub mea: Vec<MEA>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub ack: Vec<ACK>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_865LoopSac>,
    pub dtm: Vec<DTM>,
    pub po4: Option<PO4>,
    pub tax: Vec<TAX>,
    pub adv: Vec<ADV>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_865LoopQty>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_865LoopN9>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_865LoopN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _865LoopPd {
    pub pd: PD,
    pub pdd: Vec<PDD>,
}
