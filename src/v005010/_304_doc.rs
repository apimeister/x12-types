use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 304 - Shipping Instructions
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Shipping Instructions Transaction Set (304) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide shipping instructions from a shipper or its agent to an ocean carrier, freight forwarder, or customs broker. It conveys the parties, routing, equipment, commodity, rate, charge, hazardous-material, and documentation detail required to prepare a bill of lading and arrange ocean transportation.
///
/// Heading:
/// 0100 ST, 0200 B2, 0300 B2A, 0400 Y6, 0500 G1, 0600 G2, 0700 G3, 0800 N9, 0850 YNQ,
/// 0900 V1, 1000 V3, 1100 M0, 1150 CUR, LOOP M1 (1200 M1, 1250 CUR), 1300 M2, 1400 C2,
/// 1550 ITD, 1560 DTM, LOOP N1 (1600 N1, 1700 N2, 1800 N3, 1900 N4, 1950 G61),
/// LOOP R4 (2000 R4, 2100 DTM), 2160 R2A, 2200 R2, 2300 K1, 2400 L11, 2500 H3, 2600 L5,
/// 2700 X1, 2800 X2, LOOP C8 (2900 C8, 2950 C8C, 3000 SUP).
///
/// Detail:
/// LOOP LX (0100 LX, 0200 Y2, LOOP N7 [..], 1510 L11, 1600 K1, LOOP PO4 [..], LOOP L0 [..]).
///
/// Summary:
/// LOOP L3 (0100 L3, 0110 CUR, 0150 MEA, 0200 PWK, 0250 SUP, LOOP L1, LOOP TDS, LOOP SAC,
/// LOOP L9, 0480 ISS, 0500 V9, 0600 K1, 0700 L11), 0800 SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304 {
    pub st: ST,
    pub b2: B2,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b2a: Option<B2A>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub y6: Vec<Y6>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g1: Option<G1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g2: Option<G2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g3: Option<G3>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ynq: Vec<YNQ>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub v1: Vec<V1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v3: Option<V3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub m0: Option<M0>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "M1")]
    pub loop_m1: Vec<_304LoopM1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub m2: Option<M2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c2: Option<C2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub itd: Option<ITD>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_304LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "R4")]
    pub loop_r4: Vec<_304LoopR4>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r2a: Vec<R2A>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r2: Vec<R2>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub k1: Vec<K1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub l11: Vec<L11>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub h3: Vec<H3>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub l5: Vec<L5>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub x1: Vec<X1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub x2: Vec<X2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "C8")]
    pub loop_c8: Vec<_304LoopC8>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_304LoopLx>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "L3")]
    pub loop_l3: Vec<_304LoopL3>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopM1 {
    pub m1: M1,
    pub cur: Option<CUR>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g61: Vec<G61>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopR4 {
    pub r4: R4,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopC8 {
    pub c8: C8,
    pub c8c: Vec<C8C>,
    pub sup: Vec<SUP>,
}

/// Reusable rate-and-charges loop (L1 + CUR), used in the N7, L0, and L3 loops.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopL1 {
    pub l1: L1,
    pub cur: Option<CUR>,
}

/// Reusable hazardous-material loop (H1 + H2), used in the N7 and L0 loops.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopH1 {
    pub h1: H1,
    pub h2: Vec<H2>,
}

/// Reusable hazardous-identification loop (LH1 ..), used in the N7 and L0 loops.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopLh1 {
    pub lh1: LH1,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Vec<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Vec<LHT>,
    pub lhr: Vec<LHR>,
    pub per: Vec<PER>,
}

/// Reusable item-physical-details loop (PO4 ..), used in the equipment and line-item loops.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopPo4 {
    pub po4: PO4,
    pub mea: Vec<MEA>,
    pub man: Vec<MAN>,
    pub n9: Vec<N9>,
}

/// Reusable service/charge loop (SAC + CUR), used in the L0 and L3 loops.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopSac {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

/// Reusable charge-detail loop (L9 + CUR), used in the L0 and L3 loops.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopL9 {
    pub l9: L9,
    pub cur: Option<CUR>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopLx {
    pub lx: LX,
    pub y2: Vec<Y2>,
    #[x12(loop_trigger = "N7")]
    pub loop_n7: Vec<_304LoopLxN7>,
    pub l11: Vec<L11>,
    pub k1: Vec<K1>,
    #[x12(loop_trigger = "PO4")]
    pub loop_po4: Vec<_304LoopPo4>,
    #[x12(loop_trigger = "L0")]
    pub loop_l0: Vec<_304LoopL0>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopLxN7 {
    pub n7: N7,
    pub qty: Option<QTY>,
    pub l4: Option<L4>,
    pub n12: Option<N12>,
    pub m7: Vec<M7>,
    pub m7a: Vec<M7A>,
    pub w09: Option<W09>,
    pub lh6: Vec<LH6>,
    #[x12(loop_trigger = "L1")]
    pub loop_l1: Vec<_304LoopL1>,
    pub l7: Option<L7>,
    pub x1: Vec<X1>,
    pub x2: Vec<X2>,
    pub n9: Vec<N9>,
    #[x12(loop_trigger = "H1")]
    pub loop_h1: Vec<_304LoopH1>,
    #[x12(loop_trigger = "LH1")]
    pub loop_lh1: Vec<_304LoopLh1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopL0 {
    pub l0: L0,
    pub mea: Vec<MEA>,
    pub n9: Vec<N9>,
    #[x12(loop_trigger = "PO4")]
    pub loop_po4: Vec<_304LoopPo4>,
    pub qty: Vec<QTY>,
    pub l4: Option<L4>,
    pub lh6: Vec<LH6>,
    #[x12(loop_trigger = "PAL")]
    pub loop_pal: Vec<_304LoopPal>,
    #[x12(loop_trigger = "CTP")]
    pub loop_ctp: Vec<_304LoopCtp>,
    pub l5: Vec<L5>,
    pub lin: Option<LIN>,
    pub l12: Vec<L12>,
    pub ynq: Vec<YNQ>,
    #[x12(loop_trigger = "L1")]
    pub loop_l1: Vec<_304LoopL1>,
    pub l7: Option<L7>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_304LoopSac>,
    #[x12(loop_trigger = "L9")]
    pub loop_l9: Vec<_304LoopL9>,
    pub x1: Vec<X1>,
    pub x2: Vec<X2>,
    #[x12(loop_trigger = "C8")]
    pub loop_c8: Vec<_304LoopC8>,
    #[x12(loop_trigger = "H1")]
    pub loop_h1: Vec<_304LoopH1>,
    #[x12(loop_trigger = "LH1")]
    pub loop_lh1: Vec<_304LoopLh1>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_304LoopN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopPal {
    pub pal: PAL,
    pub qty: Option<QTY>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopCtp {
    pub ctp: CTP,
    pub cur: Option<CUR>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopL3 {
    pub l3: L3,
    pub cur: Option<CUR>,
    pub mea: Vec<MEA>,
    pub pwk: Vec<PWK>,
    pub sup: Vec<SUP>,
    #[x12(loop_trigger = "L1")]
    pub loop_l1: Vec<_304LoopL1>,
    #[x12(loop_trigger = "TDS")]
    pub loop_tds: Vec<_304LoopTds>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_304LoopSac>,
    #[x12(loop_trigger = "L9")]
    pub loop_l9: Vec<_304LoopL9>,
    pub iss: Vec<ISS>,
    pub v9: Vec<V9>,
    pub k1: Vec<K1>,
    pub l11: Vec<L11>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _304LoopTds {
    pub tds: TDS,
    pub cur: Option<CUR>,
}
