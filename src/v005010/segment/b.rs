use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::{DisplaySegment, ParseSegment};

/// BEN - Financial Contribution
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
pub struct BEN {
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

/// BGN - Beginning Segment
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
pub struct BGN {
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
}

/// BHT - Beginning of Hierarchical Transaction
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BHT {
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

/// BPR - Beginning Segment for Payment Order/Remittance Advice
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BPR {
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
}

/// BEG - Beginning Segment for Purchase Order
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
pub struct BEG {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: String,
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

/// BAK - Beginning Segment for Purchase Order Acknowledgment
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
pub struct BAK {
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

/// BCH - Beginning Segment for Purchase Order Change
///
/// To indicate the beginning of the Purchase Order Change Transaction Set and transmit identifying numbers and dates
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BCH {
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
}

/// BIA - Beginning Segment for Inventory Inquiry/Advice
///
/// To indicate the beginning of an inventory transaction set
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BIA {
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

/// BFR - Beginning Segment for Planning Schedule
///
/// To carry beginning segment for planning schedule data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BFR {
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

/// BSS - Beginning Segment for Shipping Schedule
///
/// To carry beginning segment for shipping schedule data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BSS {
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
}

/// BRA - Beginning Segment for Receiving Advice
///
/// To carry beginning segment for receiving advice data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BRA {
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

/// BCT - Price/Sales Catalog Header
///
/// To carry price/sales catalog header data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BCT {
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

/// BCD - Beginning Segment for Credit/Debit Adjustment
///
/// To carry beginning segment for credit/debit adjustment data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BCD {
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

/// BQT - Beginning Segment for Request for Quotation
///
/// To carry beginning segment for request for quotation data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BQT {
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

/// BQR - Beginning Segment for Response to Request for Quotation
///
/// To carry beginning segment for response to request for quotation data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BQR {
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
}

/// BCA - Beginning Segment for Purchase Order Change Acknowledgment
///
/// To carry beginning segment for purchase order change acknowledgment data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BCA {
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
}

/// BSR - Beginning Segment for Order Status Report
///
/// To carry beginning segment for order status report data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BSR {
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

/// BMG - Beginning Segment for Text Message
///
/// To carry beginning segment for text message data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BMG {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

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
