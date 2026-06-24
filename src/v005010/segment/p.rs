use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::{DisplaySegment, ParseSegment};

/// P4 - Port Information
///
/// To transmit identifying information for a port and the dates relating to an event
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 310 | Location Identifier | 1 | M | AN | 1/30
/// 02 | 373 | Date | 1 | M | DT | 8/8
/// 03 | 380 | Quantity | 1 | O | R | 1/15
/// 04 | 310 | Location Identifier | 1 | O | AN | 1/30
/// 05 | 337 | Time | 1 | O | TM | 4/8
/// 06 | 373 | Date | 1 | O | DT | 8/8
/// 07 | 337 | Time | 1 | O | TM | 4/8
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
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
}

/// PAT - Patient Information
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PAT {
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

/// PDI - Product/Item Description Information
///
/// To convey practice detail information
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 1068 | Gender Code | O | ID | 1/1
/// 02 | 740 | Range Minimum | O | R | 1/20
/// 03 | 741 | Range Maximum | O | R | 1/20
/// 04 | 1073 | Yes/No Condition or Response Code | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PDI {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// PUN - Beginning Segment for Motor Carrier Pickup Notification
///
/// To transmit identifying numbers, dates, and other basic data relating to the pickup notification
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 140 | Standard Carrier Alpha Code | M | ID | 2/4
/// 02 | 373 | Date | M | DT | 8/8
/// 03 | 337 | Time | O | TM | 4/8
/// 04 | 127 | Reference Identification | O | AN | 1/50
/// 05 | 337 | Time | O | TM | 4/8
/// 06 | 353 | Transaction Set Purpose Code | M | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PUN {
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
    pub _06: String,
}

/// PER - Administrative Communications Contact
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PER {
    /// 366 - Contact Function Code
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E366,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 365 - Communication Number Qualifier
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E365>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 365 - Communication Number Qualifier
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E365>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 365 - Communication Number Qualifier
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E365>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
}

/// PLA - Place or Location
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PLA {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

/// PLB - Provider Level Adjustment
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PLB {
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
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
}

/// PM - Electronic Funds Transfer Information
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PM {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: String,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

/// PRS - Part Release Status
///
/// To specify the status of the release of a part
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 682 | Part Release Status Code | 1 | M | ID | 1/2
/// 02 | 352 | Description | 1 | O | AN | 1/80
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PRS {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// PRV - Provider Information
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PRV {
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

/// PS1 - Purchase Service
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PS1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// PSA - Partner Share Accounting
///
/// To specify the partner identification and that partner's share of a project
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 66 | Identification Code Qualifier | 1 | M | ID | 1/2
/// 02 | 67 | Identification Code | 1 | M | AN | 2/80
/// 03 | 826 | Owners Share | 1 | M | R | 1/8
/// 04 | 782 | Monetary Amount | 1 | X | R | 1/18
/// 05 | 522 | Amount Qualifier Code | 1 | O | ID | 1/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PSA {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

/// PTD - Product Transfer and Resale Detail
///
/// To provide the basic data associated with a product transfer or resale detail
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 521 | Product Transfer Type Code | 1 | M | ID | 2/2
/// 02 | 648 | Price Multiplier Qualifier | 1 | X | ID | 3/3
/// 03 | 649 | Multiplier | 1 | X | R | 1/10
/// 04 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 05 | 127 | Reference Identification | 1 | X | AN | 1/50
/// 06 | 486 | Product Transfer Movement Type Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PTD {
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

/// PWK - Disability Information
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
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

/// PD - Pricing Data
///
/// To specify pricing data for a proposal or contract
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 344 | Unit of Time Period or Interval | 1 | M | ID | 2/2
/// 02 | 373 | Date | 1 | M | DT | 8/8
/// 03 | C001 | Composite Unit of Measure | 1 | M | | 1/1
/// 04 | 380 | Quantity | 1 | M | R | 1/15
/// 05 | 93 | Name | 1 | M | AN | 1/60
/// 06 | 352 | Description | 1 | O | AN | 1/80
/// 07 | 1196 | Breakdown Structure Detail Code | 1 | O | ID | 2/2
/// 08 | 127 | Reference Identification | 1 | O | AN | 1/50
/// 09 | 352 | Description | 1 | O | AN | 1/80
/// 10 | 1401 | Proposal Data Detail Identifier Code | 1 | O | ID | 1/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PD {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: String,
    #[serde(rename = "05")]
    pub _05: String,
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

/// PDD - Pricing Data Detail
///
/// To specify the detail of pricing data
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 350 | Assigned Identification | 1 | M | AN | 1/20
/// 02 | 380 | Quantity | 1 | X | R | 1/15
/// 03 | 782 | Monetary Amount | 1 | X | R | 1/18
/// 04 | 954 | Percentage as Decimal | 1 | X | R | 1/10
/// 05 | 1401 | Proposal Data Detail Identifier Code | 1 | O | ID | 1/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PDD {
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

/// PDR - Property Description - Real
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PDR {
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
    #[serde(rename = "41")]
    pub _41: Option<String>,
    #[serde(rename = "42")]
    pub _42: Option<String>,
    #[serde(rename = "43")]
    pub _43: Option<String>,
    #[serde(rename = "44")]
    pub _44: Option<String>,
    #[serde(rename = "45")]
    pub _45: Option<String>,
    #[serde(rename = "46")]
    pub _46: Option<String>,
    #[serde(rename = "47")]
    pub _47: Option<String>,
    #[serde(rename = "48")]
    pub _48: Option<String>,
    #[serde(rename = "49")]
    pub _49: Option<String>,
    #[serde(rename = "50")]
    pub _50: Option<String>,
}

/// PDP - Property Description - Personal
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PDP {
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
    #[serde(rename = "41")]
    pub _41: Option<String>,
    #[serde(rename = "42")]
    pub _42: Option<String>,
    #[serde(rename = "43")]
    pub _43: Option<String>,
    #[serde(rename = "44")]
    pub _44: Option<String>,
    #[serde(rename = "45")]
    pub _45: Option<String>,
    #[serde(rename = "46")]
    pub _46: Option<String>,
    #[serde(rename = "47")]
    pub _47: Option<String>,
    #[serde(rename = "48")]
    pub _48: Option<String>,
    #[serde(rename = "49")]
    pub _49: Option<String>,
    #[serde(rename = "50")]
    pub _50: Option<String>,
}

/// PID - Product/Item Description
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PID {
    /// 349 - Item Description Type
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E349>,
    /// 750 - Product/Process Characteristic Code
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E750>,
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

/// PCT - Percent Amounts
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PCT {
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
}

/// PKD - Packaging Description
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PKD {
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
}

/// PEN - Pension Information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PEN {
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
}

/// PYD - Payroll Deduction
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PYD {
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
}

/// PO1 - Baseline Item Data
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PO1 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: String,
    /// 355 - Unit or Basis for Measurement Code
    #[serde(rename = "03")]
    pub _03: crate::v005010::element::E355,
    #[serde(rename = "04")]
    pub _04: String,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 235 - Product/Service ID Qualifier
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E235>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    /// 235 - Product/Service ID Qualifier
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E235>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    /// 235 - Product/Service ID Qualifier
    #[serde(rename = "10")]
    pub _10: Option<crate::v005010::element::E235>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    /// 235 - Product/Service ID Qualifier
    #[serde(rename = "12")]
    pub _12: Option<crate::v005010::element::E235>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    /// 235 - Product/Service ID Qualifier
    #[serde(rename = "14")]
    pub _14: Option<crate::v005010::element::E235>,
    #[serde(rename = "15")]
    pub _15: Option<String>,
    /// 235 - Product/Service ID Qualifier
    #[serde(rename = "16")]
    pub _16: Option<crate::v005010::element::E235>,
    #[serde(rename = "17")]
    pub _17: Option<String>,
    /// 235 - Product/Service ID Qualifier
    #[serde(rename = "18")]
    pub _18: Option<crate::v005010::element::E235>,
    #[serde(rename = "19")]
    pub _19: Option<String>,
    /// 235 - Product/Service ID Qualifier
    #[serde(rename = "20")]
    pub _20: Option<crate::v005010::element::E235>,
    #[serde(rename = "21")]
    pub _21: Option<String>,
    /// 235 - Product/Service ID Qualifier
    #[serde(rename = "22")]
    pub _22: Option<crate::v005010::element::E235>,
    #[serde(rename = "23")]
    pub _23: Option<String>,
    /// 235 - Product/Service ID Qualifier
    #[serde(rename = "24")]
    pub _24: Option<crate::v005010::element::E235>,
    #[serde(rename = "25")]
    pub _25: Option<String>,
}

/// PO3 - Additional Item Detail
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PO3 {
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

/// PO4 - Item Physical Details
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PO4 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 355 - Unit or Basis for Measurement Code
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E355>,
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

/// PAM - Period Amount
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PAM {
    /// 673 - Quantity Qualifier
    #[serde(rename = "01")]
    pub _01: Option<String>,
    /// 380 - Quantity
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// C001 - Composite Unit of Measure
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 522 - Amount Qualifier Code
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 782 - Monetary Amount
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 344 - Unit of Time Period or Interval
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 374 - Date/Time Qualifier
    #[serde(rename = "07")]
    pub _07: Option<String>,
    /// 373 - Date
    #[serde(rename = "08")]
    pub _08: Option<String>,
    /// 337 - Time
    #[serde(rename = "09")]
    pub _09: Option<String>,
    /// 374 - Date/Time Qualifier
    #[serde(rename = "10")]
    pub _10: Option<String>,
    /// 373 - Date
    #[serde(rename = "11")]
    pub _11: Option<String>,
    /// 337 - Time
    #[serde(rename = "12")]
    pub _12: Option<String>,
    /// 1000 - Percent Qualifier
    #[serde(rename = "13")]
    pub _13: Option<String>,
    /// 954 - Percentage as Decimal
    #[serde(rename = "14")]
    pub _14: Option<String>,
    /// 1073 - Yes/No Condition or Response Code
    #[serde(rename = "15")]
    pub _15: Option<String>,
}

/// PKL - Pack Location
///
/// To describe the physical packaging and stacking dimensions of an item
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 235 | Product/Service ID Qualifier | M | ID | 2/2
/// 02 | 234 | Product/Service ID | M | AN | 1/48
/// 03 | 355 | Unit or Basis for Measurement Code | M | ID | 2/2
/// 04 | 380 | Quantity | M | R | 1/15
/// 05 | 65 | Height | C | R | 1/8
/// 06 | 189 | Width | C | R | 1/8
/// 07 | 677 | Item Depth | C | R | 1/6
/// 08 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
/// 09 | 384 | Gross Weight per Pack | C | R | 1/9
/// 10 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
/// 11 | 385 | Gross Volume per Pack | C | R | 1/9
/// 12 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
/// 13 | 1073 | Yes/No Condition or Response Code | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PKL {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: String,
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
}

/// PKG - Marking, Packaging, Loading
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    Validate,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
)]
pub struct PKG {
    /// 349 - Item Description Type
    #[serde(rename = "01")]
    pub _01: Option<String>,
    /// 753 - Packaging Characteristic Code
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 559 - Agency Qualifier Code
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 754 - Packaging Description Code
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 352 - Description
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 400 - Unit Load Option Code
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 819 - Language Code
    #[serde(rename = "07")]
    pub _07: Option<String>,
}

/// PAD - Product Adjustment Detail
///
/// To specify the product adjustment detail for a price authorization
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 350 | Assigned Identification | C | AN | 1/20
/// 02 | 521 | Product Transfer Type Code | C | ID | 2/2
/// 03 | 670 | Change or Response Type Code | O | ID | 2/2
/// 04 | 648 | Price Multiplier Qualifier | C | ID | 3/3
/// 05 | 649 | Multiplier | C | R | 1/10
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct PAD {
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

/// POC - Line Item Change
///
/// To specify changes to a line item
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct POC {
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
}

/// POD - Proof of Delivery
///
/// To specify proof of delivery information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct POD {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// PI - Price Authority Information
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

/// PS - Protective Service Instructions
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
