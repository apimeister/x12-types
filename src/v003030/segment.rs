use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// IEA - Interchange Control Trailer NEW
///
/// To define the end of an interchange of one or more functional groups and interchange-related control segments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | I16 | Number of Included Functional Groups | 1 | M | N0 | 1/5
/// 02 | I12 | Interchange Control Number | 1 | M | N0 | 9/9
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplaySegment, ParseSegment)]
pub struct IEA {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

/// ISA - Interchange Control Header NEW
///
/// To start and identify an interchange of one or more functional groups and interchange-related control segments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|-----|------|-------
/// 01 | I01 | Authorization Information Qualifier | 1 | M | ID | 2/2
/// 02 | I02 | Authorization Information | 1 | M | AN | 10/10
/// 03 | I03 | Security Information Qualifier | 1 | M | ID | 2/2
/// 04 | I04 | Security Information | 1 | M | AN | 10/10
/// 05 | I05 | Interchange ID Qualifier | 1 | M | ID | 2/2
/// 06 | I06 | Interchange Sender ID | 1 | M | AN | 15/15
/// 07 | I05 | Interchange ID Qualifier | 1 | M | ID | 2/2
/// 08 | I07 | Interchange Receiver ID | 1 | M | AN | 15/15
/// 09 | I08 | Interchange Date | 1 | M | DT | 6/6
/// 10 | I09 | Interchange Time | 1 | M | TM | 4/4
/// 11 | I10 | Interchange Control Standards Identifier | 1 | M | ID | 1/1
/// 12 | I11 | Interchange Control Version Number | 1 | M | ID | 5/5
/// 13 | I12 | Interchange Control Number | 1 | M | N0 | 9/9
/// 14 | I13 | Acknowledgment Requested | 1 | M | ID | 1/1
/// 15 | I14 | Test Indicator | 1 | M | ID | 1/1
/// 16 | I15 | Subelement Separator | 1 | M | AN | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplaySegment, ParseSegment)]
pub struct ISA {
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
    #[serde(rename = "10")]
    pub _10: String,
    #[serde(rename = "11")]
    pub _11: String,
    #[serde(rename = "12")]
    pub _12: String,
    #[serde(rename = "13")]
    pub _13: String,
    #[serde(rename = "14")]
    pub _14: String,
    #[serde(rename = "15")]
    pub _15: String,
    #[serde(rename = "16")]
    pub _16: String,
}

/// GE - Functional Group Trailer
///
/// To indicate the end of a functional group and to provide control information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 97 | Number of Transaction Sets Included | 1 | M | N0 | 1/6
/// 02 | 28 | Group Control Number | 1 | M/Z | N0 | 1/9
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplaySegment, ParseSegment)]
pub struct GE {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

/// GS - Functional Group Header
///
/// To indicate the beginning of a functional group and to provide control information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 479 | Functional Identifier Code | 1 | M | ID | 2/2
/// 02 | 142 | Application Sender's Code | 1 | M | AN | 2/15
/// 03 | 124 | Application Receiver's Code | 1 | M | AN | 2/15
/// 04 | 373 | Date | 1 | M/Z | DT | 6/6
/// 05 | 337 | Time | 1 | M/Z | TM | 4/6
/// 06 | 28 | Group Control Number | 1 | M/Z | N0 | 1/9
/// 07 | 455 | Responsible Agency Code | 1 | M | ID | 1/2
/// 08 | 480 | Version / Release / Industry Identifier Code | 1 | M | AN | 1/12
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplaySegment, ParseSegment)]
pub struct GS {
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
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: String,
}

/// SE - Transaction Set Trailer
///
/// To indicate the end of the transaction set and provide the count of the transmitted segments (including the beginning (ST) and ending (SE) segments).
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|-----|------|-------
/// 01 | 96 | Number of Included Segments | 1 | M | N0 | 1/10
/// 02 | 329 | Transaction Set Control Number | 1 | M | AN | 4/9
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplaySegment, ParseSegment)]
pub struct SE {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

/// ST - Transaction Set Header
///
/// To indicate the start of a transaction set and to assign a control number
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 143 | Transaction Set Identifier Code | 1 | M/Z | ID | 3/3
/// 02 | 329 | Transaction Set Control Number | 1 | M | AN | 4/9
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplaySegment, ParseSegment)]
pub struct ST {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

/// ZD - Transaction Set Deletion - ID, Reason, and Source
///
/// This segment is used to specify the transaction set to be canceled
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 143 | Transaction Set Identifier Code | 1 | M | ID | 3/3
/// 02 | 145 | Shipment Identification Number | 1 | O | AN | 1/30
/// 03 | 206 | Equipment Initial | 1 | M | AN | 1/4
/// 04 | 207 | Equipment Number | 1 | M | AN | 1/10
/// 05 | 244 | Transaction Reference Number | 1 | O | AN | 1/15
/// 06 | 243 | Transaction Reference Date | 1 | O | DT | 6/6
/// 07 | 202 | Correction Indicator | 1 | M | ID | 2/2
/// 08 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplaySegment, ParseSegment)]
pub struct ZD {
    #[serde(rename = "01")]
    pub _01: String,
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
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: Option<String>,
}
