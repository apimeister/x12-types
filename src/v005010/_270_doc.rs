use super::segment::*;
use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 270 - Eligibility, Coverage or Benefit Inquiry
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _270 {
    pub st: ST,
    pub bht: BHT,
    pub loop_2000: Vec<_270Loop2000>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _270, nom::error::Error<&'a str>> for _270 {
    fn parse(input: &'a str) -> IResult<&'a str, _270> {
        let mut output = _270::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = BHT::parse(rest)?;
        output.bht = obj;

        // loop 2000 - Information Source Level
        let mut loop_2000 = vec![];
        let mut loop_rest = rest;
        while peek(opt(HL::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, hl) = HL::parse(loop_rest)?;
            let (rest, trn) = many0(TRN::parse).parse(rest)?;

            // loop 2100 - Information Receiver Level
            let mut loop_2100 = vec![];
            let mut loop_2100_rest = rest;
            while peek(opt(NM1::parse)).parse(loop_2100_rest)?.1.is_some() {
                let (rest, nm1) = NM1::parse(loop_2100_rest)?;
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                let (rest, n2) = opt(N2::parse).parse(rest)?;
                let (rest, n3) = opt(N3::parse).parse(rest)?;
                let (rest, n4) = opt(N4::parse).parse(rest)?;
                let (rest, per) = many0(PER::parse).parse(rest)?;
                let (rest, prv) = opt(PRV::parse).parse(rest)?;
                let (rest, dmg) = opt(DMG::parse).parse(rest)?;
                let (rest, ins) = opt(INS::parse).parse(rest)?;
                let (rest, hi) = opt(HI::parse).parse(rest)?;
                let (rest, dtp) = many0(DTP::parse).parse(rest)?;
                let (rest, mpi) = many0(MPI::parse).parse(rest)?;

                // loop 2110 - Subscriber Level
                let mut loop_2110 = vec![];
                let mut loop_2110_rest = rest;
                while peek(opt(EQ::parse)).parse(loop_2110_rest)?.1.is_some() {
                    let (rest, eq) = EQ::parse(loop_2110_rest)?;
                    let (rest, amt) = many0(AMT::parse).parse(rest)?;
                    let (rest, veh) = opt(VEH::parse).parse(rest)?;
                    let (rest, pdr) = opt(PDR::parse).parse(rest)?;
                    let (rest, pdp) = opt(PDP::parse).parse(rest)?;
                    let (rest, iii) = many0(III::parse).parse(rest)?;
                    let (rest, r#ref) = opt(REF::parse).parse(rest)?;
                    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
                    loop_2110_rest = rest;
                    loop_2110.push(_270Loop2110 {
                        eq,
                        amt,
                        veh,
                        pdr,
                        pdp,
                        iii,
                        r#ref,
                        dtp,
                    });
                }
                loop_2100_rest = loop_2110_rest;
                loop_2100.push(_270Loop2100 {
                    nm1,
                    r#ref,
                    n2,
                    n3,
                    n4,
                    per,
                    prv,
                    dmg,
                    ins,
                    hi,
                    dtp,
                    mpi,
                    loop_2110,
                });
            }
            loop_rest = loop_2100_rest;
            loop_2000.push(_270Loop2000 { hl, trn, loop_2100 });
        }
        let rest = loop_rest;
        output.loop_2000 = loop_2000;

        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

/// Loop 2000 - Information Source Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _270Loop2000 {
    pub hl: HL,
    pub trn: Vec<TRN>,
    pub loop_2100: Vec<_270Loop2100>,
}

/// Loop 2100 - Information Receiver Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _270Loop2100 {
    pub nm1: NM1,
    pub r#ref: Vec<REF>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub prv: Option<PRV>,
    pub dmg: Option<DMG>,
    pub ins: Option<INS>,
    pub hi: Option<HI>,
    pub dtp: Vec<DTP>,
    pub mpi: Vec<MPI>,
    pub loop_2110: Vec<_270Loop2110>,
}

/// Loop 2110 - Subscriber Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _270Loop2110 {
    pub eq: EQ,
    pub amt: Vec<AMT>,
    pub veh: Option<VEH>,
    pub pdr: Option<PDR>,
    pub pdp: Option<PDP>,
    pub iii: Vec<III>,
    pub r#ref: Option<REF>,
    pub dtp: Vec<DTP>,
}
