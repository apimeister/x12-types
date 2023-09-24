use crate::util::{self, Parser};
use nom::IResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// ACT - Account Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct ACT {
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

impl<'a> Parser<&'a str, ACT, nom::error::Error<&'a str>> for ACT {
    fn parse(input: &'a str) -> IResult<&'a str, ACT> {
        let (rest, vars) = crate::util::parse_line(input, "ACT")?;
        let obj = ACT {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// AD1 - Adjustment Amount
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct AD1 {
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

impl<'a> Parser<&'a str, AD1, nom::error::Error<&'a str>> for AD1 {
    fn parse(input: &'a str) -> IResult<&'a str, AD1> {
        let (rest, vars) = crate::util::parse_line(input, "AD1")?;
        let obj = AD1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// AIN - Income
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct AIN {
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
}

impl<'a> Parser<&'a str, AIN, nom::error::Error<&'a str>> for AIN {
    fn parse(input: &'a str) -> IResult<&'a str, AIN> {
        let (rest, vars) = crate::util::parse_line(input, "AIN")?;
        let obj = AIN {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
            _10: vars.get(9).map(util::unborrow_string),
            _11: vars.get(10).map(util::unborrow_string),
            _12: vars.get(11).map(util::unborrow_string),
            _13: vars.get(12).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// AMT - Monitary Amount Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct AMT {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, AMT, nom::error::Error<&'a str>> for AMT {
    fn parse(input: &'a str) -> IResult<&'a str, AMT> {
        let (rest, vars) = crate::util::parse_line(input, "AMT")?;
        let obj = AMT {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// BEN - Financial Contribution
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct BEN {
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

impl<'a> Parser<&'a str, BEN, nom::error::Error<&'a str>> for BEN {
    fn parse(input: &'a str) -> IResult<&'a str, BEN> {
        let (rest, vars) = crate::util::parse_line(input, "BEN")?;
        let obj = BEN {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// BGN - Beginning Segment
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct BGN {
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
}

impl<'a> Parser<&'a str, BGN, nom::error::Error<&'a str>> for BGN {
    fn parse(input: &'a str) -> IResult<&'a str, BGN> {
        let (rest, vars) = crate::util::parse_line(input, "BGN")?;
        let obj = BGN {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// COB - Coordination of Benefits
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct COB {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, COB, nom::error::Error<&'a str>> for COB {
    fn parse(input: &'a str) -> IResult<&'a str, COB> {
        let (rest, vars) = crate::util::parse_line(input, "COB")?;
        let obj = COB {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// DMG - Demographic Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct DMG {
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

impl<'a> Parser<&'a str, DMG, nom::error::Error<&'a str>> for DMG {
    fn parse(input: &'a str) -> IResult<&'a str, DMG> {
        let (rest, vars) = crate::util::parse_line(input, "DMG")?;
        let obj = DMG {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
            _10: vars.get(9).map(util::unborrow_string),
            _11: vars.get(10).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// DSB - Disability Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct DSB {
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

impl<'a> Parser<&'a str, DSB, nom::error::Error<&'a str>> for DSB {
    fn parse(input: &'a str) -> IResult<&'a str, DSB> {
        let (rest, vars) = crate::util::parse_line(input, "DSB")?;
        let obj = DSB {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// DTP - Date or Time or Period
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct DTP {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
}

impl<'a> Parser<&'a str, DTP, nom::error::Error<&'a str>> for DTP {
    fn parse(input: &'a str) -> IResult<&'a str, DTP> {
        let (rest, vars) = crate::util::parse_line(input, "DTP")?;
        let obj = DTP {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// EC - Employment Class
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct EC {
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

impl<'a> Parser<&'a str, EC, nom::error::Error<&'a str>> for EC {
    fn parse(input: &'a str) -> IResult<&'a str, EC> {
        let (rest, vars) = crate::util::parse_line(input, "EC")?;
        let obj = EC {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// ENT - Entity
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct ENT {
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

impl<'a> Parser<&'a str, ENT, nom::error::Error<&'a str>> for ENT {
    fn parse(input: &'a str) -> IResult<&'a str, ENT> {
        let (rest, vars) = crate::util::parse_line(input, "ENT")?;
        let obj = ENT {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// FC - Financial Contribution
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct FC {
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

impl<'a> Parser<&'a str, FC, nom::error::Error<&'a str>> for FC {
    fn parse(input: &'a str) -> IResult<&'a str, FC> {
        let (rest, vars) = crate::util::parse_line(input, "FC")?;
        let obj = FC {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// FSA - Flexible Spending Amount
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct FSA {
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

impl<'a> Parser<&'a str, FSA, nom::error::Error<&'a str>> for FSA {
    fn parse(input: &'a str) -> IResult<&'a str, FSA> {
        let (rest, vars) = crate::util::parse_line(input, "FSA")?;
        let obj = FSA {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// GE - Functional Group Trailer
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct GE {
    #[serde(rename = "01")]
    pub _01: String,
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

/// GS - FunctionalGroup Header
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct GS {
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

/// HD - Health Coverage
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct HD {
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

impl<'a> Parser<&'a str, HD, nom::error::Error<&'a str>> for HD {
    fn parse(input: &'a str) -> IResult<&'a str, HD> {
        let (rest, vars) = crate::util::parse_line(input, "HD")?;
        let obj = HD {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
            _10: vars.get(9).map(util::unborrow_string),
            _11: vars.get(10).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// HI - Health Care Information Codes
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct HI {
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

impl<'a> Parser<&'a str, HI, nom::error::Error<&'a str>> for HI {
    fn parse(input: &'a str) -> IResult<&'a str, HI> {
        let (rest, vars) = crate::util::parse_line(input, "HI")?;
        let obj = HI {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
            _10: vars.get(9).map(util::unborrow_string),
            _11: vars.get(10).map(util::unborrow_string),
            _12: vars.get(11).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// HLH - Health Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct HLH {
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

impl<'a> Parser<&'a str, HLH, nom::error::Error<&'a str>> for HLH {
    fn parse(input: &'a str) -> IResult<&'a str, HLH> {
        let (rest, vars) = crate::util::parse_line(input, "HLH")?;
        let obj = HLH {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// ICM - Individual Income
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct ICM {
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

impl<'a> Parser<&'a str, ICM, nom::error::Error<&'a str>> for ICM {
    fn parse(input: &'a str) -> IResult<&'a str, ICM> {
        let (rest, vars) = crate::util::parse_line(input, "ICM")?;
        let obj = ICM {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// IDC - Health Coverage
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct IDC {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, IDC, nom::error::Error<&'a str>> for IDC {
    fn parse(input: &'a str) -> IResult<&'a str, IDC> {
        let (rest, vars) = crate::util::parse_line(input, "IDC")?;
        let obj = IDC {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
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

/// INS - Insured Benefit
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct INS {
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
    #[serde(rename = "17")]
    pub _17: Option<String>,
}

impl<'a> Parser<&'a str, INS, nom::error::Error<&'a str>> for INS {
    fn parse(input: &'a str) -> IResult<&'a str, INS> {
        let (rest, vars) = crate::util::parse_line(input, "INS")?;
        let obj = INS {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
            _10: vars.get(9).map(util::unborrow_string),
            _11: vars.get(10).map(util::unborrow_string),
            _12: vars.get(11).map(util::unborrow_string),
            _13: vars.get(12).map(util::unborrow_string),
            _14: vars.get(13).map(util::unborrow_string),
            _15: vars.get(14).map(util::unborrow_string),
            _16: vars.get(15).map(util::unborrow_string),
            _17: vars.get(16).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// INV - Incestment Vehicle Selection
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct INV {
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

impl<'a> Parser<&'a str, INV, nom::error::Error<&'a str>> for INV {
    fn parse(input: &'a str) -> IResult<&'a str, INV> {
        let (rest, vars) = crate::util::parse_line(input, "INV")?;
        let obj = INV {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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

/// K3 - File Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct K3 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, K3, nom::error::Error<&'a str>> for K3 {
    fn parse(input: &'a str) -> IResult<&'a str, K3> {
        let (rest, vars) = crate::util::parse_line(input, "K3")?;
        let obj = K3 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// LC - Life Coverage
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct LC {
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

impl<'a> Parser<&'a str, LC, nom::error::Error<&'a str>> for LC {
    fn parse(input: &'a str) -> IResult<&'a str, LC> {
        let (rest, vars) = crate::util::parse_line(input, "LC")?;
        let obj = LC {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// LE - Loop Trailer
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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

/// LS - Loop Header
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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

/// LUI - Language Use
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct LUI {
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

impl<'a> Parser<&'a str, LUI, nom::error::Error<&'a str>> for LUI {
    fn parse(input: &'a str) -> IResult<&'a str, LUI> {
        let (rest, vars) = crate::util::parse_line(input, "LUI")?;
        let obj = LUI {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// LX - Transaction Set Line Number
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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

/// N1 - Party Identifier
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// N2 - Additional Name Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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
            _02: vars.get(1).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// N3 - Party Location
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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
            _02: vars.get(1).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// N4 - Geographic Location
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// NM1 - Individual or Organizational Name
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct NM1 {
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

impl<'a> Parser<&'a str, NM1, nom::error::Error<&'a str>> for NM1 {
    fn parse(input: &'a str) -> IResult<&'a str, NM1> {
        let (rest, vars) = crate::util::parse_line(input, "NM1")?;
        let obj = NM1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
            _10: vars.get(9).map(util::unborrow_string),
            _11: vars.get(10).map(util::unborrow_string),
            _12: vars.get(11).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// NX1 - Property or Entity Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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

impl<'a> Parser<&'a str, NX1, nom::error::Error<&'a str>> for NX1 {
    fn parse(input: &'a str) -> IResult<&'a str, NX1> {
        let (rest, vars) = crate::util::parse_line(input, "NX1")?;
        let obj = NX1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// PER - Administrative Communications Contact
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// PLA - Place or Location
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct PLA {
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
}

impl<'a> Parser<&'a str, PLA, nom::error::Error<&'a str>> for PLA {
    fn parse(input: &'a str) -> IResult<&'a str, PLA> {
        let (rest, vars) = crate::util::parse_line(input, "PLA")?;
        let obj = PLA {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// PM - Electronic Funds Transfer Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct PM {
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
}

impl<'a> Parser<&'a str, PM, nom::error::Error<&'a str>> for PM {
    fn parse(input: &'a str) -> IResult<&'a str, PM> {
        let (rest, vars) = crate::util::parse_line(input, "PM")?;
        let obj = PM {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// PRV - Provider Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct PRV {
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

impl<'a> Parser<&'a str, PRV, nom::error::Error<&'a str>> for PRV {
    fn parse(input: &'a str) -> IResult<&'a str, PRV> {
        let (rest, vars) = crate::util::parse_line(input, "PRV")?;
        let obj = PRV {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// REF - Reference Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// REL - Relationship
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct REL {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

impl<'a> Parser<&'a str, REL, nom::error::Error<&'a str>> for REL {
    fn parse(input: &'a str) -> IResult<&'a str, REL> {
        let (rest, vars) = crate::util::parse_line(input, "REL")?;
        let obj = REL {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// RP - Retirement Product
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct RP {
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

impl<'a> Parser<&'a str, RP, nom::error::Error<&'a str>> for RP {
    fn parse(input: &'a str) -> IResult<&'a str, RP> {
        let (rest, vars) = crate::util::parse_line(input, "RP")?;
        let obj = RP {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
            _08: vars.get(7).map(util::unborrow_string),
            _09: vars.get(8).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// SE - Transaction Set Trailer
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct SE {
    #[serde(rename = "01")]
    pub _01: String,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct ST {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, ST, nom::error::Error<&'a str>> for ST {
    fn parse(input: &'a str) -> IResult<&'a str, ST> {
        let (rest, vars) = crate::util::parse_line(input, "ST")?;
        let obj = ST {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// QTY - Beginning Segment
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
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
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}
