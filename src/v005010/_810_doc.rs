use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 810 - Invoice
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810 {
    pub st: ST,
    pub big: BIG,
    pub nte: Vec<NTE>,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub ynq: Vec<YNQ>,
    pub per: Vec<PER>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_810LoopN1>,
    pub itd: Vec<ITD>,
    pub dtm: Vec<DTM>,
    pub fob: Option<FOB>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub l7: Option<L7>,
    pub bal: Vec<BAL>,
    pub inc: Option<INC>,
    pub pam: Vec<PAM>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_810LoopLM>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_810LoopN9>,
    #[x12(loop_trigger = "V1")]
    pub loop_v1: Vec<_810LoopV1>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_810LoopFA1>,
    #[x12(loop_trigger = "IT1")]
    pub loop_it1: Vec<_810LoopIT1>,
    pub tds: TDS,
    pub txi: Vec<TXI>,
    pub cad: Option<CAD>,
    pub amt: Vec<AMT>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_810LoopSAC>,
    #[x12(loop_trigger = "ISS")]
    pub loop_iss: Vec<_810LoopISS>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dmg: Option<DMG>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810LoopLM {
    pub lm: LM,
    pub lq: LQ,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810LoopN9 {
    pub n9: N9,
    pub msg: Vec<MSG>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810LoopV1 {
    pub v1: V1,
    pub r4: Vec<R4>,
    pub dtm: Vec<DTM>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810LoopFA1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810LoopIT1 {
    pub it1: IT1,
    pub crc: Option<CRC>,
    pub qty: Vec<QTY>,
    pub cur: Option<CUR>,
    pub it3: Vec<IT3>,
    pub txi: Vec<TXI>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub mea: Vec<MEA>,
    #[x12(loop_trigger = "PID")]
    pub loop_pid: Vec<_810LoopPID>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub po4: Option<PO4>,
    pub itd: Vec<ITD>,
    pub r#ref: Vec<REF>,
    pub ynq: Vec<YNQ>,
    pub per: Vec<PER>,
    pub sdq: Vec<SDQ>,
    pub dtm: Vec<DTM>,
    pub cad: Vec<CAD>,
    pub l7: Vec<L7>,
    pub sr: Option<SR>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_810LoopSAC>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_810LoopSLN>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_810LoopN1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_810LoopLM>,
    #[x12(loop_trigger = "V1")]
    pub loop_v1: Vec<_810LoopV1>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_810LoopFA1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810LoopPID {
    pub pid: PID,
    pub mea: Vec<MEA>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810LoopSAC {
    pub sac: SAC,
    pub txi: Vec<TXI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810LoopSLN {
    pub sln: SLN,
    pub dtm: Option<DTM>,
    pub r#ref: Vec<REF>,
    pub pid: Vec<PID>,
    pub sac: Vec<SAC>,
    pub tc2: Vec<TC2>,
    pub txi: Vec<TXI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _810LoopISS {
    pub iss: ISS,
    pub pid: Option<PID>,
}
