use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::{DisplaySegment, ParseSegment};

/// GE - Functional Group Trailer
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
pub struct GE {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

/// GRI - Statistical Government Information
///
/// To convey statistical or government-required reporting data
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1260 | Reported Data ID Code | 1 | M | AN | 1/6
/// 02 | 1261 | Reported Data Response | 1 | X | AN | 1/10
/// 03 | 673 | Quantity Qualifier | 1 | O | ID | 2/2
/// 04 | 380 | Quantity | 1 | X | R | 1/15
/// 05 | 522 | Amount Qualifier Code | 1 | O | ID | 1/3
/// 06 | 782 | Monetary Amount | 1 | X | R | 1/18
/// 07 | 1004 | Percent Qualifier | 1 | O | ID | 1/2
/// 08 | 488 | Percent, Integer Format | 1 | X | N0 | 1/3
/// 09 | 374 | Date/Time Qualifier | 1 | O | ID | 3/3
/// 10 | 373 | Date | 1 | X | DT | 8/8
/// 11 | 352 | Description | 1 | O | AN | 1/80
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct GRI {
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
    pub _06: Option<crate::v005010::element::E782>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E1004>,
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E488>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
}

/// GS - FunctionalGroup Header
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
pub struct GS {
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
    pub _06: String,
    #[serde(rename = "07")]
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: String,
}

/// G53 - Maintenance Type
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G53 {
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

/// G61 - Contact
///
/// To identify a person or office to whom communications should be directed
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 366 | Contact Function Code | 1 | M | ID | 2/2
/// 02 | 93 | Name | 1 | M | AN | 1/60
/// 03 | 365 | Communication Number Qualifier | 1 | X | ID | 2/2
/// 04 | 364 | Communication Number | 1 | X | AN | 1/80
/// 05 | 443 | Contact Inquiry Reference | 1 | O | AN | 1/20
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G61 {
    #[serde(rename = "01")]
    pub _01: String,
    /// 93 - Name
    ///
    /// Free-form name
    /// - TYPE=AN
    /// - MIN=1
    /// - MAX=60
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

/// G62 - Date/Time
///
/// To specify pertinent dates and times
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 432 | Date Qualifier | 1 | X | ID | 2/2
/// 02 | 373 | Date | 1 | X | DT | 8/8
/// 03 | 176 | Time Qualifier | 1 | X | ID | 1/2
/// 04 | 337 | Time | 1 | X | TM | 4/8
/// 05 | 623 | Time Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G62 {
    /// 432 - Date Qualifier
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E432>,
    /// 373 - Date
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E373>,
    /// 176 - Time Qualifier
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E176>,
    /// 337 - Time
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E337>,
    /// 623 - Time Code
    ///
    /// Code identifying the time. In accordance with International Standards Organization standard 8601, time can be specified by a + or - and an indication in hours in relation to Universal Time Coordinate (UTC) time; since + is a restricted character, + and - are substituted by P and M in the codes that follow
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E623>,
}

/// G66 - Transportation Instructions
///
/// To specify transportation instructions relating to shipment
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G66 {
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

/// G69 - Line Item Detail - Description
///
/// To describe an item in industry terminology
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G69 {
    #[serde(rename = "01")]
    pub _01: String,
}

/// G95 - Performance Requirements
///
/// To specify the performance requirements associated with a promotion
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 293 | Promotion Condition Qualifier | 1 | O | ID | 2/2
/// 02 | 422 | Promotion Condition Code | 1 | M | ID | 2/2
/// 03 | 554 | Assigned Number | 1 | O | N0 | 1/6
/// 04 | 380 | Quantity | 1 | X | R | 1/15
/// 05 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 06 | 352 | Description | 1 | O | AN | 1/80
/// 07 | 1470 | Number | 1 | O | N0 | 1/9
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G95 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E293>,
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E422,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E554>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E1470>,
}

/// G68 - Line Item Detail - Product
///
/// To specify item detail for a grocery product line item
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 380 | Quantity | 1 | M | R | 1/15
/// 02 | 355 | Unit or Basis for Measurement Code | 1 | M | ID | 2/2
/// 03 | 237 | Item List Cost | 1 | O | R | 1/9
/// 04 | 438 | U.P.C. Case Code | 1 | X | AN | 12/12
/// 05 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 06 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 07 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 08 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 09 | 417 | Price Bracket Identifier | 1 | O | AN | 1/3
/// 10 | 258 | Quantity Cost | 1 | X | N | 1/9
/// 11 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 12 | 439 | Price List Number | 1 | O | AN | 1/16
/// 13 | 440 | Price List Issue Number | 1 | O | AN | 1/16
/// 14 | 857 | Pre-Price Quantity Designator | 1 | O | N | 1/9
/// 15 | 858 | Retail Pre-Price | 1 | O | R | 1/9
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G68 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E237>,
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
    pub _10: Option<crate::v005010::element::E258>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<crate::v005010::element::E857>,
    #[serde(rename = "15")]
    pub _15: Option<crate::v005010::element::E858>,
}

/// G70 - Line Item Detail - Miscellaneous
///
/// To specify miscellaneous line-item detail for a grocery product
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 356 | Pack | 1 | O | N0 | 1/6
/// 02 | 357 | Size | 1 | X | R | 1/8
/// 03 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 04 | 444 | Purchase Order Instruction Code | 1 | O | ID | 2/2
/// 05 | 381 | Price Reason Code | 1 | O | ID | 1/1
/// 06 | 445 | Terms Exception Code | 1 | O | ID | 2/2
/// 07 | 441 | Tax Exempt Code | 1 | O | ID | 1/1
/// 08 | 397 | Color | 1 | O | AN | 1/10
/// 09 | 416 | Pallet Block and Tiers | 1 | O | N0 | 6/6
/// 10 | 810 | Inner Pack | 1 | O | N0 | 1/6
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G70 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E356>,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E357>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E444>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E381>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E445>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E441>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<crate::v005010::element::E416>,
    #[serde(rename = "10")]
    pub _10: Option<crate::v005010::element::E810>,
}

/// G73 - Allowance or Charge Description
///
/// To provide a free-form description of an allowance or charge
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 369 | Free-form Description | 1 | M | AN | 1/45
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G73 {
    #[serde(rename = "01")]
    pub _01: String,
}

/// G76 - Total Purchase Order
///
/// To convey the totals for a grocery products purchase order
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 380 | Quantity | 1 | M | R | 1/15
/// 02 | 355 | Unit or Basis for Measurement Code | 1 | M | ID | 2/2
/// 03 | 81 | Weight | 1 | X | R | 1/10
/// 04 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 05 | 183 | Volume | 1 | X | R | 1/8
/// 06 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 07 | 398 | Order Sizing Factor | 1 | O | R | 1/10
/// 08 | 610 | Amount | 1 | O | N2 | 1/15
/// 09 | 417 | Price Bracket Identifier | 1 | O | AN | 1/3
/// 10 | 107 | Payment Method Type Code | 1 | O | ID | 1/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G76 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E81>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E183>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E398>,
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E610>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<crate::v005010::element::E107>,
}

/// G72 - Allowance or Charge
///
/// To specify allowances, charges, or services
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G72 {
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

/// G1 - Shipment Type Information
///
/// To indicate the basic shipment characteristics regarding the type of move
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 311 | Shipment Type Code | 1 | M | ID | 1/2
/// 02 | 312 | Special Indicator Code | 1 | O | ID | 1/1
/// 03 | 312 | Special Indicator Code | 1 | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G1 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E311,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E312>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E312>,
}

/// G2 - Beyond Routing
///
/// To specify routing beyond the destination
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 312 | Special Indicator Code | 1 | M | ID | 1/1
/// 02 | 352 | Description | 1 | O | AN | 1/80
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G2 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E312,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// G17 - Item Detail (Invoice)
///
/// To specify the invoice detail for a grocery product line item
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 358 | Quantity Invoiced | 1 | M | R | 1/15
/// 02 | 355 | Unit or Basis for Measurement Code | 1 | M | ID | 2/2
/// 03 | 237 | Item List Cost | 1 | X | R | 1/9
/// 04 | 438 | U.P.C. Case Code | 1 | X | AN | 12/12
/// 05 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 06 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 07 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 08 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 09 | 417 | Price Bracket Identifier | 1 | O | AN | 1/3
/// 10 | 382 | Number of Units Shipped | 1 | X | R | 1/10
/// 11 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 12 | 439 | Price List Number | 1 | O | AN | 1/16
/// 13 | 440 | Price List Issue Number | 1 | O | AN | 1/16
/// 14 | 782 | Monetary Amount | 1 | X | R | 1/18
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G17 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E358,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E237>,
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
    pub _10: Option<crate::v005010::element::E382>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<crate::v005010::element::E782>,
}

/// G19 - Item Detail (Quantity/Unit of Measure/Price Differences)
///
/// To specify quantity, unit-of-measure, and price difference detail
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 382 | Number of Units Shipped | 1 | X | R | 1/10
/// 02 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 03 | 383 | Quantity Difference | 1 | X | R | 1/9
/// 04 | 368 | Shipment/Order Status Code | 1 | X | ID | 2/2
/// 05 | 381 | Price Reason Code | 1 | O | ID | 1/1
/// 06 | 445 | Terms Exception Code | 1 | O | ID | 2/2
/// 07 | 438 | U.P.C. Case Code | 1 | O | AN | 12/12
/// 08 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 09 | 234 | Product/Service ID | 1 | X | AN | 1/48
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G19 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E382>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E383>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E368>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E381>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E445>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
}

/// G20 - Item Packing Detail
///
/// To specify packing detail for a grocery product line item
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 356 | Pack | 1 | O | N0 | 1/6
/// 02 | 357 | Size | 1 | X | R | 1/8
/// 03 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 04 | 81 | Weight | 1 | X | R | 1/10
/// 05 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 06 | 183 | Volume | 1 | X | R | 1/8
/// 07 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 08 | 397 | Color | 1 | O | AN | 1/10
/// 09 | 810 | Inner Pack | 1 | O | N0 | 1/6
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G20 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E356>,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E357>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E81>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E183>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<crate::v005010::element::E810>,
}

/// G25 - F.O.B. Information
///
/// To specify the F.O.B. terms for a grocery transaction
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 146 | Shipment Method of Payment | 1 | M | ID | 2/2
/// 02 | 433 | F.O.B. Point Code | 1 | M | ID | 2/2
/// 03 | 434 | F.O.B. Point | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G25 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E433,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// GY - Geography
///
/// To specify a geographic area
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 262 | Geography Qualifier Code | M | ID | 1/1
/// 02 | 699 | Commodity/Geographic Logical Connector Code | O | ID | 1/1
/// 03 | 309 | Location Qualifier | C | ID | 1/2
/// 04 | 156 | State or Province Code | O | ID | 2/2
/// 05 | 310 | Location Identifier | C | AN | 1/30
/// 06 | 310 | Location Identifier | O | AN | 1/30
/// 07 | 140 | Standard Carrier Alpha Code | O | ID | 2/4
/// 08 | 259 | Change Type Code | O | ID | 1/1
/// 09 | 140 | Standard Carrier Alpha Code | C | ID | 2/4
/// 10 | 697 | Docket Control Number | C | AN | 1/7
/// 11 | 690 | Docket Identification | C | AN | 1/11
/// 12 | 260 | Group Title | O | AN | 2/30
/// 13 | 156 | State or Province Code | C | ID | 2/2
/// 14 | 19 | City Name | C | AN | 2/30
/// 15 | 1073 | Yes/No Condition or Response Code | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct GY {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E262,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E699>,
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
    pub _08: Option<crate::v005010::element::E259>,
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
    pub _15: Option<crate::v005010::element::E1073>,
}

/// G22 - Pre-Pricing Information
///
/// To provide pre-pricing information
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 288 | Pre-priced Option Code | M | ID | 1/1
/// 02 | 420 | Price New, Suggested Retail | O | N2 | 2/7
/// 03 | 289 | Multiple Price Quantity | O | N0 | 1/2
/// 04 | 3 | Free-form Message | O | AN | 1/60
/// 05 | 373 | Date | O | DT | 8/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G22 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E288,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E420>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E289>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

/// G24 - Promotion/Deal Number
///
/// To reference a promotion number related to the price change
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 341 | Allowance or Charge Number | M | AN | 1/16
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G24 {
    #[serde(rename = "01")]
    pub _01: String,
}

/// G46 - Promotion/Price Reduction Allowance or Charge
///
/// To specify the allowance or charge associated with a promotion or price reduction
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 340 | Allowance or Charge Code | M | ID | 1/3
/// 02 | 331 | Allowance or Charge Method of Handling Code | M | ID | 2/2
/// 03 | 359 | Allowance or Charge Rate | C | R | 1/15
/// 04 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
/// 05 | 610 | Amount | C | N2 | 1/15
/// 06 | 378 | Allowance/Charge Percent Qualifier | C | ID | 1/1
/// 07 | 332 | Percent, Decimal Format | C | R | 1/6
/// 08 | 769 | Exception Number | O | AN | 1/16
/// 09 | 770 | Option Number | O | AN | 1/20
/// 10 | 352 | Description | O | AN | 1/80
/// 11 | 236 | Price Identifier Code | O | ID | 3/3
/// 12 | 1470 | Number | O | N0 | 1/9
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G46 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E340,
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E331,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E359>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E610>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E378>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E332>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<crate::v005010::element::E236>,
    #[serde(rename = "12")]
    pub _12: Option<crate::v005010::element::E1470>,
}

/// G23 - Terms of Sale
///
/// To specify the terms of sale for a grocery transaction
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 336 | Terms Type Code | 1 | M | ID | 2/2
/// 02 | 333 | Terms Basis Date Code | 1 | M | ID | 1/2
/// 03 | 282 | Terms Start Date | 1 | O | DT | 8/8
/// 04 | 283 | Terms Due Date Qualifier | 1 | O | ID | 2/2
/// 05 | 338 | Terms Discount Percent | 1 | O | R | 1/6
/// 06 | 370 | Terms Discount Due Date | 1 | O | DT | 8/8
/// 07 | 351 | Terms Discount Days Due | 1 | O | N0 | 1/3
/// 08 | 446 | Terms Net Due Date | 1 | X | DT | 8/8
/// 09 | 386 | Terms Net Days | 1 | X | N0 | 1/3
/// 10 | 362 | Terms Discount Amount | 1 | O | N2 | 1/10
/// 11 | 391 | Discounted Amount Due | 1 | O | N2 | 1/10
/// 12 | 390 | Amount Subject to Terms Discount | 1 | O | N2 | 1/10
/// 13 | 343 | Installment Total Invoice Amount Due | 1 | O | N2 | 1/10
/// 14 | 342 | Percent of Invoice Payable | 1 | O | R | 1/5
/// 15 | 3 | Free-form Message | 1 | O | AN | 1/60
/// 16 | 713 | Installment Group Indicator | 1 | O | N0 | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G23 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E333,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E282>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E283>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E338>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E370>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E351>,
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E446>,
    #[serde(rename = "09")]
    pub _09: Option<crate::v005010::element::E386>,
    #[serde(rename = "10")]
    pub _10: Option<crate::v005010::element::E362>,
    #[serde(rename = "11")]
    pub _11: Option<crate::v005010::element::E391>,
    #[serde(rename = "12")]
    pub _12: Option<crate::v005010::element::E390>,
    #[serde(rename = "13")]
    pub _13: Option<crate::v005010::element::E343>,
    #[serde(rename = "14")]
    pub _14: Option<crate::v005010::element::E342>,
    #[serde(rename = "15")]
    pub _15: Option<String>,
    #[serde(rename = "16")]
    pub _16: Option<crate::v005010::element::E713>,
}

/// G4 - Scale Identification
///
/// To identify the scale type and the location and time a shipment is weighed
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 19 | City Name | M | AN | 2/30
/// 02 | 156 | State or Province Code | M | ID | 2/2
/// 03 | 93 | Name | O | AN | 1/60
/// 04 | 373 | Date | M | DT | 8/8
/// 05 | 337 | Time | O | TM | 4/8
/// 06 | 570 | Scale Type Code | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G4 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: String,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E570>,
}

/// G3 - Compensation Information
///
/// To convey brokerage, freight forwarder compensation, and other compensation information related to shipments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 315 | Compensation Paid | O |  | R 2/5
/// 02 | 317 | Total Compensation Amount | M |  | N0 3/10
/// 03 | 93 | Name | O |  | AN 1/60
/// 04 | 201 | Business Transaction Status | O |  | ID 1/3
/// 05 | 782 | Monetary Amount | O |  | R 1/18
/// 06 | 73 | Compensation Qualifier | O |  | ID 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G3 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E315>,
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E317,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E201>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E782>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E73>,
}

/// GF - Furnished Goods and Services
///
/// To provide information related to furnished goods and services
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 02 | 127 | Reference Identification | 1 | X | AN | 1/50
/// 03 | 367 | Contract Number | 1 | O | AN | 1/30
/// 04 | 782 | Monetary Amount | 1 | O | R | 1/18
/// 05 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 06 | 127 | Reference Identification | 1 | X | AN | 1/50
/// 07 | 328 | Release Number | 1 | O | AN | 1/30
/// 08 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 09 | 127 | Reference Identification | 1 | X | AN | 1/50
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct GF {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E782>,
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

/// G05 - Total Shipment Information
///
/// To specify the total quantity, weight, and volume of a shipment
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 382 | Number of Units Shipped | C | R | 1/10
/// 02 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
/// 03 | 81 | Weight | C | R | 1/10
/// 04 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
/// 05 | 183 | Volume | C | R | 1/8
/// 06 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
/// 07 | 80 | Lading Quantity | C | N0 | 1/7
/// 08 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G05 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E382>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E81>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E183>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E80>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
}

/// G08 - Pallet Receipt Disposition
///
/// To convey the disposition of pallets received
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 409 | Quantity of Pallets Received | 1 | O | N0 | 1/3
/// 02 | 410 | Quantity of Pallets Returned | 1 | O | N0 | 1/3
/// 03 | 411 | Quantity Contested | 1 | X | R | 1/7
/// 04 | 412 | Receiving Condition Code | 1 | X | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G08 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E409>,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E410>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E411>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E412>,
}

/// G38 - Claim Payment Information
///
/// To convey claim payment amount, method, and disposition
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 782 | Monetary Amount | 1 | M | R | 1/18
/// 02 | 591 | Payment Method Code | 1 | O | ID | 3/3
/// 03 | 1292 | Returns Disposition Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G38 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E782,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E1292>,
}

/// G31 - Total Invoice Quantity and Weight
///
/// To specify the total invoice quantity and weight
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 380 | Quantity | 1 | M | R | 1/15
/// 02 | 81 | Weight | 1 | M | R | 1/10
/// 03 | 188 | Weight Unit Code | 1 | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G31 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E81,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// G33 - Total Invoice Amount
///
/// To specify the total invoice amount
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 610 | Amount | 1 | M | N2 | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G33 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E610,
}

/// G26 - Pricing Conditions
///
/// To provide pricing conditions
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 290 | Price Condition Code | M | ID | 2/2
/// 02 | 432 | Date Qualifier | C | ID | 2/2
/// 03 | 373 | Date | C | DT | 8/8
/// 04 | 292 | Quantity Basis | C | ID | 3/3
/// 05 | 380 | Quantity | C | R | 1/15
/// 06 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G26 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E290,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E292>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

/// G36 - Price List References and Dates
///
/// To provide price list references and descriptions
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 439 | Price List Number | M | AN | 1/16
/// 02 | 440 | Price List Issue Number | O | AN | 1/16
/// 03 | 373 | Date | M | DT | 8/8
/// 04 | 291 | Price Condition Applies Code | O | ID | 3/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G36 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E291>,
}

/// G39 - Item Characteristics - Manufacturer
///
/// To transmit physical characteristics for an item at the manufacturer level
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G39 {
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
}

/// G40 - Pricing Information
///
/// To provide item cost and price information
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 417 | Price Bracket Identifier | O | AN | 1/3
/// 02 | 418 | Item List Cost - New | M | R | 1/9
/// 03 | 419 | Item List Cost - Old | O | R | 1/9
/// 04 | 369 | Free-form Description | O | AN | 1/45
/// 05 | 420 | Price New, Suggested Retail | O | N2 | 2/7
/// 06 | 421 | Price Old, Suggested Retail | O | N2 | 2/7
/// 07 | 355 | Unit or Basis for Measurement Code | O | ID | 2/2
/// 08 | 236 | Price Identifier Code | O | ID | 3/3
/// 09 | 1470 | Number | O | N0 | 1/9
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G40 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E418,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E419>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E420>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E421>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E236>,
    #[serde(rename = "09")]
    pub _09: Option<crate::v005010::element::E1470>,
}

/// G43 - Market Area
///
/// To specify the geographic area in which a promotion or price is in effect
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 771 | Market Area Code Qualifier | M | ID | 1/3
/// 02 | 767 | Market Area Code Identifier | O | AN | 1/13
/// 03 | 352 | Description | O | AN | 1/80
/// 04 | 687 | Class of Trade Code | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G43 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E771,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E687>,
}

/// G54 - Minimum Order Quantity
///
/// To specify the minimum order quantity for a product
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 380 | Quantity | M | R | 1/15
/// 02 | 355 | Unit or Basis for Measurement Code | M | ID | 2/2
/// 03 | 438 | U.P.C. Case Code | X | AN | 12/12
/// 04 | 235 | Product/Service ID Qualifier | X | ID | 2/2
/// 05 | 234 | Product/Service ID | X | AN | 1/48
/// 06 | 369 | Free-form Description | O | AN | 1/45
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G54 {
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

/// G55 - Item Physical Details
///
/// To specify the physical details for a catalog item
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G55 {
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
}

/// G93 - Bracket/Pricing Information
///
/// To convey bracket and pricing information for an item
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 417 | Price Bracket Identifier | O | AN | 1/3
/// 02 | 380 | Quantity | C | R | 1/15
/// 03 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
/// 04 | 369 | Free-form Description | O | AN | 1/45
/// 05 | 91 | Transportation Method/Type Code | O | ID | 1/2
/// 06 | 236 | Price Identifier Code | O | ID | 3/3
/// 07 | 306 | Action Code | O | ID | 1/2
/// 08 | 1073 | Yes/No Condition or Response Code | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G93 {
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
    pub _06: Option<crate::v005010::element::E236>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E1073>,
}

/// G42 - Promotion Announcement Identification
///
/// To identify promotion activities between trading partners
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 423 | Promotion Status Code | M | ID | 2/2
/// 02 | 341 | Allowance or Charge Number | M | AN | 1/16
/// 03 | 640 | Transaction Type Code | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G42 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E423,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// G45 - Promotional Product Identification
///
/// To identify the product to which a promotion applies
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G45 {
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
}

/// G51 - Free Goods
///
/// To specify the free-goods or buy/get promotion terms
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 773 | Quantity Free | C | N0 | 1/9
/// 02 | 355 | Unit or Basis for Measurement Code | C | ID | 2/2
/// 03 | 768 | Quantity Must Purchase | M | N0 | 1/9
/// 04 | 355 | Unit or Basis for Measurement Code | M | ID | 2/2
/// 05 | 438 | U.P.C. Case Code | O | AN | 12/12
/// 06 | 766 | U.P.C./EAN Consumer Package Code | O | AN | 12/12
/// 07 | 235 | Product/Service ID Qualifier | C | ID | 2/2
/// 08 | 234 | Product/Service ID | C | AN | 1/48
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G51 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E773>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: crate::v005010::element::E768,
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
}

/// G94 - Promotion Conditions
///
/// To indicate the option number associated with a promotion and to specify the 'AND' or 'OR' condition
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 293 | Promotion Condition Qualifier | O | ID | 2/2
/// 02 | 770 | Option Number | M | AN | 1/20
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G94 {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E293>,
    #[serde(rename = "02")]
    pub _02: String,
}

/// G50 - Purchase Order Identification
///
/// To carry purchase order identification data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G50 {
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

/// G01 - Invoice Identification
///
/// To carry invoice identification data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct G01 {
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

/// GA - Price Authority Information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct GA {
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
}
