use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// E1 - Expiration
///
/// To specify expiration information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Entity Identifier Code | 1 | M | ID | 2/3
/// 02 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 03 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct E1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// E4 - Extended Reference Information
///
/// To specify extended reference information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 19 | City Name | 1 | M | AN | 2/30
/// 02 | 156 | State or Province Code | 1 | M | ID | 2/2
/// 03 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 04 | 26 | Country Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct E4 {
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// E5 - Extended Reference Information
///
/// To specify extended reference information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Entity Identifier Code | 1 | M | ID | 2/3
/// 02 | 100 | Entity Identifier Code | 1 | M | ID | 2/3
/// 03 | 19 | City Name | 1 | O | AN | 2/30
/// 04 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct E5 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// EA - Equipment Attributes
///
/// To specify equipment attributes
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Entity Identifier Code | 1 | M | ID | 2/3
/// 02 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 03 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct EA {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// EFI - External Filing Information
///
/// To specify external filing information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Entity Identifier Code | 1 | M | ID | 2/3
/// 02 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 03 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 04 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 05 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 06 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 07 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 08 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 09 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 10 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 11 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 12 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 13 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 14 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 15 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 16 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct EFI {
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

/// EM - Equipment Details
///
/// To specify equipment details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 02 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 03 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 04 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 05 | 26 | Country Code | 1 | O | ID | 2/3
/// 06 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 07 | 373 | Date | 1 | O | DT | 8/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct EM {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "07")]
    pub _07: Option<String>,
}

/// ETD - Excess Transportation Detail
///
/// To specify the excess transportation details for the shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 02 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 03 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 04 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 05 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 06 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 07 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 08 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 09 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
/// 10 | 100 | Entity Identifier Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct ETD {
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
    #[serde(rename = "10")]
    pub _10: Option<String>,
}
