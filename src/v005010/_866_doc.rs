use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 866 - Production Sequence
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Production Sequence Transaction Set (866) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to transmit the exact sequence in which the receiver wants production and shipment to occur, typically in support of just-in-time manufacturing.
///
/// Heading: ST, BSS, UIT, then the N1 party loop.
/// Detail LOOP DTM: DTM, UIT, QTY, REF, with a LIN loop (REF, QTY, PID, OQS), an SLN
///   subline loop (party + PER) and a PID description loop (QTY, MEA).
/// Summary: CTT, SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _866 {
    pub st: ST,
    pub bss: BSS,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uit: Option<UIT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_866LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "DTM")]
    pub loop_dtm: Vec<_866LoopDtm>,
    pub ctt: CTT,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _866LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub fob: Option<FOB>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _866LoopDtm {
    pub dtm: DTM,
    pub uit: Option<UIT>,
    pub qty: Option<QTY>,
    pub r#ref: Vec<REF>,
    #[x12(loop_trigger = "LIN")]
    pub loop_lin: Vec<_866LoopLin>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _866LoopLin {
    pub lin: LIN,
    pub r#ref: Vec<REF>,
    pub qty: Option<QTY>,
    pub pid: Option<PID>,
    pub oqs: Option<OQS>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_866LoopSln>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _866LoopSln {
    pub sln: SLN,
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub r#ref: Option<REF>,
    pub per: Option<PER>,
    #[x12(loop_trigger = "PID")]
    pub loop_pid: Vec<_866LoopPid>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _866LoopPid {
    pub pid: PID,
    pub qty: Option<QTY>,
    pub mea: Vec<MEA>,
}
