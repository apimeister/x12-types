use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// SE - Transaction Set Trailer
///
/// To indicate the end of a transaction set and provide the count of transmitted segments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 96 | Number of Included Segments | 1 | M | N0 | 1/10
/// 02 | 329 | Transaction Set Control Number | 1 | M | AN | 4/9
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct SE {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

/// SN1 - Shipment Detail
///
/// To specify line-item detail relative to shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 350 | Assigned Identification | 1 | O | AN | 1/20
/// 02 | 382 | Number of Units Shipped | 1 | M | R | 1/10
/// 03 | 355 | Unit or Basis for Measurement Code | 1 | M | ID | 2/2
/// 04 | 646 | Quantity Shipped to Date | 1 | O | R | 1/10
/// 05 | 330 | Quantity Ordered | 1 | O | R | 1/10
/// 06 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 07 | 362 | Unacceptable | 1 | O | ID | 1/1
/// 08 | 639 | Basis of Unit Price Code | 1 | O | ID | 2/2
/// 09 | 499 | Relationship Code | 1 | O | ID | 1/1
/// 10 | 289 | Unit Price | 1 | O | R | 1/17
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct SN1 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
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

/// SR - Service, Promotion, Allowance, or Charge Information
///
/// To request or identify a service, promotion, allowance, or charge; to specify the amount or percentage for the service, promotion, allowance, or charge
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 559 | Agency Qualifier Code | 1 | M | ID | 2/2
/// 02 | 560 | Service, Promotion, Allowance, or Charge Code | 1 | M | AN | 1/4
/// 03 | 561 | Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/4
/// 04 | 562 | Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/4
/// 05 | 563 | Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/4
/// 06 | 564 | Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/4
/// 07 | 565 | Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/4
/// 08 | 566 | Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/4
/// 09 | 567 | Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/4
/// 10 | 568 | Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/4
/// 11 | 569 | Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/4
/// 12 | 352 | Description | 1 | O | AN | 1/80
/// 13 | 559 | Agency Qualifier Code | 1 | O | ID | 2/2
/// 14 | 560 | Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/4
/// 15 | 380 | Amount | 1 | O | R | 1/15
/// 16 | 380 | Amount | 1 | O | R | 1/15
/// 17 | 380 | Amount | 1 | O | R | 1/15
/// 18 | 380 | Amount | 1 | O | R | 1/15
/// 19 | 380 | Amount | 1 | O | R | 1/15
/// 20 | 380 | Amount | 1 | O | R | 1/15
/// 21 | 380 | Amount | 1 | O | R | 1/15
/// 22 | 380 | Amount | 1 | O | R | 1/15
/// 23 | 380 | Amount | 1 | O | R | 1/15
/// 24 | 380 | Amount | 1 | O | R | 1/15
/// 25 | 380 | Amount | 1 | O | R | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct SR {
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

/// SG - Shipment Status
///
/// To specify the status of a shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 374 | Date/Time Qualifier | 1 | M | ID | 3/3
/// 02 | 373 | Date | 1 | M | DT | 8/8
/// 03 | 337 | Time | 1 | O | TM | 4/8
/// 04 | 623 | Time Code | 1 | O | ID | 2/2
/// 05 | 1250 | Shipment Status Code | 1 | O | ID | 2/2
/// 06 | 1251 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct SG {
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
}

/// SDQ - Destination Quantity
///
/// To specify the quantity shipped to a specific destination
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 350 | Assigned Identification | 1 | M | AN | 1/20
/// 02 | 380 | Quantity | 1 | M | R | 1/15
/// 03 | 355 | Unit or Basis for Measurement Code | 1 | M | ID | 2/2
/// 04 | 1250 | Shipment Status Code | 1 | O | ID | 2/2
/// 05 | 1251 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 06 | 1252 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 07 | 1253 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 08 | 1254 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 09 | 1255 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 10 | 1256 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 11 | 1257 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 12 | 1258 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 13 | 1259 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 14 | 1260 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 15 | 1261 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 16 | 1262 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 17 | 1263 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 18 | 1264 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 19 | 1265 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 20 | 1266 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 21 | 1267 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 22 | 1268 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 23 | 1269 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 24 | 1270 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
/// 25 | 1271 | Shipment Status or Appointments Reason Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct SDQ {
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

/// ST - Transaction Set Header
///
/// To indicate the start of a transaction set and to assign a control number
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 143 | Transaction Set Identifier Code | 1 | M/Z | ID | 3/3
/// 02 | 329 | Transaction Set Control Number | 1 | M | AN | 4/9
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct ST {
    /// 143 - Transaction Set Identifier Code 3/3
    #[serde(rename = "01")]
    pub _01: String,
    /// 329 - Transaction Set Control Number 4/9
    #[serde(rename = "02")]
    pub _02: String,
}

/// S1 - Equipment Details
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
pub struct S1 {
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

/// S2 - Equipment Details
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
pub struct S2 {
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

/// S5 - Stop Off Details
///
/// To specify the stop off details
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
pub struct S5 {
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

/// S9 - Equipment Details
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
pub struct S9 {
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

/// SAC - Service, Promotion, Allowance, or Charge Information
///
/// To request or identify a service, promotion, allowance, or charge; to specify the monetary amount or percentage for the service, promotion, allowance, or charge
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 248 | Allowance or Charge Indicator | 1 | M | ID | 1/1
/// 02 | 1300 | Service, Promotion, Allowance, or Charge Code | 1 | M | ID | 4/4
/// 03 | 782 | Monetary Amount | 1 | O | R | 1/18
/// 04 | 559 | Agency Qualifier Code | 1 | O | ID | 2/2
/// 05 | 1301 | Agency Service, Promotion, Allowance, or Charge Code | 1 | O | AN | 1/10
/// 06 | 610 | Allowance or Charge Percent Qualifier | 1 | O | ID | 1/1
/// 07 | 954 | Percent | 1 | O | R | 1/6
/// 08 | 380 | Quantity | 1 | O | R | 1/15
/// 09 | 380 | Quantity | 1 | O | R | 1/15
/// 10 | 332 | Rate | 1 | O | R | 1/9
/// 11 | 118 | Amount | 1 | O | R | 1/15
/// 12 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 13 | 380 | Quantity | 1 | O | R | 1/15
/// 14 | 380 | Quantity | 1 | O | R | 1/15
/// 15 | 380 | Quantity | 1 | O | R | 1/15
/// 16 | 380 | Quantity | 1 | O | R | 1/15
/// 17 | 380 | Quantity | 1 | O | R | 1/15
/// 18 | 380 | Quantity | 1 | O | R | 1/15
/// 19 | 380 | Quantity | 1 | O | R | 1/15
/// 20 | 380 | Quantity | 1 | O | R | 1/15
/// 21 | 380 | Quantity | 1 | O | R | 1/15
/// 22 | 380 | Quantity | 1 | O | R | 1/15
/// 23 | 380 | Quantity | 1 | O | R | 1/15
/// 24 | 380 | Quantity | 1 | O | R | 1/15
/// 25 | 380 | Quantity | 1 | O | R | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct SAC {
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

/// SLN - Subline Item Detail
///
/// To specify product and quantity information for a subline item
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 350 | Assigned Identification | 1 | M | AN | 1/20
/// 02 | 350 | Assigned Identification | 1 | O | AN | 1/20
/// 03 | 235 | Product/Service ID Qualifier | 1 | M | ID | 2/2
/// 04 | 234 | Product/Service ID | 1 | M | AN | 1/48
/// 05 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 06 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 07 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 08 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 09 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 10 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 11 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 12 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 13 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 14 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 15 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 16 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 17 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 18 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 19 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 20 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 21 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 22 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 23 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 24 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 25 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 26 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 27 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 28 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 29 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 30 | 234 | Product/Service ID | 1 | O | AN | 1/48
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct SLN {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
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
}

/// SPO - Equipment Details
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
pub struct SPO {
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
