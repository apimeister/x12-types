use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 278 - Health Care Services Review Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _278 {
    pub st: ST,
    pub bht: BHT,
    #[x12(loop_trigger = "HL")]
    pub loops: Vec<_278Loop2000>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _278Loop2000 {
    pub hl: HL,
    pub trn: Vec<TRN>,
    pub aaa: Vec<AAA>,
    pub um: Option<UM>,
    pub hcr: Option<HCR>,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub hi: Option<HI>,
    pub sv1: Option<SV1>,
    pub sv2: Option<SV2>,
    pub sv3: Option<SV3>,
    pub too: Vec<TOO>,
    pub hsd: Option<HSD>,
    pub crc: Vec<CRC>,
    pub cl1: Option<CL1>,
    pub cr1: Option<CR1>,
    pub cr2: Option<CR2>,
    pub cr4: Option<CR4>,
    pub cr5: Option<CR5>,
    pub cr6: Option<CR6>,
    pub cr7: Option<CR7>,
    pub cr8: Option<CR8>,
    pub pwk: Vec<PWK>,
    pub msg: Vec<MSG>,
    #[x12(loop_trigger = "NM1")]
    pub loop_nm1: Vec<_278LoopNM1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _278LoopNM1 {
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
    pub dtp: Vec<DTP>,
}
