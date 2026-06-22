use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 824 - Application Advice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Application Advice Transaction Set (824) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide the ability to report the results of an application system's data content edits of transaction sets.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _824 {
    pub st: ST,
    pub bgn: BGN,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_824LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_oti: Vec<_824LoopOti>,
    pub se: SE,
}

/// Loop N1 - Party Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _824LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub r#ref: Vec<REF>,
}

/// Loop OTI - Original Transaction Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _824LoopOti {
    pub oti: OTI,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub per: Vec<PER>,
    pub amt: Vec<AMT>,
    pub qty: Vec<QTY>,
    pub loop_ted: Vec<_824LoopTed>,
    pub nte: Vec<NTE>,
}

/// Loop TED - Technical Error Description
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _824LoopTed {
    pub ted: TED,
    pub red: Vec<RED>,
}
