use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 990 - Response to a Load Tender
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Response to a Load Tender Transaction Set (990) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a carrier to provide a shipper with the status of a shipment tendered for transport.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _990 {
    pub st: ST,
    pub b1: B1,
    pub l11: Vec<L11>,
    pub g62: Vec<G62>,
    pub n9: Vec<N9>,
    pub n7: Vec<N7>,
    pub se: SE,
}
