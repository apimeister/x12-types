use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// Q2 - Status Details (Ocean)
///
/// To specify the status of a shipment
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
/// 11 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 12 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 13 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 14 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 15 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 16 | 127 | Reference Identification | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct Q2 {
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

/// Q5 - Status Details (Ocean)
///
/// To specify the status of a shipment
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
pub struct Q5 {
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

/// Q7 - Status Details (Ocean)
///
/// To specify the status of a shipment
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
pub struct Q7 {
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

/// QTY - Quantity
///
/// To specify quantity information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 673 | Quantity Qualifier | 1 | M | ID | 2/2
/// 02 | 380 | Quantity | 1 | M | R | 1/15
/// 03 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 04 | 374 | Date/Time Qualifier | 1 | O | ID | 3/3
/// 05 | 373 | Date | 1 | O | DT | 8/8
/// 06 | 337 | Time | 1 | O | TM | 4/8
/// 07 | 374 | Date/Time Qualifier | 1 | O | ID | 3/3
/// 08 | 373 | Date | 1 | O | DT | 8/8
/// 09 | 337 | Time | 1 | O | TM | 4/8
/// 10 | 374 | Date/Time Qualifier | 1 | O | ID | 3/3
/// 11 | 373 | Date | 1 | O | DT | 8/8
/// 12 | 337 | Time | 1 | O | TM | 4/8
/// 13 | 374 | Date/Time Qualifier | 1 | O | ID | 3/3
/// 14 | 373 | Date | 1 | O | DT | 8/8
/// 15 | 337 | Time | 1 | O | TM | 4/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct QTY {
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
}
