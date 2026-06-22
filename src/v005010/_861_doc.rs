use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 861 - Receiving Advice/Acceptance Certificate
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Receiving Advice/Acceptance Certificate Transaction Set (861) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to report the receipt, and the evaluation of the receipt, of goods or services.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _861 {
    pub st: ST,
    pub bra: BRA,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_861LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_rcd: Vec<_861LoopRcd>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _861LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _861LoopRcd {
    pub rcd: RCD,
    pub pid: Vec<PID>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
}
