use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 812 - Credit/Debit Adjustment
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Credit/Debit Adjustment Transaction Set (812) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set provides the ability to report adjustment details relating to credits or debits.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812 {
    pub st: ST,
    pub bcd: BCD,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub n9: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_812LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_it1: Vec<_812LoopIt1>,
    pub tds: Option<TDS>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _812LoopIt1 {
    pub it1: IT1,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub amt: Vec<AMT>,
    pub sac: Vec<SAC>,
}
