use nom::IResult;
use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::DisplaySegment;

use crate::util::Parser;

/// BL - Billing Information
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
            _17: vars.get(16).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// BNX - Rail Shipment Information
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

/// CD - Shipment Condition
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct CD {
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

impl<'a> Parser<&'a str, CD, nom::error::Error<&'a str>> for CD {
    fn parse(input: &'a str) -> IResult<&'a str, CD> {
        let (rest, vars) = crate::util::parse_line(input, "CD")?;
        let obj = CD {
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

/// CM - Cargo Manifest
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct CM {
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct D9 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
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
}

impl<'a> Parser<&'a str, D9, nom::error::Error<&'a str>> for D9 {
    fn parse(input: &'a str) -> IResult<&'a str, D9> {
        let (rest, vars) = crate::util::parse_line(input, "D9")?;
        let obj = D9 {
            _01: vars.first().map(|x| x.to_string()),
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
            _12: vars.get(12).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// DTM - Date/Time Reference
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct DTM {
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

impl<'a> Parser<&'a str, DTM, nom::error::Error<&'a str>> for DTM {
    fn parse(input: &'a str) -> IResult<&'a str, DTM> {
        let (rest, vars) = crate::util::parse_line(input, "DTM")?;
        let obj = DTM {
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

/// E1 - Empty Car Disposition - Pended Destination Consignee
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct E1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, E1, nom::error::Error<&'a str>> for E1 {
    fn parse(input: &'a str) -> IResult<&'a str, E1> {
        let (rest, vars) = crate::util::parse_line(input, "E1")?;
        let obj = E1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// E4 - Empty Car Disposition - Pended Destination City
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct E4 {
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
}

impl<'a> Parser<&'a str, E4, nom::error::Error<&'a str>> for E4 {
    fn parse(input: &'a str) -> IResult<&'a str, E4> {
        let (rest, vars) = crate::util::parse_line(input, "E4")?;
        let obj = E4 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
            _05: vars.get(4).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// E5 - Empty Car Disposition - Pended Destination Route
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct E5 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, E5, nom::error::Error<&'a str>> for E5 {
    fn parse(input: &'a str) -> IResult<&'a str, E5> {
        let (rest, vars) = crate::util::parse_line(input, "E5")?;
        let obj = E5 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// EM - Equipment Characteristics
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
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct F9 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
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
}

impl<'a> Parser<&'a str, F9, nom::error::Error<&'a str>> for F9 {
    fn parse(input: &'a str) -> IResult<&'a str, F9> {
        let (rest, vars) = crate::util::parse_line(input, "F9")?;
        let obj = F9 {
            _01: vars.first().map(|x| x.to_string()),
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
        };
        Ok((rest, obj))
    }
}

/// GA - Price Authority Information
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
            _18: vars.get(17).map(|x| x.to_string()),
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

/// H3 - Special Handling Instructions
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
    #[serde(rename = "09")]
    pub _09: Option<String>,
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
            _09: vars.get(8).map(|x| x.to_string()),
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

/// IM - Intermodal Movement Information
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
    Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment,
)]
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

/// L0 - Line Item - Quantity and Weight
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

/// L1 - Rates and Charges
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
    #[serde(rename = "22")]
    pub _22: Option<String>,
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
            _22: vars.get(21).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// L3 - Total Weight and Charges
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

/// L5 - Description, Marks and Numbers
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

/// LE - Loop Trailer
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

/// LFH - Free-form Hazardous Material Information
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
    #[serde(rename = "08")]
    pub _08: Option<String>,
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
            _08: vars.get(7).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// LH1 - Hazardous Identification Information
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
    #[serde(rename = "12")]
    pub _12: Option<String>,
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
            _12: vars.get(11).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// LH2 - Hazardous Identification Information
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
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
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
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// LH3 - Hazardous Material Shipping Name Information
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LH4 {
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LH6 {
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

/// LHR - Hazardous Material Identification Reference Numbers
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct LHR {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
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

/// LHT - Hazardous Material Identification Reference Numbers
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

/// LX - Transaction Set Line Number
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

/// M1 - Insurance
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct M1 {
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

impl<'a> Parser<&'a str, M1, nom::error::Error<&'a str>> for M1 {
    fn parse(input: &'a str) -> IResult<&'a str, M1> {
        let (rest, vars) = crate::util::parse_line(input, "M1")?;
        let obj = M1 {
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

/// M12 - In-bond Indifying Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
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
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
            _14: vars.get(13).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// M3 - Release
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, M3, nom::error::Error<&'a str>> for M3 {
    fn parse(input: &'a str) -> IResult<&'a str, M3> {
        let (rest, vars) = crate::util::parse_line(input, "M3")?;
        let obj = M3 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(|x| x.to_string()),
            _03: vars.get(2).map(|x| x.to_string()),
            _04: vars.get(3).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// M7 - Seal Numbers
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

/// MEA - Measurements
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
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
            _11: vars.get(10).map(|x| x.to_string()),
            _12: vars.get(11).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// N1 - Party Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct N1 {
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

/// N10 - Quantity and Description
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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

/// N2 - Additional Name Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct N2 {
    #[serde(rename = "01")]
    pub _01: String,
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

/// N3 - Party Location
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct N4 {
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
            _07: vars.get(6).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// N5 - Equipment Ordered
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct N7 {
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

impl<'a> Parser<&'a str, N7, nom::error::Error<&'a str>> for N7 {
    fn parse(input: &'a str) -> IResult<&'a str, N7> {
        let (rest, vars) = crate::util::parse_line(input, "N7")?;
        let obj = N7 {
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
            _21: vars.get(20).map(|x| x.to_string()),
            _22: vars.get(21).map(|x| x.to_string()),
            _23: vars.get(22).map(|x| x.to_string()),
            _24: vars.get(23).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// N9 - Extended Reference Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct N9 {
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

impl<'a> Parser<&'a str, N9, nom::error::Error<&'a str>> for N9 {
    fn parse(input: &'a str) -> IResult<&'a str, N9> {
        let (rest, vars) = crate::util::parse_line(input, "N9")?;
        let obj = N9 {
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

/// NA - Cross-Reference Equipment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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
            _12: vars.get(11).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// PER - Administrative Communication Contract
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct PER {
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

/// PI - Price Authority Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct PI {
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
            _16: vars.get(15).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// PS - Protective Service Instructions
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct PS {
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
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
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

/// R2 - Route Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct R2 {
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
    pub _12: Option<String>,
    #[serde(rename = "13")]
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

/// R9 - Code Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct R9 {
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
            _09: vars.get(8).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// REF - Party Identification
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
    pub _06: String,
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
            _06: vars.get(5).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// S2 - Stop-off Address
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

/// S9 - Stop-off Name
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct S9 {
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
    pub _06: String,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct T1 {
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

impl<'a> Parser<&'a str, T1, nom::error::Error<&'a str>> for T1 {
    fn parse(input: &'a str) -> IResult<&'a str, T1> {
        let (rest, vars) = crate::util::parse_line(input, "T1")?;
        let obj = T1 {
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

/// T2 - Transit Inbound Lading
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
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
}

impl<'a> Parser<&'a str, T2, nom::error::Error<&'a str>> for T2 {
    fn parse(input: &'a str) -> IResult<&'a str, T2> {
        let (rest, vars) = crate::util::parse_line(input, "T2")?;
        let obj = T2 {
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

/// T3 - Transit Inbound Route
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct T3 {
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

impl<'a> Parser<&'a str, T3, nom::error::Error<&'a str>> for T3 {
    fn parse(input: &'a str) -> IResult<&'a str, T3> {
        let (rest, vars) = crate::util::parse_line(input, "T3")?;
        let obj = T3 {
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

/// T6 - Transit Inbound Rates
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct T6 {
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

impl<'a> Parser<&'a str, T6, nom::error::Error<&'a str>> for T6 {
    fn parse(input: &'a str) -> IResult<&'a str, T6> {
        let (rest, vars) = crate::util::parse_line(input, "T6")?;
        let obj = T6 {
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

/// T8 - Transit Inbound Rates
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct T8 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

impl<'a> Parser<&'a str, T8, nom::error::Error<&'a str>> for T8 {
    fn parse(input: &'a str) -> IResult<&'a str, T8> {
        let (rest, vars) = crate::util::parse_line(input, "T8")?;
        let obj = T8 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// VC - Motor Vehicle Control
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
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
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
            _12: vars.get(11).map(|x| x.to_string()),
            _13: vars.get(12).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}

/// X1 - Export License
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct X1 {
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
}

impl<'a> Parser<&'a str, X1, nom::error::Error<&'a str>> for X1 {
    fn parse(input: &'a str) -> IResult<&'a str, X1> {
        let (rest, vars) = crate::util::parse_line(input, "X1")?;
        let obj = X1 {
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

/// X7 - Customs Information
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

/// ZC1 - Beginning Segment for Data Correction or Change
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct ZC1 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
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
    pub _09: Option<String>,
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
            _08: vars.get(7).unwrap().to_string(),
            _09: vars.get(8).map(|x| x.to_string()),
        };
        Ok((rest, obj))
    }
}
