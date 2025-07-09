use super::segment::*;
use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 271 - Eligibility, Coverage or Benefit Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _271 {
    pub st: ST,
    pub bht: BHT,
    pub loop_2000: Vec<_271Loop2000>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _271, nom::error::Error<&'a str>> for _271 {
    fn parse(input: &'a str) -> IResult<&'a str, _271> {
        let mut output = _271::default();
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
            let (rest, aaa) = many0(AAA::parse).parse(rest)?;

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
                let (rest, aaa) = many0(AAA::parse).parse(rest)?;
                let (rest, prv) = opt(PRV::parse).parse(rest)?;
                let (rest, dmg) = opt(DMG::parse).parse(rest)?;
                let (rest, ins) = opt(INS::parse).parse(rest)?;
                let (rest, hi) = opt(HI::parse).parse(rest)?;
                let (rest, dtp) = many0(DTP::parse).parse(rest)?;
                let (rest, lui) = many0(LUI::parse).parse(rest)?;
                let (rest, mpi) = many0(MPI::parse).parse(rest)?;

                // loop 2110 - Subscriber Level
                let mut loop_2110 = vec![];
                let mut loop_2110_rest = rest;
                while peek(opt(EB::parse)).parse(loop_2110_rest)?.1.is_some() {
                    let (rest, eb) = EB::parse(loop_2110_rest)?;
                    let (rest, hsd) = many0(HSD::parse).parse(rest)?;
                    let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
                    let (rest, aaa) = many0(AAA::parse).parse(rest)?;
                    let (rest, veh) = opt(VEH::parse).parse(rest)?;
                    let (rest, pid) = opt(PID::parse).parse(rest)?;
                    let (rest, pdr) = opt(PDR::parse).parse(rest)?;
                    let (rest, pdp) = opt(PDP::parse).parse(rest)?;
                    let (rest, lin) = opt(LIN::parse).parse(rest)?;
                    let (rest, em) = opt(EM::parse).parse(rest)?;
                    let (rest, sd1) = opt(SD1::parse).parse(rest)?;
                    let (rest, pkd) = opt(PKD::parse).parse(rest)?;
                    let (rest, msg) = many0(MSG::parse).parse(rest)?;

                    // loop 2115 - Information
                    let mut loop_2115 = vec![];
                    let mut loop_2115_rest = rest;
                    while peek(opt(III::parse)).parse(loop_2115_rest)?.1.is_some() {
                        let (rest, iii) = III::parse(loop_2115_rest)?;
                        let (rest, dtp) = many0(DTP::parse).parse(rest)?;
                        let (rest, amt) = many0(AMT::parse).parse(rest)?;
                        let (rest, pct) = many0(PCT::parse).parse(rest)?;

                        // loop 2117 - Industry Code
                        let mut loop_2117 = vec![];
                        let mut loop_2117_rest = rest;
                        while peek(opt(LQ::parse)).parse(loop_2117_rest)?.1.is_some() {
                            let (rest, lq) = LQ::parse(loop_2117_rest)?;
                            let (rest, amt) = many0(AMT::parse).parse(rest)?;
                            let (rest, pct) = many0(PCT::parse).parse(rest)?;
                            loop_2117_rest = rest;
                            loop_2117.push(_271Loop2117 { lq, amt, pct });
                        }
                        loop_2115_rest = loop_2117_rest;
                        loop_2115.push(_271Loop2115 {
                            iii,
                            dtp,
                            amt,
                            pct,
                            loop_2117,
                        });
                    }

                    // loop 2120 - Additional Information
                    let mut loop_2120 = vec![];
                    let mut loop_2120_rest = loop_2115_rest;
                    if peek(opt(LS::parse)).parse(loop_2120_rest)?.1.is_some() {
                        let (rest, ls) = LS::parse(loop_2120_rest)?;
                        let mut loop_2120_content = vec![];
                        let mut loop_2120_content_rest = rest;
                        while peek(opt(NM1::parse))
                            .parse(loop_2120_content_rest)?
                            .1
                            .is_some()
                        {
                            let (rest, nm1) = NM1::parse(loop_2120_content_rest)?;
                            let (rest, n2) = opt(N2::parse).parse(rest)?;
                            let (rest, n3) = opt(N3::parse).parse(rest)?;
                            let (rest, n4) = opt(N4::parse).parse(rest)?;
                            let (rest, per) = many0(PER::parse).parse(rest)?;
                            let (rest, prv) = opt(PRV::parse).parse(rest)?;
                            loop_2120_content_rest = rest;
                            loop_2120_content.push(_271Loop2120Content {
                                nm1,
                                n2,
                                n3,
                                n4,
                                per,
                                prv,
                            });
                        }
                        let (rest, le) = LE::parse(loop_2120_content_rest)?;
                        loop_2120_rest = rest;
                        loop_2120.push(_271Loop2120 {
                            ls,
                            content: loop_2120_content,
                            le,
                        });
                    }

                    loop_2110_rest = loop_2120_rest;
                    loop_2110.push(_271Loop2110 {
                        eb,
                        hsd,
                        r#ref,
                        dtp,
                        aaa,
                        veh,
                        pid,
                        pdr,
                        pdp,
                        lin,
                        em,
                        sd1,
                        pkd,
                        msg,
                        loop_2115,
                        loop_2120,
                    });
                }
                loop_2100_rest = loop_2110_rest;
                loop_2100.push(_271Loop2100 {
                    nm1,
                    r#ref,
                    n2,
                    n3,
                    n4,
                    per,
                    aaa,
                    prv,
                    dmg,
                    ins,
                    hi,
                    dtp,
                    lui,
                    mpi,
                    loop_2110,
                });
            }
            loop_rest = loop_2100_rest;
            loop_2000.push(_271Loop2000 {
                hl,
                trn,
                aaa,
                loop_2100,
            });
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
pub struct _271Loop2000 {
    pub hl: HL,
    pub trn: Vec<TRN>,
    pub aaa: Vec<AAA>,
    pub loop_2100: Vec<_271Loop2100>,
}

/// Loop 2100 - Information Receiver Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _271Loop2100 {
    pub nm1: NM1,
    pub r#ref: Vec<REF>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub aaa: Vec<AAA>,
    pub prv: Option<PRV>,
    pub dmg: Option<DMG>,
    pub ins: Option<INS>,
    pub hi: Option<HI>,
    pub dtp: Vec<DTP>,
    pub lui: Vec<LUI>,
    pub mpi: Vec<MPI>,
    pub loop_2110: Vec<_271Loop2110>,
}

/// Loop 2110 - Subscriber Level
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _271Loop2110 {
    pub eb: EB,
    pub hsd: Vec<HSD>,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub aaa: Vec<AAA>,
    pub veh: Option<VEH>,
    pub pid: Option<PID>,
    pub pdr: Option<PDR>,
    pub pdp: Option<PDP>,
    pub lin: Option<LIN>,
    pub em: Option<EM>,
    pub sd1: Option<SD1>,
    pub pkd: Option<PKD>,
    pub msg: Vec<MSG>,
    pub loop_2115: Vec<_271Loop2115>,
    pub loop_2120: Vec<_271Loop2120>,
}

/// Loop 2115 - Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _271Loop2115 {
    pub iii: III,
    pub dtp: Vec<DTP>,
    pub amt: Vec<AMT>,
    pub pct: Vec<PCT>,
    pub loop_2117: Vec<_271Loop2117>,
}

/// Loop 2117 - Industry Code
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _271Loop2117 {
    pub lq: LQ,
    pub amt: Vec<AMT>,
    pub pct: Vec<PCT>,
}

/// Loop 2120 - Additional Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _271Loop2120 {
    pub ls: LS,
    pub content: Vec<_271Loop2120Content>,
    pub le: LE,
}

/// Loop 2120 Content
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _271Loop2120Content {
    pub nm1: NM1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub prv: Option<PRV>,
}
