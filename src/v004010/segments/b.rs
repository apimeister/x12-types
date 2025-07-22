use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// B1 - Beginning Segment for Booking or Pick-up/Delivery
///
/// To transmit identifying numbers, dates, and other basic data relating to the transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 353 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 02 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 03 | 373 | Date | 1 | O | DT | 8/8
/// 04 | 284 | Reservation Action Code | 1 | M | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct B1 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: String,
}

/// B2 - Beginning Segment for Shipment Information
///
/// To transmit identifying numbers, dates, and other basic data relating to the shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 353 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 02 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 03 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 04 | 373 | Date | 1 | O | DT | 8/8
/// 05 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 06 | 284 | Shipment Identification Number | 1 | M | AN | 1/30
/// 07 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 08 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 09 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 10 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 11 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 12 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct B2 {
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
    pub _06: String,
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

/// B2A - Set Purpose
///
/// To identify the purpose for which the transaction set is being sent
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 353 | Transaction Set Purpose Code | 1 | M | ID | 2/2
/// 02 | 786 | Application Type | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct B2A {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// B3 - Beginning Segment for Car Shipment Information
///
/// To transmit identifying numbers, dates, and other basic data relating to the shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 353 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 02 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 03 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 04 | 373 | Date | 1 | M | DT | 8/8
/// 05 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 06 | 284 | Shipment Identification Number | 1 | M | AN | 1/30
/// 07 | 284 | Shipment Identification Number | 1 | M | AN | 1/30
/// 08 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 09 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 10 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 11 | 284 | Shipment Identification Number | 1 | M | AN | 1/30
/// 12 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 13 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 14 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct B3 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: String,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: String,
    #[serde(rename = "07")]
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: String,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
}

/// B4 - Beginning Segment for Inland Carriers
///
/// To transmit identifying numbers, dates, and other basic data relating to the shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 152 | Special Handling Code | 1 | O | ID | 2/3
/// 02 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 03 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 04 | 373 | Date | 1 | O | DT | 8/8
/// 05 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 06 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 07 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 08 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 09 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 10 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 11 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 12 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 13 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct B4 {
    /// 152 - Special Handling Code
    ///
    /// Code specifying special transportation handling instructions
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "01", skip_serializing_if = "Option::is_none")]
    pub _01: Option<String>,
    #[serde(rename = "02", skip_serializing_if = "Option::is_none")]
    pub _02: Option<String>,
    #[serde(rename = "03", skip_serializing_if = "Option::is_none")]
    pub _03: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "04", skip_serializing_if = "Option::is_none")]
    pub _04: Option<String>,
    #[serde(rename = "05", skip_serializing_if = "Option::is_none")]
    pub _05: Option<String>,
    #[serde(rename = "06", skip_serializing_if = "Option::is_none")]
    pub _06: Option<String>,
    #[serde(rename = "07", skip_serializing_if = "Option::is_none")]
    pub _07: Option<String>,
    #[serde(rename = "08", skip_serializing_if = "Option::is_none")]
    pub _08: Option<String>,
    #[serde(rename = "09", skip_serializing_if = "Option::is_none")]
    pub _09: Option<String>,
    #[serde(rename = "10", skip_serializing_if = "Option::is_none")]
    pub _10: Option<String>,
    #[serde(rename = "11", skip_serializing_if = "Option::is_none")]
    pub _11: Option<String>,
    #[serde(rename = "12", skip_serializing_if = "Option::is_none")]
    pub _12: Option<String>,
    #[serde(rename = "13", skip_serializing_if = "Option::is_none")]
    pub _13: Option<String>,
}

/// B10 - Beginning Segment for Transportation Carrier Shipment Status Message
///
/// To transmit identifying numbers, dates, and other basic data relating to the shipment status message
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 353 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 02 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 03 | 284 | Shipment Identification Number | 1 | M | AN | 1/30
/// 04 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 05 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 06 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 07 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct B10 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
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
}

/// BAL - Balance Detail
///
/// To specify the total balance amount
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Entity Identifier Code | 1 | M | ID | 2/3
/// 02 | 782 | Monetary Amount | 1 | M | R | 1/18
/// 03 | 478 | Credit/Debit Flag Code | 1 | M | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BAL {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
}

/// BIG - Beginning Segment for Invoice
///
/// To indicate the beginning of an invoice transaction set and transmit identifying numbers and dates
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 76 | Invoice Date | 1 | M | DT | 8/8
/// 02 | 324 | Invoice Number | 1 | M | AN | 1/22
/// 03 | 373 | Date | 1 | O | DT | 8/8
/// 04 | 324 | Purchase Order Number | 1 | O | AN | 1/22
/// 05 | 373 | Date | 1 | O | DT | 8/8
/// 06 | 324 | Release Number | 1 | O | AN | 1/30
/// 07 | 324 | Change Order Sequence Number | 1 | O | AN | 1/8
/// 08 | 324 | Transaction Set Purpose Code | 1 | O | ID | 2/2
/// 09 | 324 | Transaction Type Code | 1 | O | ID | 2/2
/// 10 | 324 | Action Code | 1 | O | ID | 1/2
/// 11 | 324 | Invoice Number | 1 | O | AN | 1/22
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BIG {
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

/// BIN - Binary Data Segment
///
/// To transmit binary data in a transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1000 | Binary Data | 1 | M | B | 1/99999
/// 02 | 1001 | Binary Data | 1 | O | B | 1/99999
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BIN {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

/// BL - Bill of Lading
///
/// To specify the bill of lading details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 02 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 03 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 04 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 05 | 19 | City Name | 1 | O | AN | 2/30
/// 06 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 07 | 26 | Country Code | 1 | O | ID | 2/3
/// 08 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 09 | 19 | City Name | 1 | O | AN | 2/30
/// 10 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 11 | 26 | Country Code | 1 | O | ID | 2/3
/// 12 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 13 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 14 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 15 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 16 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
/// 17 | 284 | Shipment Identification Number | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BL {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
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
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
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

/// BNX - Vessel Information
///
/// To specify vessel information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 284 | Vessel Code | 1 | O | AN | 1/30
/// 02 | 284 | Vessel Name | 1 | O | AN | 1/30
/// 03 | 284 | Vessel Code | 1 | O | AN | 1/30
/// 04 | 284 | Vessel Name | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BNX {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// BX - Vessel Schedule and Itinerary Information
///
/// To specify vessel schedule and itinerary information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 284 | Vessel Code | 1 | M | AN | 1/30
/// 02 | 284 | Vessel Name | 1 | M | AN | 1/30
/// 03 | 284 | Vessel Code | 1 | M | AN | 1/30
/// 04 | 284 | Vessel Name | 1 | O | AN | 1/30
/// 05 | 284 | Vessel Code | 1 | O | AN | 1/30
/// 06 | 284 | Vessel Name | 1 | O | AN | 1/30
/// 07 | 284 | Vessel Code | 1 | O | AN | 1/30
/// 08 | 284 | Vessel Name | 1 | O | AN | 1/30
/// 09 | 284 | Vessel Code | 1 | O | AN | 1/30
/// 10 | 284 | Vessel Name | 1 | O | AN | 1/30
/// 11 | 284 | Vessel Code | 1 | O | AN | 1/30
/// 12 | 284 | Vessel Name | 1 | O | AN | 1/30
/// 13 | 284 | Vessel Code | 1 | O | AN | 1/30
/// 14 | 284 | Vessel Name | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BX {
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
}

/// BSN - Beginning Segment for Ship Notice
///
/// To transmit identifying numbers, dates, and other basic data relating to the transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 353 | Shipment Identification | 1 | M | AN | 2/30
/// 02 | 373 | Date | 1 | M | DT | 8/8
/// 03 | 337 | Time | 1 | M | TM | 4/8
/// 04 | 1000 | Hierarchical Structure Code | 1 | O | ID | 4/4
/// 05 | 1001 | Transaction Set Purpose Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BSN {
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
