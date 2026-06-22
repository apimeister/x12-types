use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 860 - Purchase Order Change Request - Buyer Initiated
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Purchase Order Change Request - Buyer Initiated Transaction Set (860) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide the information required for the customary and established business and industry practice relative to a purchase order change.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860 {
    pub st: ST,
    pub bch: BCH,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub tax: Vec<TAX>,
    pub fob: Vec<FOB>,
    pub ctp: Vec<CTP>,
    pub sac: Vec<SAC>,
    pub itd: Vec<ITD>,
    pub dis: Vec<DIS>,
    pub dtm: Vec<DTM>,
    pub pid: Vec<PID>,
    pub td5: Vec<TD5>,
    pub man: Vec<MAN>,
    pub txi: Vec<TXI>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_860LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_poc: Vec<_860LoopPoc>,
    pub ctt: Option<CTT>,
    pub amt: Option<AMT>,
    pub se: SE,
}

/// Loop N1 - Party Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

/// Loop POC - Line Item Change
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopPoc {
    pub poc: POC,
    pub lin: Vec<LIN>,
    pub ctp: Vec<CTP>,
    pub pid: Vec<PID>,
    pub r#ref: Vec<REF>,
    pub sac: Vec<SAC>,
    pub dtm: Vec<DTM>,
    pub sch: Vec<SCH>,
    pub loop_n1: Vec<_860LoopPocN1>,
}

/// Loop POC -> N1 - Party Identification (line level)
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _860LoopPocN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
}
