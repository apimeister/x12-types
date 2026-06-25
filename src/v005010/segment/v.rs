use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// VEH - Vehicle Information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct VEH {
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

/// VC - Motor Vehicle Control
///
/// To define motor vehicle control detail for the transport of vehicles
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 539 | Vehicle Identification Number | M | AN | 1/30
/// 02 | 836 | Vehicle Deck Position Code | O | ID | 2/2
/// 03 | 837 | Vehicle Type Code | O | ID | 1/1
/// 04 | 838 | Dealer Code | O | AN | 2/9
/// 05 | 1 | Route Code | O | AN | 1/13
/// 06 | 839 | Bay Location | O | AN | 1/6
/// 07 | 833 | Automotive Manufacturers Code | O | ID | 2/2
/// 08 | 308 | Damage Exception Indicator | O | ID | 1/1
/// 09 | 835 | Supplemental Inspection Code | O | ID | 1/1
/// 10 | 583 | Factory Car Order Number | O | AN | 6/10
/// 11 | 877 | Vessel Stowage Location | O | AN | 1/12
/// 12 | 1543 | Equipment Orientation Code | O | ID | 1/1
/// 13 | 310 | Location Identifier | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct VC {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E836>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E837>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E308>,
    #[serde(rename = "09")]
    pub _09: Option<crate::v005010::element::E835>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<crate::v005010::element::E1543>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
}

/// VID - Conveyance Identification
///
/// To identify a conveyance and its physical characteristics
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 40 | Equipment Description Code | 1 | M | ID | 2/2
/// 02 | 206 | Equipment Initial | 1 | O | AN | 1/4
/// 03 | 207 | Equipment Number | 1 | M | AN | 1/15
/// 04 | 225 | Seal Number | 1 | O | AN | 2/15
/// 05 | 225 | Seal Number | 1 | O | AN | 2/15
/// 06 | 567 | Equipment Length | 1 | O | N0 | 4/5
/// 07 | 65 | Height | 1 | O | R | 1/8
/// 08 | 189 | Width | 1 | O | R | 1/8
/// 09 | 24 | Equipment Type | 1 | O | ID | 4/4
/// 10 | 322 | Load/Empty Status Code | 1 | O | ID | 1/1
/// 11 | 56 | Type of Service Code | 1 | O | ID | 2/2
/// 12 | 310 | Location Identifier | 1 | O | AN | 1/30
/// 13 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct VID {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E567>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E65>,
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E189>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<crate::v005010::element::E322>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
}

/// V1 - Vessel Identification
///
/// To specify the vessel information
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
pub struct V1 {
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

/// V3 - Vessel Schedule
///
/// To provide the ports of call and the estimated arrival and departure dates
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 318 | Current Port of Loading | 1 | M | AN | 2/24
/// 02 | 373 | Date | 1 | M | DT | 8/8
/// 03 | 316 | Next Port of Discharge | 1 | O | AN | 2/24
/// 04 | 373 | Date | 1 | O | DT | 8/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct V3 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// V4 - Vessel Information
///
/// To specify the vessel information
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
pub struct V4 {
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

/// V9 - Event Detail
///
/// To specify the event information
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
pub struct V9 {
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
