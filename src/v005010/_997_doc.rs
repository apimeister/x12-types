use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 997 - Functional Acknowledgment
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Functional Acknowledgment Transaction Set (997) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to define the control structures for a set of acknowledgments to indicate the results of the syntactical analysis of the electronically encoded documents. The encoded documents are the transaction sets, which are grouped in functional groups, used in defining transactions for business data interchange. This standard does not cover the semantic meaning of the information encoded in the transaction sets.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1 |   |
/// 0020 | AK1 | Functional Group Response Header | M | 1 |   |
/// LOOP ID - AK2 | 999999
/// AK2 -> 0030 | AK2 | Transaction Set Response Header | O | 1 |   |
/// AK2 -> LOOP ID - AK3 | 999999 |
/// AK2 -> AK3 -> 0040 | AK3 | Data Segment Note | O | 1 |   |
/// AK2 -> AK3 -> 0050 | AK4 | Data Element Note | O | 99 |   |
/// AK2 -> 0060 | AK5 | Transaction Set Response Trailer | M | 1 |   |
/// 0070 | AK9 | Functional Group Response Trailer | M | 1 |   |
/// 0080 | SE | Transaction Set Trailer | M | 1 |
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _997 {
    pub st: ST,
    pub ak1: AK1,
    #[x12(loop_trigger = "AK2")]
    pub loop_ak2: Vec<_997LoopAK2>,
    pub ak9: AK9,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _997LoopAK2 {
    pub ak2: AK2,
    #[x12(loop_trigger = "AK3")]
    pub loop_ak3: Vec<_997LoopAK3>,
    pub ak5: AK5,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _997LoopAK3 {
    pub ak3: AK3,
    pub ak4: Vec<AK4>,
}
