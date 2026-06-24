use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 845 - Price Authorization Acknowledgment/Status
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Price Authorization Acknowledgment/Status Transaction Set (845) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a seller to acknowledge, or to convey the status of, a price authorization (a buyer's request for a special price/quantity on goods or services).
///
/// Heading: ST, BPA, CUR, NTE, REF, PER, DTM, then the N1 party loop.
/// Detail LOOP CON: CON, REF, PER, DTM, CTB, ITD, an N1 party loop and a PAD product
///   adjustment loop (with item pricing detail and LIN, N1 and CTP sub-loops).
/// Summary: CTT, SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _845 {
    pub st: ST,
    pub bpa: BPA,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_845LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "CON")]
    pub loop_con: Vec<_845LoopCon>,
    pub ctt: CTT,
    pub se: SE,
}

/// Reusable party loop (N1) used at the heading, CON and PAD levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _845LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dtm: Vec<DTM>,
    pub ctb: Vec<CTB>,
}

/// Detail contract loop (CON).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _845LoopCon {
    pub con: CON,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dtm: Vec<DTM>,
    pub ctb: Vec<CTB>,
    pub itd: Vec<ITD>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_845LoopN1>,
    #[x12(loop_trigger = "PAD")]
    pub loop_pad: Vec<_845LoopPad>,
}

/// Product adjustment loop (PAD) nested in the CON loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _845LoopPad {
    pub pad: PAD,
    pub ctb: Vec<CTB>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub uit: Option<UIT>,
    pub qty: Vec<QTY>,
    pub amt: Vec<AMT>,
    pub cur: Option<CUR>,
    pub sss: Vec<SSS>,
    pub shp: Vec<SHP>,
    pub dtm: Vec<DTM>,
    #[x12(loop_trigger = "LIN")]
    pub loop_lin: Vec<_845LoopLin>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_845LoopN1>,
    #[x12(loop_trigger = "CTP")]
    pub loop_ctp: Vec<_845LoopCtp>,
}

/// Item-identification loop (LIN) nested in the PAD loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _845LoopLin {
    pub lin: LIN,
    pub g53: Option<G53>,
    pub sln: Vec<SLN>,
}

/// Pricing loop (CTP) nested in the PAD loop, with a party sub-loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _845LoopCtp {
    pub ctp: CTP,
    pub dtm: Vec<DTM>,
    pub ctb: Vec<CTB>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_845LoopCtpN1>,
}

/// Party loop (N1) nested in the CTP loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _845LoopCtpN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}
