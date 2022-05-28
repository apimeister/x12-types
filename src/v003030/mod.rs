//! v003030 repesents all entities of the 003030 specification.

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
mod segment;
pub use segment::*;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Transmission {
    pub isa: ISA,
    pub functional_group: Vec<FunctionalGroup>,
    pub iea: IEA,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct FunctionalGroup {
    pub gs: GS,
    pub segments: Vec<Segments>,
    pub ge: GE,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Segments {
    _998(_998),
}

/// 998 - Set Cancellation NEW
/// 
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Set Cancellation Transaction Set (998) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to request the deletion of a previously transmitted transaction set and will indicate the reason for this action, such as diversion or cancelled bill.
/// 
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1
/// 0020 | ZD | Transaction Set Deletion - ID, Reason, and Source | M | 1
/// 0030 | SE | Transaction Set Trailer | M | 1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _998 {
    pub st: ST,
    pub zd: ZD,
    pub se: SE,
}