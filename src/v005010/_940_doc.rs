use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 940 - Warehouse Shipping Order
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Warehouse Shipping Order Transaction Set (940) for use within the context of an Electronic Data Interchange (EDI) environment. This transaction set can be used to enable the depositor to advise a warehouse to make a shipment, confirm a shipment, or modify or cancel a previously submitted shipping order, or to report the status of the current shipping order to the depositor.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _940 {
    pub _010: ST,
    pub _020: W05,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0100: Vec<_940Loop100>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _090: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _100: Vec<G61>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _110: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _120: Vec<NTE>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _130: Option<W09>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _140: Option<W66>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _150: Option<W6>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _153: Vec<R2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _156: Option<BNX>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0200_loop: Vec<_940Loop200>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0300_loop: Vec<_940Loop300>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _0400: Option<W76>,
    pub _0500: SE,
}

// Loop Structures for 940

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _940Loop100 {
    pub _040: N1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _050: Vec<N2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _060: Vec<N3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _070: Option<N4>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _080: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _940Loop200 {
    pub _160: LM,
    pub _170: Vec<LQ>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _940Loop300 {
    pub _005: LX,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _010: Vec<MAN>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _015: Vec<SDQ>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _016: Option<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _017: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0310_loop: Vec<_940Loop310>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _940Loop310 {
    pub _020: W01,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _030: Vec<G69>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _040: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _045: Vec<NTE>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _050: Vec<W20>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _070: Vec<QTY>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _080: Option<AMT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _090: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _100: Option<G66>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _110: Vec<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _112: Vec<PER>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _114: Vec<LH2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _116: Option<LHR>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _118: Vec<LH6>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0320_loop: Vec<_940Loop320>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _940Loop320 {
    pub _120: LM,
    #[serde(default)]
    pub _130: Vec<LQ>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _135: Option<LS>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0330_loop: Vec<_940Loop330>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _940Loop330 {
    pub _140: LX,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _150: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _160: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _170: Option<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _175: Vec<SDQ>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0331_loop: Vec<_940Loop331>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _940Loop331 {
    pub _180: LM,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _190: Vec<LQ>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0332_loop: Vec<_940Loop332>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _940Loop332 {
    pub _200: LH1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _210: Vec<LH2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _220: Vec<LH3>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _230: Vec<LFH>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _240: Vec<LEP>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _250: Option<LH4>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _260: Vec<LHT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _270: Vec<LHR>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _280: Vec<PER>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _285: Option<LE>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _340_loop: Vec<_940Loop340>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _940Loop340 {
    pub _290: FA1,
    pub _300: Vec<FA2>,
}
