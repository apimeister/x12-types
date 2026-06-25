use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// YNQ - Yes/No Question
///
/// To identify and answer yes and no questions, including conditional questions, used throughout the transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1021 | Assigned Identification | 1 | M | AN | 1/20
/// 02 | 1073 | Yes/No Condition or Response Code | 1 | M | ID | 1/1
/// 03 | 1250 | Shipment Status Code | 1 | O | ID | 2/2
/// 04 | 1251 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 05 | 1252 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 06 | 1253 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 07 | 1254 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 08 | 1255 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 09 | 1256 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 10 | 1257 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct YNQ {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E1021,
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E1073,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E1254>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<crate::v005010::element::E1257>,
}

/// Y2 - Container Details
///
/// To specify the container details for an ocean shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 95 | Number of Containers | 1 | M | N0 | 1/4
/// 02 | 78 | Container Type Request Code | 1 | O | ID | 1/1
/// 03 | 56 | Type of Service Code | 1 | O | ID | 2/2
/// 04 | 24 | Equipment Type | 1 | M | ID | 4/4
/// 05 | 91 | Transportation Method/Type Code | 1 | O | ID | 1/2
/// 06 | 177 | Intermodal Service Code | 1 | O | ID | 1/2
/// 07 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 08 | 464 | Container Terms Code | 1 | O | ID | 3/3
/// 09 | 465 | Container Terms Code Qualifier | 1 | O | ID | 1/1
/// 10 | 466 | Total Stop-offs | 1 | O | N0 | 1/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct Y2 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E95,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E78>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
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
    pub _09: Option<crate::v005010::element::E465>,
    #[serde(rename = "10")]
    pub _10: Option<crate::v005010::element::E466>,
}

/// Y6 - Authentication
///
/// To authenticate the booking or its amendment, and to identify the authorizing party
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 313 | Authority Identifier Code | 1 | O | ID | 2/2
/// 02 | 151 | Authority | 1 | M | AN | 1/20
/// 03 | 275 | Authorization Date | 1 | M | DT | 8/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct Y6 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E313>,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: crate::v005010::element::E275,
}

/// Y1 - Space Reservation Request
///
/// To request space reservation aboard an ocean vessel
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 135 | Sailing/Flight Date Estimated | 1 | O | DT | 8/8
/// 02 | 373 | Date | 1 | X | DT | 8/8
/// 03 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 04 | 91 | Transportation Method/Type Code | 1 | O | ID | 1/2
/// 05 | 98 | Entity Identifier Code | 1 | O | ID | 2/3
/// 06 | 19 | City Name | 1 | O | AN | 2/30
/// 07 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 08 | 375 | Tariff Service Code | 1 | O | ID | 2/2
/// 09 | 374 | Date/Time Qualifier | 1 | X | ID | 3/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct Y1 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E135>,
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
    pub _08: Option<crate::v005010::element::E375>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
}

/// Y3 - Space Confirmation
///
/// To confirm space reservation aboard an ocean vessel
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 13 | Booking Number | 1 | M | AN | 1/17
/// 02 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 03 | 373 | Date | 1 | O | DT | 8/8
/// 04 | 373 | Date | 1 | O | DT | 8/8
/// 05 | 154 | Standard Point Location Code | 1 | O | ID | 6/9
/// 06 | 112 | Pier Name | 1 | O | AN | 2/14
/// 07 | 373 | Date | 1 | O | DT | 8/8
/// 08 | 337 | Time | 1 | X | TM | 4/8
/// 09 | 91 | Transportation Method/Type Code | 1 | O | ID | 1/2
/// 10 | 375 | Tariff Service Code | 1 | O | ID | 2/2
/// 11 | 623 | Time Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct Y3 {
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
    pub _10: Option<crate::v005010::element::E375>,
    #[serde(rename = "11")]
    pub _11: Option<crate::v005010::element::E623>,
}

/// Y4 - Container Release
///
/// To transmit information relative to containers available for release
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 13 | Booking Number | 1 | O | AN | 1/17
/// 02 | 13 | Booking Number | 1 | O | AN | 1/17
/// 03 | 373 | Date | 1 | O | DT | 8/8
/// 04 | 154 | Standard Point Location Code | 1 | O | ID | 6/9
/// 05 | 95 | Number of Containers | 1 | O | N0 | 1/4
/// 06 | 24 | Equipment Type | 1 | O | ID | 4/4
/// 07 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 08 | 309 | Location Qualifier | 1 | X | ID | 1/2
/// 09 | 310 | Location Identifier | 1 | X | AN | 1/30
/// 10 | 56 | Type of Service Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct Y4 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E95>,
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

/// Y5 - Space Booking Cancellation
///
/// To cancel a previously requested space booking aboard an ocean vessel
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 13 | Booking Number | 1 | M | AN | 1/17
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct Y5 {
    #[serde(rename = "01")]
    pub _01: String,
}

/// Y7 - Cargo Booking Priority
///
/// To provide cargo booking priority and associated handling information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 467 | Priority | 1 | O | N0 | 1/1
/// 02 | 470 | Priority Code | 1 | X | N0 | 1/1
/// 03 | 471 | Priority Code Qualifier | 1 | X | AN | 1/1
/// 04 | 468 | Port Call File Number | 1 | O | N0 | 4/4
/// 05 | 373 | Date | 1 | O | DT | 8/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct Y7 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E467>,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E470>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E468>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
}
