use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 816 - Organizational Relationships
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Organizational Relationships Transaction Set (816) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to convey organizational relationships, names, addresses, and the hierarchy of these relationships.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0100 | ST | Transaction Set Header | M | 1
/// 0200 | BHT | Beginning of Hierarchical Transaction | M | 1
/// 0300 | DTM | Date/Time Reference | O | 10
/// 0350 | REF | Reference Information | O | 12
/// LOOP ID - N1 (heading) | >1
/// 0400 | N1 | Party Identification | M | 1
/// 0500 | N2 | Additional Name Information | O | 2
/// 0600 | N3 | Party Location | O | 2
/// 0700 | N4 | Geographic Location | O | 1
/// 0800 | PER | Administrative Communications Contact | O | 3
/// 0900 | REF | Reference Information | O | 12
/// LOOP ID - HL (detail) | >1
/// 0100 | HL | Hierarchical Level | M | 1
/// HL -> LOOP ID - N1 | >1
/// 0200 | N1 | Party Identification | M | 1
/// 0300 | N2 | Additional Name Information | O | 2
/// 0400 | N3 | Party Location | O | 2
/// 0500 | N4 | Geographic Location | O | 1
/// 0600 | PER | Administrative Communications Contact | O | >1
/// 0700 | REF | Reference Information | O | 12
/// 0800 | QTY | Quantity Information | O | 5
/// 0900 | DTM | Date/Time Reference | O | 10
/// 0950 | LQ | Industry Code Identification | O | >1
/// 1000 | ASI | Action or Status Indicator | O | 1
/// 1100 | SE | Transaction Set Trailer | M | 1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _816 {
    pub st: ST,
    pub bht: BHT,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_816LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "HL")]
    pub loop_hl: Vec<_816LoopHl>,
    pub se: SE,
}

/// Heading party loop (0400).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _816LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub r#ref: Vec<REF>,
}

/// Detail hierarchical-level loop (0100 detail).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _816LoopHl {
    pub hl: HL,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_816LoopHlN1>,
}

/// Party loop within a hierarchical level (0200 detail).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _816LoopHlN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub r#ref: Vec<REF>,
    pub qty: Vec<QTY>,
    pub dtm: Vec<DTM>,
    pub lq: Vec<LQ>,
    pub asi: Option<ASI>,
}
