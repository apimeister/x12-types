use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 753 - Request for Routing Instructions
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Request for Routing Instructions Transaction Set (753) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used by a shipper to request routing instructions from a carrier or controlling party.
///
/// Heading: ST, BGN, PER, N1 loop (N1, N2, N3, N4).
/// Detail: LX, then loop (N1, N2, N3, N4, L11, G62, USI, OID loop (OID, CMC)).
/// Summary: SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _753 {
    pub st: ST,
    pub bgn: BGN,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_753LoopN1>,
    pub lx: LX,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_detail: Vec<_753LoopDetail>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _753LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _753LoopDetail {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
    pub g62: Vec<G62>,
    pub usi: Option<USI>,
    #[x12(loop_trigger = "OID")]
    pub loop_oid: Vec<_753LoopOid>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _753LoopOid {
    pub oid: OID,
    pub cmc: Option<CMC>,
}
