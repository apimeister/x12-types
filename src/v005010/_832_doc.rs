use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 832 - Price/Sales Catalog
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Price/Sales Catalog Transaction Set (832) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide for the transmittal of price/sales catalog data.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832 {
    pub st: ST,
    pub bct: BCT,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_832LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lin: Vec<_832LoopLin>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _832LoopLin {
    pub lin: LIN,
    pub pid: Vec<PID>,
    pub ctp: Vec<CTP>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
}
