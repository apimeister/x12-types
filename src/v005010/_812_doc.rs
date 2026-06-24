use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 812 - Credit/Debit Adjustment
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Credit/Debit Adjustment Transaction Set (812) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set provides the ability to report adjustment details relating to credits or debits.
///
/// Heading: ST, BCD, CUR, N9, PER, ITD, DTM, FOB, SHD, SAC, N1 loop (N1, N2, N3, N4, N9,
///   PER, AMT), LM loop, FA1 loop.
/// Detail LOOP CDD: CDD, LIN, PO4, N9, DTM, SAC loop (SAC, DTM), LM loop,
///   N11 loop (N11, AMT, PCT, N1 loop (N1, AMT, PCT)), FA1 loop.
/// Summary: SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812 {
    pub st: ST,
    pub bcd: BCD,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub itd: Vec<ITD>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fob: Option<FOB>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shd: Vec<SHD>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sac: Vec<SAC>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_812LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_812LoopLm>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_812LoopFa1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "CDD")]
    pub loop_cdd: Vec<_812LoopCdd>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub n9: Vec<N9>,
    pub per: Vec<PER>,
    pub amt: Vec<AMT>,
}

/// Reusable code-source loop (LM + LQ).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Reusable financial-accounting loop (FA1 + FA2).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812LoopCdd {
    pub cdd: CDD,
    pub lin: Option<LIN>,
    pub po4: Option<PO4>,
    pub n9: Vec<N9>,
    pub dtm: Vec<DTM>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_812LoopCddSac>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_812LoopLm>,
    #[x12(loop_trigger = "N11")]
    pub loop_n11: Vec<_812LoopN11>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_812LoopFa1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812LoopCddSac {
    pub sac: SAC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812LoopN11 {
    pub n11: N11,
    pub amt: Vec<AMT>,
    pub pct: Vec<PCT>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_812LoopN11N1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812LoopN11N1 {
    pub n1: N1,
    pub amt: Vec<AMT>,
    pub pct: Vec<PCT>,
}
