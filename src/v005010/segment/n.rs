use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::{DisplaySegment, ParseSegment};

/// N1 - Party Identifier
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
pub struct N1 {
    /// 98 - Entity Identifier Code
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E98,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 66 - Identification Code Qualifier
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E66>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

/// N2 - Additional Name Information
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
pub struct N2 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// N3 - Party Location
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
pub struct N3 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// N4 - Geographic Location
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
pub struct N4 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    /// 156 - State or Province Code
    #[serde(rename = "02")]
    pub _02: Option<crate::v005010::element::E156>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 26 - Country Code
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E26>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
}

/// NM1 - Individual or Organizational Name
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
pub struct NM1 {
    /// 98 - Entity Identifier Code
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E98,
    /// 1065 - Entity Type Qualifier
    #[serde(rename = "02")]
    pub _02: crate::v005010::element::E1065,
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
    /// 66 - Identification Code Qualifier
    #[serde(rename = "08")]
    pub _08: Option<crate::v005010::element::E66>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
}

/// NTE - Note/Special Instruction
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
pub struct NTE {
    /// 363 - Note Reference Code
    #[serde(rename = "01")]
    pub _01: Option<crate::v005010::element::E363>,
    #[serde(rename = "02")]
    pub _02: String,
}

/// NX2 - Location ID Component
///
/// To identify one or more address components
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1106 | Address Component Qualifier | 1 | M | ID | 2/2
/// 02 | 166 | Address Information | 1 | M | AN | 1/55
/// 03 | 1096 | County Designator | 1 | O | ID | 5/5
/// 04 | 1106 | Address Component Qualifier | 1 | X | ID | 2/2
/// 05 | 166 | Address Information | 1 | X | AN | 1/55
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct NX2 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E1106,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<crate::v005010::element::E1106>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

/// NX1 - Property or Entity Identification
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
pub struct NX1 {
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

/// N10 - Quantity and Description
///
/// To indicate line item quantity, description, marks and numbers, commodity code, weight, and customs value
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N10 {
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
}

/// N8 - Waybill Reference
///
/// To define information required to identify a waybill
///
/// REF | ID | NAME | REQ | TYPE | MIN/MAX
/// ----|----|-------|----|------|-------
/// 01 | 186 | Waybill Number | M | N0 | 1/6
/// 02 | 373 | Date | M | DT | 8/8
/// 03 | 231 | Cross Reference Type Code | X | ID | 1/1
/// 04 | 206 | Equipment Initial | X | AN | 1/4
/// 05 | 207 | Equipment Number | X | AN | 1/15
/// 06 | 186 | Waybill Number | X | N0 | 1/6
/// 07 | 373 | Date | X | DT | 8/8
/// 08 | 19 | City Name | X | AN | 2/30
/// 09 | 156 | State or Province Code | X | ID | 2/2
/// 10 | 140 | Standard Carrier Alpha Code | X | ID | 2/4
/// 11 | 573 | Freight Station Accounting Code | X | ID | 1/5
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N8 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E186,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<crate::v005010::element::E231>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v005010::element::E186>,
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

/// N8A - Additional Reference Numbers
///
/// To transmit additional waybill cross-reference information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N8A {
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

/// N9 - Extended Reference Information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N9 {
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
}

/// N7 - Equipment Details
///
/// To identify the equipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 206 | Equipment Initial | 1 | O | AN | 1/4
/// 02 | 207 | Equipment Number | 1 | M | AN | 1/10
/// 03 | 81 | Weight | 1 | X | R | 1/10
/// 04 | 187 | Weight Qualifier | 1 | X | ID | 1/2
/// 05 | 167 | Tare Weight | 1 | X | N0 | 3/8
/// 06 | 232 | Weight Allowance | 1 | O | N0 | 2/6
/// 07 | 205 | Dunnage | 1 | O | N0 | 1/6
/// 08 | 183 | Volume | 1 | X | R | 1/8
/// 09 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 10 | 102 | Ownership Code | 1 | O | ID | 1/1
/// 11 | 40 | Equipment Description Code | 1 | O | ID | 2/2
/// 12 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 13 | 319 | Temperature Control | 1 | O | AN | 3/6
/// 14 | 219 | Position | 1 | O | AN | 1/3
/// 15 | 567 | Equipment Length | 1 | O | N0 | 4/5
/// 16 | 571 | Tare Qualifier Code | 1 | X | ID | 1/1
/// 17 | 188 | Weight Unit Code | 1 | O | ID | 1/1
/// 18 | 761 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 19 | 56 | Type of Service Code | 1 | O | ID | 2/2
/// 20 | 65 | Height | 1 | O | R | 1/8
/// 21 | 189 | Width | 1 | O | R | 1/8
/// 22 | 24 | Equipment Type | 1 | O | ID | 4/4
/// 23 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 24 | 301 | Car Type Code | 1 | O | ID | 1/4
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N7 {
    pub _01: Option<String>,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
    /// 102 - Ownership Code
    pub _10: Option<crate::v005010::element::E102>,
    /// 40 - Equipment Description Code
    pub _11: Option<crate::v005010::element::E40>,
    pub _12: Option<String>,
    pub _13: Option<String>,
    pub _14: Option<String>,
    pub _15: Option<String>,
    /// 571 - Tare Qualifier Code
    pub _16: Option<crate::v005010::element::E571>,
    /// 188 - Weight Unit Code
    pub _17: Option<crate::v005010::element::E188>,
    pub _18: Option<String>,
    /// 56 - Type of Service Code
    pub _19: Option<crate::v005010::element::E56>,
    pub _20: Option<String>,
    pub _21: Option<String>,
    pub _22: Option<String>,
    pub _23: Option<String>,
    pub _24: Option<String>,
}

/// N7A - Accessorial Equipment Details
///
/// To identify the accessorial equipment required to load or unload product
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1042 | Load or Device Code | 1 | O | ID | 2/2
/// 02 | 82 | Length | 1 | O/Z | R | 1/8
/// 03 | 1043 | Diameter | 1 | O/Z | R | 1/2
/// 04 | 1044 | Hose Type Code | 1 | O | ID | 3/3
/// 05 | 1043 | Diameter | 1 | O/Z | R | 1/2
/// 06 | 1043 | Diameter | 1 | O/Z | R | 1/2
/// 07 | 1045 | Inlet or Outlet Material Type Code | 1 | O | ID | 2/2
/// 08 | 1046 | Inlet or Outlet Fitting Type Code | 1 | O | ID | 2/2
/// 09 | 1047 | Miscellaneous Equipment Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N7A {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
}

/// N7B - Additional Equipment Details
///
/// To identify additional equipment details
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1024 | Number of Tank Compartments | 1 | O | N0 | 1/2
/// 02 | 1025 | Loading or Discharge Location Code | 1 | O | ID | 1/1
/// 03 | 1026 | Vessel Material Code | 1 | O | ID | 3/3
/// 04 | 1030 | Gasket Type Code | 1 | O | ID | 3/3
/// 05 | 1031 | Trailer Lining Type Code | 1 | O | ID | 3/3
/// 06 | 127 | Reference Identification | 1 | O/Z | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N7B {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
}

/// N11 - Store Location
///
/// To identify a store location
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 614 | Store Number | 1 | M | AN | 1/10
/// 02 | 310 | Location Identifier | 1 | O | AN | 1/30
/// 03 | 127 | Reference Identification | 1 | O | AN | 1/50
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N11 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// N12 - Equipment Environment
///
/// To describe the operating environment of the equipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 829 | Fuel Type | M |  | ID 1/1
/// 02 | C001 | Composite Unit of Measure | M
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N12 {
    #[serde(rename = "01")]
    pub _01: crate::v005010::element::E829,
    #[serde(rename = "02")]
    pub _02: String,
}

/// N5 - Equipment Ordered
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N5 {
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

/// NA - Cross-Reference Equipment
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct NA {
    #[serde(rename = "01")]
    pub _01: Option<String>,
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
}
