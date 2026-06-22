use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 875 - Grocery Products Purchase Order
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Grocery Products Purchase Order Transaction Set (875) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used by a retail or wholesale grocery operation to order products from a supplier.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _875 {
    pub st: ST,
    pub g50: G50,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_875LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lin: Vec<_875LoopLin>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _875LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _875LoopLin {
    pub lin: LIN,
    pub g69: Vec<G69>,
    pub pid: Vec<PID>,
    pub qty: Vec<QTY>,
    pub r#ref: Vec<REF>,
}
