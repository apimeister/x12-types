use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 180 - Return Merchandise Authorization and Notification
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Return Merchandise Authorization and Notification Transaction Set (180) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to request, authorize, or provide notification of returned products.
///
/// Heading: ST, BGN, RDR, PRF, DTM, N9, PER, SAC, G38, PKG, TD1, TD5, NTE, N1 loop, LM loop.
/// Detail LOOP BLI: BLI, N9, PID, RDR, SAC, AMT, MEA, CRC, NTE, PRF, DTM, DD, GF, TD5, SDQ,
///   LM loop, N1 loop, QTY loop (QTY, AMT, DTM, N1, LM loop, LX loop (LX, N9, DTM, N1,
///   LM loop)), FA1 loop.
/// Summary: SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _180 {
    pub st: ST,
    pub bgn: BGN,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdr: Option<RDR>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prf: Option<PRF>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sac: Vec<SAC>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g38: Option<G38>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pkg: Vec<PKG>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub td1: Vec<TD1>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub td5: Vec<TD5>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_180LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_180LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "BLI")]
    pub loop_bli: Vec<_180LoopBli>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _180LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

/// Reusable code-source loop (LM + LQ).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _180LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Reusable financial-accounting loop (FA1 + FA2).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _180LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _180LoopBli {
    pub bli: BLI,
    pub n9: Vec<N9>,
    pub pid: Vec<PID>,
    pub rdr: Option<RDR>,
    pub sac: Vec<SAC>,
    pub amt: Vec<AMT>,
    pub mea: Vec<MEA>,
    pub crc: Vec<CRC>,
    pub nte: Vec<NTE>,
    pub prf: Option<PRF>,
    pub dtm: Vec<DTM>,
    pub dd: Vec<DD>,
    pub gf: Option<GF>,
    pub td5: Vec<TD5>,
    pub sdq: Vec<SDQ>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_180LoopLm>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_180LoopN1>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_180LoopBliQty>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_180LoopFa1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _180LoopBliQty {
    pub qty: QTY,
    pub amt: Vec<AMT>,
    pub dtm: Vec<DTM>,
    pub n1: Option<N1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_180LoopLm>,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_180LoopBliQtyLx>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _180LoopBliQtyLx {
    pub lx: LX,
    pub n9: Vec<N9>,
    pub dtm: Vec<DTM>,
    pub n1: Option<N1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_180LoopLm>,
}
