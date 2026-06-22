use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 834 - Benefit Enrollment and Maintenance
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834 {
    pub st: ST,
    pub bgn: BGN,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub amt: Vec<AMT>,
    pub qty: Vec<QTY>,
    #[x12(loop_trigger = "N1")]
    pub loop_1000: Vec<_834Loop1000>,
    #[x12(loop_trigger = "INS")]
    pub loop_2000: Vec<_834Loop2000>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop1000 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    #[x12(loop_trigger = "ACT")]
    pub loop_1100: Vec<_834Loop1100>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop1100 {
    pub act: Option<ACT>,
    pub r#ref: Vec<REF>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub dtp: Option<DTP>,
    pub amt: Option<AMT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2000 {
    pub ins: Option<INS>,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    #[x12(loop_trigger = "NM1")]
    pub loop_2100: Vec<_834Loop2100>,
    #[x12(loop_trigger = "DSB")]
    pub loop_2200: Vec<_834Loop2200>,
    #[x12(loop_trigger = "HD")]
    pub loop_2300: Vec<_834Loop2300>,
    #[x12(loop_trigger = "LC")]
    pub loop_2400: Vec<_834Loop2400>,
    #[x12(loop_trigger = "FSA")]
    pub loop_2500: Vec<_834Loop2500>,
    #[x12(loop_trigger = "RP")]
    pub loop_2600: Vec<_834Loop2600>,
    pub ls: Option<LS>,
    #[x12(loop_trigger = "LX")]
    pub loop_2700: Vec<_834Loop2700>,
    pub le: Option<LE>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2100 {
    pub nm1: Option<NM1>,
    pub per: Option<PER>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dmg: Option<DMG>,
    pub pm: Option<PM>,
    pub ec: Vec<EC>,
    pub icm: Option<ICM>,
    pub amt: Vec<AMT>,
    pub hlh: Option<HLH>,
    pub hi: Vec<HI>,
    pub lui: Vec<LUI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2200 {
    pub dsb: Option<DSB>,
    pub dtp: Vec<DTP>,
    pub ad1: Vec<AD1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2300 {
    pub hd: Option<HD>,
    pub dtp: Vec<DTP>,
    pub amt: Vec<AMT>,
    pub r#ref: Vec<REF>,
    pub idc: Vec<IDC>,
    #[x12(loop_trigger = "LX|NM1")]
    pub loop_2310: Vec<_834Loop2310>,
    #[x12(loop_trigger = "COB|REF|DTP")]
    pub loop_2320: Vec<_834Loop2320>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2310 {
    pub lx: Option<LX>,
    pub nm1: Option<NM1>,
    pub n1: Vec<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub prv: Option<PRV>,
    pub dtp: Vec<DTP>,
    pub pla: Option<PLA>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2320 {
    pub cob: Option<COB>,
    pub r#ref: Option<REF>,
    pub dtp: Option<DTP>,
    #[x12(loop_trigger = "NM1")]
    pub loop_2330: Vec<_834Loop2330>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2330 {
    pub nm1: Option<NM1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2400 {
    pub lc: Option<LC>,
    pub amt: Vec<AMT>,
    pub dtp: Vec<DTP>,
    pub r#ref: Vec<REF>,
    #[x12(loop_trigger = "BEN")]
    pub loop_2410: Vec<_834Loop2410>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2410 {
    pub ben: Option<BEN>,
    pub nm1: Option<NM1>,
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dmg: Option<DMG>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2500 {
    pub fsa: Option<FSA>,
    pub amt: Vec<AMT>,
    pub dtp: Vec<DTP>,
    pub r#ref: Vec<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2600 {
    pub rp: Option<RP>,
    pub dtp: Vec<DTP>,
    pub r#ref: Vec<REF>,
    pub inv: Vec<INV>,
    pub amt: Vec<AMT>,
    pub qty: Vec<QTY>,
    pub k3: Vec<K3>,
    pub rel: Option<REL>,
    #[x12(loop_trigger = "NM1")]
    pub loop_2610: Vec<_834Loop2610>,
    #[x12(loop_trigger = "FC")]
    pub loop_2630: Vec<_834Loop2630>,
    #[x12(loop_trigger = "AIN")]
    pub loop_2650: Vec<_834Loop2650>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2610 {
    pub nm1: Option<NM1>,
    pub n2: Option<N2>,
    pub dmg: Option<DMG>,
    pub ben: Option<BEN>,
    pub r#ref: Vec<REF>,
    #[x12(loop_trigger = "NX1")]
    pub loop_2620: Vec<_834Loop2620>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2620 {
    pub nx1: Option<NX1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dtp: Vec<DTP>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2630 {
    pub fc: Option<FC>,
    pub dtp: Vec<DTP>,
    #[x12(loop_trigger = "INV")]
    pub loop_2640: Vec<_834Loop2640>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2640 {
    pub inv: Option<INV>,
    pub dtp: Vec<DTP>,
    pub qty: Vec<QTY>,
    pub ent: Vec<ENT>,
    pub r#ref: Vec<REF>,
    pub amt: Vec<AMT>,
    pub k3: Vec<K3>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2650 {
    pub ain: Option<AIN>,
    pub qty: Vec<QTY>,
    pub dtp: Vec<DTP>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2700 {
    pub lx: Option<LX>,
    #[x12(loop_trigger = "N1")]
    pub loop_2750: Vec<_834Loop2750>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _834Loop2750 {
    pub n1: N1,
    pub r#ref: REF,
    pub dtp: Option<DTP>,
}
