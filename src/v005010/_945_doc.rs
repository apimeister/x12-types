use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 945 - Warehouse Shipping Advice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Warehouse Shipping Advice Transaction Set (945) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by the warehouse to advise the depositor that shipment was made. It is used to reconcile order quantities with shipment quantities.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _945 {
    pub st: ST,
    pub w06: W06,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_945LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g61: Vec<G61>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w27: Option<W27>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w6: Option<W6>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w28: Option<W28>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub w10: Vec<W10>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g72: Vec<G72>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lm: Vec<_945LoopLM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lx: Vec<_945LoopLX>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w03: Option<W03>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _945LoopN1 {
    pub n1: N1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n2: Vec<N2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n3: Vec<N3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n4: Option<N4>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _945LoopLM {
    pub lm: LM,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lq: Vec<LQ>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _945LoopLX {
    pub lx: LX,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub man: Vec<MAN>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pal: Option<PAL>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_w12: Vec<_945LoopW12>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _945LoopW12 {
    pub w12: W12,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g69: Vec<G69>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub qty: Vec<QTY>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mea: Vec<MEA>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt: Option<AMT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r4: Vec<R4>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w27: Option<W27>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n1: Vec<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g72: Vec<G72>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lm: Vec<_945LoopLM>,
}
