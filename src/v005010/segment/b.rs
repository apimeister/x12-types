use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::{DisplaySegment, ParseSegment};

/// BLN - Balance Information
///
/// To provide specific balance information for financial account reporting
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 1270 | Code List Qualifier Code | M | ID | 1/3
/// 02 | 1271 | Industry Code | M | AN | 1/30
/// 03 | 782 | Monetary Amount | M | R | 1/18
/// 04 | 373 | Date | O | DT | 8/8
/// 05 | 337 | Time | X | TM | 4/8
/// 06 | 623 | Time Code | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BLN {
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
}

/// BAT - Batch Information
///
/// To identify a batch within a deposit
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 373 | Date | C | DT | 8/8
/// 02 | 337 | Time | O | TM | 4/8
/// 03 | 127 | Reference Identification | C | AN | 1/50
/// 04 | 894 | Batch Type Code | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BAT {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// B13 - Beginning Segment for Transportation Appointment Schedule
///
/// To transmit identifying information for the appointment schedule transaction set
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 127 | Reference Identification | M | AN | 1/50
/// 02 | 140 | Standard Carrier Alpha Code | O | ID | 2/4
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct B13 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// BCI - Basic Claim Information
///
/// To provide basic claim information
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 1271 | Industry Code | O | AN | 1/30
/// 02 | 1336 | Insurance Type Code | O | ID | 1/3
/// 03 | 127 | Reference Identification | O | AN | 1/50
/// 04 | 156 | State or Province Code | O | ID | 2/2
/// 05 | 1250 | Date Time Period Format Qualifier | C | ID | 2/3
/// 06 | 1251 | Date Time Period | C | AN | 1/35
/// 07 | 755 | Report Type Code | O | ID | 2/2
/// 08 | 100 | Currency Code | O | ID | 3/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BCI {
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
}

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
    /// 353 - Transaction Set Purpose Code
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E353,
    #[serde(rename = "02")]
    pub _02: String,
    /// 373 - Date
    #[serde(rename = "03")]
    pub _03: crate::v005010::element::E373,
    /// 337 - Time
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E337>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 640 - Transaction Type Code
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E640>,
    /// 306 - Action Code
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E306>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
}

/// BHT - Beginning of Hierarchical Transaction
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BHT {
    /// 1005 - Hierarchical Structure Code
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E1005,
    /// 353 - Transaction Set Purpose Code
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E353,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 373 - Date
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E373>,
    /// 337 - Time
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E337>,
    /// 640 - Transaction Type Code
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E640>,
}

/// BPA - Beginning Segment for Price Authorization Acknowledgment/Status
///
/// To indicate the beginning of a price authorization acknowledgment/status transaction set
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 353 | Transaction Set Purpose Code | M | ID | 2/2
/// 02 | 373 | Date | M | DT | 8/8
/// 03 | 128 | Reference Identification Qualifier | C | ID | 2/3
/// 04 | 127 | Reference Identification | C | AN | 1/50
/// 05 | 337 | Time | O | TM | 4/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BPA {
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

/// BPR - Beginning Segment for Payment Order/Remittance Advice
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BPR {
    /// 305 - Transaction Handling Code
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E305,
    #[serde(rename = "02")]
    pub _02: String,
    /// 478 - Credit/Debit Flag Code
    #[serde(rename = "03")]
    pub _03: crate::v005010::element::E478,
    /// 591 - Payment Method Code
    #[serde(rename = "04")]
    pub _04: crate::v005010::element::E591,
    /// 812 - Payment Format Code
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E812>,
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

/// BA1 - Export Shipment Identifying Information
///
/// To transmit identifying information for an export shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 645 | Related Company Indication Code | 1 | M | ID | 1/1
/// 02 | 306 | Action Code | 1 | M | ID | 1/2
/// 03 | 91 | Transportation Method/Type Code | 1 | M | ID | 1/2
/// 04 | 26 | Country Code | 1 | M | ID | 2/3
/// 05 | 127 | Reference Identification | 1 | M | AN | 1/50
/// 06 | 116 | Postal Code | 1 | O | ID | 3/15
/// 07 | 26 | Country Code | 1 | O | ID | 2/3
/// 08 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 09 | 151 | Authority | 1 | O | AN | 1/20
/// 10 | 140 | Standard Carrier Alpha Code | 1 | M | ID | 2/4
/// 11 | 310 | Location Identifier | 1 | O | AN | 1/30
/// 12 | 182 | Vessel Name | 1 | O | AN | 2/28
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BA1 {
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
    pub _10: String,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
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
    /// 353 - Transaction Set Purpose Code
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E353,
    /// 346 - Application Type Code
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E346>,
}

/// BOL - Beginning Segment for the Motor Carrier Bill of Lading
///
/// To transmit identifying numbers, dates, and other basic data relating to the transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 140 | Standard Carrier Alpha Code | 1 | M | ID | 2/4
/// 02 | 146 | Shipment Method of Payment | 1 | M | ID | 2/2
/// 03 | 145 | Shipment Identification Number | 1 | M | AN | 1/30
/// 04 | 373 | Date | 1 | M | DT | 8/8
/// 05 | 337 | Time | 1 | O | TM | 4/8
/// 06 | 127 | Reference Identification | 1 | O | AN | 1/50
/// 07 | 160 | Status Report Request Code | 1 | O | ID | 1/1
/// 08 | 226 | Section Seven Code | 1 | O | ID | 1/1
/// 09 | 501 | Customs Documentation Handling Code | 1 | O | ID | 2/2
/// 10 | 146 | Shipment Method of Payment | 1 | O | ID | 2/2
/// 11 | 100 | Currency Code | 1 | O | ID | 3/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BOL {
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

/// BLI - Basic Baseline Item Data
///
/// To provide basic item data for an item being returned or referenced
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 235 | Product/Service ID Qualifier | 1 | M | ID | 2/2
/// 02 | 234 | Product/Service ID | 1 | M | AN | 1/48
/// 03 | 380 | Quantity | 1 | X | R | 1/15
/// 04 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 05 | 236 | Price Identifier Code | 1 | X | ID | 3/3
/// 06 | 212 | Unit Price | 1 | X | R | 1/17
/// 07 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 08 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 09 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 10 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 11 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 12 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 13 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 14 | 1161 | Product Option Code | 1 | O | ID | 1/2
/// 15 | 1161 | Product Option Code | 1 | O | ID | 1/2
/// 16 | 1161 | Product Option Code | 1 | O | ID | 1/2
/// 17 | 1161 | Product Option Code | 1 | O | ID | 1/2
/// 18 | 594 | Frequency Code | 1 | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BLI {
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

/// BSI - Beginning Segment for Order Status Inquiry
///
/// To indicate the beginning of an order status inquiry transaction set
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 127 | Reference Identification | M | AN | 1/50
/// 02 | 373 | Date | M | DT | 8/8
/// 03 | 847 | Order/Item Code | M | ID | 1/2
/// 04 | 848 | Product/Date Code | O | ID | 1/2
/// 05 | 849 | Location Code | O | ID | 1/2
/// 06 | 337 | Time | O | TM | 4/8
/// 07 | 353 | Transaction Set Purpose Code | O | ID | 2/2
/// 08 | 640 | Transaction Type Code | O | ID | 2/2
/// 09 | 306 | Action Code | O | ID | 1/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BSI {
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
    /// 353 - Transaction Set Purpose Code
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E353,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 373 - Date
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E373>,
    /// 337 - Time
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E337>,
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
    #[serde(default, rename = "01", skip_serializing_if = "Option::is_none")]
    pub _01: Option<String>,
    #[serde(default, rename = "02", skip_serializing_if = "Option::is_none")]
    pub _02: Option<String>,
    #[serde(default, rename = "03", skip_serializing_if = "Option::is_none")]
    pub _03: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(default, rename = "04", skip_serializing_if = "Option::is_none")]
    pub _04: Option<String>,
    #[serde(default, rename = "05", skip_serializing_if = "Option::is_none")]
    pub _05: Option<String>,
    #[serde(default, rename = "06", skip_serializing_if = "Option::is_none")]
    pub _06: Option<String>,
    #[serde(default, rename = "07", skip_serializing_if = "Option::is_none")]
    pub _07: Option<String>,
    #[serde(default, rename = "08", skip_serializing_if = "Option::is_none")]
    pub _08: Option<String>,
    #[serde(default, rename = "09", skip_serializing_if = "Option::is_none")]
    pub _09: Option<String>,
    #[serde(default, rename = "10", skip_serializing_if = "Option::is_none")]
    pub _10: Option<String>,
    #[serde(default, rename = "11", skip_serializing_if = "Option::is_none")]
    pub _11: Option<String>,
    #[serde(default, rename = "12", skip_serializing_if = "Option::is_none")]
    pub _12: Option<String>,
    #[serde(default, rename = "13", skip_serializing_if = "Option::is_none")]
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

/// BPT - Beginning Segment for Product Transfer and Resale
///
/// To indicate the beginning of a product transfer and resale transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 353 | Transaction Set Purpose Code | 1 | M | ID | 2/2
/// 02 | 127 | Reference Identification | 1 | O | AN | 1/50
/// 03 | 373 | Date | 1 | M | DT | 8/8
/// 04 | 755 | Report Type Code | 1 | O | ID | 2/2
/// 05 | 648 | Price Multiplier Qualifier | 1 | X | ID | 3/3
/// 06 | 649 | Multiplier | 1 | X | R | 1/10
/// 07 | 306 | Action Code | 1 | O | ID | 1/2
/// 08 | 337 | Time | 1 | O | TM | 4/8
/// 09 | 127 | Reference Identification | 1 | O | AN | 1/50
/// 10 | 786 | Security Level Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BPT {
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

/// BLR - Transportation Carrier Identification
///
/// To identify the carrier responsible for transportation
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 140 | Standard Carrier Alpha Code | 1 | M | ID | 2/4
/// 02 | 373 | Date | 1 | X | DT | 8/8
/// 03 | 337 | Time | 1 | O | TM | 4/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct BLR {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
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
/// 01 | 140 | Standard Carrier Alpha Code | 1 | M | ID | 2/4
/// 02 | 145 | Shipment Identification Number | 1 | O | AN | 1/30
/// 03 | 373 | Date | 1 | O | DT | 8/8
/// 04 | 558 | Reservation Action Code | 1 | O | ID | 1/1
/// 05 | 1073 | Yes/No Condition or Response Code | 1 | O | ID | 1/1
/// 06 | 1658 | Shipment or Work Assignment Decline Reason Code | 1 | O | ID | 3/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct B1 {
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

/// BX - General Shipment Information
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

/// BL - Billing Information
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
}
