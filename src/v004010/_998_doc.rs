use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 998 - Set Cancellation
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Set Cancellation Transaction Set (998) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to request the deletion of a previously transmitted transaction set and will indicate the reason for this action, such as diversion or cancelled bill.
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0100 | ST | Transaction Set Header | M | 1
/// 0200 | ZD | Transaction Set Deletion - ID, Reason, and Source | M | 1
/// 0300 | SE | Transaction Set Trailer | M | 1
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _998 {
    pub st: ST,
    pub zd: ZD,
    pub se: SE,
}
