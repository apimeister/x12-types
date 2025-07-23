use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::{DisplaySegment, ParseSegment};

/// G3 - Compensation Information
///
/// To convey brokerage, freight forwarder compensation, and other compensation information related to shipments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 315 | Compensation Paid | O |  | R 2/5
/// 02 | 317 | Total Compensation Amount | M |  | N0 3/10
/// 03 | 93 | Name | O |  | AN 1/60
/// 04 | 201 | Business Transaction Status | O |  | ID 1/3
/// 05 | 782 | Monetary Amount | O |  | R 1/18
/// 06 | 73 | Compensation Qualifier | O |  | ID 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G3 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
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
}

/// G61 - Contact
///
/// To identify a person or office to whom communications should be directed
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 366 | Contact Function Code | 1 | M | ID | 2/2
/// 02 | 93 | Name | 1 | M | AN | 1/60
/// 03 | 365 | Communication Number Qualifier | 1 | X | ID | 2/2
/// 04 | 364 | Communication Number | 1 | X | AN | 1/80
/// 05 | 443 | Contact Inquiry Reference | 1 | O | AN | 1/20
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G61 {
    #[serde(rename = "01")]
    pub _01: String,
    /// 93 - Name
    ///
    /// Free-form name
    /// - TYPE=AN
    /// - MIN=1
    /// - MAX=60
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

/// G62 - Date/Time
///
/// To specify pertinent dates and times
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 432 | Date Qualifier | 1 | X | ID | 2/2
/// 02 | 373 | Date | 1 | X | DT | 8/8
/// 03 | 176 | Time Qualifier | 1 | X | ID | 1/2
/// 04 | 337 | Time | 1 | X | TM | 4/8
/// 05 | 623 | Time Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G62 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 623 - Time Code
    ///
    /// Code identifying the time. In accordance with International Standards Organization standard 8601, time can be specified by a + or - and an indication in hours in relation to Universal Time Coordinate (UTC) time; since + is a restricted character, + and - are substituted by P and M in the codes that follow
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

/// GA - Canadian Grain Information
///
/// To transmit the transportation and distribution requirements of grain at Canadian ports
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 1275 | Fumigated/Cleaned Indicator | 1 | O | ID | 1/1
/// 02 | 22 | Commodity Code | 1 | O/Z | AN | 1/30
/// 03 | 1576 | Inspected/Weighed Indicator Code | 1 | O | ID | 1/2
/// 04 | 128 | Reference Identification Qualifier | 1 | O | ID | 2/3
/// 05 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 06 | 642 | Week | 1 | O/Z | N0 | 4/4
/// 07 | 899 | Unload Terminal Elevator Code | 1 | O | ID | 3/4
/// 08 | 373 | Date | 1 | O/Z | DT | 8/8
/// 09 | 1470 | Number | 1 | O/Z | N0 | 1/9
/// 10 | 1276 | Machine Separable Indicator Code | 1 | O | ID | 2/2
/// 11 | 1277 | Canadian Wheat Board (CWB) Marketing Class Code | 1 | O | ID | 1/1
/// 12 | 1278 | Canadian Wheat Board (CWB) Marketing Class Type Code | 1 | O | ID | 1/1
/// 13 | 1073 | Yes/No Condition or Response Code | 1 | O | ID | 1/1
/// 14 | 310 | Location Identifier | 1 | X/Z | AN | 1/30
/// 15 | 156 | State or Province Code | 1 | X | ID | 2/2
/// 16 | 1004 | Percent Qualifier | 1 | X | ID | 1/2
/// 17 | 954 | Percent | 1 | X | R | 1/10
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct GA {
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
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
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
}

/// GE - Functional Group Trailer
///
/// To indicate the end of a functional group and to provide control information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 97 | Number of Transaction Sets Included | 1 | M | N0 | 1/6
/// 02 | 28 | Group Control Number | 1 | M/Z | N0 | 1/9
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
    Validate,
)]
pub struct GE {
    /// 97 - Number of Transaction Sets Included
    ///
    /// Total number of transaction sets included in the functional group or interchange (transmission) group terminated by the trailer containing this data element
    /// - TYPE=N0
    /// - MIN=1
    /// - MAX=6
    #[validate(length(min = 1, max = 6, message = "GE01 must be 1-6 characters long"))]
    #[serde(rename = "01")]
    pub _01: String,
    /// 28 - Group Control Number
    ///
    /// Assigned number originated and maintained by the sender
    /// - TYPE=N0
    /// - MIN=1
    /// - MAX=9
    #[validate(length(min = 1, max = 9, message = "GE02 must be 1-9 characters long"))]
    #[serde(rename = "02")]
    pub _02: String,
}

/// GR5 - Loading Details
///
/// To provide loading details for equipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 152 | Special Handling Code | 1 | M | ID | 2/3
/// 02 | 752 | Surface/Layer/Position Code | 1 | X | ID | 2/2
/// 03 | 739 | Measurement Value | 1 | X | R | 1/20
/// 04 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 05 | 641 | Status Reason Code | 1 | O | ID | 3/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct GR5 {
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
}

/// GS - Functional Group Header
///
/// To indicate the beginning of a functional group and to provide control information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 479 | Functional Identifier Code | 1 | M | ID | 2/2
/// 02 | 142 | Application Sender's Code | 1 | M | AN | 2/15
/// 03 | 124 | Application Receiver's Code | 1 | M | AN | 2/15
/// 04 | 373 | Date | 1 | M/Z | DT | 8/8
/// 05 | 337 | Time | 1 | M/Z | TM | 4/8
/// 06 | 28 | Group Control Number | 1 | M/Z | N0 | 1/9
/// 07 | 455 | Responsible Agency Code | 1 | M | ID | 1/2
/// 08 | 480 | Version / Release / Industry Identifier Code | 1 | M | AN | 1/12
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
    Validate,
)]
pub struct GS {
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
    pub _04: String,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
    #[serde(rename = "05")]
    pub _05: String,
    #[serde(rename = "06")]
    pub _06: String,
    #[serde(rename = "07")]
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: String,
}

/// GF - General Order Identification
///
/// To specify general order identification information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct GF {
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

/// G66 - Transportation Instructions
///
/// To specify transportation instructions relating to shipment
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G66 {
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

/// G69 - Line Item Detail - Description
///
/// To describe an item in industry terminology
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G69 {
    #[serde(rename = "01")]
    pub _01: String,
}

/// G72 - Allowance or Charge
///
/// To specify allowances, charges, or services
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G72 {
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
}
