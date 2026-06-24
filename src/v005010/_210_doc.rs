use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 210 - Motor Carrier Freight Details and Invoice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Motor Carrier Freight Details and Invoice Transaction Set (210) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by motor carriers to provide detailed freight and invoice information to a paying party.
///
/// Heading: ST, B3, C2, C3, ITD, L11, G62, R3, H3, K1, N1 loop (N1, N2, N3, N4, L11),
///   N7 loop (N7, M7), OID loop (OID, SDQ).
/// Detail: S5 loop (S5, L11, G62, H3, OID loop, N1 loop (N1..L11, N7 loop)),
///   LX loop (LX, L11, L5, H1, H2, L0, L1, L4, L7, K1, OID loop,
///   N1 loop (N1..L11, CD3 loop (CD3, L11, H6, L9, POD, G62), OID loop)).
/// Summary: L3, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210 {
    pub st: ST,
    pub b3: B3,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c2: Option<C2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c3: Option<C3>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub itd: Vec<ITD>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub l11: Vec<L11>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r3: Vec<R3>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub h3: Vec<H3>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub k1: Vec<K1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_210LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N7")]
    pub loop_n7: Vec<_210LoopN7>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "OID")]
    pub loop_oid: Vec<_210LoopOid>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "S5")]
    pub loop_s5: Vec<_210LoopS5>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_210LoopLx>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l3: Option<L3>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
}

/// Reusable equipment loop (N7 + M7), used at the heading and within stop-off party loops.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopN7 {
    pub n7: N7,
    pub m7: Vec<M7>,
}

/// Reusable order-information loop (OID + SDQ).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopOid {
    pub oid: OID,
    pub sdq: Vec<SDQ>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopS5 {
    pub s5: S5,
    pub l11: Vec<L11>,
    pub g62: Vec<G62>,
    pub h3: Vec<H3>,
    #[x12(loop_trigger = "OID")]
    pub loop_oid: Vec<_210LoopOid>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_210LoopS5N1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopS5N1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
    #[x12(loop_trigger = "N7")]
    pub loop_n7: Vec<_210LoopN7>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopLx {
    pub lx: LX,
    pub l11: Vec<L11>,
    pub l5: Vec<L5>,
    pub h1: Vec<H1>,
    pub h2: Vec<H2>,
    pub l0: Vec<L0>,
    pub l1: Vec<L1>,
    pub l4: Vec<L4>,
    pub l7: Vec<L7>,
    pub k1: Vec<K1>,
    #[x12(loop_trigger = "OID")]
    pub loop_oid: Vec<_210LoopOid>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_210LoopLxN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopLxN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
    #[x12(loop_trigger = "CD3")]
    pub loop_cd3: Vec<_210LoopCd3>,
    #[x12(loop_trigger = "OID")]
    pub loop_oid: Vec<_210LoopOid>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _210LoopCd3 {
    pub cd3: CD3,
    pub l11: Vec<L11>,
    pub h6: Vec<H6>,
    pub l9: Vec<L9>,
    pub pod: Option<POD>,
    pub g62: Vec<G62>,
}
