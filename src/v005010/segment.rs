use crate::util::{self, Parser};
use nom::IResult;
use serde::{Deserialize, Serialize};
use validator::Validate;
use x12_types_macros::DisplaySegment;

/// ACT - Account Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// BHT - Beginning of Hierarchical Transaction
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct BHT {
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

impl<'a> Parser<&'a str, BHT, nom::error::Error<&'a str>> for BHT {
    fn parse(input: &'a str) -> IResult<&'a str, BHT> {
        let (rest, vars) = crate::util::parse_line(input, "BHT")?;
        let obj = BHT {
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

/// CAS - Claims Adjustment
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CAS {
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

impl<'a> Parser<&'a str, CAS, nom::error::Error<&'a str>> for CAS {
    fn parse(input: &'a str) -> IResult<&'a str, CAS> {
        let (rest, vars) = crate::util::parse_line(input, "CAS")?;
        let obj = CAS {
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
            _14: vars.get(13).map(util::unborrow_string),
            _15: vars.get(14).map(util::unborrow_string),
            _16: vars.get(15).map(util::unborrow_string),
            _17: vars.get(16).map(util::unborrow_string),
            _18: vars.get(17).map(util::unborrow_string),
            _19: vars.get(18).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// CL1 - Claim Codes
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CL1 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, CL1, nom::error::Error<&'a str>> for CL1 {
    fn parse(input: &'a str) -> IResult<&'a str, CL1> {
        let (rest, vars) = crate::util::parse_line(input, "CL1")?;
        let obj = CL1 {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// CLM - Health Claim
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CLM {
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
}

impl<'a> Parser<&'a str, CLM, nom::error::Error<&'a str>> for CLM {
    fn parse(input: &'a str) -> IResult<&'a str, CLM> {
        let (rest, vars) = crate::util::parse_line(input, "CLM")?;
        let obj = CLM {
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
            _13: vars.get(12).map(util::unborrow_string),
            _14: vars.get(13).map(util::unborrow_string),
            _15: vars.get(14).map(util::unborrow_string),
            _16: vars.get(15).map(util::unborrow_string),
            _17: vars.get(16).map(util::unborrow_string),
            _18: vars.get(17).map(util::unborrow_string),
            _19: vars.get(18).map(util::unborrow_string),
            _20: vars.get(19).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// CN1 - Contract Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CN1 {
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

impl<'a> Parser<&'a str, CN1, nom::error::Error<&'a str>> for CN1 {
    fn parse(input: &'a str) -> IResult<&'a str, CN1> {
        let (rest, vars) = crate::util::parse_line(input, "CN1")?;
        let obj = CN1 {
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

/// COB - Coordination of Benefits
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// CR1 - Ambulance Certification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CR1 {
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

impl<'a> Parser<&'a str, CR1, nom::error::Error<&'a str>> for CR1 {
    fn parse(input: &'a str) -> IResult<&'a str, CR1> {
        let (rest, vars) = crate::util::parse_line(input, "CR1")?;
        let obj = CR1 {
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
        };
        Ok((rest, obj))
    }
}

/// CR2 - Chiropractic Certification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CR2 {
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

impl<'a> Parser<&'a str, CR2, nom::error::Error<&'a str>> for CR2 {
    fn parse(input: &'a str) -> IResult<&'a str, CR2> {
        let (rest, vars) = crate::util::parse_line(input, "CR2")?;
        let obj = CR2 {
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
            _12: vars.get(11).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// CR3 - Durable Medical Equipment Certification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CR3 {
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

impl<'a> Parser<&'a str, CR3, nom::error::Error<&'a str>> for CR3 {
    fn parse(input: &'a str) -> IResult<&'a str, CR3> {
        let (rest, vars) = crate::util::parse_line(input, "CR3")?;
        let obj = CR3 {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// CR4 - Enteral or Parenteral Therapy Certification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CR4 {
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
    #[serde(rename = "25")]
    pub _25: Option<String>,
    #[serde(rename = "26")]
    pub _26: Option<String>,
    #[serde(rename = "27")]
    pub _27: Option<String>,
    #[serde(rename = "28")]
    pub _28: Option<String>,
    #[serde(rename = "29")]
    pub _29: Option<String>,
}

impl<'a> Parser<&'a str, CR4, nom::error::Error<&'a str>> for CR4 {
    fn parse(input: &'a str) -> IResult<&'a str, CR4> {
        let (rest, vars) = crate::util::parse_line(input, "CR4")?;
        let obj = CR4 {
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
            _13: vars.get(12).map(util::unborrow_string),
            _14: vars.get(13).map(util::unborrow_string),
            _15: vars.get(14).map(util::unborrow_string),
            _16: vars.get(15).map(util::unborrow_string),
            _17: vars.get(16).map(util::unborrow_string),
            _18: vars.get(17).map(util::unborrow_string),
            _19: vars.get(18).map(util::unborrow_string),
            _20: vars.get(19).map(util::unborrow_string),
            _21: vars.get(20).map(util::unborrow_string),
            _22: vars.get(21).map(util::unborrow_string),
            _23: vars.get(22).map(util::unborrow_string),
            _24: vars.get(23).map(util::unborrow_string),
            _25: vars.get(24).map(util::unborrow_string),
            _26: vars.get(25).map(util::unborrow_string),
            _27: vars.get(26).map(util::unborrow_string),
            _28: vars.get(27).map(util::unborrow_string),
            _29: vars.get(28).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// CR5 - Oxygen Therapy Certification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CR5 {
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

impl<'a> Parser<&'a str, CR5, nom::error::Error<&'a str>> for CR5 {
    fn parse(input: &'a str) -> IResult<&'a str, CR5> {
        let (rest, vars) = crate::util::parse_line(input, "CR5")?;
        let obj = CR5 {
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
            _12: vars.get(11).map(util::unborrow_string),
            _13: vars.get(12).map(util::unborrow_string),
            _14: vars.get(13).map(util::unborrow_string),
            _15: vars.get(14).map(util::unborrow_string),
            _16: vars.get(15).map(util::unborrow_string),
            _17: vars.get(16).map(util::unborrow_string),
            _18: vars.get(17).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// CR6 - Home Health Care Certification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CR6 {
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
    #[serde(rename = "18")]
    pub _18: Option<String>,
    #[serde(rename = "19")]
    pub _19: Option<String>,
    #[serde(rename = "20")]
    pub _20: Option<String>,
    #[serde(rename = "21")]
    pub _21: Option<String>,
}

impl<'a> Parser<&'a str, CR6, nom::error::Error<&'a str>> for CR6 {
    fn parse(input: &'a str) -> IResult<&'a str, CR6> {
        let (rest, vars) = crate::util::parse_line(input, "CR6")?;
        let obj = CR6 {
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
            _18: vars.get(17).map(util::unborrow_string),
            _19: vars.get(18).map(util::unborrow_string),
            _20: vars.get(19).map(util::unborrow_string),
            _21: vars.get(20).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// CR7 - Home Health Treatment Plan Certification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CR7 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
}

impl<'a> Parser<&'a str, CR7, nom::error::Error<&'a str>> for CR7 {
    fn parse(input: &'a str) -> IResult<&'a str, CR7> {
        let (rest, vars) = crate::util::parse_line(input, "CR7")?;
        let obj = CR7 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// CR8 - Pacemaker Certification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CR8 {
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
}

impl<'a> Parser<&'a str, CR8, nom::error::Error<&'a str>> for CR8 {
    fn parse(input: &'a str) -> IResult<&'a str, CR8> {
        let (rest, vars) = crate::util::parse_line(input, "CR8")?;
        let obj = CR8 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).unwrap().to_string(),
            _06: vars.get(5).unwrap().to_string(),
            _07: vars.get(6).unwrap().to_string(),
            _08: vars.get(7).unwrap().to_string(),
            _09: vars.get(8).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// CRC - Conditions Indicator
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CRC {
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

impl<'a> Parser<&'a str, CRC, nom::error::Error<&'a str>> for CRC {
    fn parse(input: &'a str) -> IResult<&'a str, CRC> {
        let (rest, vars) = crate::util::parse_line(input, "CRC")?;
        let obj = CRC {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// CTP - Pricing Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CTP {
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

impl<'a> Parser<&'a str, CTP, nom::error::Error<&'a str>> for CTP {
    fn parse(input: &'a str) -> IResult<&'a str, CTP> {
        let (rest, vars) = crate::util::parse_line(input, "CTP")?;
        let obj = CTP {
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

/// CUR - Currency
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct CUR {
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
    #[serde(rename = "18")]
    pub _18: Option<String>,
    #[serde(rename = "19")]
    pub _19: Option<String>,
    #[serde(rename = "20")]
    pub _20: Option<String>,
    #[serde(rename = "21")]
    pub _21: Option<String>,
}

impl<'a> Parser<&'a str, CUR, nom::error::Error<&'a str>> for CUR {
    fn parse(input: &'a str) -> IResult<&'a str, CUR> {
        let (rest, vars) = crate::util::parse_line(input, "CUR")?;
        let obj = CUR {
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
            _18: vars.get(17).map(util::unborrow_string),
            _19: vars.get(18).map(util::unborrow_string),
            _20: vars.get(19).map(util::unborrow_string),
            _21: vars.get(20).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// DMG - Demographic Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// DN1 - Orthodontic Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct DN1 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

impl<'a> Parser<&'a str, DN1, nom::error::Error<&'a str>> for DN1 {
    fn parse(input: &'a str) -> IResult<&'a str, DN1> {
        let (rest, vars) = crate::util::parse_line(input, "DN1")?;
        let obj = DN1 {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// DN2 - Tooth Summary
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct DN2 {
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
    pub _06: String,
}

impl<'a> Parser<&'a str, DN2, nom::error::Error<&'a str>> for DN2 {
    fn parse(input: &'a str) -> IResult<&'a str, DN2> {
        let (rest, vars) = crate::util::parse_line(input, "DN2")?;
        let obj = DN2 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// DSB - Disability Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// FRM - Supporting Documentation
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct FRM {
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

impl<'a> Parser<&'a str, FRM, nom::error::Error<&'a str>> for FRM {
    fn parse(input: &'a str) -> IResult<&'a str, FRM> {
        let (rest, vars) = crate::util::parse_line(input, "FRM")?;
        let obj = FRM {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(1).map(util::unborrow_string),
            _04: vars.get(1).map(util::unborrow_string),
            _05: vars.get(1).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// FSA - Flexible Spending Amount
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// HCP - Health Care Pricing
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct HCP {
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

impl<'a> Parser<&'a str, HCP, nom::error::Error<&'a str>> for HCP {
    fn parse(input: &'a str) -> IResult<&'a str, HCP> {
        let (rest, vars) = crate::util::parse_line(input, "HCP")?;
        let obj = HCP {
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
            _12: vars.get(11).map(util::unborrow_string),
            _13: vars.get(12).map(util::unborrow_string),
            _14: vars.get(13).map(util::unborrow_string),
            _15: vars.get(14).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// HD - Health Coverage
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// HL - Hierarchical Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

impl<'a> Parser<&'a str, HL, nom::error::Error<&'a str>> for HL {
    fn parse(input: &'a str) -> IResult<&'a str, HL> {
        let (rest, vars) = crate::util::parse_line(input, "HL")?;
        let obj = HL {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// HLH - Health Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// HSD - Health Care Services Delivery
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct HSD {
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

impl<'a> Parser<&'a str, HSD, nom::error::Error<&'a str>> for HSD {
    fn parse(input: &'a str) -> IResult<&'a str, HSD> {
        let (rest, vars) = crate::util::parse_line(input, "HSD")?;
        let obj = HSD {
            _01: vars.first().map(util::unborrow_string),
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

/// ICM - Individual Income
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// IMM - Immunization Status
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct IMM {
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

impl<'a> Parser<&'a str, IMM, nom::error::Error<&'a str>> for IMM {
    fn parse(input: &'a str) -> IResult<&'a str, IMM> {
        let (rest, vars) = crate::util::parse_line(input, "IMM")?;
        let obj = IMM {
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

/// INS - Insured Benefit
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// K3 - File Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// LIN - Item Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct LIN {
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
    #[serde(rename = "26")]
    pub _26: Option<String>,
    #[serde(rename = "27")]
    pub _27: Option<String>,
    #[serde(rename = "28")]
    pub _28: Option<String>,
    #[serde(rename = "29")]
    pub _29: Option<String>,
    #[serde(rename = "30")]
    pub _30: Option<String>,
    #[serde(rename = "31")]
    pub _31: Option<String>,
}

impl<'a> Parser<&'a str, LIN, nom::error::Error<&'a str>> for LIN {
    fn parse(input: &'a str) -> IResult<&'a str, LIN> {
        let (rest, vars) = crate::util::parse_line(input, "LIN")?;
        let obj = LIN {
            _01: vars.first().map(util::unborrow_string),
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
            _14: vars.get(13).map(util::unborrow_string),
            _15: vars.get(14).map(util::unborrow_string),
            _16: vars.get(15).map(util::unborrow_string),
            _17: vars.get(16).map(util::unborrow_string),
            _18: vars.get(17).map(util::unborrow_string),
            _19: vars.get(18).map(util::unborrow_string),
            _20: vars.get(19).map(util::unborrow_string),
            _21: vars.get(20).map(util::unborrow_string),
            _22: vars.get(21).map(util::unborrow_string),
            _23: vars.get(22).map(util::unborrow_string),
            _24: vars.get(23).map(util::unborrow_string),
            _25: vars.get(24).map(util::unborrow_string),
            _26: vars.get(25).map(util::unborrow_string),
            _27: vars.get(26).map(util::unborrow_string),
            _28: vars.get(27).map(util::unborrow_string),
            _29: vars.get(28).map(util::unborrow_string),
            _30: vars.get(29).map(util::unborrow_string),
            _31: vars.get(30).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// LQ - Industry Code Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct LQ {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

impl<'a> Parser<&'a str, LQ, nom::error::Error<&'a str>> for LQ {
    fn parse(input: &'a str) -> IResult<&'a str, LQ> {
        let (rest, vars) = crate::util::parse_line(input, "LQ")?;
        let obj = LQ {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// LS - Loop Header
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// MEA - Measurements
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
            _12: vars.get(11).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// MIA - Medicare Inpatient Adjudication
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct MIA {
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

impl<'a> Parser<&'a str, MIA, nom::error::Error<&'a str>> for MIA {
    fn parse(input: &'a str) -> IResult<&'a str, MIA> {
        let (rest, vars) = crate::util::parse_line(input, "MIA")?;
        let obj = MIA {
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
            _13: vars.get(12).map(util::unborrow_string),
            _14: vars.get(13).map(util::unborrow_string),
            _15: vars.get(14).map(util::unborrow_string),
            _16: vars.get(15).map(util::unborrow_string),
            _17: vars.get(16).map(util::unborrow_string),
            _18: vars.get(17).map(util::unborrow_string),
            _19: vars.get(18).map(util::unborrow_string),
            _20: vars.get(19).map(util::unborrow_string),
            _21: vars.get(20).map(util::unborrow_string),
            _22: vars.get(21).map(util::unborrow_string),
            _23: vars.get(22).map(util::unborrow_string),
            _24: vars.get(23).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// MOA - Medicare Outpatient Adjudication
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct MOA {
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

impl<'a> Parser<&'a str, MOA, nom::error::Error<&'a str>> for MOA {
    fn parse(input: &'a str) -> IResult<&'a str, MOA> {
        let (rest, vars) = crate::util::parse_line(input, "MOA")?;
        let obj = MOA {
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

/// N1 - Party Identifier
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// NTE - Note/Special Instruction
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).unwrap().to_string(),
        };
        Ok((rest, obj))
    }
}

/// NX1 - Property or Entity Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// OI - Other Health Insurance Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct OI {
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

impl<'a> Parser<&'a str, OI, nom::error::Error<&'a str>> for OI {
    fn parse(input: &'a str) -> IResult<&'a str, OI> {
        let (rest, vars) = crate::util::parse_line(input, "OI")?;
        let obj = OI {
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

/// PAT - Patient Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct PAT {
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

impl<'a> Parser<&'a str, PAT, nom::error::Error<&'a str>> for PAT {
    fn parse(input: &'a str) -> IResult<&'a str, PAT> {
        let (rest, vars) = crate::util::parse_line(input, "PAT")?;
        let obj = PAT {
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

/// PER - Administrative Communications Contact
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// PS1 - Purchase Service
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct PS1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, PS1, nom::error::Error<&'a str>> for PS1 {
    fn parse(input: &'a str) -> IResult<&'a str, PS1> {
        let (rest, vars) = crate::util::parse_line(input, "PS1")?;
        let obj = PS1 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// PWK - Disability Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct PWK {
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

impl<'a> Parser<&'a str, PWK, nom::error::Error<&'a str>> for PWK {
    fn parse(input: &'a str) -> IResult<&'a str, PWK> {
        let (rest, vars) = crate::util::parse_line(input, "PWK")?;
        let obj = PWK {
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

/// QTY - Beginning Segment
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// REF - Reference Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// SBR - Subscriber Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct SBR {
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

impl<'a> Parser<&'a str, SBR, nom::error::Error<&'a str>> for SBR {
    fn parse(input: &'a str) -> IResult<&'a str, SBR> {
        let (rest, vars) = crate::util::parse_line(input, "SBR")?;
        let obj = SBR {
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
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

/// SV1 - Professional Service
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct SV1 {
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
}

impl<'a> Parser<&'a str, SV1, nom::error::Error<&'a str>> for SV1 {
    fn parse(input: &'a str) -> IResult<&'a str, SV1> {
        let (rest, vars) = crate::util::parse_line(input, "SV1")?;
        let obj = SV1 {
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
            _13: vars.get(12).map(util::unborrow_string),
            _14: vars.get(13).map(util::unborrow_string),
            _15: vars.get(14).map(util::unborrow_string),
            _16: vars.get(15).map(util::unborrow_string),
            _17: vars.get(16).map(util::unborrow_string),
            _18: vars.get(17).map(util::unborrow_string),
            _19: vars.get(18).map(util::unborrow_string),
            _20: vars.get(19).map(util::unborrow_string),
            _21: vars.get(20).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// SV2 - Institutional Service
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct SV2 {
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
}

impl<'a> Parser<&'a str, SV2, nom::error::Error<&'a str>> for SV2 {
    fn parse(input: &'a str) -> IResult<&'a str, SV2> {
        let (rest, vars) = crate::util::parse_line(input, "SV2")?;
        let obj = SV2 {
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
        };
        Ok((rest, obj))
    }
}

/// SV3 - Dental Service
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct SV3 {
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
}

impl<'a> Parser<&'a str, SV3, nom::error::Error<&'a str>> for SV3 {
    fn parse(input: &'a str) -> IResult<&'a str, SV3> {
        let (rest, vars) = crate::util::parse_line(input, "SV3")?;
        let obj = SV3 {
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
        };
        Ok((rest, obj))
    }
}

/// SV4 - Drug Service
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct SV4 {
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
}

impl<'a> Parser<&'a str, SV4, nom::error::Error<&'a str>> for SV4 {
    fn parse(input: &'a str) -> IResult<&'a str, SV4> {
        let (rest, vars) = crate::util::parse_line(input, "SV4")?;
        let obj = SV4 {
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
            _13: vars.get(12).map(util::unborrow_string),
            _14: vars.get(13).map(util::unborrow_string),
            _15: vars.get(14).map(util::unborrow_string),
            _16: vars.get(15).map(util::unborrow_string),
            _17: vars.get(16).map(util::unborrow_string),
            _18: vars.get(17).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// SV5 - Durable Medical Equipment Service
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct SV5 {
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

impl<'a> Parser<&'a str, SV5, nom::error::Error<&'a str>> for SV5 {
    fn parse(input: &'a str) -> IResult<&'a str, SV5> {
        let (rest, vars) = crate::util::parse_line(input, "SV5")?;
        let obj = SV5 {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).unwrap().to_string(),
            _04: vars.get(3).map(util::unborrow_string),
            _05: vars.get(4).map(util::unborrow_string),
            _06: vars.get(5).map(util::unborrow_string),
            _07: vars.get(6).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// SV6 - Anesthesia Service
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct SV6 {
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

impl<'a> Parser<&'a str, SV6, nom::error::Error<&'a str>> for SV6 {
    fn parse(input: &'a str) -> IResult<&'a str, SV6> {
        let (rest, vars) = crate::util::parse_line(input, "SV6")?;
        let obj = SV6 {
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

/// SV7 - Drug Adjudication
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct SV7 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: String,
    #[serde(rename = "05")]
    pub _05: String,
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

impl<'a> Parser<&'a str, SV7, nom::error::Error<&'a str>> for SV7 {
    fn parse(input: &'a str) -> IResult<&'a str, SV7> {
        let (rest, vars) = crate::util::parse_line(input, "SV7")?;
        let obj = SV7 {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
            _04: vars.get(3).unwrap().to_string(),
            _05: vars.get(4).unwrap().to_string(),
            _06: vars.get(5).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// SVD - Service Line Adjudication
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct SVD {
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

impl<'a> Parser<&'a str, SVD, nom::error::Error<&'a str>> for SVD {
    fn parse(input: &'a str) -> IResult<&'a str, SVD> {
        let (rest, vars) = crate::util::parse_line(input, "SVD")?;
        let obj = SVD {
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

/// TOO - Tooth Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq, DisplaySegment)]
pub struct TOO {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

impl<'a> Parser<&'a str, TOO, nom::error::Error<&'a str>> for TOO {
    fn parse(input: &'a str) -> IResult<&'a str, TOO> {
        let (rest, vars) = crate::util::parse_line(input, "TOO")?;
        let obj = TOO {
            _01: vars.first().map(util::unborrow_string),
            _02: vars.get(1).map(util::unborrow_string),
            _03: vars.get(2).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}

/// UR - Peer Review Organization or Utilization Review
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment)]
pub struct UR {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

impl<'a> Parser<&'a str, UR, nom::error::Error<&'a str>> for UR {
    fn parse(input: &'a str) -> IResult<&'a str, UR> {
        let (rest, vars) = crate::util::parse_line(input, "UR")?;
        let obj = UR {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}
