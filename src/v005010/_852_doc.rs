use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 852 - Product Activity Data
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Product Activity Data Transaction Set (852) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a supplier to evaluate the movement and disposition of products by a trading partner.
///
/// Heading: ST, XQ, XPO, N9, PER, N1 loop (N1, N2, N3, N4, FOB, TD5, DTM, N9, PER).
/// Detail LOOP LIN: LIN, CTP, SAC, PO4, N9, AMT, PAL, QTY, ZA loop (ZA, QTY, CTP, SDQ,
///   G95 loop (G95, DTM)).
/// Summary: CTT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _852 {
    pub st: ST,
    pub xq: XQ,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub xpo: Vec<XPO>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_852LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LIN")]
    pub loop_lin: Vec<_852LoopLin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _852LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub fob: Option<FOB>,
    pub td5: Option<TD5>,
    pub dtm: Vec<DTM>,
    pub n9: Vec<N9>,
    pub per: Vec<PER>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _852LoopLin {
    pub lin: LIN,
    pub ctp: Vec<CTP>,
    pub sac: Vec<SAC>,
    pub po4: Option<PO4>,
    pub n9: Vec<N9>,
    pub amt: Vec<AMT>,
    pub pal: Option<PAL>,
    pub qty: Vec<QTY>,
    #[x12(loop_trigger = "ZA")]
    pub loop_za: Vec<_852LoopZa>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _852LoopZa {
    pub za: ZA,
    pub qty: Vec<QTY>,
    pub ctp: Vec<CTP>,
    pub sdq: Vec<SDQ>,
    #[x12(loop_trigger = "G95")]
    pub loop_g95: Vec<_852LoopG95>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _852LoopG95 {
    pub g95: G95,
    pub dtm: Vec<DTM>,
}
