use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 822 - Account Analysis
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Account Analysis Transaction Set (822) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a financial institution to report account analysis data — balances, service charges and the rates used — to a customer.
///
/// Heading: ST, BGN, DTM, CUR, the N1 forwarder loop and the RTE default-rate loop.
/// Detail LOOP ENT: ENT, an N1 party loop and an ACT account loop (CUR, ADJ plus RTE, LX
///   (BLN balances) and SER service-charge sub-loops).
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _822 {
    pub st: ST,
    pub bgn: BGN,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_822LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "RTE")]
    pub loop_rte: Vec<_822LoopRte>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "ENT")]
    pub loop_ent: Vec<_822LoopEnt>,
    pub se: SE,
}

/// Reusable party loop (N1), used at the heading and entity levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _822LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

/// Reusable rate loop (RTE + DTM), used at the heading and account levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _822LoopRte {
    pub rte: RTE,
    pub dtm: Option<DTM>,
}

/// Detail entity loop (ENT).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _822LoopEnt {
    pub ent: ENT,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_822LoopN1>,
    #[x12(loop_trigger = "ACT")]
    pub loop_act: Vec<_822LoopAct>,
}

/// Account loop (ACT) nested in the entity loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _822LoopAct {
    pub act: ACT,
    pub cur: Option<CUR>,
    pub adj: Vec<ADJ>,
    #[x12(loop_trigger = "RTE")]
    pub loop_rte: Vec<_822LoopRte>,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_822LoopLx>,
    #[x12(loop_trigger = "SER")]
    pub loop_ser: Vec<_822LoopSer>,
}

/// Balance-reporting loop (LX) nested in the account loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _822LoopLx {
    pub lx: LX,
    pub bln: Vec<BLN>,
    pub dtm: Vec<DTM>,
}

/// Service-charge loop (SER) nested in the account loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _822LoopSer {
    pub ser: SER,
    pub ctp: Vec<CTP>,
    pub dtm: Vec<DTM>,
}
