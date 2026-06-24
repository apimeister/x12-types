use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 861 - Receiving Advice/Acceptance Certificate
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Receiving Advice/Acceptance Certificate Transaction Set (861) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to report the receipt, and the evaluation of the receipt, of goods or services.
///
/// Heading: ST, BRA, CUR, REF, PER, DTM, PRF, TD1, TD5, TD3, TD4, MEA, N1 loop, LM loop, FA1 loop.
/// Detail LOOP RCD: RCD, SN1, CUR, LIN, PID, PO4, REF, PER, DTM, PRF, MEA, FOB, TD1, TD5, TD3,
///   TD4, SAC, MAN, LM loop, SLN loop (SLN, PID, NM1, LM loop), N1 loop, FA1 loop.
/// Summary: CTT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _861 {
    pub st: ST,
    pub bra: BRA,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prf: Vec<PRF>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td1: Vec<TD1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td5: Vec<TD5>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td3: Vec<TD3>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub td4: Vec<TD4>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mea: Vec<MEA>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_861LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_861LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_861LoopFa1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "RCD")]
    pub loop_rcd: Vec<_861LoopRcd>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _861LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub fob: Option<FOB>,
}

/// Reusable code-source loop (LM + LQ), used at the heading, RCD and SLN levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _861LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Reusable financial-accounting loop (FA1 + FA2), used at the heading and RCD levels.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _861LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _861LoopRcd {
    pub rcd: RCD,
    pub sn1: Option<SN1>,
    pub cur: Option<CUR>,
    pub lin: Vec<LIN>,
    pub pid: Vec<PID>,
    pub po4: Vec<PO4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dtm: Vec<DTM>,
    pub prf: Vec<PRF>,
    pub mea: Vec<MEA>,
    pub fob: Option<FOB>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub sac: Vec<SAC>,
    pub man: Vec<MAN>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_861LoopLm>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_861LoopSln>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_861LoopN1>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_861LoopFa1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _861LoopSln {
    pub sln: SLN,
    pub pid: Vec<PID>,
    pub nm1: Option<NM1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_861LoopLm>,
}
