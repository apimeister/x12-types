use serde::{Deserialize, Serialize};

/// AK1 - Functional Group Response Header
/// 
/// To start acknowledgment of a functional group
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 479 | Functional Identifier Code | 1 | M/Z | ID | 2/2
/// 02 | 28 | Group Control Number | 1 | M/Z | N0 | 1/9
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct AK1 {
    pub _01: String,
    pub _02: String,
}

/// AK2 - Transaction Set Response Header
/// 
/// To start acknowledgment of a single transaction set
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 143 | Transaction Set Identifier Code | 1 | M/Z | ID | 3/3
/// 02 | 329 | Transaction Set Control Number | 1 | M/Z | AN | 4/9
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct AK2 {
    pub _01: String,
    pub _02: String,
}

/// AK3 - Data Segment Note
/// 
/// To report errors in a data segment and identify the location of the data segment
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 721 | Segment ID Code | 1 | M | ID | 2/3
/// 02 | 719 | Segment Position in Transaction Set | 1 | M | N0 | 1/6
/// 03 | 447 | Loop Identifier Code | 1 | O | AN | 1/6
/// 04 | 720 | Segment Syntax Error Code | 1 | O | ID | 1/3
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct AK3 {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: Option<String>,
}

/// AK4 - Data Element Note
/// 
/// To report errors in a data element or composite data structure and identify the location of the data element
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | C030 | Position in Segment | 1 | M |  | 
/// 02 | 725 | Data Element Reference Number | 1 | O | N0 | 1/4
/// 03 | 723 | Data Element Syntax Error Code | 1 | M | ID | 1/3
/// 04 | 724 | Copy of Bad Data Element | 1 | O/Z | AN | 1/99
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct AK4 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: String,
    pub _04: Option<String>,
}

/// AK9 - Functional Group Response Trailer
/// 
/// To acknowledge acceptance or rejection of a functional group and report the number of included transaction sets from the original trailer, the accepted sets, and the received sets in this functional group
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 715 | Functional Group Acknowledge Code | 1 | M | ID | 1/1
/// 02 | 97 | Number of Transaction Sets Included | 1 | M | N0 | 1/6
/// 03 | 123 | Number of Received Transaction Sets | 1 | M | N0 | 1/6
/// 04 | 2 | Number of Accepted Transaction Sets | 1 | M | N0 | 1/6
/// 05 | 716 | Functional Group Syntax Error Code | 1 | O | ID | 1/3
/// 06 | 716 | Functional Group Syntax Error Code | 1 | O | ID | 1/3
/// 07 | 716 | Functional Group Syntax Error Code | 1 | O | ID | 1/3
/// 08 | 716 | Functional Group Syntax Error Code | 1 | O | ID | 1/3
/// 09 | 716 | Functional Group Syntax Error Code | 1 | O | ID | 1/3
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct AK9 {
    pub _01: String,
    pub _02: String,
    pub _03: String,
    pub _04: String,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
}

/// B4 - Beginning Segment for Inquiry or Reply
///
/// To transmit identifying numbers, dates, and other basic data relating to the transaction set
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 152 | Special Handling Code | 1 | O | ID | 2/3
/// 02 | 71 | Inquiry Request Number | 1 | O | N0 | 1/3
/// 03 | 157 | Shipment Status Code | 1 | O | ID | 1/2
/// 04 | 373 | Date | 1 | O/Z | DT | 8/8
/// 05 | 161 | Status Time | 1 | O | TM | 4/4
/// 06 | 159 | Status Location | 1 | O | AN | 3/5
/// 07 | 206 | Equipment Initial | 1 | X | AN | 1/4
/// 08 | 207 | Equipment Number | 1 | X | AN | 1/10
/// 09 | 578 | Equipment Status Code | 1 | O | ID | 1/2
/// 10 | 24 | Equipment Type | 1 | O | ID | 4/4
/// 11 | 310 | Location Identifier | 1 | X | AN | 1/30
/// 12 | 309 | Location Qualifier | 1 | X | ID | 1/2
/// 13 | 761 | Equipment Number Check Digit | 1 | O | N0 | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct B4 {
    /// 152 - Special Handling Code
    ///
    /// Code specifying special transportation handling instructions
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
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
}

/// BL - Billing Information
/// 
/// To identify the individual billing segments within a movement when joint rail rates have been established between carriers but do not cover the entire movement
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 747 | Rebill Reason Code | 1 | M | ID | 2/2
/// 02 | 573 | Freight Station Accounting Code | 1 | M/Z | ID | 1/5
/// 03 | 573 | Freight Station Accounting Code | 1 | M/Z | ID | 1/5
/// 04 | 154 | Standard Point Location Code | 1 | X/Z | ID | 6/9
/// 05 | 19 | City Name | 1 | X/Z | AN | 2/30
/// 06 | 156 | State or Province Code | 1 | X | ID | 2/2
/// 07 | 26 | Country Code | 1 | O | ID | 2/3
/// 08 | 154 | Standard Point Location Code | 1 | X/Z | ID | 6/9
/// 09 | 19 | City Name | 1 | X/Z | AN | 2/30
/// 10 | 156 | State or Province Code | 1 | X | ID | 2/2
/// 11 | 26 | Country Code | 1 | O | ID | 2/3
/// 12 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 13 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 14 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 15 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 16 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 17 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct BL {
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
    pub _12: Option<String>,
    pub _13: Option<String>,
    pub _14: Option<String>,
    pub _15: Option<String>,
    pub _16: Option<String>,
    pub _17: Option<String>,
}

/// BNX - Rail Shipment Information
/// 
/// To transmit rail-specific shipment data
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 460 | Shipment Weight Code | 1 | O | ID | 1/1
/// 02 | 129 | Referenced Pattern Identifier | 1 | O | AN | 1/13
/// 03 | 11 | Billing Code | 1 | O | ID | 1/1
/// 04 | 223 | Repetitive Pattern Number | 1 | O | N0 | 5/5
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct BNX {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
}

/// BX - General Shipment Information
/// 
/// To transmit identification numbers and other basic shipment data
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 353 | Transaction Set Purpose Code | 1 | M | ID | 2/2
/// 02 | 91 | Transportation Method/Type Code | 1 | M | ID | 1/2
/// 03 | 146 | Shipment Method of Payment | 1 | M | ID | 2/2
/// 04 | 145 | Shipment Identification Number | 1 | O | AN | 1/30
/// 05 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 06 | 188 | Weight Unit Code | 1 | O | ID | 1/1
/// 07 | 147 | Shipment Qualifier | 1 | O | ID | 1/1
/// 08 | 226 | Section Seven Code | 1 | O | ID | 1/1
/// 09 | 195 | Capacity Load Code | 1 | O | ID | 1/1
/// 10 | 160 | Status Report Request Code | 1 | O | ID | 1/1
/// 11 | 501 | Customs Documentation Handling Code | 1 | O | ID | 2/2
/// 12 | 199 | Confidential Billing Request Code | 1 | O | ID | 1/1
/// 13 | 714 | Goods and Services Tax Reason Code | 1 | O | ID | 1/1
/// 14 | 346 | Application Type | 1 | O | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct BX {
    pub _01: String,
    pub _02: String,
    pub _03: String,
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
}

/// CM - Cargo Manifest
/// 
/// To identify specific flight or voyage information for multimodal shipments
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 55 | Flight/Voyage Number | 1 | O | AN | 2/10
/// 02 | 115 | Port or Terminal Function Code | 1 | X | ID | 1/1
/// 03 | 114 | Port Name | 1 | O | AN | 2/24
/// 04 | 373 | Date | 1 | O/Z | DT | 8/8
/// 05 | 13 | Booking Number | 1 | O | AN | 1/17
/// 06 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 07 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 08 | 373 | Date | 1 | O/Z | DT | 8/8
/// 09 | 182 | Vessel Name | 1 | O | AN | 2/28
/// 10 | 113 | Pier Number | 1 | O | AN | 1/4
/// 11 | 112 | Pier Name | 1 | O | AN | 2/14
/// 12 | 174 | Terminal Name | 1 | O | AN | 2/30
/// 13 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 14 | 26 | Country Code | 1 | O | ID | 2/3
/// 15 | 127 | Reference Identification | 1 | O/Z | AN | 1/30
/// 16 | 202 | Correction Indicator NEW | 1 | O | ID | 2/2
/// 17 | 91 | Transportation Method/Type Code NEW | 1 | O | ID | 1/2
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct CM {
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
    pub _11: Option<String>,
    pub _12: Option<String>,
    pub _13: Option<String>,
    pub _14: Option<String>,
    pub _15: Option<String>,
    pub _16: Option<String>,
    pub _17: Option<String>,
}

/// D9 - Destination Station
/// 
/// To identify the rail destination of the shipment
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 573 | Freight Station Accounting Code | 1 | O | ID | 1/5
/// 02 | 19 | City Name | 1 | M/Z | AN | 2/30
/// 03 | 156 | State or Province Code | 1 | M | ID | 2/2
/// 04 | 26 | Country Code | 1 | O/Z | ID | 2/3
/// 05 | 573 | Freight Station Accounting Code | 1 | O/Z | ID | 1/5
/// 06 | 19 | City Name | 1 | O | AN | 2/30
/// 07 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 08 | 154 | Standard Point Location Code | 1 | O/Z | ID | 6/9
/// 09 | 116 | Postal Code | 1 | O/Z | ID | 3/15
/// 10 | 154 | Standard Point Location Code NEW | 1 | O/Z | ID | 6/9
/// 11 | 116 | Postal Code NEW | 1 | O/Z | ID | 3/15
/// 12 | 26 | Country Code NEW | 1 | O/Z | ID | 2/3
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct D9 {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: String,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
    pub _10: Option<String>,
    pub _11: Option<String>,
    pub _12: Option<String>,
}

/// DTM - Date/Time Reference
///
/// To specify pertinent dates and times
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 374 | Date/Time Qualifier | 1 | M | ID | 3/3
/// 02 | 373 | Date | 1 | X | DT | 8/8
/// 03 | 337 | Time | 1 | X | TM | 4/8
/// 04 | 623 | Time Code | 1 | O | ID | 2/2
/// 05 | 1250 | Date Time Period Format Qualifier | 1 | X | ID | 2/3
/// 06 | 1251 | Date Time Period | 1 | X | AN | 1/35
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct DTM {
    pub _01: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _02: String,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
}

/// E1 - Empty Car Disposition - Pended Destination Consignee
/// 
/// To identify the party receiving the empty car
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 459 | Name (30 Character Format) | 1 | M | AN | 2/30
/// 02 | 66 | Identification Code Qualifier | 1 | X | ID | 1/2
/// 03 | 67 | Identification Code | 1 | X | AN | 2/80
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct E1 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
}

/// E4 - Empty Car Disposition - Pended Destination City
/// 
/// To specify the geographic place of named party receiving the empty car
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 19 | City Name | 1 | M | AN | 2/30
/// 02 | 156 | State or Province Code | 1 | M | ID | 2/2
/// 03 | 116 | Postal Code | 1 | O | ID | 3/15
/// 04 | 26 | Country Code | 1 | O | ID | 2/3
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct E4 {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: Option<String>,
}

/// E5 - Empty Car Disposition - Pended Destination Route
/// 
/// To specify the routing of the empty car
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 140 | Standard Carrier Alpha Code | 1 | M | ID | 2/4
/// 02 | 133 | Routing Sequence Code | 1 | M | ID | 1/2
/// 03 | 19 | City Name | 1 | O | AN | 2/30
/// 04 | 154 | Standard Point Location Code | 1 | O | ID | 6/9
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct E5 {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: Option<String>,
}

/// EM - Equipment Characteristics
/// 
/// To send additional information regarding a specific piece of equipment
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 02 | 81 | Weight | 1 | O/Z | R | 1/10
/// 03 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 04 | 183 | Volume | 1 | O/Z | R | 1/8
/// 05 | 26 | Country Code | 1 | O/Z | ID | 2/3
/// 06 | 1429 | Construction Type | 1 | O | ID | 1/2
/// 07 | 373 | Date | 1 | O/Z | DT | 8/8
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct EM {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
}

/// F9 - Origin Station
/// 
/// To identify the rail origin of the shipment
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 573 | Freight Station Accounting Code | 1 | O | ID | 1/5
/// 02 | 19 | City Name | 1 | M/Z | AN | 2/30
/// 03 | 156 | State or Province Code | 1 | M | ID | 2/2
/// 04 | 26 | Country Code | 1 | O/Z | ID | 2/3
/// 05 | 573 | Freight Station Accounting Code | 1 | O/Z | ID | 1/5
/// 06 | 19 | City Name | 1 | O | AN | 2/30
/// 07 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 08 | 154 | Standard Point Location Code | 1 | O/Z | ID | 6/9
/// 09 | 116 | Postal Code | 1 | O/Z | ID | 3/15
/// 10 | 154 | Standard Point Location Code NEW | 1 | O/Z | ID | 6/9
/// 11 | 116 | Postal Code NEW | 1 | O/Z | ID | 3/15
/// 12 | 26 | Country Code NEW | 1 | O/Z | ID | 2/3
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct F9 {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: String,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
    pub _10: Option<String>,
    pub _11: Option<String>,
    pub _12: Option<String>,
}

/// GA - Canadian Grain Information
/// 
/// To transmit the transportation and distribution requirements of grain at Canadian ports
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 1275 | Fumigated/Cleaned Indicator | 1 | O | ID | 1/1
/// 02 | 22 | Commodity Code | 1 | O/Z | AN | 1/30
/// 03 | 1576 | Inspected/Weighed Indicator Code | 1 | O | ID | 1/2
/// 04 | 128 | Reference Identification Qualifier | 1 | O | ID | 2/3
/// 05 | 127 | Reference Identification | 1 | O | AN | 1/30
/// 06 | 642 | Week | 1 | O/Z | N0 | 4/4
/// 07 | 899 | Unload Terminal Elevator Code | 1 | O | ID | 3/4
/// 08 | 373 | Date | 1 | O/Z | DT | 8/8
/// 09 | 1470 | Number | 1 | O/Z | N0 | 1/9
/// 10 | 1276 | Machine Separable Indicator Code | 1 | O | ID | 2/2
/// 11 | 1277 | Canadian Wheat Board (CWB) Marketing Class Code | 1 | O | ID | 1/1
/// 12 | 1278 | Canadian Wheat Board (CWB) Marketing Class Type Code | 1 | O | ID | 1/1
/// 13 | 1073 | Yes/No Condition or Response Code | 1 | O | ID | 1/1
/// 14 | 310 | Location Identifier | 1 | X/Z | AN | 1/30
/// 15 | 156 | State or Province Code | 1 | X | ID | 2/2
/// 16 | 1004 | Percent Qualifier | 1 | X | ID | 1/2
/// 17 | 954 | Percent | 1 | X | R | 1/10
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct GA {
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
    pub _11: Option<String>,
    pub _12: Option<String>,
    pub _13: Option<String>,
    pub _14: Option<String>,
    pub _15: Option<String>,
    pub _16: Option<String>,
    pub _17: Option<String>,
}

/// GE - Functional Group Trailer
///
/// To indicate the end of a functional group and to provide control information
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 97 | Number of Transaction Sets Included | 1 | M | N0 | 1/6
/// 02 | 28 | Group Control Number | 1 | M/Z | N0 | 1/9
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct GE {
    pub _01: String,
    pub _02: String,
}

/// GS - Functional Group Header
///
/// To indicate the beginning of a functional group and to provide control information
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 479 | Functional Identifier Code | 1 | M | ID | 2/2
/// 02 | 142 | Application Sender's Code | 1 | M | AN | 2/15
/// 03 | 124 | Application Receiver's Code | 1 | M | AN | 2/15
/// 04 | 373 | Date | 1 | M/Z | DT | 8/8
/// 05 | 337 | Time | 1 | M/Z | TM | 4/8
/// 06 | 28 | Group Control Number | 1 | M/Z | N0 | 1/9
/// 07 | 455 | Responsible Agency Code | 1 | M | ID | 1/2
/// 08 | 480 | Version / Release / Industry Identifier Code | 1 | M | AN | 1/12
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct GS {
    pub _01: String,
    pub _02: String,
    pub _03: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _04: String,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
    pub _05: String,
    pub _06: String,
    pub _07: String,
    pub _08: String,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct H3 {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
}

/// IC - Intermodal Chassis Equipment
/// 
/// To specify the chassis equipment details in terms of identifying numbers, weights, and ownership
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 206 | Equipment Initial | 1 | M | AN | 1/4
/// 02 | 207 | Equipment Number | 1 | M | AN | 1/10
/// 03 | 167 | Tare Weight | 1 | X | N0 | 3/8
/// 04 | 571 | Tare Qualifier Code | 1 | X | ID | 1/1
/// 05 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 06 | 567 | Equipment Length | 1 | O | N0 | 4/5
/// 07 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 08 | 845 | Chassis Type | 1 | O | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct IC {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
}

/// IM - Intermodal Movement Information
/// 
/// To specify the overall movement of a shipment
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 533 | Water Movement Code | 1 | O | ID | 1/1
/// 02 | 152 | Special Handling Code | 1 | O | ID | 2/3
/// 03 | 534 | Inland Transportation Code | 1 | O/Z | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct IM {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
}

/// IEA - Interchange Control Trailer
///
/// To define the end of an interchange of zero or more functional groups and interchange-related control segments
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | I16 | Number of Included Functional Groups | 1 | M | N0 | 1/5
/// 02 | I12 | Interchange Control Number | 1 | M | N0 | 9/9
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct IEA {
    /// I16 - Number of Included Functional Groups
    /// A count of the number of functional groups included in an interchange
    pub _01: String,
    /// I12 - Interchange Control Number
    /// A control number assigned by the interchange sender
    pub _02: String,
}

/// ISA - Interchange Control Header
///
/// To start and identify an interchange of zero or more functional groups and interchange-related control segments
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
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
/// 15 | I14 | Usage Indicator | 1 | M | ID | 1/1
/// 16 | I15 | Component Element Separator | 1 | M |  | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ISA {
    pub _01: String,
    pub _02: String,
    pub _03: String,
    pub _04: String,
    pub _05: String,
    pub _06: String,
    pub _07: String,
    pub _08: String,
    pub _09: String,
    pub _10: String,
    pub _11: String,
    pub _12: String,
    /// I12 - Interchange Control Number
    ///
    /// A control number assigned by the interchange sender
    /// - TYPE=N0
    /// - MIN=9
    /// - MAX=9
    pub _13: String,
    pub _14: String,
    pub _15: String,
    pub _16: String,
}

/// L0 - Line Item - Quantity and Weight
/// 
/// To specify quantity, weight, volume, and type of service for a line item including applicable "quantity/rate-as" data
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 213 | Lading Line Item Number | 1 | O | N0 | 1/3
/// 02 | 220 | Billed/Rated-as Quantity | 1 | X | R | 1/11
/// 03 | 221 | Billed/Rated-as Qualifier | 1 | X | ID | 2/2
/// 04 | 81 | Weight | 1 | X | R | 1/10
/// 05 | 187 | Weight Qualifier | 1 | X | ID | 1/2
/// 06 | 183 | Volume | 1 | X | R | 1/8
/// 07 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 08 | 80 | Lading Quantity | 1 | X/Z | N0 | 1/7
/// 09 | 211 | Packaging Form Code | 1 | X | ID | 3/3
/// 10 | 458 | Dunnage Description | 1 | O | AN | 2/25
/// 11 | 188 | Weight Unit Code | 1 | O | ID | 1/1
/// 12 | 56 | Type of Service Code | 1 | O | ID | 2/2
/// 13 | 380 | Quantity | 1 | X/Z | R | 1/15
/// 14 | 211 | Packaging Form Code | 1 | O | ID | 3/3
/// 15 | 1073 | Yes/No Condition or Response Code | 1 | X/Z | ID | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct L0 {
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
    pub _11: Option<String>,
    pub _12: Option<String>,
    pub _13: Option<String>,
    pub _14: Option<String>,
    pub _15: Option<String>,
}

/// L1 - Rate and Charges
/// 
/// To specify rate and charges detail relative to a line item including freight charges, advances, special charges, and entitlements
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 213 | Lading Line Item Number | 1 | O | N0 | 1/3
/// 02 | 60 | Freight Rate | 1 | X | R | 1/9
/// 03 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 04 | 58 | Charge | 1 | X | N2 | 1/12
/// 05 | 191 | Advances | 1 | X | N2 | 1/9
/// 06 | 117 | Prepaid Amount | 1 | X | N2 | 1/9
/// 07 | 120 | Rate Combination Point Code | 1 | O | AN | 3/9
/// 08 | 150 | Special Charge or Allowance Code | 1 | O | ID | 3/3
/// 09 | 121 | Rate Class Code | 1 | O | ID | 1/3
/// 10 | 39 | Entitlement Code | 1 | O | ID | 1/1
/// 11 | 16 | Charge Method of Payment | 1 | O | ID | 1/1
/// 12 | 276 | Special Charge Description | 1 | O | AN | 2/25
/// 13 | 257 | Tariff Application Code | 1 | O | ID | 1/1
/// 14 | 74 | Declared Value | 1 | X | N2 | 2/12
/// 15 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 16 | 372 | Lading Liability Code | 1 | O | ID | 1/1
/// 17 | 220 | Billed/Rated-as Quantity | 1 | X | R | 1/11
/// 18 | 221 | Billed/Rated-as Qualifier | 1 | X | ID | 2/2
/// 19 | 954 | Percent | 1 | O/Z | R | 1/10
/// 20 | 100 | Currency Code | 1 | O/Z | ID | 3/3
/// 21 | 610 | Amount | 1 | O/Z | N2 | 1/15
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct L1 {
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
}

/// L3 - Total Weight and Charges
/// 
/// To specify the total shipment in terms of weight, volume, rates, charges, advances, and prepaid amounts applicable to one or more line items
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 81 | Weight | 1 | X | R | 1/10
/// 02 | 187 | Weight Qualifier | 1 | X | ID | 1/2
/// 03 | 60 | Freight Rate | 1 | X | R | 1/9
/// 04 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 05 | 58 | Charge | 1 | O/Z | N2 | 1/12
/// 06 | 191 | Advances | 1 | O | N2 | 1/9
/// 07 | 117 | Prepaid Amount | 1 | O | N2 | 1/9
/// 08 | 150 | Special Charge or Allowance Code | 1 | O | ID | 3/3
/// 09 | 183 | Volume | 1 | X | R | 1/8
/// 10 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 11 | 80 | Lading Quantity | 1 | O | N0 | 1/7
/// 12 | 188 | Weight Unit Code | 1 | O | ID | 1/1
/// 13 | 171 | Tariff Number | 1 | O | AN | 1/7
/// 14 | 74 | Declared Value | 1 | X | N2 | 2/12
/// 15 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct L3 {
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
    pub _11: Option<String>,
    pub _12: Option<String>,
    pub _13: Option<String>,
    pub _14: Option<String>,
    pub _15: Option<String>,
}

/// L5 - Description, Marks and Numbers
/// 
/// To specify the line item in terms of description, quantity, packaging, and marks and numbers
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 213 | Lading Line Item Number | 1 | O | N0 | 1/3
/// 02 | 79 | Lading Description | 1 | O | AN | 1/50
/// 03 | 22 | Commodity Code | 1 | X | AN | 1/30
/// 04 | 23 | Commodity Code Qualifier | 1 | X | ID | 1/1
/// 05 | 103 | Packaging Code | 1 | O | AN | 3/5
/// 06 | 87 | Marks and Numbers | 1 | X | AN | 1/48
/// 07 | 88 | Marks and Numbers Qualifier | 1 | O | ID | 1/2
/// 08 | 23 | Commodity Code Qualifier | 1 | X | ID | 1/1
/// 09 | 22 | Commodity Code | 1 | X | AN | 1/30
/// 10 | 595 | Compartment ID Code | 1 | O | ID | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct L5 {
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
}

/// LE - Loop Trailer
/// 
/// To indicate that the loop immediately preceding this segment is complete
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 447 | Loop Identifier Code | 1 | M | AN | 1/6
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LE {
    pub _01: String,
}

/// LEP - EPA Required Data
/// 
/// To specify the Environmental Protection Agency (EPA) information relating to shipments of hazardous material
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 806 | EPA Waste Stream Number Code | 1 | O | ID | 4/6
/// 02 | 807 | Waste Characteristics Code | 1 | O | ID | 12/16
/// 03 | 156 | State or Province Code NEW | 1 | X/Z | ID | 2/2
/// 04 | 127 | Reference Identification NEW | 1 | X/Z | AN | 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LEP {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
}

/// LFH - Freeform Hazardous Material Information
/// 
/// To uniquely identify the variable information required by government regulation covering the transportation of hazardous material shipments
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 808 | Hazardous Material Shipment Information Qualifier | 1 | M | ID | 3/3
/// 02 | 809 | Hazardous Material Shipment Information | 1 | M | AN | 1/25
/// 03 | 809 | Hazardous Material Shipment Information | 1 | O | AN | 1/25
/// 04 | 1023 | Hazard Zone Code | 1 | O | ID | 1/1
/// 05 | 355 | Unit or Basis for Measurement Code NEW | 1 | X | ID | 2/2
/// 06 | 380 | Quantity NEW | 1 | X/Z | R | 1/15
/// 07 | 380 | Quantity NEW | 1 | O/Z | R | 1/15
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LFH {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
}

/// LH1 - Hazardous Identification Information
/// 
/// To specify the hazardous commodity identification reference number and quantity
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 355 | Unit or Basis for Measurement Code | 1 | M | ID | 2/2
/// 02 | 80 | Lading Quantity | 1 | M | N0 | 1/7
/// 03 | 277 | UN/NA Identification Code | 1 | O | ID | 6/6
/// 04 | 200 | Hazardous Materials Page | 1 | O | AN | 1/6
/// 05 | 22 | Commodity Code | 1 | O | AN | 1/30
/// 06 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 07 | 380 | Quantity | 1 | O | R | 1/15
/// 08 | 595 | Compartment ID Code | 1 | O | ID | 1/1
/// 09 | 665 | Residue Indicator Code | 1 | O | ID | 1/1
/// 10 | 254 | Packing Group Code | 1 | O | ID | 1/3
/// 11 | 1375 | Interim Hazardous Material Regulatory Number | 1 | O | AN | 1/5
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LH1 {
    pub _01: String,
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
}

/// LH2 - Hazardous Classification Information
/// 
/// To specify the hazardous notation and endorsement information
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 215 | Hazardous Classification | 1 | O | ID | 1/30
/// 02 | 983 | Hazardous Class Qualifier | 1 | O | ID | 1/1
/// 03 | 218 | Hazardous Placard Notation | 1 | O | ID | 14/40
/// 04 | 222 | Hazardous Endorsement | 1 | O | ID | 4/25
/// 05 | 759 | Reportable Quantity Code | 1 | O | ID | 2/2
/// 06 | 355 | Unit or Basis for Measurement Code | 1 | X/Z | ID | 2/2
/// 07 | 408 | Temperature | 1 | X | R | 1/4
/// 08 | 355 | Unit or Basis for Measurement Code NEW | 1 | X/Z | ID | 2/2
/// 09 | 408 | Temperature NEW | 1 | X | R | 1/4
/// 10 | 355 | Unit or Basis for Measurement Code NEW | 1 | X/Z | ID | 2/2
/// 11 | 408 | Temperature NEW | 1 | X | R | 1/4
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LH2 {
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
    pub _11: Option<String>,
}

/// LH3 - Hazardous Material Shipping Name
/// 
/// To specify the hazardous material shipping name and additional descriptive requirements
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 224 | Hazardous Material Shipping Name | 1 | X | AN | 1/25
/// 02 | 984 | Hazardous Material Shipping Name Qualifier | 1 | X | ID | 1/1
/// 03 | 985 | N.O.S. Indicator Code | 1 | O | ID | 3/3
/// 04 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LH3 {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
}

/// LH4 - Canadian Dangerous Requirements
/// 
/// To specify additional Transport Canada requirements covering transportation of dangerous goods in Canada
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// --- | --- | ---- | ------ | --- | ---- | -----
/// 01 | 238 | Emergency Response Plan Number | 1 | O | AN | 1/12
/// 02 | 364 | Communication Number | 1 | O | AN | 1/80
/// 03 | 254 | Packing Group Code | 1 | O | ID | 1/3
/// 04 | 230 | Subsidiary Classification | 1 | O | ID | 1/3
/// 05 | 230 | Subsidiary Classification | 1 | O | ID | 1/3
/// 06 | 230 | Subsidiary Classification | 1 | O | ID | 1/3
/// 07 | 271 | Subsidiary Risk Indicator | 1 | O | ID | 1/2
/// 08 | 267 | Net Explosive Quantity | 1 | X | N0 | 1/6
/// 09 | 805 | Canadian Hazardous Notation | 1 | O | AN | 1/25
/// 10 | 986 | Special Commodity Indicator Code | 1 | O | ID | 1/1
/// 11 | 364 | Communication Number | 1 | O/Z | AN | 1/80
/// 12 | 355 | Unit or Basis for Measurement Code NEW | 1 | X | ID | 2/2
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LH4 {
    _01: Option<String>,
    _02: Option<String>,
    _03: Option<String>,
    _04: Option<String>,
    _05: Option<String>,
    _06: Option<String>,
    _07: Option<String>,
    _08: Option<String>,
    _09: Option<String>,
    _10: Option<String>,
    _11: Option<String>,
    _12: Option<String>,
}

/// LH6 - Hazardous Certification
/// 
/// To specify the name of the person certifying that the shipment complies with the regulations and/or the actual certification
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 93 | Name | 1 | O | AN | 1/60
/// 02 | 272 | Hazardous Certification Code | 1 | X | ID | 1/1
/// 03 | 273 | Hazardous Certification Declaration | 1 | X | AN | 1/25
/// 04 | 273 | Hazardous Certification Declaration | 1 | O | AN | 1/25
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LH6 {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
}

/// LHR - Hazardous Material Identifying Reference Numbers
/// 
/// To transmit specific hazardous material reference numbers
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 128 | Reference Identification Qualifier | 1 | M | ID | 2/3
/// 02 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 03 | 373 | Date NEW | 1 | O | DT | 8/8
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LHR {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
}

/// LHT - Transborder Hazardous Requirements
/// 
/// To specify the placard information required by the second government agency when shipment is to cross into another country
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 215 | Hazardous Classification | 1 | O | ID | 1/30
/// 02 | 218 | Hazardous Placard Notation | 1 | O | ID | 14/40
/// 03 | 222 | Hazardous Endorsement | 1 | O | ID | 4/25
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LHT {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
}

/// LS - Loop Header
/// 
/// To indicate that the next segment begins a loop
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 447 | Loop Identifier Code | 1 | M | AN | 1/6
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LS {
    pub _01: String,
}

/// LX - Assigned Number
/// 
/// To reference a line number in a transaction set
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 554 | Assigned Number | 1 | M | N0 | 1/6
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LX {
    pub _01: String,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct M1 {
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
    pub _11: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct M3 {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct M7 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct M12 {
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct MEA {
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
}

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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct N1 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct N2 {
    pub _01: String,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct N3 {
    pub _01: String,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct N4 {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
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


/// N9 - Reference Identification
/// To transmit identifying information as specified by the Reference Identification Qualifier
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 128 | Reference Identification Qualifier | 1 | M | ID | 2/3
/// 02 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 03 | 369 | Free-form Description | 1 | X | AN | 1/45
/// 04 | 373 | Date | 1 | O | DT | 8/8
/// 05 | 337 | Time | 1 | X | TM | 4/8
/// 06 | 623 | Time Code | 1 | O/Z | ID | 2/2
/// 07 | C040 | Reference Identifier | 1 | O/Z
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct N9 {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _04: Option<String>,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
    pub _05: Option<String>,
    pub _06: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
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
    pub _11: Option<String>,
    pub _12: Option<String>,
    pub _13: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
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

/// PER - Administrative Communications Contact
/// 
/// To identify a person or office to whom administrative communications should be directed
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 366 | Contact Function Code | 1 | M | ID | 2/2
/// 02 | 93 | Name | 1 | O | AN | 1/60
/// 03 | 365 | Communication Number Qualifier | 1 | X | ID | 2/2
/// 04 | 364 | Communication Number | 1 | X | AN | 1/80
/// 05 | 365 | Communication Number Qualifier | 1 | X | ID | 2/2
/// 06 | 364 | Communication Number | 1 | X | AN | 1/80
/// 07 | 365 | Communication Number Qualifier | 1 | X | ID | 2/2
/// 08 | 364 | Communication Number | 1 | X | AN | 1/80
/// 09 | 443 | Contact Inquiry Reference | 1 | O | AN | 1/20
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PER {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
}

/// PI - Price Authority Identification
/// 
/// To communicate basis of pricing, such as contract number, quote number, or tariff number
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 128 | Reference Identification Qualifier | 1 | M | ID | 2/3
/// 02 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 03 | 436 | Primary Publication Authority Code | 1 | O | ID | 2/2
/// 04 | 930 | Regulatory Agency Code | 1 | O | ID | 3/5
/// 05 | 168 | Tariff Agency Code | 1 | O | ID | 1/4
/// 06 | 965 | Issuing Carrier Identifier | 1 | O | AN | 1/10
/// 07 | 660 | Contract Suffix | 1 | O/Z | AN | 1/2
/// 08 | 169 | Tariff Item Number | 1 | O/Z | AN | 1/16
/// 09 | 173 | Tariff Supplement Identifier | 1 | O/Z | AN | 1/4
/// 10 | 172 | Tariff Section | 1 | O/Z | AN | 1/2
/// 11 | 660 | Contract Suffix | 1 | O/Z | AN | 1/2
/// 12 | 373 | Date | 1 | X/Z | DT | 8/8
/// 13 | 373 | Date | 1 | X/Z | DT | 8/8
/// 14 | 629 | Alternation Precedence Code | 1 | O | ID | 1/1
/// 15 | 629 | Alternation Precedence Code | 1 | O | ID | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PI {
    pub _01: String,
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
}

/// PS - Protective Service Instructions
/// 
/// To specify mechanical protective service and ventilation instructions
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 746 | Protective Service Rule Code | 1 | M | ID | 3/9
/// 02 | 241 | Protective Service Code | 1 | M | ID | 1/4
/// 03 | 355 | Unit or Basis for Measurement Code | 1 | X/Z | ID | 2/2
/// 04 | 408 | Temperature | 1 | X/Z | R | 1/4
/// 05 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
/// 06 | 573 | Freight Station Accounting Code | 1 | O | ID | 1/5
/// 07 | 19 | City Name | 1 | O | AN | 2/30
/// 08 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 09 | 81 | Weight | 1 | O | R | 1/10
/// 10 | 745 | Pre-Cooled (Rule 710) Code | 1 | O | ID | 1/1
/// 11 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
/// 12 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
/// 13 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
/// 14 | 408 | Temperature | 1 | X/Z | R | 1/4
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PS {
    pub _01: String,
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
}

/// Q2 - Status Details (Ocean)
/// To transmit identifying information relative to identification of vessel, transportation dates, lading quantity, weight, and cube
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 597 | Vessel Code | 1 | O | ID | 1/8
/// 02 | 26 | Country Code | 1 | O/Z | ID | 2/3
/// 03 | 373 | Date | 1 | O/Z | DT | 8/8
/// 04 | 373 | Date | 1 | O/Z | DT | 8/8
/// 05 | 373 | Date | 1 | O/Z | DT | 8/8
/// 06 | 80 | Lading Quantity | 1 | O | N0 | 1/7
/// 07 | 81 | Weight | 1 | X | R | 1/10
/// 08 | 187 | Weight Qualifier | 1 | X | ID | 1/2
/// 09 | 55 | Flight/Voyage Number | 1 | O | AN | 2/10
/// 10 | 128 | Reference Identification Qualifier | 1 | O | ID | 2/3
/// 11 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 12 | 897 | Vessel Code Qualifier | 1 | O | ID | 1/1
/// 13 | 182 | Vessel Name | 1 | O | AN | 2/28
/// 14 | 183 | Volume | 1 | X | R | 1/8
/// 15 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 16 | 188 | Weight Unit Code | 1 | X | ID | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Q2 {
    pub _01: String,
    pub _02: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _03: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _04: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
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
}

/// R2 - Route Information
/// 
/// To specify carrier and routing sequences and details
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 140 | Standard Carrier Alpha Code | 1 | M | ID | 2/4
/// 02 | 133 | Routing Sequence Code | 1 | M | ID | 1/2
/// 03 | 19 | City Name | 1 | O/Z | AN | 2/30
/// 04 | 154 | Standard Point Location Code | 1 | O | ID | 6/9
/// 05 | 177 | Intermodal Service Code | 1 | O | ID | 1/2
/// 06 | 91 | Transportation Method/Type Code | 1 | O | ID | 1/2
/// 07 | 296 | Intermediate Switch Carrier | 1 | X | ID | 2/4
/// 08 | 296 | Intermediate Switch Carrier | 1 | O | ID | 2/4
/// 09 | 76 | Invoice Number | 1 | O | AN | 1/22
/// 10 | 373 | Date | 1 | O/Z | DT | 8/8
/// 11 | 369 | Free-form Description | 1 | O | AN | 1/45
/// 12 | 56 | Type of Service Code | 1 | O | ID | 2/2
/// 13 | 742 | Route Description | 1 | O | AN | 1/35
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct R2 {
    pub _01: String,
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
}


/// R4 - Port or Terminal
/// 
/// Contractual or operational port or point relevant to the movement of the cargo
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 115 | Port or Terminal Function Code | 1 | M | ID | 1/1
/// 02 | 309 | Location Qualifier | 1 | X | ID | 1/2
/// 03 | 310 | Location Identifier | 1 | X | AN | 1/30
/// 04 | 114 | Port Name | 1 | O | AN | 2/24
/// 05 | 26 | Country Code | 1 | O | ID | 2/3
/// 06 | 174 | Terminal Name | 1 | O | AN | 2/30
/// 07 | 113 | Pier Number | 1 | O | AN | 1/4
/// 08 | 156 | State or Province Code | 1 | O | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct R4 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
}

/// R9 - Route Code
/// 
/// To specify the route using a single code
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 1 | Route Code | 1 | M | AN | 1/13
/// 02 | 192 | Agent/Shipper Routing Code | 1 | O | ID | 1/1
/// 03 | 177 | Intermodal Service Code | 1 | O | ID | 1/2
/// 04 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 05 | 306 | Action Code | 1 | O | ID | 1/2
/// 06 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 07 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
/// 08 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct R9 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
}

/// REF - Reference Identification
/// 
/// To specify identifying information
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 128 | Reference Identification Qualifier | 1 | M | ID | 2/3
/// 02 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 03 | 352 | Description | 1 | X | AN | 1/80
/// 04 | C040 | Reference Identifier | 1 | O/Z	
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct REF {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
}

/// S1 - Stop-off Name
/// 
/// To identify a stop-off party
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 165 | Stop Sequence Number | 1 | M | N0 | 1/3
/// 02 | 459 | Name (30 Character Format) | 1 | M | AN | 2/30
/// 03 | 66 | Identification Code Qualifier | 1 | X | ID | 1/2
/// 04 | 67 | Identification Code | 1 | X | AN | 2/80
/// 05 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 06 | 190 | Accomplish Code | 1 | M | ID | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct S1 {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
}

/// S2 - Stop-off Address
/// 
/// To specify the address of the stop-off party
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 165 | Stop Sequence Number | 1 | M | N0 | 1/3
/// 02 | 166 | Address Information | 1 | M | AN | 1/55
/// 03 | 166 | Address Information | 1 | O | AN | 1/55
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct S2 {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
}

/// S9 - Stop-off Station
/// 
/// To specify location details for a stop-off
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 165 | Stop Sequence Number | 1 | M | N0 | 1/3
/// 02 | 154 | Standard Point Location Code | 1 | O | ID | 6/9
/// 03 | 19 | City Name | 1 | M | AN | 2/30
/// 04 | 156 | State or Province Code | 1 | M | ID | 2/2
/// 05 | 26 | Country Code | 1 | O | ID | 2/3
/// 06 | 163 | Stop Reason Code | 1 | M | ID | 2/2
/// 07 | 309 | Location Qualifier | 1 | X | ID | 1/2
/// 08 | 310 | Location Identifier | 1 | X | AN | 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct S9 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: String,
    pub _04: String,
    pub _05: Option<String>,
    pub _06: String,
    pub _07: Option<String>,
    pub _08: Option<String>,
}

/// SE - Transaction Set Trailer
///
/// To indicate the end of the transaction set and provide the count of the transmitted segments (including the beginning (ST) and ending (SE) segments)
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 96 | Number of Included Segments | 1 | M | N0 | 1/10
/// 02 | 329 | Transaction Set Control Number | 1 | M | AN | 4/9
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct SE {
    pub _01: String,
    pub _02: String,
}

/// SG - Shipment Status
///
/// To convey the status of a shipment
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 157 | Shipment Status Code | 1 | X | ID | 1/2
/// 02 | 641 | Status Reason Code | 1 | X | ID | 3/3
/// 03 | 35 | Disposition Code | 1 | X | ID | 2/2
/// 04 | 373 | Date | 1 | O | DT | 8/8
/// 05 | 337 | Time | 1 | X | TM | 4/8
/// 06 | 623 | Time Code | 1 | O | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct SG {
    pub _01: String,
    pub _02: String,
    pub _03: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _04: Option<String>,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
    pub _05: String,
    pub _06: Option<String>,
}

/// ST - Transaction Set Header
///
/// To indicate the start of a transaction set and to assign a control number
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 143 | Transaction Set Identifier Code | 1 | M/Z | ID | 3/3
/// 02 | 329 | Transaction Set Control Number | 1 | M | AN | 4/9
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ST {
    pub _01: String,
    pub _02: String,
}

/// T1 - Transit Inbound Origin
/// 
/// To specify origin point and waybill references of movement to transit waybill point
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 554 | Assigned Number | 1 | M | N0 | 1/6
/// 02 | 186 | Waybill Number | 1 | O/Z | N0 | 1/6
/// 03 | 373 | Date | 1 | O/Z | DT | 8/8
/// 04 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 05 | 19 | City Name | 1 | X/Z | AN | 2/30
/// 06 | 156 | State or Province Code | 1 | X | ID | 2/2
/// 07 | 154 | Standard Point Location Code | 1 | O | ID | 6/9
/// 08 | 229 | Transit Registration Number | 1 | O | AN | 1/6
/// 09 | 461 | Transit Level Code | 1 | O | ID | 1/3
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct T1 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
}

/// T2 - Transit Inbound Lading
/// 
/// To specify lading description, including weight and rate details applying to the associated T1 segment
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 554 | Assigned Number | 1 | M | N0 | 1/6
/// 02 | 79 | Lading Description | 1 | O | AN | 1/50
/// 03 | 81 | Weight | 1 | O/Z | R | 1/10
/// 04 | 187 | Weight Qualifier | 1 | O | ID | 1/2
/// 05 | 60 | Freight Rate | 1 | X | R | 1/9
/// 06 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 07 | 60 | Freight Rate | 1 | X | R | 1/9
/// 08 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 09 | 19 | City Name | 1 | O/Z | AN | 2/30
/// 10 | 19 | City Name | 1 | O/Z | AN | 2/30
/// 11 | 462 | Through Surcharge Percent | 1 | O | N2 | 2/4
/// 12 | 463 | Paid-In Surcharge Percent | 1 | O | N2 | 2/4
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct T2 {
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
    pub _12: Option<String>,
}

/// T3 - Transit Inbound Route
/// 
/// To specify transit inbound routing, including equipment identifications for associated T1 and T2 segments
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 554 | Assigned Number | 1 | M | N0 | 1/6
/// 02 | 140 | Standard Carrier Alpha Code | 1 | M | ID | 2/4
/// 03 | 133 | Routing Sequence Code | 1 | O | ID | 1/2
/// 04 | 19 | City Name | 1 | O | AN | 2/30
/// 05 | 154 | Standard Point Location Code | 1 | O | ID | 6/9
/// 06 | 206 | Equipment Initial | 1 | X/Z | AN | 1/4
/// 07 | 207 | Equipment Number | 1 | X/Z | AN | 1/10
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct T3 {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
}

/// T6 - Transit Inbound Rates
/// 
/// To identify the transit inbound prior origin point and waybill reference of movement to the point specified in T1 segment
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 554 | Assigned Number | 1 | M | N0 | 1/6
/// 02 | 60 | Freight Rate | 1 | X/Z | R | 1/9
/// 03 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 04 | 19 | City Name | 1 | O/Z | AN | 2/30
/// 05 | 60 | Freight Rate | 1 | X/Z | R | 1/9
/// 06 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 07 | 19 | City Name | 1 | O | AN | 2/30
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct T6 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
}

/// T8 - Free-form Transit Data
/// 
/// To transmit information in a free-form format relating to a specified transit sequence number
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 554 | Assigned Number | 1 | M | N0 | 1/6
/// 02 | 299 | Free-form Transit Data | 1 | M | AN | 1/80
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct T8 {
    pub _01: String,
    pub _02: String,
}

/// V9 - Event Detail
///
/// To specify information about a specific event
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 304 | Event Code | 1 | M | ID | 3/3
/// 02 | 106 | Event | 1 | O | AN | 1/25
/// 03 | 373 | Date | 1 | O/Z | DT | 8/8
/// 04 | 337 | Time | 1 | X/Z | TM | 4/8
/// 05 | 19 | City Name | 1 | O | AN | 2/30
/// 06 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 07 | 26 | Country Code | 1 | O | ID | 2/3
/// 08 | 641 | Status Reason Code | 1 | O | ID | 3/3
/// 09 | 154 | Standard Point Location Code | 1 | X/Z | ID | 6/9
/// 10 | 380 | Quantity | 1 | X/Z | R | 1/15
/// 11 | 1274 | Train Delay Reason Code | 1 | X | AN | 2/2
/// 12 | 61 | Free-Form Message | 1 | O | AN | 1/30
/// 13 | 623 | Time Code | 1 | O/Z | ID | 2/2
/// 14 | 380 | Quantity | 1 | O/Z | R | 1/15
/// 15 | 154 | Standard Point Location Code NEW | 1 | O/Z | ID | 6/9
/// 16 | 86 | Total Equipment NEW | 1 | O/Z | N0 | 1/3
/// 17 | 86 | Total Equipment NEW | 1 | O/Z | N0 | 1/3
/// 18 | 86 | Total Equipment NEW | 1 | O/Z | N0 | 1/3
/// 19 | 81 | Weight NEW | 1 | O/Z | R | 1/10
/// 20 | 82 | Length NEW | 1 | O/Z | R | 1/8
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct V9 {
    pub _01: String,
    pub _02: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _03: Option<String>,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
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
}

/// VC - Motor Vehicle Control
/// 
/// To define motor vehicle identification and logistics
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 539 | Vehicle Identification Number | 1 | M | AN | 1/25
/// 02 | 836 | Vehicle Deck Position Code | 1 | O | ID | 2/2
/// 03 | 837 | Vehicle Type Code | 1 | O | ID | 1/1
/// 04 | 838 | Dealer Code | 1 | O | AN | 2/9
/// 05 | 1 | Route Code | 1 | O/Z | AN | 1/13
/// 06 | 839 | Bay Location | 1 | O | AN | 1/6
/// 07 | 833 | Automotive Manufacturers Code | 1 | O | ID | 2/2
/// 08 | 308 | Damage Exception Indicator | 1 | O | ID | 1/1
/// 09 | 835 | Supplemental Inspection Code | 1 | O | ID | 1/1
/// 10 | 583 | Factory Car Order Number | 1 | O | AN | 6/10
/// 11 | 877 | Vessel Stowage Location | 1 | O | AN | 1/12
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VC {
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

/// X1 - Export License
/// 
/// To transmit information contained on an export license
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 83 | Licensing Agency Code | 1 | O | ID | 1/1
/// 02 | 50 | Export License Number | 1 | O | AN | 6/12
/// 03 | 51 | Export License Status Code | 1 | O | ID | 1/1
/// 04 | 373 | Date | 1 | O/Z | DT | 8/8
/// 05 | 52 | Export License Symbol Code | 1 | O | ID | 1/2
/// 06 | 48 | Export License Control Code | 1 | O | ID | 1/1
/// 07 | 26 | Country Code | 1 | O | ID | 2/3
/// 08 | 141 | Schedule B Code | 1 | O | ID | 7/10
/// 09 | 210 | International/Domestic Code | 1 | O | ID | 1/1
/// 10 | 80 | Lading Quantity | 1 | O | N0 | 1/7
/// 11 | 148 | Lading Value | 1 | O | R | 2/9
/// 12 | 47 | Export Filing Key Code | 1 | O | ID | 1/1
/// 13 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 14 | 212 | Unit Price | 1 | O | R | 1/17
/// 15 | 1306 | U.S. Government License Type | 1 | O | AN | 1/1
/// 16 | 67 | Identification Code NEW | 1 | O/Z | AN | 2/80
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct X1 {
    pub _01: String,
    pub _02: String,
    pub _03: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _04: Option<String>,
    pub _05: String,
    pub _06: String,
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
}

/// X7 - Customs Information
/// 
/// To indicate customs information
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 61 | Free-Form Message | 1 | M | AN | 1/30
/// 02 | 61 | Free-Form Message | 1 | O | AN | 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct X7 {
    pub _01: String,
    pub _02: Option<String>,
}

/// XH - Pro Forma - B13 Information
/// 
/// This segment is used to specify a pro forma invoice and B13 Canadian Customs Export Declaration information, required by U.S. and Canadian customs
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 100 | Currency Code | 1 | M | ID | 3/3
/// 02 | 645 | Related Company Indication Code | 1 | O | ID | 1/1
/// 03 | 150 | Special Charge or Allowance Code | 1 | O | ID | 3/3
/// 04 | 610 | Amount | 1 | O/Z | N2 | 1/15
/// 05 | 503 | Block 20 Code | 1 | O | ID | 1/1
/// 06 | 504 | Chemical Analysis Percentage | 1 | O/Z | N2 | 2/9
/// 07 | 212 | Unit Price | 1 | O/Z | R | 1/17
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct XH {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
}

/// ZC1 - Beginning Segment For Data Correction Or Change
/// 
/// To transmit identifying numbers, dates, and other basic data relating to the transaction set
/// 
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 145 | Shipment Identification Number | 1 | O | AN | 1/30
/// 02 | 206 | Equipment Initial | 1 | O | AN | 1/4
/// 03 | 207 | Equipment Number | 1 | M | AN | 1/10
/// 04 | 244 | Transaction Reference Number | 1 | M | AN | 1/15
/// 05 | 243 | Transaction Reference Date | 1 | M | DT | 8/8
/// 06 | 202 | Correction Indicator | 1 | M | ID | 2/2
/// 07 | 140 | Standard Carrier Alpha Code | 1 | M | ID | 2/4
/// 08 | 91 | Transportation Method/Type Code | 1 | M/Z | ID | 1/2
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ZC1 {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: String,
    pub _04: String,
    /// 243 - Transaction Reference Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _05: String,
    pub _06: String,
    pub _07: String,
    pub _08: Option<String>,
}

/// ZD - Transaction Set Deletion - ID, Reason, and Source
///
/// This segment is used to specify the transaction set to be canceled
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 143 | Transaction Set Identifier Code | 1 | M | ID | 3/3
/// 02 | 145 | Shipment Identification Number | 1 | O | AN | 1/30
/// 03 | 206 | Equipment Initial | 1 | M | AN | 1/4
/// 04 | 207 | Equipment Number | 1 | M | AN | 1/15
/// 05 | 244 | Transaction Reference Number | 1 | O | AN | 1/50
/// 06 | 243 | Transaction Reference Date | 1 | O | DT | 8/8
/// 07 | 202 | Correction Indicator Code | 1 | M | ID | 2/2
/// 08 | 140 | Standard Carrier Alpha Code | 1 | O | ID | 2/4
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ZD {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: String,
    pub _04: String,
    pub _05: Option<String>,
    /// 243 - Transaction Reference Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _06: Option<String>,
    pub _07: String,
    pub _08: Option<String>,
}
