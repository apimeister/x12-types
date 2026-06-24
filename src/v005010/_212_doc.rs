use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 212 - Motor Carrier Delivery Trailer Manifest
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Motor Carrier Delivery Trailer Manifest Transaction Set (212) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a motor carrier to provide a delivery trailer manifest, listing the shipments loaded on a trailer for delivery.
///
/// Heading: ST, ATA, B2A, L11, then the N1 delivery-location loop (0100) and the AT7
///   shipment-status loop (0150) with its MS2 equipment sub-loop (0160).
/// Detail LOOP 0200 (LX): LX, L11, AT7, BLR, MAN, AT8, Q7, G62, TSD, with an OID order
///   sub-loop (0210) and an N1 shipper sub-loop (0220).
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _212 {
    pub st: ST,
    pub ata: ATA,
    pub b2a: B2A,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub l11: Vec<L11>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_0100: Vec<_212Loop0100>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "AT7")]
    pub loop_0150: Vec<_212Loop0150>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "LX")]
    pub loop_0200: Vec<_212Loop0200>,
    pub se: SE,
}

/// Delivery-location loop (N1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _212Loop0100 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g61: Option<G61>,
    pub g62: Option<G62>,
    pub l11: Vec<L11>,
}

/// Shipment-status loop (AT7) with an equipment sub-loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _212Loop0150 {
    pub at7: AT7,
    pub g62: Vec<G62>,
    pub ms1: Option<MS1>,
    #[x12(loop_trigger = "MS2")]
    pub loop_0160: Vec<_212Loop0160>,
}

/// Equipment-details loop (MS2).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _212Loop0160 {
    pub ms2: MS2,
    pub m7: Option<M7>,
    pub at9: Option<AT9>,
}

/// Detail shipment line-item loop (LX).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _212Loop0200 {
    pub lx: LX,
    pub l11: Vec<L11>,
    pub at7: Option<AT7>,
    pub blr: Option<BLR>,
    pub man: Vec<MAN>,
    pub at8: Option<AT8>,
    pub q7: Vec<Q7>,
    pub g62: Vec<G62>,
    pub tsd: Option<TSD>,
    #[x12(loop_trigger = "OID")]
    pub loop_0210: Vec<_212Loop0210>,
    #[x12(loop_trigger = "N1")]
    pub loop_0220: Vec<_212Loop0220>,
}

/// Order-details loop (OID + SDQ).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _212Loop0210 {
    pub oid: OID,
    pub sdq: Vec<SDQ>,
}

/// Shipper-identification loop (N1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _212Loop0220 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
}
