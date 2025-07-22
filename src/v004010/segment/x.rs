use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// X1 - Equipment Details
///
/// To specify the equipment details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 02 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 03 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 04 | 373 | Date | 1 | O | DT | 8/8
/// 05 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 06 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 07 | 26 | Country Code | 1 | O | ID | 2/3
/// 08 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 09 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 10 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 11 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 12 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 13 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 14 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 15 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 16 | 127 | Reference Identification | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct X1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: String,
    #[serde(rename = "06")]
    pub _06: String,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
    #[serde(rename = "15")]
    pub _15: Option<String>,
    #[serde(rename = "16")]
    pub _16: Option<String>,
}

/// X2 - Equipment Details
///
/// To specify the equipment details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 02 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 03 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 04 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 05 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 06 | 127 | Reference Identification | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct X2 {
    #[serde(rename = "01")]
    pub _01: String,
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
}

/// X7 - Equipment Details
///
/// To specify the equipment details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 02 | 127 | Reference Identification | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct X7 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// XH - Equipment Details
///
/// To specify the equipment details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 02 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 03 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 04 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 05 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 06 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 07 | 127 | Reference Identification | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct XH {
    #[serde(rename = "01")]
    pub _01: String,
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
}
