use crate::util::Parser;
use crate::v004010::element;

use nom::error::ErrorKind;
use nom::IResult;
use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::{DisplaySegment, ParseSegment};

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
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct IC {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<crate::v004010::element::E167>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<crate::v004010::element::E567>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<crate::v004010::element::E845>,
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
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct IM {
    #[serde(rename = "01")]
    pub _01: Option<crate::v004010::element::E533>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<crate::v004010::element::E534>,
}

/// INC - Installment Information
///
/// To specify installment billing arrangement
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct INC {
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
}

/// IEA - Interchange Control Trailer
///
/// To define the end of an interchange of zero or more functional groups and interchange-related control segments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | I16 | Number of Included Functional Groups | 1 | M | N0 | 1/5
/// 02 | I12 | Interchange Control Number | 1 | M | N0 | 9/9
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    PartialEq,
    Eq,
    DisplaySegment,
    ParseSegment,
    Validate,
)]
pub struct IEA {
    /// I16 - Number of Included Functional Groups
    ///
    /// A count of the number of functional groups included in an interchange
    /// - TYPE=N0
    /// - MIN=1
    /// - MAX=5
    #[serde(rename = "01")]
    pub _01: element::I16,
    /// I12 - Interchange Control Number
    ///
    /// A control number assigned by the interchange sender
    /// - TYPE=N0
    /// - MIN=9
    /// - MAX=9
    #[serde(rename = "02")]
    pub _02: element::I12,
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
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, Validate,
)]
pub struct ISA {
    /// I01 - Authorization Information Qualifier
    ///
    /// Code to identify the type of information in the Authorization Information
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "01")]
    pub _01: element::I01,
    /// I02 - Authorization Information
    ///
    /// Information used for additional identification or authorization of the interchange sender or the data in the interchange; the type of information is set by the Authorization Information Qualifier (I01)
    /// - TYPE=AN
    /// - MIN=10
    /// - MAX=10
    #[validate(length(equal = 10, message = "ISA 02 (I02) must be 10 characters long"))]
    #[serde(rename = "02")]
    pub _02: String,
    /// I03 - Security Information Qualifier
    ///
    /// Code to identify the type of information in the Security Information
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "03")]
    pub _03: element::I03,
    /// I04 - Security Information
    ///
    /// This is used for identifying the security information about the interchange sender or the data in the interchange; the type of information is set by the Security Information Qualifier (I03)
    /// - TYPE=AN
    /// - MIN=10
    /// - MAX=10
    #[validate(length(equal = 10, message = "ISA 04 (I04) must be 10 characters long"))]
    #[serde(rename = "04")]
    pub _04: String,
    /// I05 - Interchange ID Qualifier
    ///
    /// Qualifier to designate the system/method of code structure used to designate the sender or receiver ID element being qualified
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "05")]
    pub _05: element::I05,
    /// I06 - Interchange Sender ID
    ///
    /// Identification code published by the sender for other parties to use as the receiver ID to route data to them; the sender always codes this value in the sender ID element
    /// - TYPE=AN
    /// - MIN=15
    /// - MAX=15
    #[validate(length(equal = 15, message = "ISA 06 (I06) must be 15 characters long"))]
    #[serde(rename = "06")]
    pub _06: String,
    /// I05 - Interchange ID Qualifier
    ///
    /// Qualifier to designate the system/method of code structure used to designate the sender or receiver ID element being qualified
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=2
    #[serde(rename = "07")]
    pub _07: element::I05,
    /// I07 - Interchange Receiver ID
    ///
    /// Identification code published by the receiver of the data; When sending, it is used by the sender as their sending ID, thus other parties sending to them will use this as a receiving ID to route data to them
    /// - TYPE=AN
    /// - MIN=15
    /// - MAX=15
    #[validate(length(equal = 15, message = "ISA 08 (I07) must be 15 characters long"))]
    #[serde(rename = "08")]
    pub _08: String,
    /// I08 - Interchange Date
    ///
    /// Date of the interchange
    /// - TYPE=DT
    /// - MIN=6
    /// - MAX=6
    #[serde(rename = "09")]
    pub _09: element::I08,
    /// I09 - Interchange Time
    ///
    /// Time of the interchange
    /// - TYPE=TM
    /// - MIN=4
    /// - MAX=4
    #[serde(rename = "10")]
    pub _10: element::I09,
    /// I10 - Interchange Control Standards Identifier
    ///
    /// Code to identify the agency responsible for the control standard used by the message that is enclosed by the interchange header and trailer
    /// - TYPE=ID
    /// - MIN=1
    /// - MAX=1
    #[serde(rename = "11")]
    pub _11: element::I10,
    /// I11 - Interchange Control Version Number
    ///
    /// This version number covers the interchange control segments
    /// - TYPE=ID
    /// - MIN=5
    /// - MAX=5
    #[serde(rename = "12")]
    pub _12: element::I11,
    /// I12 - Interchange Control Number
    ///
    /// A control number assigned by the interchange sender
    /// - TYPE=N0
    /// - MIN=9
    /// - MAX=9
    #[serde(rename = "13")]
    pub _13: element::I12,
    /// I13 - Acknowledgment Requested
    ///
    /// Code sent by the sender to request an interchange acknowledgment (TA1)
    /// - TYPE=ID
    /// - MIN=1
    /// - MAX=1
    #[serde(rename = "14")]
    pub _14: element::I13,
    /// I14 - Usage Indicator
    ///
    /// Code to indicate whether data enclosed by this interchange envelope is test, production or information
    /// - TYPE=ID
    /// - MIN=1
    /// - MAX=1
    #[serde(rename = "15")]
    pub _15: element::I14,
    /// I15 - Component Element Separator
    ///
    /// Type is not applicable; the component element separator is a delimiter and not a data element; this field provides the delimiter used to separate component data elements within a composite data structure; this value must be different than the data element separator and the segment terminator
    /// - TYPE=
    /// - MIN=1
    /// - MAX=1
    #[validate(length(equal = 1, message = "ISA 16 (I15) must be 1 characters long"))]
    #[serde(rename = "16")]
    pub _16: String,
}

#[allow(clippy::just_underscores_and_digits)]
impl<'a> Parser<&'a str, ISA, nom::error::Error<&'a str>> for ISA {
    fn parse(input: &'a str) -> IResult<&'a str, ISA> {
        // The ISA segment has 16 fields separated by the element separator.
        // The element separator is typically '*' but can be other characters.
        // The last field (ISA16) is the component element separator, followed by the segment terminator (usually '~').
        // We need to parse the segment so that the ISA16 value is not swallowed by the segment terminator.

        // 1. Find the element separator (ISA is always the first segment, so the 4th character is the element separator)
        let element_sep = input
            .chars()
            .nth(3)
            .ok_or_else(|| nom::Err::Error(nom::error::Error::new(input, ErrorKind::Char)))?;

        // 2. Find the start of the ISA segment
        let input = input
            .strip_prefix("ISA")
            .ok_or_else(|| nom::Err::Error(nom::error::Error::new(input, ErrorKind::Tag)))?;

        // 3. Remove the segment tag and the first element separator
        let input = input
            .strip_prefix(element_sep)
            .ok_or_else(|| nom::Err::Error(nom::error::Error::new(input, ErrorKind::Char)))?;

        // 4. Parse the 15 fields (ISA01-ISA15), separated by the element separator
        let mut remaining_input = input;
        let mut fields = Vec::with_capacity(15);

        // Parse fields 1-15
        for field_num in 1..=15 {
            let field_end = remaining_input.find(element_sep).ok_or_else(|| {
                nom::Err::Error(nom::error::Error::new(remaining_input, ErrorKind::Char))
            })?;

            if field_end == 0 && field_num > 1 {
                // Empty field is only allowed for certain fields, but we'll be permissive
                fields.push(String::new());
            } else {
                let field_value = remaining_input[..field_end].to_string();
                fields.push(field_value);
            }

            remaining_input = &remaining_input[field_end + 1..];
        }

        // 5. Parse ISA16 - Component Element Separator and determine segment terminator
        // The component separator is 1 character, followed by the segment terminator
        // The segment terminator can be any character that's not the component separator

        let mut chars = remaining_input.chars();
        let component_separator_char = chars.next().ok_or_else(|| {
            nom::Err::Error(nom::error::Error::new(remaining_input, ErrorKind::Char))
        })?;

        let segment_terminator = chars.next().ok_or_else(|| {
            nom::Err::Error(nom::error::Error::new(remaining_input, ErrorKind::Char))
        })?;

        // Calculate how many bytes to skip (component separator + segment terminator)
        let component_separator_bytes = component_separator_char.len_utf8();
        let segment_terminator_bytes = segment_terminator.len_utf8();
        let remaining_input =
            &remaining_input[component_separator_bytes + segment_terminator_bytes..];

        let component_separator = component_separator_char.to_string();

        // 6. Handle potential newline after segment terminator (if segment terminator is not already a newline)
        let (remaining_input, _) = if segment_terminator != '\n' {
            // Only try to consume a newline if there's actually input remaining and it starts with \n
            if let Some(stripped) = remaining_input.strip_prefix('\n') {
                (stripped, Some('\n'))
            } else {
                (remaining_input, None)
            }
        } else {
            // If segment terminator is already a newline, don't consume another one
            (remaining_input, None)
        };

        // 8. Validate we have exactly 15 fields
        if fields.len() != 15 {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                ErrorKind::Count,
            )));
        }

        // 9. Build the ISA struct with proper field assignment
        Ok((
            remaining_input,
            ISA {
                _01: <element::I01 as crate::util::X12Element>::from_x12(&fields[0]),
                _02: fields[1].clone(),
                _03: <element::I03 as crate::util::X12Element>::from_x12(&fields[2]),
                _04: fields[3].clone(),
                _05: <element::I05 as crate::util::X12Element>::from_x12(&fields[4]),
                _06: fields[5].clone(),
                _07: <element::I05 as crate::util::X12Element>::from_x12(&fields[6]),
                _08: fields[7].clone(),
                _09: <element::I08 as crate::util::X12Element>::from_x12(&fields[8]),
                _10: <element::I09 as crate::util::X12Element>::from_x12(&fields[9]),
                _11: <element::I10 as crate::util::X12Element>::from_x12(&fields[10]),
                _12: <element::I11 as crate::util::X12Element>::from_x12(&fields[11]),
                _13: <element::I12 as crate::util::X12Element>::from_x12(&fields[12]),
                _14: <element::I13 as crate::util::X12Element>::from_x12(&fields[13]),
                _15: <element::I14 as crate::util::X12Element>::from_x12(&fields[14]),
                _16: component_separator.to_string(),
            },
        ))
    }
}

/// ISS - Invoice Shipment Summary
///
/// To specify summary details of total items shipped in terms of quantity, weight, and volume
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct ISS {
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
}

/// IT1 - Baseline Item Data (Invoice)
///
/// To specify the basic and most frequently used line item data for the invoice and related transactions
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct IT1 {
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
}

/// IT3 - Additional Item Data
///
/// To specify additional item details relating to variations between ordered and shipped quantities, or to specify alternate units of measures and quantities
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct IT3 {
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

/// ITD - Terms of Sale/Deferred Terms of Sale
///
/// To specify terms of sale
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct ITD {
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
