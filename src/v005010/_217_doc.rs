use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 217 - Motor Carrier Loading and Route Guide
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Motor Carrier Loading and Route Guide Transaction Set (217) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a motor carrier to provide a loading and route guide, identifying terminals, the service matrix and service points.
///
/// Heading: ST, BLR, then the N1 terminal loop (0100).
/// Detail: an LS-bracketed service-matrix section — the 0200 N1 loop (with GY/N4) carrying
///   an LS-bracketed 0210 LX loop (N1/GY/N4/SV/RST) — followed by the 0300 LX service-points
///   loop (N1/GY/N4/SV/RST).
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _217 {
    pub st: ST,
    pub blr: BLR,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_0100: Vec<_217Loop0100>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LS")]
    pub loop_0200_section: Vec<_217LoopLs>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LX")]
    pub loop_0300: Vec<_217Loop0300>,
    pub se: SE,
}

/// Terminal party loop (N1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _217Loop0100 {
    pub n1: N1,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Option<L11>,
    pub g61: Option<G61>,
}

/// LS-bracketed service-matrix section containing the 0200 N1 loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _217LoopLs {
    pub ls: LS,
    #[x12(loop_trigger = "N1")]
    pub loop_0200: Vec<_217Loop0200>,
    pub le: Option<LE>,
}

/// Service-matrix party loop (N1), with an LS-bracketed 0210 line-item sub-section.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _217Loop0200 {
    pub n1: N1,
    pub gy: Vec<GY>,
    pub n4: Vec<N4>,
    #[x12(loop_trigger = "LS")]
    pub loop_0210_section: Vec<_217Loop0210Ls>,
}

/// LS-bracketed sub-section containing the 0210 LX loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _217Loop0210Ls {
    pub ls: LS,
    #[x12(loop_trigger = "LX")]
    pub loop_0210: Vec<_217Loop0210>,
    pub le: Option<LE>,
}

/// Line-item loop (LX) within the service matrix.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _217Loop0210 {
    pub lx: LX,
    pub n1: Vec<N1>,
    pub gy: Vec<GY>,
    pub n4: Vec<N4>,
    pub sv: SV,
    pub rst: Vec<RST>,
}

/// Service-points line-item loop (LX).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _217Loop0300 {
    pub lx: LX,
    pub n1: Vec<N1>,
    pub gy: Vec<GY>,
    pub n4: Vec<N4>,
    pub sv: Option<SV>,
    pub rst: Vec<RST>,
}
