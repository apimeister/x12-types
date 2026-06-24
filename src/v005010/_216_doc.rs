use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 216 - Motor Carrier Shipment Pickup Notification
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Motor Carrier Shipment Pickup Notification Transaction Set (216) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used to provide a motor carrier with notification that a shipment is available for pickup, conveying the pickup (ship-from) and delivery (ship-to) locations.
///
/// Heading: ST, PUN, G61, TEM, PRF, AT5, K2, then the N1 location loop.
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _216 {
    pub st: ST,
    pub pun: PUN,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g61: Option<G61>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tem: Option<TEM>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prf: Vec<PRF>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub at5: Vec<AT5>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k2: Option<K2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_0100: Vec<_216Loop0100>,
    pub se: SE,
}

/// Pickup/delivery-location loop (N1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _216Loop0100 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}
