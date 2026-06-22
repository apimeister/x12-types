use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 855 - Purchase Order Acknowledgment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855 {
    pub st: ST,
    pub bak: BAK,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub tax: Vec<TAX>,
    pub fob: Vec<FOB>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub csh: Option<CSH>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_855LoopSac>,
    pub itd: Vec<ITD>,
    pub dis: Vec<DIS>,
    pub inc: Option<INC>,
    pub dtm: Vec<DTM>,
    pub ldt: Vec<LDT>,
    pub lin: Vec<LIN>,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub man: Vec<MAN>,
    pub txi: Vec<TXI>,
    pub ctb: Vec<CTB>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_855LoopN9>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_855LoopN1>,
    #[x12(loop_trigger = "PO1")]
    pub loop_po1: Vec<_855LoopPo1>,
    #[x12(loop_trigger = "CTT")]
    pub loop_ctt: Vec<_855LoopCtt>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855LoopSac {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855LoopN9 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub pwk: Vec<PWK>,
    pub efi: Vec<EFI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub si: Vec<SI>,
    pub fob: Option<FOB>,
    pub dtm: Option<DTM>,
    pub ldt: Option<LDT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855LoopPo1 {
    pub po1: PO1,
    pub lin: Vec<LIN>,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub po4: Option<PO4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_855LoopSacPo1>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub ack: Vec<ACK>,
    pub dtm: Vec<DTM>,
    pub ctb: Vec<CTB>,
    pub txi: Vec<TXI>,
    pub ldt: Vec<LDT>,
    pub man: Vec<MAN>,
    pub sdq: Vec<SDQ>,
    #[x12(loop_trigger = "SCH")]
    pub loop_sch: Vec<_855LoopSch>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_855LoopN9Po1>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_855LoopN1Po1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_855LoopLm>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855LoopSacPo1 {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855LoopSch {
    pub sch: SCH,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub r#ref: Vec<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855LoopN9Po1 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855LoopN1Po1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _855LoopCtt {
    pub ctt: CTT,
    pub amt: Option<AMT>,
}
