use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

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

/// M3 - Release
///
/// To indicate that the equipment is or is not to be released
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 132 | Release Code | 1 | M/Z | ID | 1/1
/// 02 | 373 | Date | 1 | X | DT | 8/8
/// 03 | 337 | Time | 1 | X | TM | 4/8
/// 04 | 623 | Time Code | 1 | O/Z | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M3 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 623 - Time Code
    ///
    /// Code identifying the time. In accordance with International Standards Organization standard 8601, time can be specified by a + or - and an indication in hours in relation to Universal Time Coordinate (UTC) time; since + is a restricted character, + and - are substituted by P and M in the codes that follow
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "04")]
    pub _04: Option<String>,
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

/// M10 - Manifest Identifying Information
///
/// To transmit manifest identifying information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 140 | Standard Carrier Alpha Code | M | ID | 2/4
/// 02 | 91  | Transportation Method/Type Code M ID 1/2
/// 03 | 26 | Country Code M ID 2/3
/// 04 | 597 | Vessel Code X ID 1/8
/// 05 | 182 | Vessel Name X AN 2/28
/// 06 | 55 | Flight/Voyage Number M AN 2/10
/// 07 | 127 | Reference Identification O AN 1/30
/// 08 | 380 | Quantity O R 1/15
/// 09 | 256 | Manifest Type Code M ID 1/1
/// 10 | 897 | Vessel Code Qualifier X ID 1/1
/// 11 | 1073 | Yes/No Condition or Response Code O ID 1/1
/// 12 | 127 | Reference Identification O AN 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M10 {
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

/// M11 - Manifest Bill of Lading Details
///
/// To transmit bill of lading detail information for a manifest
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 598 | Bill of Lading/Waybill Number | M |  | AN 1/12
/// 02 | 310 | Location Identifier | M |  | AN 1/30
/// 03 | 380 | Quantity | M |  | R 1/15
/// 04 | 599 | Manifest Unit Code | M |  | ID 1/3
/// 05 | 81 | Weight | M |  | R 1/10
/// 06 | 188 | Weight Unit Code | M |  | ID 1/1
/// 07 | 183 | Volume | X |  | R 1/8
/// 08 | 184 | Volume Unit Qualifier | X |  | ID 1/1
/// 09 | 582 | Bill of Lading Type Code | O |  | ID 2/2
/// 10 | 600 | Place of Receipt by Pre-carrier | O |  | AN 1/17
/// 11 | 598 | Bill of Lading/Waybill Number | X |  | AN 1/12
/// 12 | 140 | Standard Carrier Alpha Code | M |  | ID 2/4
/// 13 | 140 | Standard Carrier Alpha Code | X |  | ID 2/4
/// 14 | 140 | Standard Carrier Alpha Code | X |  | ID 2/4
/// 15 | 140 | Standard Carrier Alpha Code | X |  | ID 2/4
/// 16 | 1302 | Shipper's Export Declaration Requirements | O |  | AN 1/2
/// 17 | 1578 | Export Exception Code | O |  | ID 2/2
/// 18 | 140 | Standard Carrier Alpha Code | X |  | ID 2/4
/// 19 | 140 | Standard Carrier Alpha Code | O |  | ID 2/4
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct M11 {
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
}

/// M12 - In-bond Identifying Information
///
/// To transmit in-bond information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 581 | Customs Entry Type Code | 1 | M | ID | 2/2
/// 02 | 601 | Customs Entry Number | 1 | X | AN | 1/15
/// 03 | 310 | Location Identifier | 1 | O/Z | AN | 1/30
/// 04 | 310 | Location Identifier | 1 | O/Z | AN | 1/30
/// 05 | 602 | Customs Shipment Value | 1 | O | AN | 2/8
/// 06 | 603 | In-bond Control Number | 1 | X | AN | 1/25
/// 07 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 08 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 09 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 10 | 91 | Transportation Method/Type Code | 1 | X | ID | 1/2
/// 11 | 182 | Vessel Name | 1 | X | AN | 2/28
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
/// To correct a manifest record prior to conveyance arrival or to amend a manifest record after conveyance arrival
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 140 | Standard Carrier Alpha Code | M |  | ID 2/4
/// 02 | 310 | Location Identifier | M |  | AN 1/30
/// 03 | 580 | Amendment Type Code | O |  | ID 1/1
/// 04 | 598 | Bill of Lading/Waybill Number | M |  | AN 1/12
/// 05 | 380 | Quantity | O |  | R 1/15
/// 06 | 393 | Amendment Code | O |  | ID 2/2
/// 07 | 306 | Action Code | O |  | ID 1/2
/// 08 | 598 | Bill of Lading/Waybill Number | X |  | AN 1/12
/// 09 | 140 | Standard Carrier Alpha Code | M |  | ID 2/4
/// 10 | 140 | Standard Carrier Alpha Code | X |  | ID 2/4
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
}

/// MAN - Marks and Numbers
///
/// To indicate identifying marks and numbers for shipping containers
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 88 | Marks and Numbers Qualifier | 1 | M/Z | ID | 1/2
/// 02 | 87 | Marks and Numbers | 1 | M/Z | AN | 1/48
/// 03 | 87 | Marks and Numbers | 1 | O | AN | 1/48
/// 04 | 88 | Marks and Numbers Qualifier | 1 | X | ID | 1/2
/// 05 | 87 | Marks and Numbers | 1 | X/Z | AN | 1/48
/// 06 | 87 | Marks and Numbers | 1 | O | AN | 1/48
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct MAN {
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

/// MEA - Measurements
///
/// To specify physical measurements or counts, including dimensions, tolerances, variances, and weights (See Figures Appendix for example of use of C001)
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 737 | Measurement Reference ID Code | 1 | O | ID | 2/2
/// 02 | 738 | Measurement Qualifier | 1 | O | ID | 1/3
/// 03 | 739 | Measurement Value | 1 | X | R | 1/20
/// 04 | C001 | Composite Unit of Measure | 1 | X/Z |  |
/// 05 | 740 | Range Minimum | 1 | X | R | 1/20
/// 06 | 741 | Range Maximum | 1 | X | R | 1/20
/// 07 | 935 | Measurement Significance Code | 1 | O | ID | 2/2
/// 08 | 936 | Measurement Attribute Code | 1 | X | ID | 2/2
/// 09 | 752 | Surface/Layer/Position Code | 1 | O | ID | 2/2
/// 10 | 1373 | Measurement Method or Device | 1 | O | ID | 2/4
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct MEA {
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

/// MSG - Message Text
///
/// To provide a free-form format that allows the transmission of text information
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
