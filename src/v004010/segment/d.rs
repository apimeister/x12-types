use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// D9 - Route Carrier
///
/// To specify the route carrier details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 02 | 19 | City Name | 1 | O | AN | 2/30
/// 03 | 156 | State or Province Code | 1 | M | ID | 2/2
/// 04 | 26 | Country Code | 1 | O | ID | 2/3
/// 05 | 100 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 06 | 19 | City Name | 1 | O | AN | 2/30
/// 07 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 08 | 100 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 09 | 100 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 10 | 100 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 11 | 100 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 12 | 26 | Country Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct D9 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
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
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "12")]
    pub _12: Option<String>,
}

/// DMG - Demographic Information
///
/// To supply demographic information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1250 | Date Time Period Format Qualifier | 1 | O | ID | 2/3
/// 02 | 1251 | Date Time Period | 1 | O | AN | 1/35
/// 03 | 1068 | Gender Code | 1 | O | ID | 1/1
/// 04 | 1067 | Marital Status Code | 1 | O | ID | 1/1
/// 05 | 1109 | Race or Ethnicity Code | 1 | O | ID | 1/1
/// 06 | 1066 | Citizenship Status Code | 1 | O | ID | 1/1
/// 07 | 26 | Country Code | 1 | O | ID | 2/3
/// 08 | 659 | Basis of Verification Code | 1 | O | ID | 1/2
/// 09 | 380 | Quantity | 1 | O | R | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct DMG {
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

/// DTM - Date/Time Reference
///
/// To specify pertinent dates and times
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 374 | Date/Time Qualifier | 1 | M | ID | 3/3
/// 02 | 373 | Date | 1 | O | DT | 8/8
/// 03 | 337 | Time | 1 | O | TM | 4/8
/// 04 | 623 | Time Code | 1 | O | ID | 2/2
/// 05 | 1250 | Date Time Period Format Qualifier | 1 | O | ID | 2/3
/// 06 | 1251 | Date Time Period | 1 | O | AN | 1/35
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct DTM {
    /// 374 - Date/Time Qualifier
    ///
    /// Code specifying type of date or time, or both date and time
    /// - TYPE=ID
    /// - MIN=3
    /// - MAX=3
    #[serde(rename = "01")]
    pub _01: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 623 - Time Code
    ///
    /// Code identifying the time. In accordance with International Standards Organization standard 8601, time can be specified by a + or - and an indication in hours in relation to Universal Time Coordinate (UTC) time; since + is a restricted character, + and - are substituted by P and M in the codes that follow
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
}
