use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 310 - Freight Receipt and Invoice (Ocean)
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Freight Receipt and Invoice (Ocean) Transaction Set (310) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide ocean bill of lading information. It is sent by ocean carriers to interested parties and can be used as the receipt for the shipment; to substitute for a paper bill of lading where the parties have agreed that a paper bill of lading is not necessary; to allow shipper or forwarder to verify bill of lading information before an original is printed and released; for information purposes, i.e., as a bill of lading copy; by the carrier to convey manifest information to a terminal operator; and as an invoice for freight.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _310 {
    pub st: ST,
    pub b3: B3,
    pub b2a: Option<B2A>,
    #[serde(default)]
    pub y6: Vec<Y6>,
    pub g3: Option<G3>,
    #[serde(default)]
    pub n9: Vec<N9>,
    #[serde(default)]
    pub v1: Vec<V1>,
    pub m0: Option<M0>,
    #[serde(default)]
    pub m1: Vec<M1>,
    pub c2: Option<C2>,
    pub c3: Option<C3>,
    #[serde(default)]
    pub y2: Vec<Y2>,
    #[serde(default)]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_310LoopN1>,
    #[serde(default)]
    pub g61: Vec<G61>,
    #[serde(default)]
    #[x12(loop_trigger = "R4")]
    pub loop_r4: Vec<_310LoopR4>,
    #[serde(default)]
    pub r2a: Vec<R2A>,
    #[serde(default)]
    pub r2: Vec<R2>,
    /// heading remarks
    #[serde(default)]
    pub k1: Vec<K1>,
    #[serde(default)]
    pub h3: Vec<H3>,
    pub l5: Option<L5>,
    #[serde(default)]
    #[x12(loop_trigger = "C8")]
    pub loop_c8: Vec<_310LoopC8>,
    #[serde(default)]
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_310LoopLX>,
    pub l3: L3,
    #[serde(default)]
    pub pwk: Vec<PWK>,
    #[serde(default)]
    #[x12(loop_trigger = "L1")]
    pub loop_l1: Vec<_310LoopL1>,
    pub v9: Vec<V9>,
    pub c8: Vec<C8>,
    ///TODO summary remarks
    pub k1_2: Vec<K1>,
    pub l11: Option<L11>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _310LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _310LoopR4 {
    pub r4: R4,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtm: Option<DTM>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _310LoopC8 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c8: Option<C8>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub c8c: Vec<C8C>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _310LoopLX {
    pub lx: LX,
    #[x12(loop_trigger = "N7|L1")]
    pub loop_n7: Vec<_310LoopN7>,
    #[x12(loop_trigger = "L0")]
    pub loop_l0: Vec<_310LoopL0>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _310LoopN7 {
    pub n7: Option<N7>,
    pub qty: Option<QTY>,
    pub v4: Option<V4>,
    pub n12: Option<N12>,
    pub m7: Vec<M7>,
    pub w09: Option<W09>,
    #[x12(loop_trigger = "L1")]
    pub loop_l1: Vec<_310LoopL1>,
    pub l7: Option<L7>,
    pub x1: Option<X1>,
    pub x2: Option<X2>,
    pub n9: Vec<N9>,
    #[x12(loop_trigger = "H1")]
    pub loop_h1: Vec<_310LoopH1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _310LoopL0 {
    pub l0: Option<L0>,
    pub l5: Vec<L5>,
    #[x12(loop_trigger = "L1")]
    pub loop_l1: Vec<_310LoopL1>,
    pub l7: Option<L7>,
    pub x1: Option<X1>,
    pub x2: Option<X2>,
    #[x12(loop_trigger = "C8")]
    pub loop_c8: Vec<_310LoopC8>,
    #[x12(loop_trigger = "H1")]
    pub loop_h1: Vec<_310LoopH1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _310LoopL1 {
    pub l1: Option<L1>,
    pub c3: Option<C3>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _310LoopH1 {
    pub h1: Option<H1>,
    pub h2: Vec<H2>,
}
