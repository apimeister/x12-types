pub use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::{many0, many1},
    IResult, Parser as _,
};

/// 276 - Health Claim Status Request
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276 {
    /// Transaction set header
    pub st: ST,
    /// Beginning of hierarchical transaction
    pub bht: BHT,
    /// Information Source loops (2000A)
    pub loop_2000a: Vec<_276Loop2000A>,
    /// Information Receiver loops (2000B)
    pub loop_2000b: Vec<_276Loop2000B>,
    /// Subscriber loops (2000C)
    pub loop_2000c: Vec<_276Loop2000C>,
    /// Dependent loops (2000D)
    pub loop_2000d: Vec<_276Loop2000D>,
    /// Service Provider loops (2000E)
    pub loop_2000e: Vec<_276Loop2000E>,
    /// Transaction set trailer
    pub se: SE,
}

// Helper functions to manage the SE segment count. The caller is going to
// have a hard time calculating this, so the library should help out.
// It'd be good if all of the doc types had something like this, and better
// if it could be maintained automatically.
impl _276 {
    // Need a better way to do this. This is a hokey implementation for now.
    // It'd be better if we could calculate this without doing so much work.
    pub fn get_segment_count(&self) -> usize {
        let str_276 = self.to_string();
        let segments: Vec<&str> = dbg!(str_276
            .split("~")
            .filter(|seg| seg.trim().len() > 0)
            .collect());
        segments.len()
    }

    /// Set the SE01 segment count. Normally pass None for count
    /// to allow counting automatically and setting to calculated count.
    /// If the count is failing for some reason, you can pass one in.
    pub fn set_se_segment_count(&mut self, count: Option<usize>) {
        let count = if let Some(cnt) = count {
            cnt
        } else {
            self.get_segment_count()
        };

        self.se._01 = count.to_string()
    }
}

/// Loop 2000A - Information Source (Payer)
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2000A {
    pub hl: HL,
    pub loop_2100a: _276Loop2100A,
    pub loop_2200a: Vec<_276Loop2200A>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2100A {
    pub nm1: NM1,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2200A {
    pub trn: TRN,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
}

/// Loop 2000B - Information Receiver
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2000B {
    pub hl: HL,
    pub loop_2100b: _276Loop2100B,
    pub loop_2200b: Vec<_276Loop2200B>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2100B {
    pub nm1: NM1,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2200B {
    pub trn: TRN,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
}

/// Loop 2000C - Subscriber
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2000C {
    pub hl: HL,
    pub loop_2100c: _276Loop2100C,
    pub loop_2200c: Vec<_276Loop2200C>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2100C {
    pub nm1: NM1,
    pub dmg: DMG,
    pub r#ref: Vec<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2200C {
    pub trn: TRN,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub svc: Option<SVC>,
}

/// Loop 2000D - Dependent
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2000D {
    pub hl: HL,
    pub loop_2100d: _276Loop2100D,
    pub loop_2200d: Vec<_276Loop2200D>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2100D {
    pub nm1: NM1,
    pub dmg: DMG,
    pub r#ref: Vec<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2200D {
    pub trn: TRN,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub svc: Option<SVC>,
}

/// Loop 2000E - Service Provider
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2000E {
    pub hl: HL,
    pub loop_2100e: _276Loop2100E,
    pub loop_2200e: Vec<_276Loop2200E>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2100E {
    pub nm1: NM1,
    pub per: Option<PER>,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub svc: SVC,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2200E {
    pub trn: TRN,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub svc: Option<SVC>,
}




pub enum Generic2000Loop {
    A(_276Loop2000A),
    B(_276Loop2000B),
    C(_276Loop2000C),
    D(_276Loop2000D),
    E(_276Loop2000E),
}

impl<'a> Parser<&'a str, _276, nom::error::Error<&'a str>> for _276 {
    fn parse(input: &'a str) -> IResult<&'a str, _276> {
        parse_276(input)
    }
}

pub fn parse_276(input: &str) -> IResult<&str, _276> {
    let (rest, st) = ST::parse(input)?;
    let (rest, bht) = BHT::parse(rest)?;

    let mut doc = _276::default();
    doc.st = st;
    doc.bht = bht;

    let mut rest = rest;
    loop {
        if peek(opt(ST::parse)).parse(rest)?.1.is_some() {
            break;
        }
        match parse_2000_any(rest) {
            Ok((next, loop_obj)) => {
                match loop_obj {
                    Generic2000Loop::A(a) => doc.loop_2000a.push(a),
                    Generic2000Loop::B(b) => doc.loop_2000b.push(b),
                    Generic2000Loop::C(c) => doc.loop_2000c.push(c),
                    Generic2000Loop::D(d) => doc.loop_2000d.push(d),
                    Generic2000Loop::E(e) => doc.loop_2000e.push(e),
                }
                rest = next;
            }
            Err(_) => break,
        }
    }

    let (rest, se) = SE::parse(rest)?;
    doc.se = se;
    Ok((rest, doc))
}

fn parse_2000_any(input: &str) -> IResult<&str, Generic2000Loop> {
    let (rest, hl) = HL::parse(input)?;
    match hl._03.as_str() {
        "20" => parse_loop_2000A(hl, rest),
        "21" => parse_loop_2000B(hl, rest),
        "22" => parse_loop_2000C(hl, rest),
        "23" => parse_loop_2000D(hl, rest),
        "24" => parse_loop_2000E(hl, rest),
        _ => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::NoneOf))),
    }
}

fn parse_loop_2000A(hl: HL, input: &str) -> IResult<&str, Generic2000Loop> {
    let mut output = _276Loop2000A::default();
    output.hl = hl;
    let (rest, loop_2100a) = parse_loop_2100A(input)?;
    output.loop_2100a = loop_2100a;
    let (rest, loop_2200a) = many0(parse_loop_2200A).parse(rest)?;
    output.loop_2200a = loop_2200a;
    Ok((rest, Generic2000Loop::A(output)))
}

fn parse_loop_2100A(input: &str) -> IResult<&str, _276Loop2100A> {
    let (rest, nm1) = NM1::parse(input)?;
    let (rest, per) = opt(PER::parse).parse(rest)?;
    Ok((rest, _276Loop2100A { nm1, per }))
}

fn parse_loop_2200A(input: &str) -> IResult<&str, _276Loop2200A> {
    let (rest, trn) = TRN::parse(input)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    Ok((rest, _276Loop2200A { trn, r#ref: rref, dtp }))
}

fn parse_loop_2000B(hl: HL, input: &str) -> IResult<&str, Generic2000Loop> {
    let mut output = _276Loop2000B::default();
    output.hl = hl;
    let (rest, loop_2100b) = parse_loop_2100B(input)?;
    output.loop_2100b = loop_2100b;
    let (rest, loop_2200b) = many0(parse_loop_2200B).parse(rest)?;
    output.loop_2200b = loop_2200b;
    Ok((rest, Generic2000Loop::B(output)))
}

fn parse_loop_2100B(input: &str) -> IResult<&str, _276Loop2100B> {
    let (rest, nm1) = NM1::parse(input)?;
    let (rest, per) = opt(PER::parse).parse(rest)?;
    Ok((rest, _276Loop2100B { nm1, per }))
}

fn parse_loop_2200B(input: &str) -> IResult<&str, _276Loop2200B> {
    let (rest, trn) = TRN::parse(input)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    Ok((rest, _276Loop2200B { trn, r#ref: rref, dtp }))
}

fn parse_loop_2000C(hl: HL, input: &str) -> IResult<&str, Generic2000Loop> {
    let mut output = _276Loop2000C::default();
    output.hl = hl;
    let (rest, loop_2100c) = parse_loop_2100C(input)?;
    output.loop_2100c = loop_2100c;
    let (rest, loop_2200c) = many0(parse_loop_2200C).parse(rest)?;
    output.loop_2200c = loop_2200c;
    Ok((rest, Generic2000Loop::C(output)))
}

fn parse_loop_2100C(input: &str) -> IResult<&str, _276Loop2100C> {
    let (rest, nm1) = NM1::parse(input)?;
    let (rest, dmg) = DMG::parse.parse(rest)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    Ok((rest, _276Loop2100C { nm1, dmg, r#ref: rref }))
}

fn parse_loop_2200C(input: &str) -> IResult<&str, _276Loop2200C> {
    let (rest, trn) = TRN::parse(input)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    let (rest, svc) = opt(SVC::parse).parse(rest)?;
    Ok((rest, _276Loop2200C { trn, r#ref: rref, dtp, svc }))
}

fn parse_loop_2000D(hl: HL, input: &str) -> IResult<&str, Generic2000Loop> {
    let mut output = _276Loop2000D::default();
    output.hl = hl;
    let (rest, loop_2100d) = parse_loop_2100D(input)?;
    output.loop_2100d = loop_2100d;
    let (rest, loop_2200d) = many0(parse_loop_2200D).parse(rest)?;
    output.loop_2200d = loop_2200d;
    Ok((rest, Generic2000Loop::D(output)))
}

fn parse_loop_2100D(input: &str) -> IResult<&str, _276Loop2100D> {
    let (rest, nm1) = NM1::parse(input)?;
    let (rest, dmg) = DMG::parse.parse(rest)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    Ok((rest, _276Loop2100D { nm1, dmg, r#ref: rref }))
}

fn parse_loop_2200D(input: &str) -> IResult<&str, _276Loop2200D> {
    let (rest, trn) = TRN::parse(input)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    let (rest, svc) = opt(SVC::parse).parse(rest)?;
    Ok((rest, _276Loop2200D { trn, r#ref: rref, dtp, svc }))
}

fn parse_loop_2000E(hl: HL, input: &str) -> IResult<&str, Generic2000Loop> {
    let mut output = _276Loop2000E::default();
    output.hl = hl;
    let (rest, loop_2100e) = parse_loop_2100E(input)?;
    output.loop_2100e = loop_2100e;
    let (rest, loop_2200e) = many0(parse_loop_2200E).parse(rest)?;
    output.loop_2200e = loop_2200e;
    Ok((rest, Generic2000Loop::E(output)))
}

fn parse_loop_2100E(input: &str) -> IResult<&str, _276Loop2100E> {
    let (rest, nm1) = NM1::parse(input)?;
    let (rest, per) = opt(PER::parse).parse(rest)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    let (rest, svc) = SVC::parse.parse(rest)?;
    Ok((rest, _276Loop2100E { nm1, per, r#ref: rref, dtp, svc }))
}

fn parse_loop_2200E(input: &str) -> IResult<&str, _276Loop2200E> {
    let (rest, trn) = TRN::parse(input)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    let (rest, svc) = opt(SVC::parse).parse(rest)?;
    Ok((rest, _276Loop2200E { trn, r#ref: rref, dtp, svc }))
}
