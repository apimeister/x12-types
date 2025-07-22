use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// P4 - U.S. Port Information
///
/// To transmit identifying information for a U.S. port
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|-----|------|-------
/// 01 | 310 | Location Identifier M AN 1/30
/// 02 | 373 | Date M DT 8/8
/// 03 | 380 | Quantity O R 1/15
/// 04 | 310 | Location Identifier O AN 1/30
/// 05 | 337 | Time O TM 4/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct P4 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

/// P5 - Port Information
///
/// To indicate port-related data
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|-----|------|-------
/// 01 | 115 | Port or Terminal Function Code | M |  | ID 1/1
/// 02 | 309 | Location Qualifier | M |  | ID 1/2
/// 03 | 310 | Location Identifier | M |  | AN 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct P5 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
}

/// PAM - Period Amount
///
/// To indicate a quantity, and/or amount for an identified period
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PAM {
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
}

/// PER - Administrative Communications Contact
///
/// To identify a person or office to whom administrative communications should be directed
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 366 | Contact Function Code | 1 | M | ID | 2/2
/// 02 | 93 | Name | 1 | O | AN | 1/60
/// 03 | 365 | Communication Number Qualifier | 1 | X | ID | 2/2
/// 04 | 364 | Communication Number | 1 | X | AN | 1/80
/// 05 | 365 | Communication Number Qualifier | 1 | X | ID | 2/2
/// 06 | 364 | Communication Number | 1 | X | AN | 1/80
/// 07 | 365 | Communication Number Qualifier | 1 | X | ID | 2/2
/// 08 | 364 | Communication Number | 1 | X | AN | 1/80
/// 09 | 443 | Contact Inquiry Reference | 1 | O | AN | 1/20
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PER {
    #[serde(rename = "01")]
    pub _01: String,
    /// 93 - Name
    ///
    /// Free-form name
    /// - TYPE=AN
    /// - MIN=1
    /// - MAX=60
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

/// PI - Price Authority Identification
///
/// To communicate basis of pricing, such as contract number, quote number, or tariff number
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 128 | Reference Identification Qualifier | 1 | M | ID | 2/3
/// 02 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 03 | 436 | Primary Publication Authority Code | 1 | O | ID | 2/2
/// 04 | 930 | Regulatory Agency Code | 1 | O | ID | 3/5
/// 05 | 168 | Tariff Agency Code | 1 | O | ID | 1/4
/// 06 | 965 | Issuing Carrier Identifier | 1 | O | AN | 1/10
/// 07 | 660 | Contract Suffix | 1 | O/Z | AN | 1/2
/// 08 | 169 | Tariff Item Number | 1 | O/Z | AN | 1/16
/// 09 | 173 | Tariff Supplement Identifier | 1 | O/Z | AN | 1/4
/// 10 | 172 | Tariff Section | 1 | O/Z | AN | 1/2
/// 11 | 660 | Contract Suffix | 1 | O/Z | AN | 1/2
/// 12 | 373 | Date | 1 | X/Z | DT | 8/8
/// 13 | 373 | Date | 1 | X/Z | DT | 8/8
/// 14 | 629 | Alternation Precedence Code | 1 | O | ID | 1/1
/// 15 | 629 | Alternation Precedence Code | 1 | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PI {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
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
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "12")]
    pub _12: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
    #[serde(rename = "15")]
    pub _15: Option<String>,
}

/// PID - Product/Item Description
///
/// To describe a product or process in coded or free-form format
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PID {
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

/// PKG - Marking, Packaging, Loading
///
/// To describe marking, packaging, loading, and unloading requirements
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PKG {
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
}

/// PLD - Pallet Information NEW
///
/// To specify pallet information including quantity, exchange, and weight
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 406 | Quantity of Pallets Shipped | 1 | M | N0 | 1/3
/// 02 | 399 | Pallet Exchange Code | 1 | O | ID | 1/1
/// 03 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 04 | 81 | Weight | 1 | X | R | 1/10
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PLD {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// PO4 Item Physical Details
///
/// To specify the physical qualities, packaging, weights, and dimensions relating to the item
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PO4 {
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
    #[serde(rename = "17")]
    pub _17: Option<String>,
    #[serde(rename = "18")]
    pub _18: Option<String>,
}

/// PRF - Purchase Order Reference
///
/// To provide reference to a specific purchase order
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 324 | Purchase Order Number | 1 | M | AN | 1/22
/// 02 | 328 | Release Number | 1 | O | AN | 1/30
/// 03 | 327 | Change Order Sequence Number | 1 | O | AN | 1/8
/// 04 | 373 | Date | 1 | O/Z | DT | 8/8
/// 05 | 350 | Assigned Identification | 1 | O | AN | 1/20
/// 06 | 367 | Contract Number | 1 | O | AN | 1/30
/// 07 | 92 | Purchase Order Type Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PRF {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
}

/// PS - Protective Service Instructions
///
/// To specify mechanical protective service and ventilation instructions
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 746 | Protective Service Rule Code | 1 | M | ID | 3/9
/// 02 | 241 | Protective Service Code | 1 | M | ID | 1/4
/// 03 | 355 | Unit or Basis for Measurement Code | 1 | X/Z | ID | 2/2
/// 04 | 408 | Temperature | 1 | X/Z | R | 1/4
/// 05 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 06 | 573 | Freight Station Accounting Code | 1 | O | ID | 1/5
/// 07 | 19 | City Name | 1 | O | AN | 2/30
/// 08 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 09 | 81 | Weight | 1 | O | R | 1/10
/// 10 | 745 | Pre-Cooled (Rule 710) Code | 1 | O | ID | 1/1
/// 11 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
/// 12 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
/// 13 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
/// 14 | 408 | Temperature | 1 | X/Z | R | 1/4
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PS {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
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
}

/// PWK - Paperwork
///
/// To identify the type or transmission or both of paperwork or supporting information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 755 | Report Type Code | M |  | ID 2/2
/// 02 | 756 | Report Transmission Code | O |  | ID 1/2
/// 03 | 757 | Report Copies Needed | O |  | N0 1/2
/// 04 | 98 | Entity Identifier Code | O |  | ID 2/3
/// 05 | 66 | Identification Code Qualifier | X |  | ID 1/2
/// 06 | 67 | Identification Code | X |  | AN 2/80
/// 07 | 352 | Description | O |  | AN 1/80
/// 08 | C002 | Actions Indicated | O |  |
/// 09 | 1525 | Request Category Code | O |  | ID 1/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PWK {
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
}

/// PAL - Pallet Information
///
/// To specify the pallet information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PAL {
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
    #[serde(rename = "17")]
    pub _17: Option<String>,
    #[serde(rename = "18")]
    pub _18: Option<String>,
    #[serde(rename = "19")]
    pub _19: Option<String>,
    #[serde(rename = "20")]
    pub _20: Option<String>,
    #[serde(rename = "21")]
    pub _21: Option<String>,
    #[serde(rename = "22")]
    pub _22: Option<String>,
    #[serde(rename = "23")]
    pub _23: Option<String>,
    #[serde(rename = "24")]
    pub _24: Option<String>,
    #[serde(rename = "25")]
    pub _25: Option<String>,
    #[serde(rename = "26")]
    pub _26: Option<String>,
    #[serde(rename = "27")]
    pub _27: Option<String>,
    #[serde(rename = "28")]
    pub _28: Option<String>,
    #[serde(rename = "29")]
    pub _29: Option<String>,
    #[serde(rename = "30")]
    pub _30: Option<String>,
    #[serde(rename = "31")]
    pub _31: Option<String>,
    #[serde(rename = "32")]
    pub _32: Option<String>,
    #[serde(rename = "33")]
    pub _33: Option<String>,
    #[serde(rename = "34")]
    pub _34: Option<String>,
    #[serde(rename = "35")]
    pub _35: Option<String>,
    #[serde(rename = "36")]
    pub _36: Option<String>,
    #[serde(rename = "37")]
    pub _37: Option<String>,
    #[serde(rename = "38")]
    pub _38: Option<String>,
    #[serde(rename = "39")]
    pub _39: Option<String>,
    #[serde(rename = "40")]
    pub _40: Option<String>,
}
