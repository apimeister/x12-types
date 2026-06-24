use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 832 - Price/Sales Catalog
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Price/Sales Catalog Transaction Set (832) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide for the transmittal of price/sales catalog data.
///
/// Heading: ST, BCT, CTP, REF, YNQ, PER, DTM, CTB, CUR, ITD, LDT, SAC, TD1, TD5, TD3,
///   TD4, FOB, PKG, TXI, AAA, MTX, PWK, then the N1, LM, N9 and G93 loops.
/// Detail LOOP LIN: the item-identification loop with its CTP price-tier loop (each
///   carrying a G40 cost loop and LS-bracketed LM/N1 sub-loops), a party (N1) loop, a
///   G39 manufacturer-item loop, a PKL pack loop, an LFG hazardous-material loop (with a
///   CRC loop), an LM/LQ/PID code loop, an SLN subline loop (with LM and N1 sub-loops)
///   and an N9 loop.
/// Summary: CTT, SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832 {
    pub st: ST,
    pub bct: BCT,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ctp: Vec<CTP>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ynq: Vec<YNQ>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ctb: Vec<CTB>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cur: Vec<CUR>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub itd: Vec<ITD>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ldt: Vec<LDT>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sac: Vec<SAC>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td1: Vec<TD1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td5: Vec<TD5>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td3: Vec<TD3>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td4: Vec<TD4>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fob: Option<FOB>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pkg: Vec<PKG>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub txi: Vec<TXI>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aaa: Option<AAA>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mtx: Vec<MTX>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pwk: Vec<PWK>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_832LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_832LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_832LoopN9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "G93")]
    pub loop_g93: Vec<_832LoopG93>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LIN")]
    pub loop_lin: Vec<_832LoopLin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

/// Heading party loop (N1, N2, N3, N4, REF, PKG, PER, DTM, MTX, SPI).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Vec<N4>,
    pub r#ref: Vec<REF>,
    pub pkg: Vec<PKG>,
    pub per: Vec<PER>,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub spi: Option<SPI>,
}

/// Reusable code-source loop (LM + LQ), used at the heading, CTP and SLN levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Reusable note/attachment loop (N9, DTM, MTX, PWK, EFI), used at the heading and LIN levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopN9 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub pwk: Vec<PWK>,
    pub efi: Vec<EFI>,
}

/// Heading bracket/pricing loop (G93, SAC, N1, G26).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopG93 {
    pub g93: G93,
    pub sac: Option<SAC>,
    pub n1: Option<N1>,
    pub g26: Option<G26>,
}

/// Detail item loop (LIN).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopLin {
    pub lin: LIN,
    pub po1: Option<PO1>,
    pub g53: Option<G53>,
    pub si: Vec<SI>,
    pub dtm: Vec<DTM>,
    pub r#ref: Vec<REF>,
    pub ynq: Vec<YNQ>,
    pub per: Vec<PER>,
    pub crd: Vec<CRD>,
    pub ctb: Vec<CTB>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pkg: Vec<PKG>,
    pub po4: Option<PO4>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub itd: Vec<ITD>,
    pub ldt: Option<LDT>,
    pub sac: Vec<SAC>,
    pub fob: Option<FOB>,
    pub aaa: Option<AAA>,
    pub tc2: Vec<TC2>,
    pub txi: Vec<TXI>,
    pub mtx: Vec<MTX>,
    pub g55: Option<G55>,
    pub g54: Option<G54>,
    #[x12(loop_trigger = "CTP")]
    pub loop_ctp: Vec<_832LoopCtp>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_832LoopLinN1>,
    #[x12(loop_trigger = "G39")]
    pub loop_g39: Vec<_832LoopG39>,
    #[x12(loop_trigger = "PKL")]
    pub loop_pkl: Vec<_832LoopPkl>,
    #[x12(loop_trigger = "LFG")]
    pub loop_lfg: Vec<_832LoopLfg>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_832LoopLinLm>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_832LoopSln>,
    #[x12(loop_trigger = "N9")]
    pub loop_n9: Vec<_832LoopN9>,
}

/// Price-tier loop (CTP) nested in the LIN loop, with a G40 cost loop and LS-bracketed
/// LM/N1 sub-loops.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopCtp {
    pub ctp: CTP,
    pub dtm: Vec<DTM>,
    pub g36: Option<G36>,
    pub ldt: Option<LDT>,
    pub cur: Vec<CUR>,
    pub po4: Option<PO4>,
    pub ctb: Vec<CTB>,
    pub r#ref: Vec<REF>,
    pub g43: Vec<G43>,
    pub sac: Vec<SAC>,
    pub g26: Vec<G26>,
    pub txi: Vec<TXI>,
    pub itd: Option<ITD>,
    #[x12(loop_trigger = "G40")]
    pub loop_g40: Vec<_832LoopG40>,
    #[x12(loop_trigger = "LS")]
    pub loop_ls: Vec<_832LoopCtpLs>,
}

/// Cost loop (G40) nested in the CTP loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopG40 {
    pub g40: G40,
    pub sac: Option<SAC>,
    pub g93: Vec<G93>,
}

/// LS-bracketed loop nested in the CTP loop; each bracket carries either an LM code loop
/// or an N1 party loop, closed by LE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopCtpLs {
    pub ls: LS,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_832LoopLm>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_832LoopCtpN1>,
    pub le: Option<LE>,
}

/// Party loop (N1) nested in a CTP LS bracket.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopCtpN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Vec<N4>,
}

/// Party loop (N1) nested in the LIN loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopLinN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub pkg: Vec<PKG>,
    pub pal: Vec<PAL>,
    pub spi: Option<SPI>,
}

/// Manufacturer-item loop (G39 + CTP) nested in the LIN loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopG39 {
    pub g39: G39,
    pub ctp: Option<CTP>,
}

/// Pack loop (PKL) nested in the LIN loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopPkl {
    pub pkl: PKL,
    pub ctp: Option<CTP>,
    pub pkg: Vec<PKG>,
    pub g53: Option<G53>,
    pub dtm: Vec<DTM>,
}

/// Hazardous-material loop (LFG) nested in the LIN loop, with a CRC condition loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopLfg {
    pub lfg: LFG,
    #[x12(loop_trigger = "CRC")]
    pub loop_crc: Vec<_832LoopCrc>,
}

/// Condition loop (CRC + QTY) nested in the LFG loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopCrc {
    pub crc: CRC,
    pub qty: Option<QTY>,
}

/// Code loop (LM) nested in the LIN loop, with an LQ loop carrying a PID loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopLinLm {
    pub lm: LM,
    #[x12(loop_trigger = "LQ")]
    pub loop_lq: Vec<_832LoopLq>,
}

/// LQ loop nested in the LIN-level LM loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopLq {
    pub lq: LQ,
    pub r#ref: Vec<REF>,
    pub efi: Option<EFI>,
    pub dtm: Vec<DTM>,
    #[x12(loop_trigger = "PID")]
    pub loop_pid: Vec<_832LoopLqPid>,
}

/// PID loop nested in the LQ loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopLqPid {
    pub pid: PID,
    pub mea: Vec<MEA>,
    pub mtx: Option<MTX>,
}

/// Subline loop (SLN) nested in the LIN loop, with LM and N1 sub-loops.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopSln {
    pub sln: SLN,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub dtm: Vec<DTM>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub po4: Vec<PO4>,
    pub pkg: Vec<PKG>,
    pub qty: Vec<QTY>,
    pub mea: Vec<MEA>,
    pub sac: Vec<SAC>,
    pub mtx: Vec<MTX>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_832LoopLm>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_832LoopSlnN1>,
}

/// Party loop (N1) nested in the SLN loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopSlnN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Vec<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dtm: Vec<DTM>,
}
