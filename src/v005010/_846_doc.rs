use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 846 - Inventory Inquiry/Advice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Inventory Inquiry/Advice Transaction Set (846) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to advise a trading partner of inventory positions, and to support a variety of standard business transactions associated with inventory levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846 {
    pub st: ST,
    pub bia: BIA,
    pub nte: Vec<NTE>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_846LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lin: Vec<_846LoopLin>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

/// Loop N1 - Party Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

/// Loop LIN - Item Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopLin {
    pub lin: LIN,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub qty: Vec<QTY>,
    pub ctp: Vec<CTP>,
    pub loop_n1: Vec<_846LoopLinN1>,
}

/// Loop LIN -> N1 - Party Identification (item level)
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _846LoopLinN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}
