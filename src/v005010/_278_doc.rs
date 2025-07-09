use super::{
    AAA, BHT, CL1, CR1, CR2, CR4, CR5, CR6, CR7, CR8, CRC, DMG, DTP, HCR, HI, HL, HSD, INS, MSG,
    N2, N3, N4, NM1, PER, PRV, PWK, REF, SE, ST, SV1, SV2, SV3, TOO, TRN, UM,
};
use crate::util::Parser;
use log::{error, trace};
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 278 - Health Care Services Review Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _278 {
    pub st: ST,
    pub bht: BHT,
    pub loops: Vec<_278Loop2000>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _278Loop2000 {
    pub hl: HL,
    pub trn: Vec<TRN>,
    pub aaa: Vec<AAA>,
    pub um: Option<UM>,
    pub hcr: Option<HCR>,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub hi: Option<HI>,
    pub sv1: Option<SV1>,
    pub sv2: Option<SV2>,
    pub sv3: Option<SV3>,
    pub too: Vec<TOO>,
    pub hsd: Option<HSD>,
    pub crc: Vec<CRC>,
    pub cl1: Option<CL1>,
    pub cr1: Option<CR1>,
    pub cr2: Option<CR2>,
    pub cr4: Option<CR4>,
    pub cr5: Option<CR5>,
    pub cr6: Option<CR6>,
    pub cr7: Option<CR7>,
    pub cr8: Option<CR8>,
    pub pwk: Vec<PWK>,
    pub msg: Vec<MSG>,
    pub loop_nm1: Vec<_278LoopNM1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _278LoopNM1 {
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
    pub dtp: Vec<DTP>,
}

impl<'a> Parser<&'a str, _278, nom::error::Error<&'a str>> for _278 {
    fn parse(input: &'a str) -> IResult<&'a str, _278> {
        parse_278(input)
    }
}

pub fn parse_278(input: &str) -> IResult<&str, _278> {
    trace!("enter parse_278");
    let (rest, st) = ST::parse(input)?;
    if st._01 != "278" {
        error!(
            "ST segment declares {} document instead of expected 278",
            st._01
        );
        return Err(nom::Err::Failure(nom::error::Error::new(
            "ST segment does not declare an EDI 278",
            nom::error::ErrorKind::Fail,
        )));
    }
    let (rest, bht) = BHT::parse(rest)?;
    let mut output = _278 {
        st,
        bht,
        ..Default::default()
    };
    let mut rest = rest;
    while let Ok((r_hl, hl)) = HL::parse(rest) {
        let (r, loop_data) = parse_loop_2000(hl, r_hl)?;
        output.loops.push(loop_data);
        rest = r;
    }
    let (rest, se) = SE::parse(rest)?;
    output.se = se;
    trace!("exit parse_278");
    Ok((rest, output))
}

fn parse_loop_2000(hl: HL, input: &str) -> IResult<&str, _278Loop2000> {
    trace!("enter parse_loop_2000");
    let mut rest = input;
    let (r, trn) = many0(TRN::parse).parse(rest)?;
    rest = r;
    let (r, aaa) = many0(AAA::parse).parse(rest)?;
    rest = r;
    let (r, um) = opt(UM::parse).parse(rest)?;
    rest = r;
    let (r, hcr) = opt(HCR::parse).parse(rest)?;
    rest = r;
    let (r, rref) = many0(REF::parse).parse(rest)?;
    rest = r;
    let (r, dtp) = many0(DTP::parse).parse(rest)?;
    rest = r;
    let (r, hi) = opt(HI::parse).parse(rest)?;
    rest = r;
    let (r, sv1) = opt(SV1::parse).parse(rest)?;
    rest = r;
    let (r, sv2) = opt(SV2::parse).parse(rest)?;
    rest = r;
    let (r, sv3) = opt(SV3::parse).parse(rest)?;
    rest = r;
    let (r, too) = many0(TOO::parse).parse(rest)?;
    rest = r;
    let (r, hsd) = opt(HSD::parse).parse(rest)?;
    rest = r;
    let (r, crc) = many0(CRC::parse).parse(rest)?;
    rest = r;
    let (r, cl1) = opt(CL1::parse).parse(rest)?;
    rest = r;
    let (r, cr1) = opt(CR1::parse).parse(rest)?;
    rest = r;
    let (r, cr2) = opt(CR2::parse).parse(rest)?;
    rest = r;
    let (r, cr4) = opt(CR4::parse).parse(rest)?;
    rest = r;
    let (r, cr5) = opt(CR5::parse).parse(rest)?;
    rest = r;
    let (r, cr6) = opt(CR6::parse).parse(rest)?;
    rest = r;
    let (r, cr7) = opt(CR7::parse).parse(rest)?;
    rest = r;
    let (r, cr8) = opt(CR8::parse).parse(rest)?;
    rest = r;
    let (r, pwk) = many0(PWK::parse).parse(rest)?;
    rest = r;
    let (r, msg) = many0(MSG::parse).parse(rest)?;
    rest = r;
    let mut loop_nm1 = vec![];
    let mut check = rest;
    while peek(opt(NM1::parse)).parse(check)?.1.is_some() {
        let (r_nm1, nm1_seg) = NM1::parse(check)?;
        let (r_nm1, nm1_loop) = parse_loop_nm1(nm1_seg, r_nm1)?;
        loop_nm1.push(nm1_loop);
        check = r_nm1;
    }
    rest = check;
    trace!("exit parse_loop_2000");
    Ok((
        rest,
        _278Loop2000 {
            hl,
            trn,
            aaa,
            um,
            hcr,
            r#ref: rref,
            dtp,
            hi,
            sv1,
            sv2,
            sv3,
            too,
            hsd,
            crc,
            cl1,
            cr1,
            cr2,
            cr4,
            cr5,
            cr6,
            cr7,
            cr8,
            pwk,
            msg,
            loop_nm1,
        },
    ))
}

fn parse_loop_nm1(nm1: NM1, input: &str) -> IResult<&str, _278LoopNM1> {
    trace!("enter parse_loop_nm1");
    let mut rest = input;
    let (r, rref) = many0(REF::parse).parse(rest)?;
    rest = r;
    let (r, n2) = opt(N2::parse).parse(rest)?;
    rest = r;
    let (r, n3) = opt(N3::parse).parse(rest)?;
    rest = r;
    let (r, n4) = opt(N4::parse).parse(rest)?;
    rest = r;
    let (r, per) = many0(PER::parse).parse(rest)?;
    rest = r;
    let (r, aaa) = many0(AAA::parse).parse(rest)?;
    rest = r;
    let (r, prv) = opt(PRV::parse).parse(rest)?;
    rest = r;
    let (r, dmg) = opt(DMG::parse).parse(rest)?;
    rest = r;
    let (r, ins) = opt(INS::parse).parse(rest)?;
    rest = r;
    let (r, dtp) = many0(DTP::parse).parse(rest)?;
    rest = r;
    trace!("exit parse_loop_nm1");
    Ok((
        rest,
        _278LoopNM1 {
            nm1,
            r#ref: rref,
            n2,
            n3,
            n4,
            per,
            aaa,
            prv,
            dmg,
            ins,
            dtp,
        },
    ))
}
