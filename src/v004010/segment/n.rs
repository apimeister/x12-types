use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// N1 - Name
///
/// To identify a party by type of organization, name, and code
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 98 | Entity Identifier Code | 1 | M | ID | 2/3
/// 02 | 93 | Name | 1 | X | AN | 1/60
/// 03 | 66 | Identification Code Qualifier | 1 | X | ID | 1/2
/// 04 | 67 | Identification Code | 1 | X | AN | 2/80
/// 05 | 706 | Entity Relationship Code | 1 | O | ID | 2/2
/// 06 | 98 | Entity Identifier Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N1 {
    #[serde(rename = "01")]
    pub _01: String,
    /// 93 - Name
    ///
    /// Free-form name
    /// - TYPE=AN
    /// - MIN=1
    /// - MAX=60
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

/// N2 - Additional Name Information
///
/// To specify additional names or those longer than 35 characters in length
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 93 | Name | 1 | M | AN | 1/60
/// 02 | 93 | Name | 1 | O | AN | 1/60
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N2 {
    /// 93 - Name
    ///
    /// Free-form name
    /// - TYPE=AN
    /// - MIN=1
    /// - MAX=60
    #[serde(rename = "01")]
    pub _01: String,
    /// 93 - Name
    ///
    /// Free-form name
    /// - TYPE=AN
    /// - MIN=1
    /// - MAX=60
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// N3 - Address Information
///
/// To specify the location of the named party
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 166 | Address Information | 1 | M | AN | 1/55
/// 02 | 166 | Address Information | 1 | O | AN | 1/55
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N3 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// N4 - Geographic Location
///
/// To specify the geographic place of the named party
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 19 | City Name | 1 | O | AN | 2/30
/// 02 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 03 | 116 | Postal Code | 1 | O | ID | 3/15
/// 04 | 26 | Country Code | 1 | O | ID | 2/3
/// 05 | 309 | Location Qualifier | 1 | X | ID | 1/2
/// 06 | 310 | Location Identifier | 1 | O | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N4 {
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
}

/// N5 - Equipment Ordered
///
/// To specify carrier equipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 567 | Equipment Length | 1 | O | N0 | 4/5
/// 02 | 233 | Weight Capacity | 1 | O | N0 | 2/3
/// 03 | 203 | Cubic Capacity | 1 | O | N0 | 2/4
/// 04 | 301 | Car Type Code | 1 | O | ID | 1/4
/// 05 | 216 | Metric Qualifier | 1 | O | ID | 1/1
/// 06 | 65 | Height | 1 | O | R | 1/8
/// 07 | 643 | Lading Percentage | 1 | X | N2 | 2/4
/// 08 | 644 | Lading Percent Qualifier | 1 | X | ID | 1/1
/// 09 | 40 | Equipment Description Code | 1 | O | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N5 {
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
    pub _10: Option<String>,
    pub _11: Option<String>,
    pub _12: Option<String>,
    pub _13: Option<String>,
    pub _14: Option<String>,
    pub _15: Option<String>,
    pub _16: Option<String>,
    pub _17: Option<String>,
    pub _18: Option<String>,
    pub _19: Option<String>,
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

/// N9 - Reference Identification
/// To transmit identifying information as specified by the Reference Identification Qualifier
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 128 | Reference Identification Qualifier | 1 | M | ID | 2/3
/// 02 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 03 | 369 | Free-form Description | 1 | X | AN | 1/45
/// 04 | 373 | Date | 1 | O | DT | 8/8
/// 05 | 337 | Time | 1 | X | TM | 4/8
/// 06 | 623 | Time Code | 1 | O/Z | ID | 2/2
/// 07 | C040 | Reference Identifier | 1 | O/Z
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N9 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _03: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "04")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _04: Option<String>,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
    #[serde(rename = "05")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _05: Option<String>,
    /// 623 - Time Code
    ///
    /// Code identifying the time. In accordance with International Standards Organization standard 8601, time can be specified by a + or - and an indication in hours in relation to Universal Time Coordinate (UTC) time; since + is a restricted character, + and - are substituted by P and M in the codes that follow
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "06")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _07: Option<String>,
}

/// N10 - Quantity and Description
///
/// To indicate line item quantity, description, marks and numbers, commodity code, weight, and customs value
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 380 | Quantity | 1 | O | R | 1/15
/// 02 | 369 | Free-form Description | 1 | O | AN | 1/45
/// 03 | 87 | Marks and Numbers | 1 | O | AN | 1/48
/// 04 | 23 | Commodity Code Qualifier | 1 | X | ID | 1/1
/// 05 | 22 | Commodity Code | 1 | X | AN | 1/30
/// 06 | 602 | Customs Shipment Value | 1 | X | AN | 2/8
/// 07 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 08 | 81 | Weight | 1 | X | R | 1/10
/// 09 | 127 | Reference Identification | 1 | O/Z | AN | 1/30
/// 10 | 599 | Manifest Unit Code NEW | 1 | O | ID | 1/3
/// 11 | 26 | Country Code NEW | 1 | O/Z | ID | 2/3
/// 12 | 26 | Country Code NEW | 1 | O/Z | ID | 2/3
/// 13 | 100 | Currency Code NEW | 1 | X/Z | ID | 3/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct N10 {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
    pub _10: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    pub _11: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    pub _12: Option<String>,
    pub _13: Option<String>,
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
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

/// NA - Cross-Reference Equipment
///
/// To cross-reference additional equipment to a primary piece of equipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 128 | Reference Identification Qualifier | 1 | O | ID | 2/3
/// 02 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 03 | 206 | Equipment Initial | 1 | M | AN | 1/4
/// 04 | 207 | Equipment Number | 1 | M | AN | 1/10
/// 05 | 231 | Cross Reference Type Code | 1 | O | ID | 1/1
/// 06 | 219 | Position | 1 | O | AN | 1/3
/// 07 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 08 | 567 | Equipment Length | 1 | O | N0 | 4/5
/// 09 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 10 | 845 | Chassis Type | 1 | O | ID | 2/2
/// 11 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct NA {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: String,
    pub _04: String,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
    pub _10: Option<String>,
    pub _11: Option<String>,
}

/// NM1 - Individual or Organizational Name
///
/// To supply the full name of an individual or organizational entity
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 98 | Entity Identifier Code | 1 | M | ID | 2/3
/// 02 | 1065 | Entity Type Qualifier | 1 | M/Z | ID | 1/1
/// 03 | 1035 | Name Last or Organization Name | 1 | O | AN | 1/35
/// 04 | 1036 | Name First | 1 | O | AN | 1/25
/// 05 | 1037 | Name Middle | 1 | O | AN | 1/25
/// 06 | 1038 | Name Prefix | 1 | O | AN | 1/10
/// 07 | 1039 | Name Suffix | 1 | O | AN | 1/10
/// 08 | 66 | Identification Code Qualifier | 1 | X | ID | 1/2
/// 09 | 67 | Identification Code | 1 | X | AN | 2/80
/// 10 | 706 | Entity Relationship Code | 1 | X | ID | 2/2
/// 11 | 98 | Entity Identifier Code | 1 | O | ID | 2/3
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct NM1 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
    pub _10: Option<String>,
    pub _11: Option<String>,
}

/// NTE - Note/Special Instruction
///
/// To transmit information in a free-form format, if necessary, for comment or special instruction
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 363 | Note Reference Code | 1 | O | ID | 3/3
/// 02 | 352 | Description | 1 | M | AN | 1/80
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct NTE {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: String,
}
