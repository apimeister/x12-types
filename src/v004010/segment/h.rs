use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// H1 - Hazardous Material
///
/// To specify information relative to hazardous material
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 62 | Hazardous Material Code | 1 | M | AN | 4/10
/// 02 | 209 | Hazardous Material Class Code | 1 | O | AN | 1/4
/// 03 | 208 | Hazardous Material Code Qualifier | 1 | O | ID | 1/1
/// 04 | 64 | Hazardous Material Description | 1 | O | AN | 2/30
/// 05 | 63 | Hazardous Material Contact | 1 | O | AN | 1/24
/// 06 | 200 | Hazardous Materials Page | 1 | O | AN | 1/6
/// 07 | 77 | Flashpoint Temperature | 1 | X | N | 1/3
/// 08 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 09 | 254 | Packing Group Code | 1 | O | ID | 1/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct H1 {
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

/// H2 - Additional Hazardous Material Description
///
/// To specify free-form hazardous material descriptive data in addition to the information provided in the H1 segment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 64 | Hazardous Material Description | M |  | AN 2/30
/// 02 | 274 | Hazardous Material Classification | O |  | AN 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct H2 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// H3 - Special Handling Instructions
///
/// To specify special handling instructions in coded or free-form format
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 152 | Special Handling Code | 1 | X | ID | 2/3
/// 02 | 153 | Special Handling Description | 1 | X | AN | 2/30
/// 03 | 241 | Protective Service Code | 1 | O | ID | 1/4
/// 04 | 242 | Vent Instruction Code | 1 | O | ID | 1/7
/// 05 | 257 | Tariff Application Code | 1 | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct H3 {
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

/// HL - Hierarchical Level
///
/// To identify dependencies among and the content of hierarchically related groups of data segments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 628 | Hierarchical ID Number | 1 | M | AN | 1/12
/// 02 | 734 | Hierarchical Parent ID Number | 1 | O | AN | 1/12
/// 03 | 735 | Hierarchical Level Code | 1 | M | ID | 1/2
/// 04 | 736 | Hierarchical Child Code | 1 | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct HL {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}
