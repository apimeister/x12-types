use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 754 - Routing Instructions
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Routing Instructions Transaction Set (754) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used by a carrier or controlling party to provide routing instructions in response to a request.
///
/// Heading: ST, BGN, PER, N1 loop (N1, N2, N3, N4).
/// Detail LOOP LX: LX, L11, BLR, SMD, OID, G62, MSI, QTY loop (QTY, AT9),
///   N1 loop (N1, N2, N3, N4).
/// Summary: SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _754 {
    pub st: ST,
    pub bgn: BGN,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_754LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_754LoopLx>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _754LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _754LoopLx {
    pub lx: LX,
    pub l11: Vec<L11>,
    pub blr: Option<BLR>,
    pub smd: Vec<SMD>,
    pub oid: Vec<OID>,
    pub g62: Vec<G62>,
    pub msi: Option<MSI>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_754LoopQty>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_754LoopLxN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _754LoopQty {
    pub qty: QTY,
    pub at9: Option<AT9>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _754LoopLxN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}
