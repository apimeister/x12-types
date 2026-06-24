use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 148 - Report of Injury, Illness or Incident
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Report of Injury, Illness or Incident Transaction Set (148) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to report information about an injury, illness, incident, or disability, including the parties involved, the affected individual, the employment relationship, the impairment(s), and any related compensation. It is widely used for workers' compensation "first report of injury" and related reporting.
///
/// The detail is a hierarchical (HL) structure: each HL loop carries the level's parties
/// (NM1 loop), employment status (ESI loop with its income LX loop), loan/benefit data
/// (LN loop), an impairment LX loop (III + LM and party NM1 sub-loops, bracketed by LS/LE),
/// and compensation financial information (CFI loop with AD1 and NM1 sub-loops).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148 {
    pub st: ST,
    pub bht: BHT,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qty: Option<QTY>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_nm1: Vec<_148LoopNm1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_hl: Vec<_148LoopHl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gri: Vec<GRI>,
    pub se: SE,
}

/// Heading party loop (0300).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopNm1 {
    pub nm1: NM1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub act: Vec<ACT>,
    pub dtp: Vec<DTP>,
    pub dmg: Option<DMG>,
    pub r#ref: Vec<REF>,
    pub qty: Vec<QTY>,
    pub crc: Vec<CRC>,
}

/// Detail hierarchical-level loop (0100).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopHl {
    pub hl: HL,
    pub cri: Vec<CRI>,
    pub act: Vec<ACT>,
    pub dtp: Vec<DTP>,
    pub cur: Option<CUR>,
    pub amt: Vec<AMT>,
    pub fc: Vec<FC>,
    pub r#ref: Vec<REF>,
    pub dmg: Option<DMG>,
    pub qty: Vec<QTY>,
    pub crc: Vec<CRC>,
    pub veh: Vec<VEH>,
    pub loop_nm1: Vec<_148LoopHlNm1>,
    pub loop_esi: Vec<_148LoopEsi>,
    pub loop_ln: Vec<_148LoopLn>,
    pub ls: Option<LS>,
    pub loop_lx: Vec<_148LoopHlLx>,
    pub le: Option<LE>,
    pub loop_cfi: Vec<_148LoopCfi>,
}

/// Party loop within an HL level (1300).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopHlNm1 {
    pub nm1: NM1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub act: Vec<ACT>,
    pub dtp: Vec<DTP>,
    pub dmg: Option<DMG>,
    pub r#ref: Vec<REF>,
    pub qty: Vec<QTY>,
    pub crc: Vec<CRC>,
}

/// Employment-status loop (1830).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopEsi {
    pub esi: ESI,
    pub emt: Option<EMT>,
    pub tpb: Option<TPB>,
    pub dtp: Vec<DTP>,
    pub r#ref: Vec<REF>,
    pub qty: Vec<QTY>,
    pub nm1: Option<NM1>,
    pub per: Vec<PER>,
    pub crc: Vec<CRC>,
    pub loop_lx: Vec<_148LoopEsiLx>,
}

/// Income detail loop within an employment-status loop (2050).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopEsiLx {
    pub lx: LX,
    pub ain: Option<AIN>,
    pub cur: Option<CUR>,
    pub txi: Vec<TXI>,
    pub ws: Vec<WS>,
    pub dtp: Vec<DTP>,
    pub qty: Vec<QTY>,
    pub r#ref: Vec<REF>,
}

/// Loan/benefit loop within an HL level (2700).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopLn {
    pub ln: LN,
    pub r#ref: Vec<REF>,
    pub loop_veh: Vec<_148LoopLnVeh>,
    pub loop_amt: Vec<_148LoopLnAmt>,
    pub loop_nm1: Vec<_148LoopLnNm1>,
}

/// Vehicle loop within a loan loop (2820).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopLnVeh {
    pub veh: VEH,
    pub n4: Option<N4>,
    pub pid: Vec<PID>,
}

/// Monetary-amount loop within a loan loop (2900).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopLnAmt {
    pub amt: AMT,
    pub pct: Option<PCT>,
    pub dtp: Vec<DTP>,
}

/// Party loop within a loan loop (3050).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopLnNm1 {
    pub nm1: NM1,
    pub dmg: Option<DMG>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Option<PER>,
    pub dtp: Vec<DTP>,
    pub r#ref: Vec<REF>,
}

/// Impairment line-item loop within an HL level (3200), bracketed by LS/LE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopHlLx {
    pub lx: LX,
    pub loop_iii: Vec<_148LoopLxIii>,
    pub loop_nm1: Vec<_148LoopLxNm1>,
}

/// Impairment detail loop within an impairment LX loop (3250).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopLxIii {
    pub iii: III,
    pub imp: Vec<IMP>,
    pub dtp: Vec<DTP>,
    pub crc: Vec<CRC>,
    pub qty: Vec<QTY>,
    pub r#ref: Vec<REF>,
    pub loop_lm: Vec<_148LoopIiiLm>,
}

/// Code-source loop within an impairment detail loop (3450).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopIiiLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Party loop within an impairment LX loop (3500).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopLxNm1 {
    pub nm1: NM1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub act: Vec<ACT>,
    pub dtp: Vec<DTP>,
    pub dmg: Option<DMG>,
    pub r#ref: Vec<REF>,
    pub crc: Vec<CRC>,
}

/// Compensation financial information loop within an HL level (4000).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopCfi {
    pub cfi: CFI,
    pub cur: Option<CUR>,
    pub amt: Vec<AMT>,
    pub dtp: Vec<DTP>,
    pub qty: Vec<QTY>,
    pub rel: Vec<REL>,
    pub r#ref: Vec<REF>,
    pub crc: Vec<CRC>,
    pub loop_ad1: Vec<_148LoopCfiAd1>,
    pub loop_nm1: Vec<_148LoopCfiNm1>,
}

/// Adjustment-amount loop within a CFI loop (4460).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopCfiAd1 {
    pub ad1: AD1,
    pub dtp: Vec<DTP>,
    pub r#ref: Vec<REF>,
}

/// Party loop within a CFI loop (4500).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _148LoopCfiNm1 {
    pub nm1: NM1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub act: Vec<ACT>,
    pub dtp: Vec<DTP>,
    pub dmg: Option<DMG>,
    pub r#ref: Vec<REF>,
}
