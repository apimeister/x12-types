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
