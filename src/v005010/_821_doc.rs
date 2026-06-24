use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 821 - Financial Information Reporting
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Financial Information Reporting Transaction Set (821) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by financial institutions to report account balances, transaction summaries and detail to their customers.
///
/// Heading: ST, B2A, DTM, TRN, N1, PER, REF, the LM code loop and the FA1 loop.
/// Detail LOOP ENT: ENT, an N1 party loop, an ACT account loop (CUR plus LM, RTE, BLN, TSU
///   and FIR sub-loops; the FIR loop carries an NM1 party loop) and an FA1 loop.
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821 {
    pub st: ST,
    pub b2a: B2A,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trn: Vec<TRN>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n1: Vec<N1>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_821LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_821LoopFa1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "ENT")]
    pub loop_ent: Vec<_821LoopEnt>,
    pub se: SE,
}

/// Heading code-source loop (LM + LQ).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Reusable financial-accounting loop (FA1 + FA2).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

/// Detail entity loop (ENT).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopEnt {
    pub ent: ENT,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_821LoopEntN1>,
    #[x12(loop_trigger = "ACT")]
    pub loop_act: Vec<_821LoopAct>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_821LoopFa1>,
}

/// Party loop (N1) nested in the entity loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopEntN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

/// Account loop (ACT) nested in the entity loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopAct {
    pub act: ACT,
    pub cur: Option<CUR>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_821LoopActLm>,
    #[x12(loop_trigger = "RTE")]
    pub loop_rte: Vec<_821LoopRte>,
    #[x12(loop_trigger = "BLN")]
    pub loop_bln: Vec<_821LoopBln>,
    #[x12(loop_trigger = "TSU")]
    pub loop_tsu: Vec<_821LoopTsu>,
    #[x12(loop_trigger = "FIR")]
    pub loop_fir: Vec<_821LoopFir>,
}

/// Account-level code loop (LM) with an LQ sub-loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopActLm {
    pub lm: LM,
    #[x12(loop_trigger = "LQ")]
    pub loop_lq: Vec<_821LoopLq>,
}

/// LQ loop (LQ + DTM) nested in the account-level LM loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopLq {
    pub lq: LQ,
    pub dtm: Vec<DTM>,
}

/// Rate loop (RTE + DTM).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopRte {
    pub rte: RTE,
    pub dtm: Option<DTM>,
}

/// Balance loop (BLN + AVA).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopBln {
    pub bln: BLN,
    pub ava: Vec<AVA>,
}

/// Transaction-summary loop (TSU + AVA).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopTsu {
    pub tsu: TSU,
    pub ava: Vec<AVA>,
}

/// Financial-transaction loop (FIR) with an NM1 party sub-loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopFir {
    pub fir: FIR,
    pub dtm: Vec<DTM>,
    pub r#ref: Vec<REF>,
    pub msg: Vec<MSG>,
    pub ava: Vec<AVA>,
    pub trn: Option<TRN>,
    pub n1: Vec<N1>,
    pub amt: Vec<AMT>,
    pub ctp: Vec<CTP>,
    pub rte: Vec<RTE>,
    #[x12(loop_trigger = "NM1")]
    pub loop_nm1: Vec<_821LoopNm1>,
}

/// Party loop (NM1) nested in the FIR loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _821LoopNm1 {
    pub nm1: NM1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub dtm: Option<DTM>,
    pub n9: Vec<N9>,
}
