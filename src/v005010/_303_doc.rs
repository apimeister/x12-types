use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 303 - Booking Cancellation (Ocean)
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Booking Cancellation (Ocean) Transaction Set (303) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a shipper or its agent to cancel a previously requested or confirmed ocean booking.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0100 | ST | Transaction Set Header | M | 1
/// 0200 | B1 | Beginning Segment for Booking or Pickup/Delivery | M | 1
/// 0300 | Y6 | Authentication | O | 2
/// 0400 | Y5 | Space Booking Cancellation | M | 1
/// 0500 | V9 | Event Detail | O | 10
/// 0600 | SE | Transaction Set Trailer | M | 1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _303 {
    pub st: ST,
    pub b1: B1,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub y6: Vec<Y6>,
    pub y5: Y5,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub v9: Vec<V9>,
    pub se: SE,
}
