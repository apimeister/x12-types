use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 271 - Eligibility, Coverage or Benefit Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _271 {
    pub st: ST,
    pub bht: BHT,
    #[x12(loop_trigger = "HL")]
    pub loop_2000: Vec<_271Loop2000>,
    pub se: SE,
}

/// Loop 2000 - Information Source Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _271Loop2000 {
    pub hl: HL,
    pub trn: Vec<TRN>,
    pub aaa: Vec<AAA>,
    #[x12(loop_trigger = "NM1")]
    pub loop_2100: Vec<_271Loop2100>,
}

/// Loop 2100 - Information Receiver Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _271Loop2100 {
    pub nm1: NM1,
    pub r#ref: Vec<REF>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub aaa: Vec<AAA>,
    pub prv: Option<PRV>,
    pub dmg: Option<DMG>,
    pub ins: Option<INS>,
    pub hi: Option<HI>,
    pub dtp: Vec<DTP>,
    pub lui: Vec<LUI>,
    pub mpi: Vec<MPI>,
    #[x12(loop_trigger = "EB")]
    pub loop_2110: Vec<_271Loop2110>,
}

/// Loop 2110 - Subscriber Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _271Loop2110 {
    pub eb: EB,
    pub hsd: Vec<HSD>,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub aaa: Vec<AAA>,
    pub veh: Option<VEH>,
    pub pid: Option<PID>,
    pub pdr: Option<PDR>,
    pub pdp: Option<PDP>,
    pub lin: Option<LIN>,
    pub em: Option<EM>,
    pub sd1: Option<SD1>,
    pub pkd: Option<PKD>,
    pub msg: Vec<MSG>,
    #[x12(loop_trigger = "III")]
    pub loop_2115: Vec<_271Loop2115>,
    #[x12(loop_trigger = "LS")]
    pub loop_2120: Vec<_271Loop2120>,
}

/// Loop 2115 - Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _271Loop2115 {
    pub iii: III,
    pub dtp: Vec<DTP>,
    pub amt: Vec<AMT>,
    pub pct: Vec<PCT>,
    #[x12(loop_trigger = "LQ")]
    pub loop_2117: Vec<_271Loop2117>,
}

/// Loop 2117 - Industry Code
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _271Loop2117 {
    pub lq: LQ,
    pub amt: Vec<AMT>,
    pub pct: Vec<PCT>,
}

/// Loop 2120 - Additional Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _271Loop2120 {
    pub ls: LS,
    #[x12(loop_trigger = "NM1")]
    pub content: Vec<_271Loop2120Content>,
    pub le: LE,
}

/// Loop 2120 Content
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _271Loop2120Content {
    pub nm1: NM1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub prv: Option<PRV>,
}
