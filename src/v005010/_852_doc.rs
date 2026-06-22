use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 852 - Product Activity Data
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Product Activity Data Transaction Set (852) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a supplier to evaluate the movement and disposition of products by a trading partner.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _852 {
    pub st: ST,
    pub xq: XQ,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_852LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lin: Vec<_852LoopLin>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _852LoopN1 {
    pub n1: N1,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _852LoopLin {
    pub lin: LIN,
    pub za: Vec<ZA>,
    pub dtm: Vec<DTM>,
}
