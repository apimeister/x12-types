use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 824 - Application Advice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Application Advice Transaction Set (824) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide the ability to report the results of an application system's data content edits of transaction sets.
///
/// Heading: ST, BGN, N1 loop (N1, N2, N3, N4, REF, PER).
/// Detail LOOP OTI: OTI, REF, DTM, PER, AMT, QTY, NM1, TED loop (TED, CTX, NTE, RED),
///   LM loop (LM, LQ loop (LQ, RED)).
/// Summary: SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _824 {
    pub st: ST,
    pub bgn: BGN,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_824LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "OTI")]
    pub loop_oti: Vec<_824LoopOti>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _824LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _824LoopOti {
    pub oti: OTI,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub per: Vec<PER>,
    pub amt: Vec<AMT>,
    pub qty: Vec<QTY>,
    pub nm1: Vec<NM1>,
    #[x12(loop_trigger = "TED")]
    pub loop_ted: Vec<_824LoopTed>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_824LoopLm>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _824LoopTed {
    pub ted: TED,
    pub ctx: Vec<CTX>,
    pub nte: Vec<NTE>,
    pub red: Vec<RED>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _824LoopLm {
    pub lm: LM,
    #[x12(loop_trigger = "LQ")]
    pub loop_lq: Vec<_824LoopLq>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _824LoopLq {
    pub lq: LQ,
    pub red: Vec<RED>,
}
