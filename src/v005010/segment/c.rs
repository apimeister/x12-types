use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::{DisplaySegment, ParseSegment};

/// CAS - Claims Adjustment
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CAS {
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
}

/// CAT - Category of Patient Information Service
///
/// To convey the category, transmission method, and version of patient information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 755 | Report Type Code | 1 | O | ID | 2/2
/// 02 | 756 | Report Transmission Code | 1 | X | ID | 1/2
/// 03 | 799 | Version Identifier | 1 | O | AN | 1/30
/// 04 | 1270 | Code List Qualifier Code | 1 | X | ID | 1/3
/// 05 | 1271 | Industry Code | 1 | X | AN | 1/30
/// 06 | 1271 | Industry Code | 1 | O | AN | 1/30
/// 07 | 799 | Version Identifier | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CAT {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E755>,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E756>,
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

/// CB1 - Contract and Cost Accounting Information
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
pub struct CB1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// CDD - Credit/Debit Adjustment Detail
///
/// To convey the reason for, and the detail of, a credit or debit adjustment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 426 | Adjustment Reason Code | 1 | M | ID | 2/2
/// 02 | 478 | Credit/Debit Flag Code | 1 | M | ID | 1/1
/// 03 | 350 | Assigned Identification | 1 | O | AN | 1/20
/// 04 | 610 | Amount | 1 | X | N2 | 1/15
/// 05 | 1073 | Yes/No Condition or Response Code | 1 | O | ID | 1/1
/// 06 | 417 | Price Bracket Identifier | 1 | O | AN | 1/3
/// 07 | 477 | Credit/Debit Quantity | 1 | X | R | 1/10
/// 08 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 09 | 427 | Unit Price Difference | 1 | O | R | 1/15
/// 10 | 236 | Price Identifier Code | 1 | X | ID | 3/3
/// 11 | 212 | Unit Price | 1 | X | R | 1/17
/// 12 | 236 | Price Identifier Code | 1 | X | ID | 3/3
/// 13 | 212 | Unit Price | 1 | X | R | 1/17
/// 14 | 933 | Free-form Message Text | 1 | O | AN | 1/264
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CDD {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E426,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E610>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E1073>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E477>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<crate::v005010::element::E427>,
    #[serde(rename = "10")]
    pub _10: Option<crate::v005010::element::E236>,
    #[serde(rename = "11")]
    pub _11: Option<crate::v005010::element::E212>,
    #[serde(rename = "12")]
    pub _12: Option<crate::v005010::element::E236>,
    #[serde(rename = "13")]
    pub _13: Option<crate::v005010::element::E212>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
}

/// CL1 - Claim Codes
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CL1 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// CLM - Health Claim
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CLM {
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

/// CLP - Claim Level Data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CLP {
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
}

/// CMC - Commodity Classification
///
/// To identify the commodity and freight class of an item
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 22 | Commodity Code | 1 | O | AN | 1/30
/// 02 | 59 | Freight Class Code | 1 | O | AN | 2/5
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CMC {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// CN1 - Contract Information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CN1 {
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

/// COB - Coordination of Benefits
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct COB {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// CFI - Compensation Financial Information
///
/// To convey financial compensation classification and adjustment information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1136 | Code Category | 1 | M | ID | 2/2
/// 02 | 426 | Adjustment Reason Code | 1 | O | ID | 2/2
/// 03 | 1129 | Adjustment Reason Code Characteristic | 1 | O | ID | 1/2
/// 04 | 875 | Maintenance Type Code | 1 | O | ID | 3/3
/// 05 | 594 | Frequency Code | 1 | O | ID | 1/1
/// 06 | 1698 | Settlement Type Code | 1 | O | ID | 1/2
/// 07 | 9 | Late Reason Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CFI {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E1136,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E426>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E1129>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E875>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E594>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E1698>,
    #[serde(rename = "07")]
    pub _07: Option<crate::v005010::element::E9>,
}

/// CPR - Commodity Price Reference
///
/// To convey commodity pricing information referenced to a market exchange
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1053 | Market Exchange Identifier | 1 | M | ID | 3/3
/// 02 | 373 | Date | 1 | M | DT | 8/8
/// 03 | 212 | Unit Price | 1 | M | R | 1/17
/// 04 | 1054 | Commodity Identification | 1 | M | ID | 2/2
/// 05 | 1073 | Yes/No Condition or Response Code | 1 | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CPR {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E1053,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: crate::v005010::element::E212,
    #[serde(rename = "04")]
    pub _04: crate::v005010::element::E1054,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E1073>,
}

/// CR1 - Ambulance Certification
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CR1 {
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

/// CR2 - Chiropractic Certification
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
pub struct CR2 {
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
}

/// CR3 - Durable Medical Equipment Certification
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
pub struct CR3 {
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
}

/// CR4 - Enteral or Parenteral Therapy Certification
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
pub struct CR4 {
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

/// CR5 - Oxygen Therapy Certification
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
pub struct CR5 {
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

/// CR6 - Home Health Care Certification
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
pub struct CR6 {
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

/// CR7 - Home Health Treatment Plan Certification
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
pub struct CR7 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
}

/// CR8 - Pacemaker Certification
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
pub struct CR8 {
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
    #[serde(rename = "09")]
    pub _09: String,
}

/// CII - Conveyance Insurance Information
///
/// To specify the conveyance insurance information
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 93 | Name | M | AN | 1/60
/// 02 | 127 | Reference Identification | M | AN | 1/50
/// 03 | 1095 | Year | M | N0 | 4/4
/// 04 | 100 | Currency Code | M | ID | 3/3
/// 05 | 610 | Amount | M | N2 | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CII {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: crate::v005010::element::E1095,
    #[serde(rename = "04")]
    pub _04: String,
    #[serde(rename = "05")]
    pub _05: crate::v005010::element::E610,
}

/// CON - Contract Number
///
/// To specify contract or reference number and status
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 128 | Reference Identification Qualifier | M | ID | 2/3
/// 02 | 127 | Reference Identification | M | AN | 1/50
/// 03 | 846 | Contract Status Code | M | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CON {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: crate::v005010::element::E846,
}

/// CRD - Country of Origin Detail
///
/// To supply the detail information necessary to fulfill mandated requirements for the reporting of non-domestic materials and/or components included in domestically produced products
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 26 | Country Code | M | ID | 2/3
/// 02 | 522 | Amount Qualifier Code | C | ID | 1/3
/// 03 | 782 | Monetary Amount | C | R | 1/18
/// 04 | 488 | Percent, Integer Format | C | N0 | 1/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CRD {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E782>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E488>,
}

/// CRC - Conditions Indicator
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

/// CRI - Claim Report Information
///
/// To provide claim status and reporting information for an injury, illness, or incident
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 875 | Maintenance Type Code | 1 | X | ID | 3/3
/// 02 | 1029 | Claim Status Code | 1 | O | ID | 1/2
/// 03 | 1203 | Maintenance Reason Code | 1 | O | ID | 2/3
/// 04 | 1073 | Yes/No Condition or Response Code | 1 | O | ID | 1/1
/// 05 | 594 | Frequency Code | 1 | O | ID | 1/1
/// 06 | 1032 | Claim Filing Indicator Code | 1 | M | ID | 1/2
/// 07 | 1250 | Date Time Period Format Qualifier | 1 | X | ID | 2/3
/// 08 | 1251 | Date Time Period | 1 | X | AN | 1/35
/// 09 | 1129 | Adjustment Reason Code Characteristic | 1 | O | ID | 1/2
/// 10 | 9 | Late Reason Code | 1 | O | ID | 2/2
/// 11 | 1321 | Condition Indicator | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CRI {
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E875>,
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E1029>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E1203>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E1073>,
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E594>,
    #[serde(rename = "06")]
    pub _06: crate::v005010::element::E1032,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<crate::v005010::element::E1129>,
    #[serde(rename = "10")]
    pub _10: Option<crate::v005010::element::E9>,
    #[serde(rename = "11")]
    pub _11: Option<crate::v005010::element::E1321>,
}

/// CUR - Currency
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
pub struct CUR {
    /// 98 - Entity Identifier Code
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E98,
    /// 100 - Currency Code
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E100,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 98 - Entity Identifier Code
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E98>,
    /// 100 - Currency Code
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E100>,
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

/// CTX - Context
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CTX {
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

/// CTT - Transaction Totals
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
pub struct CTT {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// CST - Cost Analysis
///
/// To convey cost analysis detail for a line item
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 964 | Cost Code | 1 | M | ID | 3/3
/// 02 | 782 | Monetary Amount | 1 | M | R | 1/18
/// 03 | C001 | Composite Unit of Measure | 1 | X | | 1/1
/// 04 | 380 | Quantity | 1 | X | R | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CST {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E964,
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E782,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// CTB - Restrictions/Conditions
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
pub struct CTB {
    /// 688 - Restrictions/Conditions Qualifier
    #[serde(rename = "01")]
    pub _01: String,
    /// 65 - Description
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 673 - Quantity Qualifier
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 380 - Quantity
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 355 - Unit or Basis for Measurement Code
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 782 - Monetary Amount
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// C001 - Composite Unit of Measure
    #[serde(rename = "07")]
    pub _07: Option<String>,
}

/// CS - Contract Summary
///
/// To provide information about a contract
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 367 | Contract Number | 1 | O | AN | 1/30
/// 02 | 327 | Change Order Sequence Number | 1 | O | AN | 1/8
/// 03 | 328 | Release Number | 1 | O | AN | 1/30
/// 04 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 05 | 127 | Reference Identification | 1 | X | AN | 1/50
/// 06 | 324 | Purchase Order Number | 1 | O | AN | 1/22
/// 07 | 560 | Special Services Code | 1 | O | ID | 2/10
/// 08 | 433 | F.O.B. Point Code | 1 | O | ID | 2/2
/// 09 | 954 | Percentage as Decimal | 1 | O | R | 1/10
/// 10 | 954 | Percentage as Decimal | 1 | O | R | 1/10
/// 11 | 782 | Monetary Amount | 1 | O | R | 1/18
/// 12 | 336 | Terms Type Code | 1 | O | ID | 2/2
/// 13 | 560 | Special Services Code | 1 | O | ID | 2/10
/// 14 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 15 | 212 | Unit Price | 1 | O | R | 1/17
/// 16 | 336 | Terms Type Code | 1 | O | ID | 2/2
/// 17 | 1073 | Yes/No Condition or Response Code | 1 | O | ID | 1/1
/// 18 | 1073 | Yes/No Condition or Response Code | 1 | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CS {
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
    pub _07: Option<crate::v005010::element::E560>,
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E433>,
    #[serde(rename = "09")]
    pub _09: Option<crate::v005010::element::E954>,
    #[serde(rename = "10")]
    pub _10: Option<crate::v005010::element::E954>,
    #[serde(rename = "11")]
    pub _11: Option<crate::v005010::element::E782>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<crate::v005010::element::E560>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
    #[serde(rename = "15")]
    pub _15: Option<crate::v005010::element::E212>,
    #[serde(rename = "16")]
    pub _16: Option<String>,
    #[serde(rename = "17")]
    pub _17: Option<crate::v005010::element::E1073>,
    #[serde(rename = "18")]
    pub _18: Option<crate::v005010::element::E1073>,
}

/// CSH - Sales Requirements
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
pub struct CSH {
    /// 563 - Sales Requirement Code
    #[serde(rename = "01")]
    pub _01: Option<String>,
    /// 306 - Action Code
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 610 - Amount
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 508 - Account Number
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 373 - Date
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 100 - Agency Qualifier Code
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 22 - Special Services Code
    #[serde(rename = "07")]
    pub _07: Option<String>,
    /// 668 - Product/Service Substitution Code
    #[serde(rename = "08")]
    pub _08: Option<String>,
    /// 954 - Percentage as Decimal
    #[serde(rename = "09")]
    pub _09: Option<String>,
    /// 782 - Percent
    #[serde(rename = "10")]
    pub _10: Option<String>,
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

/// CM - Cargo Manifest
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

/// CD - Shipment Condition
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct CD {
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
