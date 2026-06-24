use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 274 - Healthcare Provider Information
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Healthcare Provider Information Transaction Set (274) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to transmit provider data among interested parties.
///
/// Heading: ST, BHT, DTM, PER.
/// Detail LOOP 2000 (HL): HL, TRN, then LOOP 2100 (NM1) carrying name/demographic detail
///   and the NX1 (2110), LQ (2120), HPL (2130), REF (2140) and EMS (2150) sub-loops.
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274 {
    pub st: ST,
    pub bht: BHT,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtm: Option<DTM>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per: Option<PER>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "HL")]
    pub loop_2000: Vec<_274Loop2000>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274Loop2000 {
    pub hl: HL,
    pub trn: Option<TRN>,
    #[x12(loop_trigger = "NM1")]
    pub loop_2100: Vec<_274Loop2100>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274Loop2100 {
    pub nm1: NM1,
    pub n2: Vec<N2>,
    pub per: Vec<PER>,
    pub dmg: Option<DMG>,
    pub amt: Vec<AMT>,
    pub api: Vec<API>,
    pub deg: Vec<DEG>,
    pub ind: Option<IND>,
    pub lui: Vec<LUI>,
    pub dtp: Vec<DTP>,
    pub mtx: Vec<MTX>,
    pub qty: Vec<QTY>,
    pub ws: Vec<WS>,
    pub crc: Vec<CRC>,
    pub hsd: Vec<HSD>,
    pub bci: Vec<BCI>,
    pub pdi: Option<PDI>,
    pub had: Option<HAD>,
    #[x12(loop_trigger = "NX1")]
    pub loop_2110: Vec<_274Loop2110>,
    #[x12(loop_trigger = "LQ")]
    pub loop_2120: Vec<_274Loop2120>,
    #[x12(loop_trigger = "HPL")]
    pub loop_2130: Vec<_274Loop2130>,
    #[x12(loop_trigger = "REF")]
    pub loop_2140: Vec<_274Loop2140>,
    #[x12(loop_trigger = "EMS")]
    pub loop_2150: Vec<_274Loop2150>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274Loop2110 {
    pub nx1: NX1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274Loop2120 {
    pub lq: LQ,
    pub n1: Vec<N1>,
    pub tpb: Vec<TPB>,
    pub dtp: Vec<DTP>,
    pub qty: Option<QTY>,
    pub ynq: Vec<YNQ>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274Loop2130 {
    pub hpl: HPL,
    pub dtp: Vec<DTP>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274Loop2140 {
    pub r#ref: REF,
    pub dtp: Vec<DTP>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _274Loop2150 {
    pub ems: EMS,
    pub dtp: Vec<DTP>,
}
