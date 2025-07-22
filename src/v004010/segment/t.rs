use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// T1 - Transit Inbound Origin
///
/// To specify origin point and waybill references of movement to transit waybill point
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 554 | Assigned Number | 1 | M | N0 | 1/6
/// 02 | 186 | Waybill Number | 1 | O/Z | N0 | 1/6
/// 03 | 373 | Date | 1 | O/Z | DT | 8/8
/// 04 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 05 | 19 | City Name | 1 | X/Z | AN | 2/30
/// 06 | 156 | State or Province Code | 1 | X | ID | 2/2
/// 07 | 154 | Standard Point Location Code | 1 | O | ID | 6/9
/// 08 | 229 | Transit Registration Number | 1 | O | AN | 1/6
/// 09 | 461 | Transit Level Code | 1 | O | ID | 1/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct T1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
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

/// T2 - Transit Inbound Lading
///
/// To specify lading description, including weight and rate details applying to the associated T1 segment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 554 | Assigned Number | 1 | M | N0 | 1/6
/// 02 | 79 | Lading Description | 1 | O | AN | 1/50
/// 03 | 81 | Weight | 1 | O/Z | R | 1/10
/// 04 | 187 | Weight Qualifier | 1 | O | ID | 1/2
/// 05 | 60 | Freight Rate | 1 | X | R | 1/9
/// 06 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 07 | 60 | Freight Rate | 1 | X | R | 1/9
/// 08 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 09 | 19 | City Name | 1 | O/Z | AN | 2/30
/// 10 | 19 | City Name | 1 | O/Z | AN | 2/30
/// 11 | 462 | Through Surcharge Percent | 1 | O | N2 | 2/4
/// 12 | 463 | Paid-In Surcharge Percent | 1 | O | N2 | 2/4
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct T2 {
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
}

/// TC2 - Commodity
///
/// To specify commodity details
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
/// 08 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 09 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 10 | 127 | Reference Identification | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct TC2 {
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
}

/// TD1 - Carrier Details (Equipment)
///
/// To specify the transportation details relative to the equipment that is used for the shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 103 | Packaging Code | 1 | O | ID | 3/5
/// 02 | 80 | Lading Quantity | 1 | O | R | 1/7
/// 03 | 23 | Commodity Code Qualifier | 1 | O | ID | 1/1
/// 04 | 22 | Commodity Code | 1 | O | AN | 1/30
/// 05 | 79 | Lading Description | 1 | O | AN | 1/50
/// 06 | 187 | Weight Qualifier | 1 | O | ID | 1/2
/// 07 | 81 | Weight | 1 | O | R | 1/10
/// 08 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 09 | 183 | Volume | 1 | O | R | 1/8
/// 10 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct TD1 {
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

/// TD3 - Carrier Details (Routing Sequence/Transit Time)
///
/// To specify the carrier and sequence of routing and provide transit time information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 40 | Equipment Description Code | 1 | M | ID | 2/2
/// 02 | 206 | Equipment Initial | 1 | M | AN | 1/4
/// 03 | 207 | Equipment Number | 1 | M | AN | 1/10
/// 04 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 05 | 81 | Weight | 1 | O | R | 1/10
/// 06 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 07 | 183 | Volume | 1 | O | R | 1/8
/// 08 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 09 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 10 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 11 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 12 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 13 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 14 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 15 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 16 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 17 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 18 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 19 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 20 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct TD3 {
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
}

/// TD5 - Carrier Details (Routing Sequence/Transit Time)
///
/// To specify the carrier and sequence of routing and provide transit time information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 133 | Routing Sequence Code | 1 | M | ID | 1/2
/// 02 | 66 | Identification Code Qualifier | 1 | M | ID | 2/2
/// 03 | 67 | Identification Code | 1 | M | AN | 2/80
/// 04 | 91 | Transportation Method/Type Code | 1 | O | ID | 1/2
/// 05 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 06 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 07 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 08 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 09 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 10 | 387 | Equipment Number Check Digit | 1 | O | N0 | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct TD5 {
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

/// T3 - Equipment Details
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
/// 08 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 09 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 10 | 127 | Reference Identification | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct T3 {
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
}

/// T6 - Equipment Details
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
/// 08 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 09 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 10 | 127 | Reference Identification | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct T6 {
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
}

/// T8 - Equipment Details
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
/// 08 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 09 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 10 | 127 | Reference Identification | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct T8 {
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
}

/// TDS - Total Monetary Value Summary
///
/// To specify the total monetary values for a transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 610 | Total Amount | 1 | M | R | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct TDS {
    #[serde(rename = "01")]
    pub _01: String,
}

/// TXI - Tax Information
///
/// To specify tax information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 963 | Tax Type Code | 1 | M | ID | 2/2
/// 02 | 782 | Monetary Amount | 1 | O | R | 1/18
/// 03 | 954 | Percent | 1 | O | R | 1/6
/// 04 | 955 | Tax Jurisdiction Code Qualifier | 1 | O | ID | 2/2
/// 05 | 956 | Tax Jurisdiction Code | 1 | O | AN | 1/10
/// 06 | 441 | Tax Exempt Code | 1 | O | ID | 1/1
/// 07 | 955 | Tax Jurisdiction Code Qualifier | 1 | O | ID | 2/2
/// 08 | 956 | Tax Jurisdiction Code | 1 | O | AN | 1/10
/// 09 | 441 | Tax Exempt Code | 1 | O | ID | 1/1
/// 10 | 955 | Tax Jurisdiction Code Qualifier | 1 | O | ID | 2/2
/// 11 | 956 | Tax Jurisdiction Code | 1 | O | AN | 1/10
/// 12 | 441 | Tax Exempt Code | 1 | O | ID | 1/1
/// 13 | 955 | Tax Jurisdiction Code Qualifier | 1 | O | ID | 2/2
/// 14 | 956 | Tax Jurisdiction Code | 1 | O | AN | 1/10
/// 15 | 441 | Tax Exempt Code | 1 | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct TXI {
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
}

/// TD4 - Carrier Details (Special Handling, or Hazardous Materials, or Both)
///
/// To specify transportation special handling requirements, hazardous materials, or both
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct TD4 {
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

/// TSD - Tariff Service Description
///
/// To specify the tariff service description
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct TSD {
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
}
