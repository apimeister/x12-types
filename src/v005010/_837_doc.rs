use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 837 - Health Care Claim
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837 {
    pub st: ST,
    pub bht: BHT,
    pub r#ref: Vec<REF>,
    pub loop_1000: Vec<_837Loop1000>,
    pub loop_2000: Vec<_837Loop2000>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop1000 {
    pub nm1: NM1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2000 {
    pub hl: HL,
    pub prv: Option<PRV>,
    pub sbr: Option<SBR>,
    pub pat: Option<PAT>,
    pub dtp: Option<DTP>,
    pub cur: Option<CUR>,
    pub loop_2010: Vec<_837Loop2010>,
    pub loop_2300: Vec<_837Loop2300>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2010 {
    pub nm1: NM1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dmg: Option<DMG>,
    pub r#ref: Vec<REF>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2300 {
    pub clm: CLM,
    pub dtp: Vec<DTP>,
    pub cl1: Option<CL1>,
    pub dn1: Option<DN1>,
    pub dn2: Option<DN2>,
    pub pwk: Option<PWK>,
    pub cn1: Option<CN1>,
    pub dsb: Option<DSB>,
    pub ur: Option<UR>,
    pub amt: Option<AMT>,
    pub r#ref: Vec<REF>,
    pub k3: Option<K3>,
    pub nte: Option<NTE>,
    pub cr1: Option<CR1>,
    pub cr2: Option<CR2>,
    pub cr3: Option<CR3>,
    pub cr4: Option<CR4>,
    pub cr5: Option<CR5>,
    pub cr6: Option<CR6>,
    pub cr8: Option<CR8>,
    pub crc: Option<CRC>,
    pub hi: Vec<HI>,
    pub qty: Option<QTY>,
    pub hcp: Option<HCP>,
    pub loop_2305: Vec<_837Loop2305>,
    pub loop_2310: Vec<_837Loop2310>,
    pub loop_2320: Vec<_837Loop2320>,
    pub loop_2400: Vec<_837Loop2400>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2305 {
    pub cr7: CR7,
    pub hsd: Vec<HSD>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2310 {
    pub nm1: NM1,
    pub prv: Option<PRV>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub r#ref: Option<REF>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2320 {
    pub sbr: SBR,
    pub cas: Option<CAS>,
    pub amt: Vec<AMT>,
    pub dmg: Option<DMG>,
    pub oi: Option<OI>,
    pub mia: Option<MIA>,
    pub moa: Option<MOA>,
    pub loop_2330: Vec<_837Loop2330>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2330 {
    pub nm1: NM1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Option<PER>,
    pub dtp: Option<DTP>,
    pub r#ref: Option<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2400 {
    pub lx: LX,
    pub sv1: Option<SV1>,
    pub sv2: Option<SV2>,
    pub sv3: Option<SV3>,
    pub too: Option<TOO>,
    pub sv4: Option<SV4>,
    pub sv5: Option<SV5>,
    pub sv6: Option<SV6>,
    pub sv7: Option<SV7>,
    pub hi: Option<HI>,
    pub pwk: Option<PWK>,
    pub cr1: Option<CR1>,
    pub cr2: Option<CR2>,
    pub cr3: Option<CR3>,
    pub cr4: Option<CR4>,
    pub cr5: Option<CR5>,
    pub crc: Option<CRC>,
    pub dtp: Vec<DTP>,
    pub qty: Option<QTY>,
    pub mea: Option<MEA>,
    pub cn1: Option<CN1>,
    pub r#ref: Option<REF>,
    pub amt: Vec<AMT>,
    pub k3: Vec<K3>,
    pub nte: Option<NTE>,
    pub ps1: Option<PS1>,
    pub imm: Option<IMM>,
    pub hsd: Option<HSD>,
    pub hcp: Option<HCP>,
    pub loop_2410: Vec<_837Loop2410>,
    pub loop_2420: Vec<_837Loop2420>,
    pub loop_2430: Vec<_837Loop2430>,
    pub loop_2440: Vec<_837Loop2440>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2410 {
    pub lin: LIN,
    pub ctp: Option<CTP>,
    pub r#ref: Option<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2420 {
    pub nm1: NM1,
    pub prv: Option<PRV>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub r#ref: Option<REF>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2430 {
    pub svd: SVD,
    pub cas: Vec<CAS>,
    pub dtp: Option<DTP>,
    pub amt: Option<AMT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _837Loop2440 {
    pub lq: LQ,
    pub frm: Vec<FRM>,
}
