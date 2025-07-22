use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// C2 - Currency
///
/// To specify the currency used in the transaction
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Currency Code | 1 | M | ID | 3/3
/// 02 | 100 | Exchange Rate | 1 | O | R | 1/15
/// 03 | 100 | Currency Code | 1 | O | ID | 3/3
/// 04 | 100 | Exchange Rate | 1 | O | R | 1/15
/// 05 | 100 | Currency Code | 1 | O | ID | 3/3
/// 06 | 100 | Exchange Rate | 1 | O | R | 1/15
/// 07 | 100 | Currency Code | 1 | O | ID | 3/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct C2 {
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
}

/// C3 - Currency
///
/// To specify the currency used in the transaction
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Currency Code | 1 | M | ID | 3/3
/// 02 | 100 | Exchange Rate | 1 | O | R | 1/15
/// 03 | 100 | Currency Code | 1 | O | ID | 3/3
/// 04 | 100 | Exchange Rate | 1 | O | R | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct C3 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// C8 - Certificate of Compliance
///
/// To specify certificate of compliance information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Certificate Type Code | 1 | O | ID | 2/2
/// 02 | 100 | Certificate Number | 1 | O | AN | 1/30
/// 03 | 100 | Certificate Number | 1 | O | AN | 1/30
/// 04 | 100 | Certificate Number | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct C8 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// C8C - Certificate of Compliance
///
/// To specify certificate of compliance information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Certificate Type Code | 1 | M | ID | 2/2
/// 02 | 100 | Certificate Number | 1 | O | AN | 1/30
/// 03 | 100 | Certificate Number | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct C8C {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// CAD - Carrier Detail
///
/// To specify the carrier details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Transportation Method/Type Code | 1 | M | ID | 1/2
/// 02 | 100 | Equipment Initial | 1 | O | AN | 1/4
/// 03 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 04 | 100 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 05 | 100 | Routing | 1 | O | AN | 1/35
/// 06 | 100 | Shipment/Order Status Code | 1 | O | ID | 2/2
/// 07 | 100 | Reference Identification Qualifier | 1 | O | ID | 2/3
/// 08 | 100 | Reference Identification | 1 | O | AN | 1/30
/// 09 | 100 | Service Level Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CAD {
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

/// CD3 - Car Detail
///
/// To specify the car details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Equipment Initial | 1 | O | AN | 1/4
/// 02 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 03 | 100 | Equipment Type | 1 | O | ID | 4/4
/// 04 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 05 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 06 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 07 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 08 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 09 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 10 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 11 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 12 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 13 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 14 | 100 | Country Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CD3 {
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
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "14")]
    pub _14: Option<String>,
}

/// CM - Car Shipment Information
///
/// To specify car shipment information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Equipment Initial | 1 | O | AN | 1/4
/// 02 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 03 | 100 | Equipment Type | 1 | O | ID | 4/4
/// 04 | 373 | Date | 1 | O | DT | 8/8
/// 05 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 06 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 07 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 08 | 373 | Date | 1 | O | DT | 8/8
/// 09 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 10 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 11 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 12 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 13 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 14 | 26 | Country Code | 1 | O | ID | 2/3
/// 15 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 16 | 100 | Equipment Number | 1 | O | AN | 1/10
/// 17 | 100 | Equipment Number | 1 | O | AN | 1/10
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CM {
    #[serde(rename = "01")]
    pub _01: Option<String>,
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
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "14")]
    pub _14: Option<String>,
    #[serde(rename = "15")]
    pub _15: Option<String>,
    #[serde(rename = "16")]
    pub _16: Option<String>,
    #[serde(rename = "17")]
    pub _17: Option<String>,
}

/// CRC - Conditions Indicator
///
/// To specify conditions indicator
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Code Category | 1 | M | ID | 2/2
/// 02 | 100 | Yes/No Condition or Response Code | 1 | M | ID | 1/1
/// 03 | 100 | Condition Indicator | 1 | M | ID | 2/3
/// 04 | 100 | Condition Indicator | 1 | O | ID | 2/3
/// 05 | 100 | Condition Indicator | 1 | O | ID | 2/3
/// 06 | 100 | Condition Indicator | 1 | O | ID | 2/3
/// 07 | 100 | Condition Indicator | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CRC {
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
}

/// CTP - Pricing Information
///
/// To specify pricing information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Class of Trade Code | 1 | O | ID | 2/2
/// 02 | 100 | Price Identifier Code | 1 | O | ID | 3/3
/// 03 | 100 | Unit Price | 1 | O | R | 1/17
/// 04 | 100 | Quantity | 1 | O | R | 1/15
/// 05 | 100 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 06 | 100 | Price Multiplier Qualifier | 1 | O | ID | 3/3
/// 07 | 100 | Multiplier | 1 | O | R | 1/10
/// 08 | 100 | Monetary Amount | 1 | O | R | 1/18
/// 09 | 100 | Basis of Unit Price Code | 1 | O | ID | 2/2
/// 10 | 100 | Condition Value | 1 | O | R | 1/10
/// 11 | 100 | Multiple Price Quantity | 1 | O | N0 | 1/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CTP {
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
}

/// CTT - Transaction Totals
///
/// To transmit a hash total for a specific element in the transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 354 | Number of Line Items | 1 | M | N0 | 1/6
/// 02 | 347 | Hash Total | 1 | O | R | 1/10
/// 03 | 81 | Weight | 1 | O | R | 1/10
/// 04 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 05 | 183 | Volume | 1 | O | R | 1/8
/// 06 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 07 | 352 | Description | 1 | O | AN | 1/80
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CTT {
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

/// CUR - Currency
///
/// To specify the currency used in the transaction
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 98 | Entity Identifier Code | 1 | M | ID | 2/3
/// 02 | 100 | Currency Code | 1 | M | ID | 3/3
/// 03 | 280 | Exchange Rate | 1 | O | R | 1/15
/// 04 | 98 | Entity Identifier Code | 1 | O | ID | 2/3
/// 05 | 100 | Currency Code | 1 | O | ID | 3/3
/// 06 | 280 | Exchange Rate | 1 | O | R | 1/15
/// 07 | 98 | Entity Identifier Code | 1 | O | ID | 2/3
/// 08 | 100 | Currency Code | 1 | O | ID | 3/3
/// 09 | 280 | Exchange Rate | 1 | O | R | 1/15
/// 10 | 98 | Entity Identifier Code | 1 | O | ID | 2/3
/// 11 | 100 | Currency Code | 1 | O | ID | 3/3
/// 12 | 280 | Exchange Rate | 1 | O | R | 1/15
/// 13 | 98 | Entity Identifier Code | 1 | O | ID | 2/3
/// 14 | 100 | Currency Code | 1 | O | ID | 3/3
/// 15 | 280 | Exchange Rate | 1 | O | R | 1/15
/// 16 | 98 | Entity Identifier Code | 1 | O | ID | 2/3
/// 17 | 100 | Currency Code | 1 | O | ID | 3/3
/// 18 | 280 | Exchange Rate | 1 | O | R | 1/15
/// 19 | 98 | Entity Identifier Code | 1 | O | ID | 2/3
/// 20 | 100 | Currency Code | 1 | O | ID | 3/3
/// 21 | 280 | Exchange Rate | 1 | O | R | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CUR {
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
}

/// CLD - Carrier Load Details
///
/// To specify the number of units and the type of packaging for the shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Number of Units Shipped | 1 | M | N0 | 1/10
/// 02 | 100 | Packaging Form Code | 1 | M | ID | 3/3
/// 03 | 100 | Lading Quantity | 1 | O | N0 | 1/7
/// 04 | 100 | Weight | 1 | O | R | 1/10
/// 05 | 100 | Weight Unit Code | 1 | O | ID | 1/1
/// 06 | 100 | Volume | 1 | O | R | 1/8
/// 07 | 100 | Volume Unit Qualifier | 1 | O | ID | 1/1
/// 08 | 100 | Description | 1 | O | AN | 1/80
/// 09 | 100 | Lading Line Item Number | 1 | O | N0 | 1/6
/// 10 | 100 | Commodity Code | 1 | O | AN | 1/30
/// 11 | 100 | Commodity Code | 1 | O | AN | 1/30
/// 12 | 100 | Commodity Code | 1 | O | AN | 1/30
/// 13 | 100 | Commodity Code | 1 | O | AN | 1/30
/// 14 | 100 | Commodity Code | 1 | O | AN | 1/30
/// 15 | 100 | Commodity Code | 1 | O | AN | 1/30
/// 16 | 100 | Commodity Code | 1 | O | AN | 1/30
/// 17 | 100 | Commodity Code | 1 | O | AN | 1/30
/// 18 | 100 | Commodity Code | 1 | O | AN | 1/30
/// 19 | 100 | Commodity Code | 1 | O | AN | 1/30
/// 20 | 100 | Commodity Code | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CLD {
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
}
