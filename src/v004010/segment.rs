use serde::{Serialize,Deserialize};

/// B4 - Beginning Segment for Inquiry or Reply
/// 
/// To transmit identifying numbers, dates, and other basic data relating to the transaction set
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
#[derive(Serialize,Deserialize)]
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


/// DTM - Date/Time Reference
/// To specify pertinent dates and times
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 374 | Date/Time Qualifier | 1 | M | ID | 3/3
/// 02 | 373 | Date | 1 | X | DT | 8/8
/// 03 | 337 | Time | 1 | X | TM | 4/8
/// 04 | 623 | Time Code | 1 | O | ID | 2/2
/// 05 | 1250 | Date Time Period Format Qualifier | 1 | X | ID | 2/3
/// 06 | 1251 | Date Time Period | 1 | X | AN | 1/35
#[derive(Serialize,Deserialize)]
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


/// GE - Functional Group Trailer
/// To indicate the end of a functional group and to provide control information
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 97 | Number of Transaction Sets Included | 1 | M | N0 | 1/6
/// 02 | 28 | Group Control Number | 1 | M/Z | N0 | 1/9
#[derive(Serialize,Deserialize)]
pub struct GE {
    pub _01: String,
    pub _02: String,
}

/// GS - Functional Group Header
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
#[derive(Serialize,Deserialize)]
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

/// IEA - Interchange Control Trailer
/// 
/// To define the end of an interchange of zero or more functional groups and interchange-related control segments
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | I16 | Number of Included Functional Groups | 1 | M | N0 | 1/5
/// 02 | I12 | Interchange Control Number | 1 | M | N0 | 9/9
#[derive(Serialize,Deserialize)]
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
#[derive(Serialize, Deserialize, Default)]
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
#[derive(Serialize,Deserialize)]
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
#[derive(Serialize,Deserialize)]
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

/// R4 - Port or Terminal
/// Contractual or operational port or point relevant to the movement of the cargo
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
#[derive(Serialize,Deserialize)]
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

/// SE - Transaction Set Trailer
/// 
/// To indicate the end of the transaction set and provide the count of the transmitted segments (including the beginning (ST) and ending (SE) segments)
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 96 | Number of Included Segments | 1 | M | N0 | 1/10
/// 02 | 329 | Transaction Set Control Number | 1 | M | AN | 4/9
#[derive(Serialize,Deserialize)]
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
#[derive(Serialize,Deserialize)]
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
#[derive(Serialize,Deserialize)]
pub struct ST {
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
#[derive(Serialize,Deserialize)]
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
