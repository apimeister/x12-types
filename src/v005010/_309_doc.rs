use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 309 - Customs Manifest
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Customs Manifest Transaction Set (309) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by carriers, terminal operators, port authorities, or service centers to provide Customs with manifest data for cargo arriving in or departing from oceangoing vessels, railroad trains, or other types of conveyances.
///
/// Heading: ST, M10, VEH, CII, then the NM1 party loop.
/// Detail LOOP P4 (port): P4 with an LX loop (M13/M11/N9) carrying an N1 party loop, an M12
///   in-bond loop and a VID conveyance loop (M7 plus an N10 line-item loop with VC/MAN and
///   an H1 hazardous-material loop).
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _309 {
    pub st: ST,
    pub m10: M10,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub veh: Vec<VEH>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cii: Vec<CII>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "NM1")]
    pub loop_nm1: Vec<_309LoopNm1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "P4")]
    pub loop_p4: Vec<_309LoopP4>,
    pub se: SE,
}

/// Party loop (NM1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _309LoopNm1 {
    pub nm1: NM1,
    pub dmg: Option<DMG>,
    pub dma: Option<DMA>,
    pub r#ref: Vec<REF>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}

/// Port loop (P4) with an LX line loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _309LoopP4 {
    pub p4: P4,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_309LoopLx>,
}

/// Bill-of-lading line loop (LX).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _309LoopLx {
    pub lx: LX,
    pub m13: Option<M13>,
    pub m11: Option<M11>,
    pub n9: Vec<N9>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_309LoopN1>,
    #[x12(loop_trigger = "M12")]
    pub loop_m12: Vec<_309LoopM12>,
    #[x12(loop_trigger = "VID")]
    pub loop_vid: Vec<_309LoopVid>,
}

/// Party loop (N1) nested in the LX loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _309LoopN1 {
    pub n1: N1,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub dtm: Option<DTM>,
    pub per: Option<PER>,
    pub x1: Option<X1>,
}

/// In-bond loop (M12 + R4) nested in the LX loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _309LoopM12 {
    pub m12: M12,
    pub r4: Vec<R4>,
}

/// Conveyance loop (VID) nested in the LX loop, with an N10 line-item sub-loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _309LoopVid {
    pub vid: VID,
    pub m7: Vec<M7>,
    #[x12(loop_trigger = "N10")]
    pub loop_n10: Vec<_309LoopN10>,
}

/// Line-item loop (N10) with VC/MAN and an H1 hazardous-material sub-loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _309LoopN10 {
    pub n10: N10,
    pub vc: Vec<VC>,
    pub man: Vec<MAN>,
    #[x12(loop_trigger = "H1")]
    pub loop_h1: Vec<_309LoopH1>,
}

/// Hazardous-material loop (H1 + H2).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _309LoopH1 {
    pub h1: H1,
    pub h2: Vec<H2>,
}
