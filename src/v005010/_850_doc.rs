use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 850 - Purchase Order
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850 {
    pub st: ST,
    pub beg: BEG,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub tax: Vec<TAX>,
    pub fob: Vec<FOB>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub csh: Vec<CSH>,
    pub tc2: Vec<TC2>,
    pub loop_sac: Vec<_850LoopSac>,
    pub itd: Vec<ITD>,
    pub dis: Vec<DIS>,
    pub inc: Option<INC>,
    pub dtm: Vec<DTM>,
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
    pub pct: Vec<PCT>,
    pub ctb: Vec<CTB>,
    pub txi: Vec<TXI>,
    pub loop_ldt: Vec<_850LoopLdt>,
    pub loop_amt: Vec<_850LoopAmt>,
    pub loop_n9: Vec<_850LoopN9>,
    pub loop_n1: Vec<_850LoopN1>,
    pub loop_po1: Vec<_850LoopPo1>,
    pub ctt: Option<CTT>,
    pub amt: Option<AMT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopSac {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopLdt {
    pub ldt: LDT,
    pub qty: Vec<QTY>,
    pub mtx: Vec<MTX>,
    pub r#ref: Vec<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopAmt {
    pub amt: AMT,
    pub r#ref: Vec<REF>,
    pub dtm: Option<DTM>,
    pub pct: Vec<PCT>,
    pub loop_fa1: Vec<_850LoopFa1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopN9 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub pwk: Vec<PWK>,
    pub efi: Vec<EFI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub si: Vec<SI>,
    pub fob: Option<FOB>,
    pub td1: Option<TD1>,
    pub td3: Option<TD3>,
    pub td4: Option<TD4>,
    pub td5: Vec<TD5>,
    pub pkg: Vec<PKG>,
    pub ldt: Option<LDT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopPid {
    pub pid: PID,
    pub mea: Vec<MEA>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopQty {
    pub qty: QTY,
    pub si: Vec<SI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopSch {
    pub sch: SCH,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub r#ref: Vec<REF>,
    pub ldt: Option<LDT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopPkg {
    pub pkg: PKG,
    pub mea: Vec<MEA>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopN9Detail {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopN1Detail {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dmg: Option<DMG>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopN1Spi {
    pub n1: N1,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub cb1: Option<CB1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopSpi {
    pub spi: SPI,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub msg: Vec<MSG>,
    pub loop_n1: Vec<_850LoopN1Spi>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopAdv {
    pub adv: ADV,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopPo1 {
    pub po1: PO1,
    pub lin: Vec<LIN>,
    pub si: Vec<SI>,
    pub cur: Option<CUR>,
    pub po3: Vec<PO3>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub mea: Vec<MEA>,
    pub loop_pid: Vec<_850LoopPid>,
    pub pwk: Vec<PWK>,
    pub po4: Option<PO4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub loop_sac: Vec<_850LoopSac>,
    pub itd: Vec<ITD>,
    pub dis: Vec<DIS>,
    pub inc: Option<INC>,
    pub dtm: Vec<DTM>,
    pub tax: Vec<TAX>,
    pub fob: Vec<FOB>,
    pub sdq: Vec<SDQ>,
    pub man: Vec<MAN>,
    pub td1: Option<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub pct: Vec<PCT>,
    pub ctb: Vec<CTB>,
    pub txi: Vec<TXI>,
    pub loop_ldt: Vec<_850LoopLdt>,
    pub loop_qty: Vec<_850LoopQty>,
    pub loop_sch: Vec<_850LoopSch>,
    pub loop_pkg: Vec<_850LoopPkg>,
    pub loop_n9: Vec<_850LoopN9Detail>,
    pub loop_n1: Vec<_850LoopN1Detail>,
    pub loop_spi: Vec<_850LoopSpi>,
    pub loop_sln: Vec<_850LoopSln>,
    pub loop_adv: Vec<_850LoopAdv>,
    pub loop_lm: Vec<_850LoopLm>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _850LoopSln {
    pub sln: SLN,
    pub mtx: Vec<MTX>,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
}
