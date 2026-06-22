use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 270 - Eligibility, Coverage or Benefit Inquiry
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _270 {
    pub st: ST,
    pub bht: BHT,
    #[x12(loop_trigger = "HL")]
    pub loop_2000: Vec<_270Loop2000>,
    pub se: SE,
}

/// Loop 2000 - Information Source Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _270Loop2000 {
    pub hl: HL,
    pub trn: Vec<TRN>,
    #[x12(loop_trigger = "NM1")]
    pub loop_2100: Vec<_270Loop2100>,
}

/// Loop 2100 - Information Receiver Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _270Loop2100 {
    pub nm1: NM1,
    pub r#ref: Vec<REF>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub prv: Option<PRV>,
    pub dmg: Option<DMG>,
    pub ins: Option<INS>,
    pub hi: Option<HI>,
    pub dtp: Vec<DTP>,
    pub mpi: Vec<MPI>,
    #[x12(loop_trigger = "EQ")]
    pub loop_2110: Vec<_270Loop2110>,
}

/// Loop 2110 - Subscriber Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _270Loop2110 {
    pub eq: EQ,
    pub amt: Vec<AMT>,
    pub veh: Option<VEH>,
    pub pdr: Option<PDR>,
    pub pdp: Option<PDP>,
    pub iii: Vec<III>,
    pub r#ref: Option<REF>,
    pub dtp: Vec<DTP>,
}
