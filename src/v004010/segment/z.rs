use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// ZC1 - Contract Information
///
/// To provide contract information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 02 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 03 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 04 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 05 | 243 | Transaction Reference Date | 1 | M | DT | 8/8
/// 06 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 07 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 08 | 127 | Reference Identification | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct ZC1 {
    #[serde(rename = "01", skip_serializing_if = "Option::is_none")]
    pub _01: Option<String>,
    #[serde(rename = "02", skip_serializing_if = "Option::is_none")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: String,
    /// 243 - Transaction Reference Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "05")]
    pub _05: String,
    #[serde(rename = "06")]
    pub _06: String,
    #[serde(rename = "07")]
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: Option<String>,
}

/// ZD - Transaction Set Trailer
///
/// To indicate the end of a transaction set and provide the count of transmitted segments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 96 | Number of Included Segments | 1 | M | N0 | 1/10
/// 02 | 329 | Transaction Set Control Number | 1 | O | AN | 4/9
/// 03 | 96 | Number of Included Segments | 1 | M | N0 | 1/10
/// 04 | 329 | Transaction Set Control Number | 1 | M | AN | 4/9
/// 05 | 329 | Transaction Set Control Number | 1 | O | AN | 4/9
/// 06 | 243 | Transaction Reference Date | 1 | O | DT | 8/8
/// 07 | 96 | Number of Included Segments | 1 | M | N0 | 1/10
/// 08 | 329 | Transaction Set Control Number | 1 | O | AN | 4/9
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct ZD {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: String,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 243 - Transaction Reference Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: Option<String>,
}
