use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// OID - Order Identification Detail NEW
///
/// To specify order identification detail
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | X/Z | AN | 1/30
/// 02 | 324 | Purchase Order Number | 1 | X | AN | 1/22
/// 03 | 127 | Reference Identification | 1 | O/Z | AN | 1/30
/// 04 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 05 | 380 | Quantity | 1 | X | R | 1/15
/// 06 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 07 | 81 | Weight | 1 | X | R | 1/10
/// 08 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 09 | 183 | Volume | 1 | X | R | 1/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct OID {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
}
