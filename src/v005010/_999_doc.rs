use super::segment::*;
use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 999 - Implementation Acknowledgment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _999 {
    pub st: ST,
    pub ak1: AK1,
    pub loop_ak2: Vec<_999LoopAK2>,
    pub ak9: AK9,
    pub se: SE,
}

impl<'a> Parser<&'a str, _999, nom::error::Error<&'a str>> for _999 {
    fn parse(input: &'a str) -> IResult<&'a str, _999> {
        let mut output = _999::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = AK1::parse(rest)?;
        output.ak1 = obj;
        // loop AK2
        let mut loop_ak2 = vec![];
        let mut loop_rest = rest;
        while peek(opt(AK2::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, ak2) = AK2::parse(loop_rest)?;
            // loop IK3
            let mut loop_ik3 = vec![];
            let mut ik3_rest = rest;
            while peek(opt(IK3::parse)).parse(ik3_rest)?.1.is_some() {
                let (rest, ik3) = IK3::parse(ik3_rest)?;
                let (rest, ctx) = many0(CTX::parse).parse(rest)?;
                // loop IK4
                let mut loop_ik4 = vec![];
                let mut ik4_rest = rest;
                while peek(opt(IK4::parse)).parse(ik4_rest)?.1.is_some() {
                    let (rest, ik4) = IK4::parse(ik4_rest)?;
                    let (rest, ctx) = many0(CTX::parse).parse(rest)?;
                    ik4_rest = rest;
                    loop_ik4.push(_999LoopIK4 { ik4, ctx });
                }
                ik3_rest = ik4_rest;
                loop_ik3.push(_999LoopIK3 { ik3, ctx, loop_ik4 });
            }
            let rest = ik3_rest;
            let (rest, ik5) = IK5::parse(rest)?;
            loop_rest = rest;
            loop_ak2.push(_999LoopAK2 { ak2, loop_ik3, ik5 });
        }
        let rest = loop_rest;
        let (rest, ak9) = AK9::parse(rest)?;
        output.ak9 = ak9;
        let (rest, se) = SE::parse(rest)?;
        output.se = se;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _999LoopAK2 {
    pub ak2: AK2,
    pub loop_ik3: Vec<_999LoopIK3>,
    pub ik5: IK5,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _999LoopIK3 {
    pub ik3: IK3,
    pub ctx: Vec<CTX>,
    pub loop_ik4: Vec<_999LoopIK4>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _999LoopIK4 {
    pub ik4: IK4,
    pub ctx: Vec<CTX>,
}
