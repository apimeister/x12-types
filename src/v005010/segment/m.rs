use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::{DisplaySegment, ParseSegment};

/// MEA - Measurements
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
pub struct MEA {
    /// 737 - Measurement Reference ID Code
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E737>,
    /// 738 - Measurement Qualifier
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E738>,
    /// 739 - Measurement Value
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E739>,
    /// C001 - Composite Unit of Measure (kept as raw text for now)
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 740 - Range Minimum
    #[serde(rename = "05")]
    pub _05: Option<crate::v005010::element::E740>,
    /// 741 - Range Maximum
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E741>,
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

/// MIA - Medicare Inpatient Adjudication
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
pub struct MIA {
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
}

/// MOA - Medicare Outpatient Adjudication
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
pub struct MOA {
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
}

/// MSG - Message Text
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct MSG {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// MPI - Military Personnel Information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct MPI {
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
}

/// MSI - Multi-stop Shipment Information
///
/// To indicate whether a shipment is part of a multi-stop movement and its stop sequence
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1073 | Yes/No Condition or Response Code | 1 | M | ID | 1/1
/// 02 | 165 | Stop Sequence Number | 1 | O | N0 | 1/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct MSI {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// MTX - Text
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
pub struct MTX {
    /// 363 - Note Reference Code
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E363>,
    /// 1551 - Textual Data
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 1551 - Textual Data
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 934 - Printer Carriage Control Code
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 1470 - Number
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 819 - Language Code
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

/// MAN - Marks and Numbers Information
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
pub struct MAN {
    /// 88 - Marks and Numbers Qualifier
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E88,
    /// 87 - Marks and Numbers
    #[serde(rename = "02")]
    pub _02: String,
    /// 87 - Marks and Numbers
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 88 - Marks and Numbers Qualifier
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E88>,
    /// 87 - Marks and Numbers
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 87 - Marks and Numbers
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

/// M2 - Sales/Delivery Terms
///
/// To specify the salesperson's terms and delivery conditions
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 139 | Sales Terms Code | 1 | M | ID | 2/2
/// 02 | 138 | Sales Reference Number | 1 | O | AN | 4/6
/// 03 | 137 | Sales Reference Date | 1 | O | DT | 8/8
/// 04 | 335 | Transportation Terms Code | 1 | O | ID | 3/3
/// 05 | 136 | Sales Comment | 1 | O | AN | 2/30
/// 06 | 32 | Delivery Date | 1 | O | DT | 8/8
/// 07 | 309 | Location Qualifier | 1 | X | ID | 1/2
/// 08 | 310 | Location Identifier | 1 | X | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M2 {
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

/// M7A - Seal Number Replacement
///
/// To replace or augment seal numbers reported in a previous segment or transaction
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 225 | Seal Number | 1 | M | AN | 2/15
/// 02 | 225 | Seal Number | 1 | M | AN | 2/15
/// 03 | 373 | Date | 1 | O | DT | 8/8
/// 04 | 98 | Entity Identifier Code | 1 | X | ID | 2/3
/// 05 | 93 | Name | 1 | X | AN | 1/60
/// 06 | 352 | Description | 1 | O | AN | 1/80
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M7A {
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

/// M7 - Seal Numbers
///
/// To record seal numbers used and the organization that applied the seals
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 225 | Seal Number | 1 | M | AN | 2/15
/// 02 | 225 | Seal Number | 1 | O | AN | 2/15
/// 03 | 225 | Seal Number | 1 | O | AN | 2/15
/// 04 | 225 | Seal Number | 1 | O | AN | 2/15
/// 05 | 98 | Entity Identifier Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M7 {
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

/// MS1 - Equipment, Shipment, or Real Property Location
///
/// To specify the location of a piece of equipment, a shipment, or real property in terms of city and state or longitude and latitude
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 19 | City Name | 1 | X | AN | 2/30
/// 02 | 156 | State or Province Code | 1 | X | ID | 2/2
/// 03 | 26 | Country Code | 1 | X | ID | 2/3
/// 04 | 1654 | Longitude Code | 1 | X/Z | ID | 7/7
/// 05 | 1655 | Latitude Code | 1 | X/Z | ID | 7/7
/// 06 | 1280 | Direction Identifier Code NEW | 1 | O/Z | ID | 1/1
/// 07 | 1280 | Direction Identifier Code NEW | 1 | O/Z | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct MS1 {
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
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

/// MS2 - Equipment or Container Owner and Type
///
/// To specify the owner, the identification number assigned by that owner, and the type of equipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 140 | Standard Carrier Alpha Code | 1 | X | ID | 2/4
/// 02 | 207 | Equipment Number | 1 | X | AN | 1/10
/// 03 | 40 | Equipment Description Code | 1 | O | ID | 2/2
/// 04 | 761 | Equipment Number Check Digit | 1 | O | N0 | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct MS2 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// MS3 - Interline Information
///
/// To identify the interline carrier and relevant data
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 140 | Standard Carrier Alpha Code | 1 | M/Z | ID | 2/4
/// 02 | 133 | Routing Sequence Code | 1 | M | ID | 1/2
/// 03 | 19 | City Name | 1 | X/Z | AN | 2/30
/// 04 | 91 | Transportation Method/Type Code | 1 | O | ID | 1/2
/// 05 | 156 | State or Province Code NEW | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct MS3 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

/// M11 - Manifest Bill of Lading Details
///
/// To transmit bill of lading detail for a manifest
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M11 {
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
    pub _12: String,
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
}

/// M12 - In-bond Identifying Information
///
/// To transmit in-bond and customs entry information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M12 {
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

/// M13 - Manifest Amendment Details
///
/// To amend a manifest bill of lading
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M13 {
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
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: String,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
}

/// M15 - Customs Events Advisory Details
///
/// To transmit customs-related event details such as conveyance arrivals/departures and custodial liability transfers
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M15 {
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

/// M10 - Manifest Identifying Information
///
/// To transmit manifest identifying information for a conveyance
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 140 | Standard Carrier Alpha Code | 1 | M | ID | 2/4
/// 02 | 91 | Transportation Method/Type Code | 1 | M | ID | 1/2
/// 03 | 26 | Country Code | 1 | M | ID | 2/3
/// 04 | 597 | Vessel Code | 1 | X | ID | 1/8
/// 05 | 182 | Vessel Name | 1 | X | AN | 2/28
/// 06 | 55 | Flight/Voyage Number | 1 | M | AN | 2/10
/// 07 | 127 | Reference Identification | 1 | O | AN | 1/50
/// 08 | 380 | Quantity | 1 | O | R | 1/15
/// 09 | 256 | Manifest Type Code | 1 | O | ID | 1/1
/// 10 | 897 | Vessel Code Qualifier | 1 | X | ID | 1/1
/// 11 | 1073 | Yes/No Condition or Response Code | 1 | O | ID | 1/1
/// 12 | 127 | Reference Identification | 1 | O | AN | 1/50
/// 13 | 353 | Transaction Set Purpose Code | 1 | O | ID | 2/2
/// 14 | 346 | Application Type | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M10 {
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
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
}

/// M0 - Letter of Credit Reference
///
/// To transmit letter of credit details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 250 | Letter of Credit Number | M |  | AN 2/40
/// 02 | 373 | Date | O |  | DT 8/8
/// 03 | 373 | Date | O |  | DT 8/8
/// 04 | 373 | Date | O |  | DT 8/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M0 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// M1 - Insurance
///
/// To specify details related to insurance
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 26 | Country Code | 1 | M/Z | ID | 2/3
/// 02 | 14 | Carriage Value | 1 | O | N0 | 2/8
/// 03 | 74 | Declared Value | 1 | O | N2 | 2/12
/// 04 | 122 | Rate/Value Qualifier | 1 | O | ID | 2/2
/// 05 | 98 | Entity Identifier Code | 1 | O/Z | ID | 2/3
/// 06 | 61 | Free-Form Message | 1 | O/Z | AN | 1/30
/// 07 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 08 | 782 | Monetary Amount | 1 | X | R | 1/18
/// 09 | 1004 | Percent Qualifier | 1 | X | ID | 1/2
/// 10 | 954 | Percent | 1 | X | R | 1/10
/// 11 | 1004 | Percent Qualifier | 1 | X | ID | 1/2
/// 12 | 954 | Percent | 1 | X | R | 1/10
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M1 {
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
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

/// MIT - Message Identification
///
/// To carry message identification data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct MIT {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// M3 - Release
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M3 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}
