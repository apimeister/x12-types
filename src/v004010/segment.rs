use crate::util::Parser;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while;
use nom::character::complete::newline;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplaySegment;
use validator::Validate;

/// AK1 - Functional Group Response Header
///
/// To start acknowledgment of a functional group
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 479 | Functional Identifier Code | 1 | M/Z | ID | 2/2
/// 02 | 28 | Group Control Number | 1 | M/Z | N0 | 1/9
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct AK1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct AK2 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct AK3 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct AK4 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct AK9 {
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
}

/// AT5 - Bill of Lading Handling Requirements
///
/// To identify Bill of Lading handling and service requirements
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 152 | Special Handling Code | 1 | X | ID | 2/3
/// 02 | 560 | Special Services Code | 1 | X | ID | 2/10
/// 03 | 153 | Special Handling Description | 1 | X | AN | 2/30
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct AT5 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, AT5, nom::error::Error<&'a str>> for AT5 {
    fn parse(input: &'a str) -> IResult<&'a str, AT5> {
        let (rest, vars) = delimited(tag("AT5*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = AT5 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// AT7 - Shipment Status Details
///
/// To specify the status of a shipment, the reason for that status, the date and time of the status and the date and time of any appointments scheduled.
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 1650 | Shipment Status Code | 1 | X/Z | ID | 2/2
/// 02 | 1651 | Shipment Status or Appointment Reason Code | 1 | X | ID | 2/2
/// 03 | 1652 | Shipment Appointment Status Code | 1 | X | ID | 2/2
/// 04 | 1651 | Shipment Status or Appointment Reason Code | 1 | X | ID | 2/2
/// 05 | 373 | Date | 1 | X | DT | 8/8
/// 06 | 337 | Time | 1 | X | TM | 4/8
/// 07 | 623 | Time Code | 1 | O/Z | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct AT7 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 623 - Time Code
    ///
    /// Code identifying the time. In accordance with International Standards Organization standard 8601, time can be specified by a + or - and an indication in hours in relation to Universal Time Coordinate (UTC) time; since + is a restricted character, + and - are substituted by P and M in the codes that follow
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "07")]
    pub _07: Option<String>,
}

/// AT8 - Shipment Weight, Packaging and Quantity Data
///
/// To specify shipment details in terms of weight, and quantity of handling units
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 187 | Weight Qualifier | 1 | X | ID | 1/2
/// 02 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 03 | 81 | Weight | 1 | X | R | 1/10
/// 04 | 80 | Lading Quantity | 1 | O/Z | N0 | 1/7
/// 05 | 80 | Lading Quantity | 1 | O/Z | N0 | 1/7
/// 06 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 07 | 183 | Volume | 1 | X | R | 1/8
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct AT8 {
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

impl<'a> Parser<&'a str, AT8, nom::error::Error<&'a str>> for AT8 {
    fn parse(input: &'a str) -> IResult<&'a str, AT8> {
        let (rest, vars) = delimited(tag("AT8*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = AT8 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// B1 - Beginning Segment for Booking or Pick-up/Delivery
///
/// To transmit identifying number, data, and other basic data relating to the transaction set
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct B1 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: String,
}

impl<'a> Parser<&'a str, B1, nom::error::Error<&'a str>> for B1 {
    fn parse(input: &'a str) -> IResult<&'a str, B1> {
        let (rest, vars) = delimited(tag("B1*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = B1 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).unwrap().to_string(),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// B2 - Beginning Segment for Shipment Information Transaction
///
/// To transmit basic data relating to shipment information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 375 | Tariff Service Code | 1 | O | ID | 2/2
/// 02 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 03 | 154 | Standard Point Location Code | 1 | O | ID | 6/9
/// 04 | 145 | Shipment Identification Number | 1 | O | AN | 1/30
/// 05 | 188 | Weight Unit Code | 1 | O | ID | 1/1
/// 06 | 146 | Shipment Method of Payment | 1 | M | ID | 2/2
/// 07 | 147 | Shipment Qualifier | 1 | O | ID | 1/1
/// 08 | 86 | Total Equipment | 1 | O | N0 | 1/3
/// 09 | 460 | Shipment Weight Code | 1 | O | ID | 1/1
/// 10 | 501 | Customs Documentation Handling Code | 1 | O | ID | 2/2
/// 11 | 335 | Transportation Terms Code | 1 | O/Z | ID | 3/3
/// 12 | 591 | Payment Method Code | 1 | O | ID | 3/3
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct B2 {
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
}

impl<'a> Parser<&'a str, B2, nom::error::Error<&'a str>> for B2 {
    fn parse(input: &'a str) -> IResult<&'a str, B2> {
        let (rest, vars) = delimited(tag("B2*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = B2 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).unwrap().to_string(),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// B2A - Set Purpose
///
/// To allow for positive identification of transaction set purpose
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 353 | Transaction Set Purpose Code | 1 | M | ID | 2/2
/// 02 | 346 | Application Type | 1 | O | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct B2A {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

impl<'a> Parser<&'a str, B2A, nom::error::Error<&'a str>> for B2A {
    fn parse(input: &'a str) -> IResult<&'a str, B2A> {
        let (rest, vars) = delimited(tag("B2A*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = B2A {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// B3 - Beginning Segment for Carrier's Invoice
///
/// To transmit basic data relating to the carrier's invoice
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 147 | Shipment Qualifier | O |  | ID 1/1
/// 02 | 76 | Invoice Number | M |  | AN 1/22
/// 03 | 145 | Shipment Identification Number | O |  | AN 1/30
/// 04 | 146 | Shipment Method of Payment | M |  | ID 2/2
/// 05 | 188 | Weight Unit Code | O |  | ID 1/1
/// 06 | 373 | Date | M |  | DT 8/8
/// 07 | 193 | Net Amount Due | M |  | N2 1/12
/// 08 | 202 | Correction Indicator | O |  | ID 2/2
/// 09 | 32 | Delivery Date | X |  | DT 8/8
/// 10 | 374 | Date/Time Qualifier | X |  | ID 3/3
/// 11 | 140 | Standard Carrier Alpha Code | M |  | ID 2/4
/// 12 | 373 | Date | O |  | DT 8/8
/// 13 | 375 | Tariff Service Code | O |  | ID 2/2
/// 14 | 335 | Transportation Terms Code | O |  | ID 3/3
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct B3 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: String,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: String,
    #[serde(rename = "07")]
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: String,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
}

impl<'a> Parser<&'a str, B3, nom::error::Error<&'a str>> for B3 {
    fn parse(input: &'a str) -> IResult<&'a str, B3> {
        let (rest, vars) = delimited(tag("B3*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = B3 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).unwrap().to_string(),
            _07: vars.get(6).unwrap().to_string(),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).unwrap().to_string(),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct B4 {
    /// 152 - Special Handling Code
    ///
    /// Code specifying special transportation handling instructions
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "01", skip_serializing_if = "Option::is_none")]
    pub _01: Option<String>,
    #[serde(rename = "02", skip_serializing_if = "Option::is_none")]
    pub _02: Option<String>,
    #[serde(rename = "03", skip_serializing_if = "Option::is_none")]
    pub _03: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "04", skip_serializing_if = "Option::is_none")]
    pub _04: Option<String>,
    #[serde(rename = "05", skip_serializing_if = "Option::is_none")]
    pub _05: Option<String>,
    #[serde(rename = "06", skip_serializing_if = "Option::is_none")]
    pub _06: Option<String>,
    #[serde(rename = "07", skip_serializing_if = "Option::is_none")]
    pub _07: Option<String>,
    #[serde(rename = "08", skip_serializing_if = "Option::is_none")]
    pub _08: Option<String>,
    #[serde(rename = "09", skip_serializing_if = "Option::is_none")]
    pub _09: Option<String>,
    #[serde(rename = "10", skip_serializing_if = "Option::is_none")]
    pub _10: Option<String>,
    #[serde(rename = "11", skip_serializing_if = "Option::is_none")]
    pub _11: Option<String>,
    #[serde(rename = "12", skip_serializing_if = "Option::is_none")]
    pub _12: Option<String>,
    #[serde(rename = "13", skip_serializing_if = "Option::is_none")]
    pub _13: Option<String>,
}

impl<'a> Parser<&'a str, B4, nom::error::Error<&'a str>> for B4 {
    fn parse(input: &'a str) -> IResult<&'a str, B4> {
        let (rest, vars) = crate::util::parse_line(input, "B4")?;
        let obj = B4 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// B10 - Beginning Segment for Transportation Carrier Shipment Status Message
///
/// To transmit identifying numbers and other basic data relating to the transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | X/Z | AN | 1/30
/// 02 | 145 | Shipment Identification Number | 1 | O | AN | 1/30
/// 03 | 140 | Standard Carrier Alpha Code | 1 | M | ID | 2/4
/// 04 | 71 | Inquiry Request Number | 1 | O | N0 | 1/3
/// 05 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 06 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 07 | 1073 | Yes/No Condition or Response Code NEW | 1 | O/Z | ID | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct B10 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
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

/// BIN - Binary Data
///
/// To transfer binary data in a single data segment and allow identification of the end of the data segment through a count; there is no identification of the internal structure of the binary data in this segment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 784 | Length of Binary Data | 1 | M | N0 | 1/15
/// 02 | 785 | Binary Data | 1 | M | B | 1/9999999999999999
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct BIN {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct BL {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
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

impl<'a> Parser<&'a str, BL, nom::error::Error<&'a str>> for BL {
    fn parse(input: &'a str) -> IResult<&'a str, BL> {
        let (rest, vars) = crate::util::parse_line(input, "BL")?;
        let obj = BL {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
            _16: vars.get(15).map(|x| x.to_string()),
            _17: vars.get(15).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct BNX {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, BNX, nom::error::Error<&'a str>> for BNX {
    fn parse(input: &'a str) -> IResult<&'a str, BNX> {
        let (rest, vars) = crate::util::parse_line(input, "BNX")?;
        let obj = BNX {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct BX {
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
}

impl<'a> Parser<&'a str, BX, nom::error::Error<&'a str>> for BX {
    fn parse(input: &'a str) -> IResult<&'a str, BX> {
        let (rest, vars) = crate::util::parse_line(input, "BX")?;
        let obj = BX {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// C2 - Bank ID
///
/// To specify data required for electronic payment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 8 | Bank Client Code | M |  | ID 1/1
/// 02 | 66 | Identification Code Qualifier | M |  | ID 1/2
/// 03 | 67 | Identification Code | M |  | AN 2/80
/// 04 | 20 | Client Bank Number | O |  | AN 3/9
/// 05 | 7 | Bank Account Number | O |  | AN 6/17
/// 06 | 107 | Payment Method Code | O |  | ID 1/2
/// 07 | 373 | Date | O |  | DT 8/8
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, C2, nom::error::Error<&'a str>> for C2 {
    fn parse(input: &'a str) -> IResult<&'a str, C2> {
        let (rest, vars) = delimited(tag("C2*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = C2 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// C3 - Currency
///
/// To specify the currency being used in the transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 100 | Currency Code | M |  | ID 3/3
/// 02 | 280 | Exchange Rate | O |  | R 4/10
/// 03 | 100 | Currency Code | O |  | ID 3/3
/// 04 | 100 | Currency Code | O |  | ID 3/3
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, C3, nom::error::Error<&'a str>> for C3 {
    fn parse(input: &'a str) -> IResult<&'a str, C3> {
        let (rest, vars) = delimited(tag("C3*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = C3 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// C8 - Certifications and Clauses
///
/// To specify applicable certifications and clauses
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 213 | Lading Line Item Number | O |  | N0 1/3
/// 02 | 246 | Certification/Clause Code | X |  | ID 2/4
/// 03 | 247 | Certification/Clause Text | X |  | AN 2/60
/// 04 | 1302 | Shipper's Export Declaration Requirements | O |  | AN 1/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, C8, nom::error::Error<&'a str>> for C8 {
    fn parse(input: &'a str) -> IResult<&'a str, C8> {
        let (rest, vars) = delimited(tag("C8*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = C8 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// C8C - Certifications Clauses Continuation
///
/// To specify additional applicable certifications and clauses
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 247 | Certification/Clause Text | M |  | AN 2/60
/// 02 | 247 | Certification/Clause Text | O |  | AN 2/60
/// 03 | 247 | Certification/Clause Text | O |  | AN 2/60
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct C8C {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, C8C, nom::error::Error<&'a str>> for C8C {
    fn parse(input: &'a str) -> IResult<&'a str, C8C> {
        let (rest, vars) = delimited(tag("C8C*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = C8C {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// CD3 - Carton (Package) Detail
///
/// To transmit identifying codes, weights, and other related information related to an individual carton (package)
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 187 | Weight Qualifier | 1 | X | ID | 1/2
/// 02 | 81 | Weight | 1 | X | R | 1/10
/// 03 | 619 | Zone | 1 | O | AN | 2/3
/// 04 | 34 | Service Standard | 1 | O | N1 | 1/4
/// 05 | 284 | Service Level Code | 1 | X | ID | 2/2
/// 06 | 108 | Pick-up or Delivery Code | 1 | O | ID | 1/2
/// 07 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 08 | 58 | Charge | 1 | X/Z | N2 | 1/12
/// 09 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 10 | 58 | Charge | 1 | X/Z | N2 | 1/12
/// 11 | 284 | Service Level Code | 1 | X | ID | 2/2
/// 12 | 284 | Service Level Code | 1 | O | ID | 2/2
/// 13 | 591 | Payment Method Code | 1 | O | ID | 3/3
/// 14 | 26 | Country Code | 1 | O/Z | ID | 2/3
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct CM {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
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
    #[serde(rename = "15")]
    pub _15: Option<String>,
    #[serde(rename = "16")]
    pub _16: Option<String>,
    #[serde(rename = "17")]
    pub _17: Option<String>,
}

impl<'a> Parser<&'a str, CM, nom::error::Error<&'a str>> for CM {
    fn parse(input: &'a str) -> IResult<&'a str, CM> {
        let (rest, vars) = crate::util::parse_line(input, "CM")?;
        let obj = CM {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
            _16: vars.get(15).map(|x| x.to_string()),
            _17: vars.get(16).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct D9 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
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
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "12")]
    pub _12: Option<String>,
}

impl<'a> Parser<&'a str, D9, nom::error::Error<&'a str>> for D9 {
    fn parse(input: &'a str) -> IResult<&'a str, D9> {
        let (rest, vars) = crate::util::parse_line(input, "D9")?;
        let obj = D9 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct DTM {
    /// 374 - Date/Time Qualifier
    ///
    /// Code specifying type of date or time, or both date and time
    /// - TYPE=ID
    /// - MIN=3
    /// - MAX=3
    #[serde(rename = "01")]
    pub _01: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
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
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

impl<'a> Parser<&'a str, DTM, nom::error::Error<&'a str>> for DTM {
    fn parse(input: &'a str) -> IResult<&'a str, DTM> {
        let (rest, vars) = delimited(tag("DTM*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = DTM {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct E1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct E4 {
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "04")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct E5 {
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
}

/// EA - Equipment Attributes
///
/// To specify attributes required for a piece of equipment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct EA {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, EA, nom::error::Error<&'a str>> for EA {
    fn parse(input: &'a str) -> IResult<&'a str, EA> {
        let (rest, vars) = crate::util::parse_line(input, "EA")?;
        let obj = EA {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// EFI - Electronic Format Identification
///
/// To provide basic information about the electronic format of the interchange data
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 786 | Security Level Code | 1 | M | ID | 2/2
/// 02 | 933 | Free-Form Message Text | 1 | O | AN | 1/264
/// 03 | 797 | Security Technique Code | 1 | O | ID | 2/2
/// 04 | 799 | Version Identifier | 1 | X | AN | 1/30
/// 05 | 802 | Program Identifier | 1 | O | AN | 1/30
/// 06 | 799 | Version Identifier | 1 | X | AN | 1/30
/// 07 | 801 | Interchange Format | 1 | O | AN | 1/30
/// 08 | 799 | Version Identifier | 1 | X | AN | 1/30
/// 09 | 800 | Compression Technique | 1 | O | AN | 1/30
/// 10 | 789 | Drawing Sheet Size Code | 1 | O | AN | 2/2
/// 11 | 803 | File Name | 1 | O | AN | 1/64
/// 12 | 804 | Block Type | 1 | O | AN | 1/4
/// 13 | 787 | Record Length | 1 | O | N | 1/15
/// 14 | 788 | Block Length | 1 | O | N | 1/5
/// 15 | 799 | Version Identifier | 1 | X | AN | 1/30
/// 16 | 1570 | Filter ID Code | 1 | X | ID | 3/3
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct EFI {
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct EM {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "07")]
    pub _07: Option<String>,
}

impl<'a> Parser<&'a str, EM, nom::error::Error<&'a str>> for EM {
    fn parse(input: &'a str) -> IResult<&'a str, EM> {
        let (rest, vars) = crate::util::parse_line(input, "EM")?;
        let obj = EM {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct F9 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
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
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "12")]
    pub _12: Option<String>,
}

impl<'a> Parser<&'a str, F9, nom::error::Error<&'a str>> for F9 {
    fn parse(input: &'a str) -> IResult<&'a str, F9> {
        let (rest, vars) = crate::util::parse_line(input, "F9")?;
        let obj = F9 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// G3 - Compensation Information
///
/// To convey brokerage, freight forwarder compensation, and other compensation information related to shipments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 315 | Compensation Paid | O |  | R 2/5
/// 02 | 317 | Total Compensation Amount | M |  | N0 3/10
/// 03 | 93 | Name | O |  | AN 1/60
/// 04 | 201 | Business Transaction Status | O |  | ID 1/3
/// 05 | 782 | Monetary Amount | O |  | R 1/18
/// 06 | 73 | Compensation Qualifier | O |  | ID 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct G3 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
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

impl<'a> Parser<&'a str, G3, nom::error::Error<&'a str>> for G3 {
    fn parse(input: &'a str) -> IResult<&'a str, G3> {
        let (rest, vars) = crate::util::parse_line(input, "G3")?;
        let obj = G3 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// G61 - Contact
///
/// To identify a person or office to whom communications should be directed
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 366 | Contact Function Code | 1 | M | ID | 2/2
/// 02 | 93 | Name | 1 | M | AN | 1/60
/// 03 | 365 | Communication Number Qualifier | 1 | X | ID | 2/2
/// 04 | 364 | Communication Number | 1 | X | AN | 1/80
/// 05 | 443 | Contact Inquiry Reference | 1 | O | AN | 1/20
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct G61 {
    #[serde(rename = "01")]
    pub _01: String,
    /// 93 - Name
    ///
    /// Free-form name
    /// - TYPE=AN
    /// - MIN=1
    /// - MAX=60
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

impl<'a> Parser<&'a str, G61, nom::error::Error<&'a str>> for G61 {
    fn parse(input: &'a str) -> IResult<&'a str, G61> {
        let (rest, vars) = crate::util::parse_line(input, "G61")?;
        let obj = G61 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// G62 - Date/Time
///
/// To specify pertinent dates and times
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 432 | Date Qualifier | 1 | X | ID | 2/2
/// 02 | 373 | Date | 1 | X | DT | 8/8
/// 03 | 176 | Time Qualifier | 1 | X | ID | 1/2
/// 04 | 337 | Time | 1 | X | TM | 4/8
/// 05 | 623 | Time Code | 1 | O | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct G62 {
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
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 623 - Time Code
    ///
    /// Code identifying the time. In accordance with International Standards Organization standard 8601, time can be specified by a + or - and an indication in hours in relation to Universal Time Coordinate (UTC) time; since + is a restricted character, + and - are substituted by P and M in the codes that follow
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "05")]
    pub _05: Option<String>,
}

impl<'a> Parser<&'a str, G62, nom::error::Error<&'a str>> for G62 {
    fn parse(input: &'a str) -> IResult<&'a str, G62> {
        let (rest, vars) = crate::util::parse_line(input, "G62")?;
        let obj = G62 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct GA {
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
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
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

impl<'a> Parser<&'a str, GA, nom::error::Error<&'a str>> for GA {
    fn parse(input: &'a str) -> IResult<&'a str, GA> {
        let (rest, vars) = crate::util::parse_line(input, "GA")?;
        let obj = GA {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
            _16: vars.get(15).map(|x| x.to_string()),
            _17: vars.get(16).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// GE - Functional Group Trailer
///
/// To indicate the end of a functional group and to provide control information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 97 | Number of Transaction Sets Included | 1 | M | N0 | 1/6
/// 02 | 28 | Group Control Number | 1 | M/Z | N0 | 1/9
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct GE {
    /// 97 - Number of Transaction Sets Included
    ///
    /// Total number of transaction sets included in the functional group or interchange (transmission) group terminated by the trailer containing this data element
    /// - TYPE=N0
    /// - MIN=1
    /// - MAX=6
    #[serde(rename = "01")]
    pub _01: String,
    /// 28 - Group Control Number
    ///
    /// Assigned number originated and maintained by the sender
    /// - TYPE=N0
    /// - MIN=1
    /// - MAX=9
    #[serde(rename = "02")]
    pub _02: String,
}

impl<'a> Parser<&'a str, GE, nom::error::Error<&'a str>> for GE {
    fn parse(input: &'a str) -> IResult<&'a str, GE> {
        let (rest, vars) = crate::util::parse_line(input, "GE")?;
        let obj = GE {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// GR5 - Loading Details
///
/// To provide loading details for equipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 152 | Special Handling Code | 1 | M | ID | 2/3
/// 02 | 752 | Surface/Layer/Position Code | 1 | X | ID | 2/2
/// 03 | 739 | Measurement Value | 1 | X | R | 1/20
/// 04 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 05 | 641 | Status Reason Code | 1 | O | ID | 3/3
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct GR5 {
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

/// GS - Functional Group Header
///
/// To indicate the beginning of a functional group and to provide control information
///
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct GS {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "04")]
    pub _04: String,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
    #[serde(rename = "05")]
    pub _05: String,
    #[serde(rename = "06")]
    pub _06: String,
    #[serde(rename = "07")]
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: String,
}

impl<'a> Parser<&'a str, GS, nom::error::Error<&'a str>> for GS {
    fn parse(input: &'a str) -> IResult<&'a str, GS> {
        let (rest, vars) = crate::util::parse_line(input, "GS")?;
        let obj = GS {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).unwrap().to_string(),
            _06: vars.get(5).unwrap().to_string(),
            _07: vars.get(6).unwrap().to_string(),
            _08: vars.get(7).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, H1, nom::error::Error<&'a str>> for H1 {
    fn parse(input: &'a str) -> IResult<&'a str, H1> {
        let (rest, vars) = crate::util::parse_line(input, "H1")?;
        let obj = H1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// H2 - Additional Hazardous Material Description
///
/// To specify free-form hazardous material descriptive data in addition to the information provided in the H1 segment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 64 | Hazardous Material Description | M |  | AN 2/30
/// 02 | 274 | Hazardous Material Classification | O |  | AN 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct H2 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

impl<'a> Parser<&'a str, H2, nom::error::Error<&'a str>> for H2 {
    fn parse(input: &'a str) -> IResult<&'a str, H2> {
        let (rest, vars) = crate::util::parse_line(input, "H2")?;
        let obj = H2 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, H3, nom::error::Error<&'a str>> for H3 {
    fn parse(input: &'a str) -> IResult<&'a str, H3> {
        let (rest, vars) = crate::util::parse_line(input, "H3")?;
        let obj = H3 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct IC {
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
}

impl<'a> Parser<&'a str, IC, nom::error::Error<&'a str>> for IC {
    fn parse(input: &'a str) -> IResult<&'a str, IC> {
        let (rest, vars) = crate::util::parse_line(input, "IC")?;
        let obj = IC {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct IM {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, IM, nom::error::Error<&'a str>> for IM {
    fn parse(input: &'a str) -> IResult<&'a str, IM> {
        let (rest, vars) = crate::util::parse_line(input, "IM")?;
        let obj = IM {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// IEA - Interchange Control Trailer
///
/// To define the end of an interchange of zero or more functional groups and interchange-related control segments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | I16 | Number of Included Functional Groups | 1 | M | N0 | 1/5
/// 02 | I12 | Interchange Control Number | 1 | M | N0 | 9/9
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct IEA {
    /// I16 - Number of Included Functional Groups
    ///
    /// A count of the number of functional groups included in an interchange
    /// - TYPE=N0
    /// - MIN=1
    /// - MAX=5
    #[serde(rename = "01")]
    pub _01: String,
    /// I12 - Interchange Control Number
    ///
    /// A control number assigned by the interchange sender
    /// - TYPE=N0
    /// - MIN=9
    /// - MAX=9
    #[serde(rename = "02")]
    pub _02: String,
}

impl<'a> Parser<&'a str, IEA, nom::error::Error<&'a str>> for IEA {
    fn parse(input: &'a str) -> IResult<&'a str, IEA> {
        let (rest, vars) = crate::util::parse_line(input, "IEA")?;
        let obj = IEA {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// ISA - Interchange Control Header
///
/// To start and identify an interchange of zero or more functional groups and interchange-related control segments
///
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct ISA {
    /// I01 - Authorization Information Qualifier
    ///
    /// Code to identify the type of information in the Authorization Information
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[validate(length(equal = 2))]
    #[serde(rename = "01")]
    pub _01: String,
    /// I02 - Authorization Information
    ///
    /// Information used for additional identification or authorization of the interchange sender or the data in the interchange; the type of information is set by the Authorization Information Qualifier (I01)
    /// - TYPE=AN
    /// - MIN=10
    /// - MAX=10
    #[validate(length(equal = 10, message = "I04 must be 10 characters long"))]
    #[serde(rename = "02")]
    pub _02: String,
    /// I03 - Security Information Qualifier
    ///
    /// Code to identify the type of information in the Security Information
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[validate(length(equal = 2))]
    #[serde(rename = "03")]
    pub _03: String,
    /// I04 - Security Information
    ///
    /// This is used for identifying the security information about the interchange sender or the data in the interchange; the type of information is set by the Security Information Qualifier (I03)
    /// - TYPE=AN
    /// - MIN=10
    /// - MAX=10
    #[validate(length(equal = 10, message = "I04 must be 10 characters long"))]
    #[serde(rename = "04")]
    pub _04: String,
    /// I05 - Interchange ID Qualifier
    ///
    /// Qualifier to designate the system/method of code structure used to designate the sender or receiver ID element being qualified
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "05")]
    pub _05: String,
    /// I06 - Interchange Sender ID
    ///
    /// Identification code published by the sender for other parties to use as the receiver ID to route data to them; the sender always codes this value in the sender ID element
    /// - TYPE=AN
    /// - MIN=15
    /// - MAX=15
    #[serde(rename = "06")]
    pub _06: String,
    /// I05 - Interchange ID Qualifier
    ///
    /// Qualifier to designate the system/method of code structure used to designate the sender or receiver ID element being qualified
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "07")]
    pub _07: String,
    /// I07 - Interchange Receiver ID
    ///
    /// Identification code published by the receiver of the data; When sending, it is used by the sender as their sending ID, thus other parties sending to them will use this as a receiving ID to route data to them
    /// - TYPE=AN
    /// - MIN=15
    /// - MAX=15
    #[serde(rename = "08")]
    pub _08: String,
    /// I08 - Interchange Date
    ///
    /// Date of the interchange
    /// - TYPE=DT
    /// - MIN=6
    /// - MAX=6
    #[serde(rename = "09")]
    pub _09: String,
    /// I09 - Interchange Time
    ///
    /// Time of the interchange
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=4
    #[serde(rename = "10")]
    pub _10: String,
    /// I10 - Interchange Control Standards Identifier
    ///
    /// Code to identify the agency responsible for the control standard used by the message that is enclosed by the interchange header and trailer
    /// - TYPE=ID
    /// - MIN=1
    /// - MAX=1
    #[serde(rename = "11")]
    pub _11: String,
    /// I11 - Interchange Control Version Number
    ///
    /// This version number covers the interchange control segments
    /// - TYPE=ID
    /// - MIN=5
    /// - MAX=5
    #[serde(rename = "12")]
    pub _12: String,
    /// I12 - Interchange Control Number
    ///
    /// A control number assigned by the interchange sender
    /// - TYPE=N0
    /// - MIN=9
    /// - MAX=9
    #[serde(rename = "13")]
    pub _13: String,
    /// I13 - Acknowledgment Requested
    ///
    /// Code sent by the sender to request an interchange acknowledgment (TA1)
    /// - TYPE=ID
    /// - MIN=1
    /// - MAX=1
    #[serde(rename = "14")]
    pub _14: String,
    /// I14 - Usage Indicator
    ///
    /// Code to indicate whether data enclosed by this interchange envelope is test, production or information
    /// - TYPE=ID
    /// - MIN=1
    /// - MAX=1
    #[serde(rename = "15")]
    pub _15: String,
    /// I15 - Component Element Separator
    ///
    /// Type is not applicable; the component element separator is a delimiter and not a data element; this field provides the delimiter used to separate component data elements within a composite data structure; this value must be different than the data element separator and the segment terminator
    /// - TYPE=
    /// - MIN=1
    /// - MAX=1
    #[serde(rename = "16")]
    pub _16: String,
}

impl<'a> Parser<&'a str, ISA, nom::error::Error<&'a str>> for ISA {
    fn parse(input: &'a str) -> IResult<&'a str, ISA> {
        let (rest, vars) = crate::util::parse_line(input, "ISA")?;
        let obj = ISA {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).unwrap().to_string(),
            _06: vars.get(5).unwrap().to_string(),
            _07: vars.get(6).unwrap().to_string(),
            _08: vars.get(7).unwrap().to_string(),
            _09: vars.get(8).unwrap().to_string(),
            _10: vars.get(9).unwrap().to_string(),
            _11: vars.get(10).unwrap().to_string(),
            _12: vars.get(11).unwrap().to_string(),
            _13: vars.get(12).unwrap().to_string(),
            _14: vars.get(13).unwrap().to_string(),
            _15: vars.get(14).unwrap().to_string(),
            _16: vars.get(15).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// K1 - Remarks
///
/// To transmit information in a free-form format for comment or special instruction
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 61 | Free-Form Message | 1 | M | AN | 1/30
/// 02 | 61 | Free-Form Message | 1 | O | AN | 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct K1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

impl<'a> Parser<&'a str, K1, nom::error::Error<&'a str>> for K1 {
    fn parse(input: &'a str) -> IResult<&'a str, K1> {
        let (rest, vars) = crate::util::parse_line(input, "K1")?;
        let obj = K1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct L0 {
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
}

impl<'a> Parser<&'a str, L0, nom::error::Error<&'a str>> for L0 {
    fn parse(input: &'a str) -> IResult<&'a str, L0> {
        let (rest, vars) = crate::util::parse_line(input, "L0")?;
        let obj = L0 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct L1 {
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
    #[serde(rename = "19")]
    pub _19: Option<String>,
    #[serde(rename = "20")]
    pub _20: Option<String>,
    #[serde(rename = "21")]
    pub _21: Option<String>,
}

impl<'a> Parser<&'a str, L1, nom::error::Error<&'a str>> for L1 {
    fn parse(input: &'a str) -> IResult<&'a str, L1> {
        let (rest, vars) = crate::util::parse_line(input, "L1")?;
        let obj = L1 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
            _16: vars.get(15).map(|x| x.to_string()),
            _17: vars.get(16).map(|x| x.to_string()),
            _18: vars.get(17).map(|x| x.to_string()),
            _19: vars.get(18).map(|x| x.to_string()),
            _20: vars.get(19).map(|x| x.to_string()),
            _21: vars.get(20).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// L3 - Total Weight and Charges
///
/// To specify the total shipment in terms of weight, volume, rates, charges, advances, and prepaid amounts applicable to one or more line items
///
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct L3 {
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
}

impl<'a> Parser<&'a str, L3, nom::error::Error<&'a str>> for L3 {
    fn parse(input: &'a str) -> IResult<&'a str, L3> {
        let (rest, vars) = crate::util::parse_line(input, "L3")?;
        let obj = L3 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// L4 - Measurement
///
/// To describe physical ddimensions and quantities
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct L4 {
    /// 82 - Length
    #[serde(rename = "01")]
    pub _01: String,
    /// 189 - Width
    #[serde(rename = "02")]
    pub _02: String,
    /// 65 - Height
    #[serde(rename = "03")]
    pub _03: String,
    /// 90 - Measurement Unit Qualifier
    #[serde(rename = "04")]
    pub _04: String,
    /// 380 - Quantity
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 271 - Industry Code
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

impl<'a> Parser<&'a str, L4, nom::error::Error<&'a str>> for L4 {
    fn parse(input: &'a str) -> IResult<&'a str, L4> {
        let (rest, vars) = crate::util::parse_line(input, "L4")?;
        let obj = L4 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct L5 {
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

impl<'a> Parser<&'a str, L5, nom::error::Error<&'a str>> for L5 {
    fn parse(input: &'a str) -> IResult<&'a str, L5> {
        let (rest, vars) = crate::util::parse_line(input, "L5")?;
        let obj = L5 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// L7 - Tariff Reference
///
/// To reference details of the tariff used to arrive at applicable rates or charge
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 213 | Lading Line Item Number | O |  | N0 1/3
/// 02 | 168 | Tariff Agency Code | O |  | ID 1/4
/// 03 | 171 | Tariff Number | O |  | AN 1/7
/// 04 | 172 | Tariff Section | O |  | AN 1/2
/// 05 | 169 | Tariff Item Number | O |  | AN 1/16
/// 06 | 170 | Tariff Item Part | O |  | N0 1/2
/// 07 | 59 | Freight Class Code | O |  | AN 2/5
/// 08 | 173 | Tariff Supplement Identifier | O |  | AN 1/4
/// 09 | 46 | Ex Parte | O |  | AN 4/4
/// 10 | 373 | Date | O |  | DT 8/8
/// 11 | 119 | Rate Basis Number | O |  | AN 1/6
/// 12 | 227 | Tariff Column | O |  | AN 1/2
/// 13 | 294 | Tariff Distance | O |  | N0 1/5
/// 14 | 295 | Distance Qualifier | O |  | ID 1/1
/// 15 | 19 | City Name | O |  | AN 2/30
/// 16 | 156 | State or Province Code | O |  | ID 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct L7 {
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
}

impl<'a> Parser<&'a str, L7, nom::error::Error<&'a str>> for L7 {
    fn parse(input: &'a str) -> IResult<&'a str, L7> {
        let (rest, vars) = crate::util::parse_line(input, "L7")?;
        let obj = L7 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
            _16: vars.get(15).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// L11 - Business Instructions and Reference Number
///
/// To specify instructions in this business relationship or a reference number
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 02 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 03 | 352 | Description | 1 | X | AN | 1/80
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct L11 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, L11, nom::error::Error<&'a str>> for L11 {
    fn parse(input: &'a str) -> IResult<&'a str, L11> {
        let (rest, vars) = crate::util::parse_line(input, "L11")?;
        let obj = L11 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// LAD - Lading Detail
///
/// To transmit detailed lading data pertinent to a pickup or delivery
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 211 | Packaging Form Code | 1 | X | ID | 3/3
/// 02 | 80 | Lading Quantity | 1 | X | N0 | 1/7
/// 03 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 04 | 395 | Unit Weight | 1 | X | R | 1/8
/// 05 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 06 | 81 | Weight | 1 | X | R | 1/10
/// 07 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 08 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 09 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 10 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 11 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 12 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 13 | 79 | Lading Description | 1 | O | AN | 1/50
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LAD {
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

impl<'a> Parser<&'a str, LAD, nom::error::Error<&'a str>> for LAD {
    fn parse(input: &'a str) -> IResult<&'a str, LAD> {
        let (rest, vars) = crate::util::parse_line(input, "LAD")?;
        let obj = LAD {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// LE - Loop Trailer
///
/// To indicate that the loop immediately preceding this segment is complete
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 447 | Loop Identifier Code | 1 | M | AN | 1/6
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LE {
    #[serde(rename = "01")]
    pub _01: String,
}

impl<'a> Parser<&'a str, LE, nom::error::Error<&'a str>> for LE {
    fn parse(input: &'a str) -> IResult<&'a str, LE> {
        let (rest, vars) = crate::util::parse_line(input, "LE")?;
        let obj = LE {
            _01: vars.first().unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// LEP - EPA Required Data
///
/// To specify the Environmental Protection Agency (EPA) information relating to shipments of hazardous material
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 806 | EPA Waste Stream Number Code | 1 | O | ID | 4/6
/// 02 | 807 | Waste Characteristics Code | 1 | O | ID | 12/16
/// 03 | 156 | State or Province Code NEW | 1 | X/Z | ID | 2/2
/// 04 | 127 | Reference Identification NEW | 1 | X/Z | AN | 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LEP {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, LEP, nom::error::Error<&'a str>> for LEP {
    fn parse(input: &'a str) -> IResult<&'a str, LEP> {
        let (rest, vars) = crate::util::parse_line(input, "LEP")?;
        let obj = LEP {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LFH {
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
}

impl<'a> Parser<&'a str, LFH, nom::error::Error<&'a str>> for LFH {
    fn parse(input: &'a str) -> IResult<&'a str, LFH> {
        let (rest, vars) = crate::util::parse_line(input, "LFH")?;
        let obj = LFH {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LH1 {
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
}

impl<'a> Parser<&'a str, LH1, nom::error::Error<&'a str>> for LH1 {
    fn parse(input: &'a str) -> IResult<&'a str, LH1> {
        let (rest, vars) = crate::util::parse_line(input, "LH1")?;
        let obj = LH1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LH2 {
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

impl<'a> Parser<&'a str, LH2, nom::error::Error<&'a str>> for LH2 {
    fn parse(input: &'a str) -> IResult<&'a str, LH2> {
        let (rest, vars) = crate::util::parse_line(input, "LH2")?;
        let obj = LH2 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LH3 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, LH3, nom::error::Error<&'a str>> for LH3 {
    fn parse(input: &'a str) -> IResult<&'a str, LH3> {
        let (rest, vars) = crate::util::parse_line(input, "LH3")?;
        let obj = LH3 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplaySegment)]
pub struct LH4 {
    #[serde(rename = "01")]
    _01: Option<String>,
    #[serde(rename = "02")]
    _02: Option<String>,
    #[serde(rename = "03")]
    _03: Option<String>,
    #[serde(rename = "04")]
    _04: Option<String>,
    #[serde(rename = "05")]
    _05: Option<String>,
    #[serde(rename = "06")]
    _06: Option<String>,
    #[serde(rename = "07")]
    _07: Option<String>,
    #[serde(rename = "08")]
    _08: Option<String>,
    #[serde(rename = "09")]
    _09: Option<String>,
    #[serde(rename = "10")]
    _10: Option<String>,
    #[serde(rename = "11")]
    _11: Option<String>,
    #[serde(rename = "12")]
    _12: Option<String>,
}

impl<'a> Parser<&'a str, LH4, nom::error::Error<&'a str>> for LH4 {
    fn parse(input: &'a str) -> IResult<&'a str, LH4> {
        let (rest, vars) = crate::util::parse_line(input, "LH4")?;
        let obj = LH4 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LH6 {
    /// 93 - Name
    ///
    /// Free-form name
    /// - TYPE=AN
    /// - MIN=1
    /// - MAX=60
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, LH6, nom::error::Error<&'a str>> for LH6 {
    fn parse(input: &'a str) -> IResult<&'a str, LH6> {
        let (rest, vars) = crate::util::parse_line(input, "LH6")?;
        let obj = LH6 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LHR {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, LHR, nom::error::Error<&'a str>> for LHR {
    fn parse(input: &'a str) -> IResult<&'a str, LHR> {
        let (rest, vars) = crate::util::parse_line(input, "LHR")?;
        let obj = LHR {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LHT {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, LHT, nom::error::Error<&'a str>> for LHT {
    fn parse(input: &'a str) -> IResult<&'a str, LHT> {
        let (rest, vars) = crate::util::parse_line(input, "LHT")?;
        let obj = LHT {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// LS - Loop Header
///
/// To indicate that the next segment begins a loop
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 447 | Loop Identifier Code | 1 | M | AN | 1/6
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LS {
    #[serde(rename = "01")]
    pub _01: String,
}

impl<'a> Parser<&'a str, LS, nom::error::Error<&'a str>> for LS {
    fn parse(input: &'a str) -> IResult<&'a str, LS> {
        let (rest, vars) = crate::util::parse_line(input, "LS")?;
        let obj = LS {
            _01: vars.first().unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// LX - Assigned Number
///
/// To reference a line number in a transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 554 | Assigned Number | 1 | M | N0 | 1/6
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LX {
    #[serde(rename = "01")]
    pub _01: String,
}

impl<'a> Parser<&'a str, LX, nom::error::Error<&'a str>> for LX {
    fn parse(input: &'a str) -> IResult<&'a str, LX> {
        let (rest, vars) = crate::util::parse_line(input, "LX")?;
        let obj = LX {
            _01: vars.first().unwrap().to_string(),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, M0, nom::error::Error<&'a str>> for M0 {
    fn parse(input: &'a str) -> IResult<&'a str, M0> {
        let (rest, vars) = crate::util::parse_line(input, "M0")?;
        let obj = M0 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, M1, nom::error::Error<&'a str>> for M1 {
    fn parse(input: &'a str) -> IResult<&'a str, M1> {
        let (rest, vars) = crate::util::parse_line(input, "M1")?;
        let obj = M1 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, M3, nom::error::Error<&'a str>> for M3 {
    fn parse(input: &'a str) -> IResult<&'a str, M3> {
        let (rest, vars) = crate::util::parse_line(input, "M3")?;
        let obj = M3 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, M7, nom::error::Error<&'a str>> for M7 {
    fn parse(input: &'a str) -> IResult<&'a str, M7> {
        let (rest, vars) = crate::util::parse_line(input, "M7")?;
        let obj = M7 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, M10, nom::error::Error<&'a str>> for M10 {
    fn parse(input: &'a str) -> IResult<&'a str, M10> {
        let (rest, vars) = crate::util::parse_line(input, "M10")?;
        let obj = M10 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, M11, nom::error::Error<&'a str>> for M11 {
    fn parse(input: &'a str) -> IResult<&'a str, M11> {
        let (rest, vars) = crate::util::parse_line(input, "M11")?;
        let obj = M11 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).unwrap().to_string(),
            _06: vars.get(5).unwrap().to_string(),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).unwrap().to_string(),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
            _16: vars.get(15).map(|x| x.to_string()),
            _17: vars.get(16).map(|x| x.to_string()),
            _18: vars.get(17).map(|x| x.to_string()),
            _19: vars.get(18).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, M12, nom::error::Error<&'a str>> for M12 {
    fn parse(input: &'a str) -> IResult<&'a str, M12> {
        let (rest, vars) = crate::util::parse_line(input, "M12")?;
        let obj = M12 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct M13 {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: String,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: String,
    pub _10: Option<String>,
}

impl<'a> Parser<&'a str, M13, nom::error::Error<&'a str>> for M13 {
    fn parse(input: &'a str) -> IResult<&'a str, M13> {
        let (rest, vars) = crate::util::parse_line(input, "M13")?;
        let obj = M13 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).unwrap().to_string(),
            _10: vars.get(9).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct MAN {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, MEA, nom::error::Error<&'a str>> for MEA {
    fn parse(input: &'a str) -> IResult<&'a str, MEA> {
        let (rest, vars) = crate::util::parse_line(input, "MEA")?;
        let obj = MEA {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct MS1 {
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    pub _01: Option<String>,
    pub _02: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct MS3 {
    pub _01: String,
    pub _02: String,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
}

impl<'a> Parser<&'a str, MS3, nom::error::Error<&'a str>> for MS3 {
    fn parse(input: &'a str) -> IResult<&'a str, MS3> {
        let (rest, vars) = crate::util::parse_line(input, "MS3")?;
        let obj = MS3 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, N1, nom::error::Error<&'a str>> for N1 {
    fn parse(input: &'a str) -> IResult<&'a str, N1> {
        let (rest, vars) = crate::util::parse_line(input, "N1")?;
        let obj = N1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// N2 - Additional Name Information
///
/// To specify additional names or those longer than 35 characters in length
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 93 | Name | 1 | M | AN | 1/60
/// 02 | 93 | Name | 1 | O | AN | 1/60
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, N2, nom::error::Error<&'a str>> for N2 {
    fn parse(input: &'a str) -> IResult<&'a str, N2> {
        let (rest, vars) = crate::util::parse_line(input, "N2")?;
        let obj = N2 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// N3 - Address Information
///
/// To specify the location of the named party
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 166 | Address Information | 1 | M | AN | 1/55
/// 02 | 166 | Address Information | 1 | O | AN | 1/55
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct N3 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

impl<'a> Parser<&'a str, N3, nom::error::Error<&'a str>> for N3 {
    fn parse(input: &'a str) -> IResult<&'a str, N3> {
        let (rest, vars) = crate::util::parse_line(input, "N3")?;
        let obj = N3 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, N4, nom::error::Error<&'a str>> for N4 {
    fn parse(input: &'a str) -> IResult<&'a str, N4> {
        let (rest, vars) = crate::util::parse_line(input, "N4")?;
        let obj = N4 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, N5, nom::error::Error<&'a str>> for N5 {
    fn parse(input: &'a str) -> IResult<&'a str, N5> {
        let (rest, vars) = crate::util::parse_line(input, "N5")?;
        let obj = N5 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, N7, nom::error::Error<&'a str>> for N7 {
    fn parse(input: &'a str) -> IResult<&'a str, N7> {
        let (rest, vars) = crate::util::parse_line(input, "N7")?;
        let obj = N7 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
            _16: vars.get(15).map(|x| x.to_string()),
            _17: vars.get(16).map(|x| x.to_string()),
            _18: vars.get(17).map(|x| x.to_string()),
            _19: vars.get(18).map(|x| x.to_string()),
            _20: vars.get(19).map(|x| x.to_string()),
            _21: vars.get(20).map(|x| x.to_string()),
            _22: vars.get(21).map(|x| x.to_string()),
            _23: vars.get(22).map(|x| x.to_string()),
            _24: vars.get(23).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, N7A, nom::error::Error<&'a str>> for N7A {
    fn parse(input: &'a str) -> IResult<&'a str, N7A> {
        let (rest, vars) = crate::util::parse_line(input, "N7A")?;
        let obj = N7A {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct N7B {
    pub _01: Option<String>,
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
}

impl<'a> Parser<&'a str, N7B, nom::error::Error<&'a str>> for N7B {
    fn parse(input: &'a str) -> IResult<&'a str, N7B> {
        let (rest, vars) = crate::util::parse_line(input, "N7B")?;
        let obj = N7B {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, N9, nom::error::Error<&'a str>> for N9 {
    fn parse(input: &'a str) -> IResult<&'a str, N9> {
        let (rest, vars) = crate::util::parse_line(input, "N9")?;
        let obj = N9 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, N10, nom::error::Error<&'a str>> for N10 {
    fn parse(input: &'a str) -> IResult<&'a str, N10> {
        let (rest, vars) = crate::util::parse_line(input, "N10")?;
        let obj = N10 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// N12 - Equipment Environment
///
/// To describe the operating environment of the equipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 829 | Fuel Type | M |  | ID 1/1
/// 02 | C001 | Composite Unit of Measure | M
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct N12 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

impl<'a> Parser<&'a str, N12, nom::error::Error<&'a str>> for N12 {
    fn parse(input: &'a str) -> IResult<&'a str, N12> {
        let (rest, vars) = crate::util::parse_line(input, "N12")?;
        let obj = N12 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, NA, nom::error::Error<&'a str>> for NA {
    fn parse(input: &'a str) -> IResult<&'a str, NA> {
        let (rest, vars) = crate::util::parse_line(input, "NA")?;
        let obj = NA {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct NTE {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: String,
}

impl<'a> Parser<&'a str, NTE, nom::error::Error<&'a str>> for NTE {
    fn parse(input: &'a str) -> IResult<&'a str, NTE> {
        let (rest, vars) = crate::util::parse_line(input, "NTE")?;
        let obj = NTE {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// OID - Order Identification Detail NEW
///
/// To specify order identification detail
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | X/Z | AN | 1/30
/// 02 | 324 | Purchase Order Number | 1 | X | AN | 1/22
/// 03 | 127 | Reference Identification | 1 | O/Z | AN | 1/30
/// 04 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 05 | 380 | Quantity | 1 | X | R | 1/15
/// 06 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 07 | 81 | Weight | 1 | X | R | 1/10
/// 08 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 09 | 183 | Volume | 1 | X | R | 1/8
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct OID {
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

/// P4 - U.S. Port Information
///
/// To transmit identifying information for a U.S. port
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|-----|------|-------
/// 01 | 310 | Location Identifier M AN 1/30
/// 02 | 373 | Date M DT 8/8
/// 03 | 380 | Quantity O R 1/15
/// 04 | 310 | Location Identifier O AN 1/30
/// 05 | 337 | Time O TM 4/8
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct P4 {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
}

impl<'a> Parser<&'a str, P4, nom::error::Error<&'a str>> for P4 {
    fn parse(input: &'a str) -> IResult<&'a str, P4> {
        let (rest, vars) = crate::util::parse_line(input, "P4")?;
        let obj = P4 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// P5 - Port Information
///
/// To indicate port-related data
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|-----|------|-------
/// 01 | 115 | Port or Terminal Function Code | M |  | ID 1/1
/// 02 | 309 | Location Qualifier | M |  | ID 1/2
/// 03 | 310 | Location Identifier | M |  | AN 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct P5 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
}

impl<'a> Parser<&'a str, P5, nom::error::Error<&'a str>> for P5 {
    fn parse(input: &'a str) -> IResult<&'a str, P5> {
        let (rest, vars) = crate::util::parse_line(input, "P5")?;
        let obj = P5 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct PER {
    pub _01: String,
    /// 93 - Name
    ///
    /// Free-form name
    /// - TYPE=AN
    /// - MIN=1
    /// - MAX=60
    pub _02: Option<String>,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
}

impl<'a> Parser<&'a str, PER, nom::error::Error<&'a str>> for PER {
    fn parse(input: &'a str) -> IResult<&'a str, PER> {
        let (rest, vars) = crate::util::parse_line(input, "PER")?;
        let obj = PER {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _12: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _13: Option<String>,
    pub _14: Option<String>,
    pub _15: Option<String>,
}

impl<'a> Parser<&'a str, PI, nom::error::Error<&'a str>> for PI {
    fn parse(input: &'a str) -> IResult<&'a str, PI> {
        let (rest, vars) = crate::util::parse_line(input, "PI")?;
        let obj = PI {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// PLD - Pallet Information NEW
///
/// To specify pallet information including quantity, exchange, and weight
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 406 | Quantity of Pallets Shipped | 1 | M | N0 | 1/3
/// 02 | 399 | Pallet Exchange Code | 1 | O | ID | 1/1
/// 03 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 04 | 81 | Weight | 1 | X | R | 1/10
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct PLD {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, PLD, nom::error::Error<&'a str>> for PLD {
    fn parse(input: &'a str) -> IResult<&'a str, PLD> {
        let (rest, vars) = crate::util::parse_line(input, "PLD")?;
        let obj = PLD {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// PRF - Purchase Order Reference
///
/// To provide reference to a specific purchase order
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 324 | Purchase Order Number | 1 | M | AN | 1/22
/// 02 | 328 | Release Number | 1 | O | AN | 1/30
/// 03 | 327 | Change Order Sequence Number | 1 | O | AN | 1/8
/// 04 | 373 | Date | 1 | O/Z | DT | 8/8
/// 05 | 350 | Assigned Identification | 1 | O | AN | 1/20
/// 06 | 367 | Contract Number | 1 | O | AN | 1/30
/// 07 | 92 | Purchase Order Type Code | 1 | O | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct PRF {
    pub _01: String,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct PS {
    pub _01: String,
    pub _02: String,
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
    pub _10: Option<String>,
    pub _11: Option<String>,
    pub _12: Option<String>,
    pub _13: Option<String>,
    pub _14: Option<String>,
}

impl<'a> Parser<&'a str, PS, nom::error::Error<&'a str>> for PS {
    fn parse(input: &'a str) -> IResult<&'a str, PS> {
        let (rest, vars) = crate::util::parse_line(input, "PS")?;
        let obj = PS {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// PWK - Paperwork
///
/// To identify the type or transmission or both of paperwork or supporting information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 755 | Report Type Code | M |  | ID 2/2
/// 02 | 756 | Report Transmission Code | O |  | ID 1/2
/// 03 | 757 | Report Copies Needed | O |  | N0 1/2
/// 04 | 98 | Entity Identifier Code | O |  | ID 2/3
/// 05 | 66 | Identification Code Qualifier | X |  | ID 1/2
/// 06 | 67 | Identification Code | X |  | AN 2/80
/// 07 | 352 | Description | O |  | AN 1/80
/// 08 | C002 | Actions Indicated | O |  |
/// 09 | 1525 | Request Category Code | O |  | ID 1/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct PWK {
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

impl<'a> Parser<&'a str, PWK, nom::error::Error<&'a str>> for PWK {
    fn parse(input: &'a str) -> IResult<&'a str, PWK> {
        let (rest, vars) = crate::util::parse_line(input, "PWK")?;
        let obj = PWK {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// Q2 - Status Details (Ocean)
///
/// To transmit identifying information relative to identification of vessel, transportation dates, lading quantity, weight, and cube
///
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct Q2 {
    #[serde(rename = "01")]
    pub _01: String,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "02")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _02: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
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
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "05")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _14: Option<String>,
    #[serde(rename = "15")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _15: Option<String>,
    #[serde(rename = "16")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _16: Option<String>,
}

impl<'a> Parser<&'a str, Q2, nom::error::Error<&'a str>> for Q2 {
    fn parse(input: &'a str) -> IResult<&'a str, Q2> {
        let (rest, vars) = crate::util::parse_line(input, "Q2")?;
        let obj = Q2 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
            _16: vars.get(15).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// Q5 - Status Details
///
/// To specify the status of the shipment in terms of dates, time, reference numbers, and location
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 157 | Shipment Status Code | 1 | O | ID | 1/2
/// 02 | 373 | Date | 1 | O/Z | DT | 8/8
/// 03 | 337 | Time | 1 | X/Z | TM | 4/8
/// 04 | 623 | Time Code | 1 | X | ID | 2/2
/// 05 | 641 | Status Reason Code | 1 | O | ID | 3/3
/// 06 | 19 | City Name | 1 | X | AN | 2/30
/// 07 | 156 | State or Province Code | 1 | O | ID | 2/2
/// 08 | 26 | Country Code | 1 | O | ID | 2/3
/// 09 | 206 | Equipment Initial | 1 | O | AN | 1/4
/// 10 | 207 | Equipment Number | 1 | O | AN | 1/10
/// 11 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 12 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 13 | 1280 | Direction Identifier Code | 1 | O/Z | ID | 1/1
/// 14 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 15 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 16 | 1280 | Direction Identifier Code | 1 | O/Z | ID | 1/1
/// 17 | 954 | Percent | 1 | O/Z | R | 1/10
/// 18 | 108 | Pick-up or Delivery Code | 1 | O | ID | 1/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct Q5 {
    pub _01: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _02: Option<String>,
    pub _03: Option<String>,
    /// 623 - Time Code
    ///
    /// Code identifying the time. In accordance with International Standards Organization standard 8601, time can be specified by a + or - and an indication in hours in relation to Universal Time Coordinate (UTC) time; since + is a restricted character, + and - are substituted by P and M in the codes that follow
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    pub _04: Option<String>,
    pub _05: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    pub _06: Option<String>,
    pub _07: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
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
}

/// Q7 - Lading Exception Code
///
/// To specify the status of the shipment in terms of lading exception information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 33 | Lading Exception Code | 1 | M | ID | 1/1
/// 02 | 211 | Packaging Form Code | 1 | O | ID | 3/3
/// 03 | 80 | Lading Quantity | 1 | X | N0 | 1/7
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct Q7 {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<String>,
}

/// QTY - Quantity
///
/// To specify quantity information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 673 | Quantity Qualifier | M |  | ID 2/2
/// 02 | 380 | Quantity | X |  | R 1/15
/// 03 | C001 | Composite Unit of Measure | O |  |
/// 04 | 61 | Free-Form Message | X |  | AN 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct QTY {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, QTY, nom::error::Error<&'a str>> for QTY {
    fn parse(input: &'a str) -> IResult<&'a str, QTY> {
        let (rest, vars) = crate::util::parse_line(input, "QTY")?;
        let obj = QTY {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct R2 {
    pub _01: String,
    pub _02: String,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    pub _03: Option<String>,
    pub _04: Option<String>,
    pub _05: Option<String>,
    pub _06: Option<String>,
    pub _07: Option<String>,
    pub _08: Option<String>,
    pub _09: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    pub _10: Option<String>,
    pub _11: Option<String>,
    pub _12: Option<String>,
    pub _13: Option<String>,
}

impl<'a> Parser<&'a str, R2, nom::error::Error<&'a str>> for R2 {
    fn parse(input: &'a str) -> IResult<&'a str, R2> {
        let (rest, vars) = crate::util::parse_line(input, "R2")?;
        let obj = R2 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// R2A - Route Information with Preference
///
/// To specify the responsibilities and carrier preference
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 133 | Routing Sequence Code | M |  | ID 1/2
/// 02 | 1431 | Preference | M |  | ID 1/1
/// 03 | 91 | Transportation Method/Type Code | O |  | ID 1/2
/// 04 | 140 | Standard Carrier Alpha Code | O |  | ID 2/4
/// 05 | 309 | Location Qualifier | O |  | ID 1/2
/// 06 | 310 | Location Identifier | X |  | AN 1/30
/// 07 | 56 | Type of Service Code | O |  | ID 2/2
/// 08 | 1 | Route Code | O |  | AN 1/13
/// 09 | 742 | Route Description | O |  | AN 1/35
/// 10 | 98 | Entity Identifier Code | O |  | ID 2/3
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct R2A {
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
}

impl<'a> Parser<&'a str, R2A, nom::error::Error<&'a str>> for R2A {
    fn parse(input: &'a str) -> IResult<&'a str, R2A> {
        let (rest, vars) = crate::util::parse_line(input, "R2A")?;
        let obj = R2A {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct R4 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _04: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "05")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _08: Option<String>,
}

impl<'a> Parser<&'a str, R4, nom::error::Error<&'a str>> for R4 {
    fn parse(input: &'a str) -> IResult<&'a str, R4> {
        let (rest, vars) = crate::util::parse_line(input, "R4")?;
        let obj = R4 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, R9, nom::error::Error<&'a str>> for R9 {
    fn parse(input: &'a str) -> IResult<&'a str, R9> {
        let (rest, vars) = crate::util::parse_line(input, "R9")?;
        let obj = R9 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct REF {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, REF, nom::error::Error<&'a str>> for REF {
    fn parse(input: &'a str) -> IResult<&'a str, REF> {
        let (rest, vars) = crate::util::parse_line(input, "REF")?;
        let obj = REF {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct S1 {
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

impl<'a> Parser<&'a str, S1, nom::error::Error<&'a str>> for S1 {
    fn parse(input: &'a str) -> IResult<&'a str, S1> {
        let (rest, vars) = crate::util::parse_line(input, "S1")?;
        let obj = S1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct S2 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, S2, nom::error::Error<&'a str>> for S2 {
    fn parse(input: &'a str) -> IResult<&'a str, S2> {
        let (rest, vars) = crate::util::parse_line(input, "S2")?;
        let obj = S2 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// S5 - Stop Off Details
///
/// To specify stop-off detail reference numbers and stop reason
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 165 | Stop Sequence Number | 1 | M | N0 | 1/3
/// 02 | 163 | Stop Reason Code | 1 | M | ID | 2/2
/// 03 | 81 | Weight | 1 | X | R | 1/10
/// 04 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 05 | 382 | Number of Units Shipped | 1 | X | R | 1/10
/// 06 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 07 | 183 | Volume | 1 | X | R | 1/8
/// 08 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 09 | 352 | Description | 1 | O/Z | AN | 1/80
/// 10 | 154 | Standard Point Location Code | 1 | O | ID | 6/9
/// 11 | 190 | Accomplish Code | 1 | O | ID | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct S5 {
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

impl<'a> Parser<&'a str, S5, nom::error::Error<&'a str>> for S5 {
    fn parse(input: &'a str) -> IResult<&'a str, S5> {
        let (rest, vars) = crate::util::parse_line(input, "S5")?;
        let obj = S5 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct S9 {
    pub _01: String,
    pub _02: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    pub _03: String,
    pub _04: String,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    pub _05: Option<String>,
    pub _06: String,
    pub _07: Option<String>,
    pub _08: Option<String>,
}

impl<'a> Parser<&'a str, S9, nom::error::Error<&'a str>> for S9 {
    fn parse(input: &'a str) -> IResult<&'a str, S9> {
        let (rest, vars) = crate::util::parse_line(input, "S9")?;
        let obj = S9 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).unwrap().to_string(),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// SDQ - Destination Quantity
///
/// To specify destination and quantity detail
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 355 | Unit or Basis for Measurement Code | 1 | M | ID | 2/2
/// 02 | 66 | Identification Code Qualifier | 1 | O | ID | 1/2
/// 03 | 67 | Identification Code | 1 | M | AN | 2/80
/// 04 | 380 | Quantity | 1 | M | R | 1/15
/// 05 | 67 | Identification Code | 1 | X | AN | 2/80
/// 06 | 380 | Quantity | 1 | X | R | 1/15
/// 07 | 67 | Identification Code | 1 | X | AN | 2/80
/// 08 | 380 | Quantity | 1 | X | R | 1/15
/// 09 | 67 | Identification Code | 1 | X | AN | 2/80
/// 10 | 380 | Quantity | 1 | X | R | 1/15
/// 11 | 67 | Identification Code | 1 | X | AN | 2/80
/// 12 | 380 | Quantity | 1 | X | R | 1/15
/// 13 | 67 | Identification Code | 1 | X | AN | 2/80
/// 14 | 380 | Quantity | 1 | X | R | 1/15
/// 15 | 67 | Identification Code | 1 | X | AN | 2/80
/// 16 | 380 | Quantity | 1 | X | R | 1/15
/// 17 | 67 | Identification Code | 1 | X | AN | 2/80
/// 18 | 380 | Quantity | 1 | X | R | 1/15
/// 19 | 67 | Identification Code | 1 | X | AN | 2/80
/// 20 | 380 | Quantity | 1 | X | R | 1/15
/// 21 | 67 | Identification Code | 1 | X | AN | 2/80
/// 22 | 380 | Quantity | 1 | X | R | 1/15
/// 23 | 310 | Location Identifier | 1 | O/Z | AN | 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct SDQ {
    pub _01: String,
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
}

/// SE - Transaction Set Trailer
///
/// To indicate the end of the transaction set and provide the count of the transmitted segments (including the beginning (ST) and ending (SE) segments)
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 96 | Number of Included Segments | 1 | M | N0 | 1/10
/// 02 | 329 | Transaction Set Control Number | 1 | M | AN | 4/9
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct SE {
    /// 96 - Number of Included Segments
    ///
    /// Total number of segments included in a transaction set including ST and SE segments
    /// - TYPE=N0
    /// - MIN=1
    /// - MAX=10
    #[serde(rename = "01")]
    pub _01: String,
    /// 329 - Transaction Set Control Number
    ///
    /// Identifying control number that must be unique within the transaction set functional group assigned by the originator for a transaction set
    /// - TYPE=AN
    /// - MIN=4
    /// - MAX=9
    #[serde(rename = "02")]
    pub _02: String,
}

impl<'a> Parser<&'a str, SE, nom::error::Error<&'a str>> for SE {
    fn parse(input: &'a str) -> IResult<&'a str, SE> {
        let (rest, vars) = crate::util::parse_line(input, "SE")?;
        let obj = SE {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// SG - Shipment Status
///
/// To convey the status of a shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 157 | Shipment Status Code | 1 | X | ID | 1/2
/// 02 | 641 | Status Reason Code | 1 | X | ID | 3/3
/// 03 | 35 | Disposition Code | 1 | X | ID | 2/2
/// 04 | 373 | Date | 1 | O | DT | 8/8
/// 05 | 337 | Time | 1 | X | TM | 4/8
/// 06 | 623 | Time Code | 1 | O | ID | 2/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct SG {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
    #[serde(rename = "05")]
    pub _05: String,
    /// 623 - Time Code
    ///
    /// Code identifying the time. In accordance with International Standards Organization standard 8601, time can be specified by a + or - and an indication in hours in relation to Universal Time Coordinate (UTC) time; since + is a restricted character, + and - are substituted by P and M in the codes that follow
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

impl<'a> Parser<&'a str, SG, nom::error::Error<&'a str>> for SG {
    fn parse(input: &'a str) -> IResult<&'a str, SG> {
        let (rest, vars) = crate::util::parse_line(input, "SG")?;
        let obj = SG {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).unwrap().to_string(),
            _06: vars.get(5).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// SPO - Shipment Purchase Order Detail
///
/// To specify the purchase order details for a shipment
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 324 | Purchase Order Number | 1 | M | AN | 1/22
/// 02 | 127 | Reference Identification | 1 | O/Z | AN | 1/30
/// 03 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 04 | 380 | Quantity | 1 | X/Z | R | 1/15
/// 05 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 06 | 81 | Weight | 1 | X/Z | R | 1/10
/// 07 | 647 | Application Error Condition Code | 1 | O/Z | ID | 1/3
/// 08 | 127 | Reference Identification | 1 | O/Z | AN | 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct SPO {
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

/// ST - Transaction Set Header
///
/// To indicate the start of a transaction set and to assign a control number
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|------|--------|----|------|-------
/// 01 | 143 | Transaction Set Identifier Code | 1 | M/Z | ID | 3/3
/// 02 | 329 | Transaction Set Control Number | 1 | M | AN | 4/9
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct ST {
    /// 143 - Transaction Set Identifier Code 3/3
    #[serde(rename = "01")]
    pub _01: String,
    /// 329 - Transaction Set Control Number 4/9
    #[serde(rename = "02")]
    pub _02: String,
}

impl<'a> Parser<&'a str, ST, nom::error::Error<&'a str>> for ST {
    fn parse(input: &'a str) -> IResult<&'a str, ST> {
        let (rest, vars) = crate::util::parse_line(input, "ST")?;
        let obj = ST {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct T1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct T2 {
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
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "09")]
    pub _09: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct T3 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct T6 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "07")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct T8 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

/// V1 - Vessel Identification
///
/// To provide vessel details and voyage number
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 597 | Vessel Code | 1 | X | ID | 1/8
/// 02 | 182 | Vessel Name | 1 | X | AN | 2/28
/// 03 | 26 | Country Code | 1 | O/Z | ID | 2/3
/// 04 | 55 | Flight/Voyage Number | 1 | O | AN | 2/10
/// 05 | 140 | Standard Carrier Alpha Code | 1 | O/Z | ID | 2/4
/// 06 | 249 | Vessel Requirement Code | 1 | O | ID | 1/1
/// 07 | 854 | Vessel Type Code | 1 | O | ID | 2/2
/// 08 | 897 | Vessel Code Qualifier | 1 | O | ID | 1/1
/// 09 | 91 | Transportation Method/Type Code | 1 | O | ID | 1/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct V1 {
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
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
}

impl<'a> Parser<&'a str, V1, nom::error::Error<&'a str>> for V1 {
    fn parse(input: &'a str) -> IResult<&'a str, V1> {
        let (rest, vars) = crate::util::parse_line(input, "V1")?;
        let obj = V1 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// V4 - Cargo Location Reference
///
/// To specify the cargo location on board the vessel
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 877 | Vessel Stowage Location | 1 | M | AN | 1/12
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct V4 {
    #[serde(rename = "01")]
    pub _01: String,
}

impl<'a> Parser<&'a str, V4, nom::error::Error<&'a str>> for V4 {
    fn parse(input: &'a str) -> IResult<&'a str, V4> {
        let (rest, vars) = crate::util::parse_line(input, "V4")?;
        let obj = V4 {
            _01: vars.first().unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// V9 - Event Detail
///
/// To specify information about a specific event
///
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct V9 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "03")]
    pub _03: Option<String>,
    /// 337 - Time
    ///
    /// Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=8
    #[serde(rename = "04")]
    pub _04: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
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
    /// 623 - Time Code
    ///
    /// Code identifying the time. In accordance with International Standards Organization standard 8601, time can be specified by a + or - and an indication in hours in relation to Universal Time Coordinate (UTC) time; since + is a restricted character, + and - are substituted by P and M in the codes that follow
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
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

impl<'a> Parser<&'a str, V9, nom::error::Error<&'a str>> for V9 {
    fn parse(input: &'a str) -> IResult<&'a str, V9> {
        let (rest, vars) = crate::util::parse_line(input, "V9")?;
        let obj = V9 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
            _16: vars.get(15).map(|x| x.to_string()),
            _17: vars.get(16).map(|x| x.to_string()),
            _18: vars.get(17).map(|x| x.to_string()),
            _19: vars.get(18).map(|x| x.to_string()),
            _20: vars.get(19).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct VC {
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

impl<'a> Parser<&'a str, VC, nom::error::Error<&'a str>> for VC {
    fn parse(input: &'a str) -> IResult<&'a str, VC> {
        let (rest, vars) = crate::util::parse_line(input, "VC")?;
        let obj = VC {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// VID - Conveyance Identification
///
/// To identify a conveyance and its attributes
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 40 | Equipment Description Code | M |  | ID 2/2
/// 02 | 206 | Equipment Initial | O |  | AN 1/4
/// 03 | 207 | Equipment Number | M |  | AN 1/10
/// 04 | 225 | Seal Number | O |  | AN 2/15
/// 05 | 225 | Seal Number | O |  | AN 2/15
/// 06 | 567 | Equipment Length | O |  | N0 4/5
/// 07 | 65 | Height | O |  | R 1/8
/// 08 | 189 | Width | O |  | R 1/8
/// 09 | 24 | Equipment Type | O |  | ID 4/4
/// 10 | 322 | Load/Empty Status Code | O |  | ID 1/1
/// 11 | 56 | Type of Service Code | O |  | ID 2/2
/// 12 | 310 | Location Identifier | O |  | AN 1/30
/// 13 | 140 | Standard Carrier Alpha Code | O |  | ID 2/4
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct VID {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
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
}

impl<'a> Parser<&'a str, VID, nom::error::Error<&'a str>> for VID {
    fn parse(input: &'a str) -> IResult<&'a str, VID> {
        let (rest, vars) = crate::util::parse_line(input, "VID")?;
        let obj = VID {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// W2 - Equipment Identification
///
/// To identify equipment and the commodity being carried
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 206 | Equipment Initial | 1 | M | AN | 1/4
/// 02 | 207 | Equipment Number | 1 | M | AN | 1/10
/// 03 | 22 | Commodity Code | 1 | O/Z | AN | 1/30
/// 04 | 40 | Equipment Description Code | 1 | M | ID | 2/2
/// 05 | 578 | Equipment Status Code | 1 | M | ID | 1/2
/// 06 | 577 | Net Tons | 1 | O | N0 | 1/3
/// 07 | 177 | Intermodal Service Code | 1 | O | ID | 1/2
/// 08 | 240 | Car Service Order Code | 1 | O | ID | 3/5
/// 09 | 373 | Date | 1 | X/Z | DT | 8/8
/// 10 | 502 | Type of Locomotive Maintenance Code | 1 | X | AN | 2/2
/// 11 | 206 | Equipment Initial | 1 | X | AN | 1/4
/// 12 | 207 | Equipment Number | 1 | X/Z | AN | 1/10
/// 13 | 761 | Equipment Number Check Digit | 1 | O | N0 | 1/1
/// 14 | 219 | Position | 1 | O | AN | 1/3
/// 15 | 301 | Car Type Code | 1 | O | ID | 1/4
/// 16 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct W2 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: String,
    #[serde(rename = "05")]
    pub _05: String,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
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
}

/// W09 - Equipment and Temperature
///
/// To relate equipment type and required temperatures
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 40 | Equipment Description Code | 1 | M | ID | 2/2
/// 02 | 408 | Temperature | 1 | X/Z | R | 1/4
/// 03 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 04 | 408 | Temperature | 1 | X/Z | R | 1/4
/// 05 | 355 | Unit or Basis for Measurement Code | 1 | X | ID | 2/2
/// 06 | 3 | Free Form Message | 1 | O/Z | AN | 1/60
/// 07 | 1122 | Vent Setting Code | 1 | O | ID | 1/1
/// 08 | 488 | Percent | 1 | O/Z | N0 | 1/3
/// 09 | 380 | Quantity | 1 | O/Z | R | 1/15
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct W09 {
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

impl<'a> Parser<&'a str, W09, nom::error::Error<&'a str>> for W09 {
    fn parse(input: &'a str) -> IResult<&'a str, W09> {
        let (rest, vars) = crate::util::parse_line(input, "W09")?;
        let obj = W09 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct X1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: String,
    #[serde(rename = "06")]
    pub _06: String,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
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
}

impl<'a> Parser<&'a str, X1, nom::error::Error<&'a str>> for X1 {
    fn parse(input: &'a str) -> IResult<&'a str, X1> {
        let (rest, vars) = crate::util::parse_line(input, "X1")?;
        let obj = X1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).unwrap().to_string(),
            _06: vars.get(5).unwrap().to_string(),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
            _15: vars.get(14).map(|x| x.to_string()),
            _16: vars.get(15).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// X2 - Import License
///
/// To transmit import license number and effective dates
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 70 | Import License Number | M |  | AN 6/30
/// 02 | 373 | Date | O |  | DT 8/8
/// 03 | 373 | Date | O |  | DT 8/8
/// 04 | 70 | Import License Number | X |  | AN 6/30
/// 05 | 373 | Date | O |  | DT 8/8
/// 06 | 373 | Date | O |  | DT 8/8
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct X2 {
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

impl<'a> Parser<&'a str, X2, nom::error::Error<&'a str>> for X2 {
    fn parse(input: &'a str) -> IResult<&'a str, X2> {
        let (rest, vars) = crate::util::parse_line(input, "X2")?;
        let obj = X2 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// X7 - Customs Information
///
/// To indicate customs information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 61 | Free-Form Message | 1 | M | AN | 1/30
/// 02 | 61 | Free-Form Message | 1 | O | AN | 1/30
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct X7 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

impl<'a> Parser<&'a str, X7, nom::error::Error<&'a str>> for X7 {
    fn parse(input: &'a str) -> IResult<&'a str, X7> {
        let (rest, vars) = crate::util::parse_line(input, "X7")?;
        let obj = X7 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct XH {
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
}

impl<'a> Parser<&'a str, XH, nom::error::Error<&'a str>> for XH {
    fn parse(input: &'a str) -> IResult<&'a str, XH> {
        let (rest, vars) = crate::util::parse_line(input, "XH")?;
        let obj = XH {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// Y2 - Container Details
///
/// To specify container information and transportation service to be used
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 95 | Number of Containers | M |  | N0 1/4
/// 02 | 78 | Container Type Request Code | O |  | ID 1/1
/// 03 | 56 | Type of Service Code | O |  | ID 2/2
/// 04 | 24 | Equipment Type | M |  | ID 4/4
/// 05 | 91 | Transportation Method/Type Code | O |  | ID 1/2
/// 06 | 177 | Intermodal Service Code | O |  | ID 1/2
/// 07 | 140 | Standard Carrier Alpha Code | O |  | ID 2/4
/// 08 | 464 | Container Terms Code | O |  | ID 3/3
/// 09 | 465 | Container Terms Code Qualifier | O |  | ID 1/1
/// 10 | 466 | Total Stop-offs | O |  | N0 1/2
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct Y2 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
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
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
}

impl<'a> Parser<&'a str, Y2, nom::error::Error<&'a str>> for Y2 {
    fn parse(input: &'a str) -> IResult<&'a str, Y2> {
        let (rest, vars) = delimited(tag("Y2*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = Y2 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// Y3 - Space Confirmation
///
/// To specify confirmation information for space booking including number, dates and load time
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct Y3 {
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

impl<'a> Parser<&'a str, Y3, nom::error::Error<&'a str>> for Y3 {
    fn parse(input: &'a str) -> IResult<&'a str, Y3> {
        let (rest, vars) = delimited(tag("Y3*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = Y3 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
            _11: vars.get(10).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// Y4 - Container Release
///
/// To transmit information relative to containers available for release
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct Y4 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
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
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
}

impl<'a> Parser<&'a str, Y4, nom::error::Error<&'a str>> for Y4 {
    fn parse(input: &'a str) -> IResult<&'a str, Y4> {
        let (rest, vars) = delimited(tag("Y4*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = Y4 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).map(|x| x.to_string()),
            _08: vars.get(7).map(|x| x.to_string()),
            _09: vars.get(8).map(|x| x.to_string()),
            _10: vars.get(9).map(|x| x.to_string()),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// Y6 - Authentication
///
/// To specify the authority for authorizing an action and the date authentication is made
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 313 | Authority Identifier Code | O |  | ID 2/2
/// 02 | 151 | Authority | M |  | AN | 1/20
/// 03 | 275 | Authorization Date | M |  | DT 8/8
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct Y6 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
}

impl<'a> Parser<&'a str, Y6, nom::error::Error<&'a str>> for Y6 {
    fn parse(input: &'a str) -> IResult<&'a str, Y6> {
        let (rest, vars) = delimited(tag("Y6*"), take_until("~"), tag("~"))(input)?;
        let (_, vars) = separated_list0(
            tag("*"),
            take_while(|x: char| {
                x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
            }),
        )(vars)?;
        let obj = Y6 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
        };
        // look for trailing newline
        let (rest, _) = opt(newline)(rest)?;
        Ok((rest, obj))
    }
}

/// Y7 - Priority
///
/// To assign a priority to a booking which would increase the possibility that this cargo would be booked on said voyage and not be shut out
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 467 | Priority | 1 | O | N0 | 1/1
/// 02 | 470 | Priority Code | 1 | X | N0 | 1/1
/// 03 | 471 | Priority Code Qualifier | 1 | X | AN | 1/1
/// 04 | 468 | Port Call File Number | 1 | O | N0 | 4/4
/// 05 | 373 | Date | 1 | O/Z | DT | 8/8
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct Y7 {
    #[serde(rename = "01", skip_serializing_if = "Option::is_none")]
    pub _01: Option<String>,
    #[serde(rename = "02", skip_serializing_if = "Option::is_none")]
    pub _02: Option<String>,
    #[serde(rename = "03", skip_serializing_if = "Option::is_none")]
    pub _03: Option<String>,
    #[serde(rename = "04", skip_serializing_if = "Option::is_none")]
    pub _04: Option<String>,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "05", skip_serializing_if = "Option::is_none")]
    pub _05: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct ZC1 {
    #[serde(rename = "01", skip_serializing_if = "Option::is_none")]
    pub _01: Option<String>,
    #[serde(rename = "02", skip_serializing_if = "Option::is_none")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: String,
    /// 243 - Transaction Reference Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "05")]
    pub _05: String,
    #[serde(rename = "06")]
    pub _06: String,
    #[serde(rename = "07")]
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: Option<String>,
}

impl<'a> Parser<&'a str, ZC1, nom::error::Error<&'a str>> for ZC1 {
    fn parse(input: &'a str) -> IResult<&'a str, ZC1> {
        let (rest, vars) = crate::util::parse_line(input, "ZC1")?;
        let obj = ZC1 {
            _01: vars.first().map(|x| x.to_string()),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).unwrap().to_string(),
            _06: vars.get(5).unwrap().to_string(),
            _07: vars.get(6).unwrap().to_string(),
            _08: vars.get(7).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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
    /// 243 - Transaction Reference Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: String,
    #[serde(rename = "08")]
    pub _08: Option<String>,
}

impl<'a> Parser<&'a str, ZD, nom::error::Error<&'a str>> for ZD {
    fn parse(input: &'a str) -> IResult<&'a str, ZD> {
        let (rest, vars) = crate::util::parse_line(input, "ZD")?;
        let obj = ZD {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).map(|x| x.to_string()),
            _06: vars.get(5).map(|x| x.to_string()),
            _07: vars.get(6).unwrap().to_string(),
            _08: vars.get(7).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}
