use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 869 - Order Status Inquiry
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Order Status Inquiry Transaction Set (869) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a buyer or buyer's representative to request the status of an order previously sent to a seller. It is the inquiry counterpart of the 870 Order Status Report.
///
/// Heading: ST, BSI, NTE.
/// Detail LOOP HL: HL, PRF, DTM, LIN, PID, MEA, QTY, DD, GF, then the REF, N1, LM and FA1
///   sub-loops.
/// Summary: CTT, SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _869 {
    pub st: ST,
    pub bsi: BSI,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "HL")]
    pub loop_hl: Vec<_869LoopHl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _869LoopHl {
    pub hl: HL,
    pub prf: Option<PRF>,
    pub dtm: Vec<DTM>,
    pub lin: Vec<LIN>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub qty: Option<QTY>,
    pub dd: Vec<DD>,
    pub gf: Option<GF>,
    #[x12(loop_trigger = "REF")]
    pub loop_ref: Vec<_869LoopRef>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_869LoopN1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_869LoopLm>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_869LoopFa1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _869LoopRef {
    pub r#ref: REF,
    pub dtm: Vec<DTM>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _869LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _869LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _869LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}
