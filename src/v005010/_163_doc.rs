use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 163 - Transportation Appointment Schedule Information
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Transportation Appointment Schedule Information Transaction Set (163) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to request, confirm, or change an appointment for the pickup or delivery of a shipment at a facility.
///
/// Heading: ST, B13, B2A, H6, N7, G62, L11, H3, G05.
/// LOOP 0100 (N1): party identification with a nested OID order loop (0150).
/// LOOP 0300 (S5): stop-off details with party/contact segments and a nested OID order
///   loop (0350).
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _163 {
    pub st: ST,
    pub b13: B13,
    pub b2a: B2A,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub h6: Vec<H6>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub n7: Vec<N7>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub l11: Vec<L11>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub h3: Vec<H3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g05: Option<G05>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_0100: Vec<_163Loop0100>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "S5")]
    pub loop_0300: Vec<_163Loop0300>,
    pub se: SE,
}

/// Party loop (N1) with a nested OID order loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _163Loop0100 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
    pub g61: Vec<G61>,
    pub g62: Vec<G62>,
    #[x12(loop_trigger = "OID")]
    pub loop_0150: Vec<_163LoopOid>,
}

/// Order information loop (OID + SDQ), used in the party and stop-off loops.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _163LoopOid {
    pub oid: OID,
    pub sdq: Vec<SDQ>,
}

/// Stop-off loop (S5) with party/contact segments and a nested OID order loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _163Loop0300 {
    pub s5: S5,
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
    pub h6: Vec<H6>,
    pub g61: Vec<G61>,
    pub g62: Vec<G62>,
    #[x12(loop_trigger = "OID")]
    pub loop_0350: Vec<_163LoopOid>,
}
