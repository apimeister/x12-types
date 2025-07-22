use super::segment::*;
use crate::util::Parser;
use log::{error, trace};
use nom::{combinator::opt, multi::many0, IResult, Parser as _};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 276 - Health Claim Status Request
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276 {
    pub st: ST,
    pub bht: BHT,
    pub loop_2000a: Vec<_276Loop2000A>,
    pub loop_2000b: Vec<_276Loop2000B>,
    pub loop_2000c: Vec<_276Loop2000C>,
    pub loop_2000d: Vec<_276Loop2000D>,
    pub loop_2000e: Vec<_276Loop2000E>,
    pub se: SE,
}

impl _276 {
    pub fn get_segment_count(&self) -> usize {
        self.to_string()
            .split('~')
            .filter(|s| !s.is_empty())
            .count()
    }
    pub fn set_se_segment_count(&mut self, count: Option<usize>) {
        let cnt = count.unwrap_or_else(|| self.get_segment_count());
        self.se._01 = cnt.to_string();
    }
}

// Loop structs
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2000C {
    pub hl: HL,
    pub loop_2100c: _276Loop2100C,
    pub loop_2200c: Vec<_276Loop2200C>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2100C {
    pub nm1: NM1,
    pub dmg: Option<DMG>,
    pub r#ref: Vec<REF>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2200C {
    pub trn: Option<TRN>,
    pub r#ref: Vec<REF>,
    pub amt: Vec<AMT>,
    pub dtp: Vec<DTP>,
    pub svc: Option<SVC>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2000D {
    pub hl: HL,
    pub dmg: DMG,
    pub loop_2100d: _276Loop2100D,
    pub loop_2200d: Vec<_276Loop2200D>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2100D {
    pub nm1: NM1,
    pub r#ref: Vec<REF>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2200D {
    pub trn: TRN,
    pub r#ref: Vec<REF>,
    pub amt: Vec<AMT>,
    pub dtp: Vec<DTP>,
    pub svc: Option<SVC>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2000E {
    pub hl: HL,
    pub dmg: DMG,
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

// Enum for dynamic dispatch
#[allow(clippy::large_enum_variant)]
pub enum _276Generic2000Loop {
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

// Top-level 276 parser
pub fn parse_276(input: &str) -> IResult<&str, _276> {
    trace!("enter parse_276");
    let (rest, st) = ST::parse(input)?;

    if st._01 != "276" {
        error!(
            "ST segment declares {} document instead of expected 276",
            st._01
        );
        return Err(nom::Err::Failure(nom::error::Error::new(
            "ST segment does not declare an EDI 276",
            nom::error::ErrorKind::Fail,
        )));
    }

    let (rest, bht) = BHT::parse(rest)?;

    let mut doc = _276 {
        st,
        bht,
        ..Default::default()
    };

    let mut rest = rest;
    while let Ok((r_hl, hl)) = HL::parse(rest) {
        let (r, loop_result) = match hl._03.as_str() {
            "20" => parse_loop_2000_a(hl, r_hl),
            "21" => parse_loop_2000_b(hl, r_hl),
            "19" => parse_loop_2000_c(hl, r_hl),
            "22" => parse_loop_2000_d(hl, r_hl),
            "23" => parse_loop_2000_e(hl, r_hl),
            _ => {
                trace!("Unknown HL03 value: {}", hl._03);
                break;
            }
        }?;

        match loop_result {
            _276Generic2000Loop::A(l) => doc.loop_2000a.push(l),
            _276Generic2000Loop::B(l) => doc.loop_2000b.push(l),
            _276Generic2000Loop::C(l) => doc.loop_2000c.push(l),
            _276Generic2000Loop::D(l) => doc.loop_2000d.push(l),
            _276Generic2000Loop::E(l) => doc.loop_2000e.push(l),
        }

        rest = r;
    }

    let (rest, se) = SE::parse(rest)?;
    doc.se = se;
    trace!("exit parse_276");
    Ok((rest, doc))
}

// Parse Loop 2000A
fn parse_loop_2000_a(hl: HL, input: &str) -> IResult<&str, _276Generic2000Loop> {
    trace!("enter parse_loop_2000A input: \"{input}\"");
    let mut output = _276Loop2000A {
        hl,
        ..Default::default()
    };
    let (rest, loop_2100a) = parse_loop_2100_a(input)?;
    output.loop_2100a = loop_2100a;
    let (rest, loop_2200a) = many0(parse_loop_2200_a).parse(rest)?;
    output.loop_2200a = loop_2200a;
    let res = Ok((rest, _276Generic2000Loop::A(output)));
    trace!("exit parse_loop_2000A");
    res
}

fn parse_loop_2100_a(input: &str) -> IResult<&str, _276Loop2100A> {
    trace!("enter parse_loop_2100A input: \"{input}\"");
    let (rest, nm1) = NM1::parse(input)?;
    let (rest, per) = opt(PER::parse).parse(rest)?;
    let res = Ok((rest, _276Loop2100A { nm1, per }));
    trace!("exit parse_loop_2100A");
    res
}

fn parse_loop_2200_a(input: &str) -> IResult<&str, _276Loop2200A> {
    trace!("enter parse_loop_2200A input: \"{input}\"");
    let (rest, trn) = TRN::parse(input)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    let res = Ok((
        rest,
        _276Loop2200A {
            trn,
            r#ref: rref,
            dtp,
        },
    ));
    trace!("exit parse_loop_2200A");
    res
}

// Parse Loop 2000B
fn parse_loop_2000_b(hl: HL, input: &str) -> IResult<&str, _276Generic2000Loop> {
    trace!("enter parse_loop_2000B input: \"{input}\"");
    let mut output = _276Loop2000B {
        hl,
        ..Default::default()
    };
    let (rest, loop_2100b) = parse_loop_2100_b(input)?;
    output.loop_2100b = loop_2100b;
    let (rest, loop_2200b) = many0(parse_loop_2200_b).parse(rest)?;
    output.loop_2200b = loop_2200b;
    let res = Ok((rest, _276Generic2000Loop::B(output)));
    trace!("exit parse_loop_2000B");
    res
}

fn parse_loop_2100_b(input: &str) -> IResult<&str, _276Loop2100B> {
    trace!("enter parse_loop_2100B input: \"{input}\"");
    let (rest, nm1) = NM1::parse(input)?;
    let (rest, per) = opt(PER::parse).parse(rest)?;
    let res = Ok((rest, _276Loop2100B { nm1, per }));
    trace!("exit parse_loop_2100B");
    res
}

fn parse_loop_2200_b(input: &str) -> IResult<&str, _276Loop2200B> {
    trace!("enter parse_loop_2200B input: \"{input}\"");
    let (rest, trn) = TRN::parse(input)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    let res = Ok((
        rest,
        _276Loop2200B {
            trn,
            r#ref: rref,
            dtp,
        },
    ));
    trace!("exit parse_loop_2200B");
    res
}

// Parse Loop 2000C
fn parse_loop_2000_c(hl: HL, input: &str) -> IResult<&str, _276Generic2000Loop> {
    trace!("enter parse_loop_2000C input: \"{input}\"");
    let mut output = _276Loop2000C {
        hl,
        ..Default::default()
    };
    let (rest, loop_2100c) = parse_loop_2100_c(input)?;
    output.loop_2100c = loop_2100c;
    let (rest, loop_2200c) = many0(parse_loop_2200_c).parse(rest)?;
    output.loop_2200c = loop_2200c;
    let res = Ok((rest, _276Generic2000Loop::C(output)));
    trace!("exit parse_loop_2000C");
    res
}

fn parse_loop_2100_c(input: &str) -> IResult<&str, _276Loop2100C> {
    trace!("enter parse_loop_2100C input: \"{input}\"");
    let (rest, nm1) = NM1::parse(input)?;
    let (rest, dmg) = opt(DMG::parse).parse(rest)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let res = Ok((
        rest,
        _276Loop2100C {
            nm1,
            dmg,
            r#ref: rref,
        },
    ));
    trace!("exit parse_loop_2100C");
    res
}

fn parse_loop_2200_c(input: &str) -> IResult<&str, _276Loop2200C> {
    trace!("enter parse_loop_2200C input: \"{input}\"");
    // Only attempt 2200C loop when the next segment is one of TRN, REF, AMT, DTP, or SVC
    if input.starts_with("TRN*")
        || input.starts_with("REF*")
        || input.starts_with("AMT*")
        || input.starts_with("DTP*")
        || input.starts_with("SVC*")
    {
        let (after_trn, trn_opt) = opt(TRN::parse).parse(input)?;
        let (rest, rref) = many0(REF::parse).parse(after_trn)?;
        let (rest, amt) = many0(AMT::parse).parse(rest)?;
        let (rest, dtp) = many0(DTP::parse).parse(rest)?;
        let (rest, svc) = opt(SVC::parse).parse(rest)?;
        trace!("exit parse_loop_2200C");
        Ok((
            rest,
            _276Loop2200C {
                trn: trn_opt,
                r#ref: rref,
                amt,
                dtp,
                svc,
            },
        ))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

// Parse Loop 2000D
fn parse_loop_2000_d(hl: HL, input: &str) -> IResult<&str, _276Generic2000Loop> {
    trace!("enter parse_loop_2000D input: \"{input}\"");
    let mut output = _276Loop2000D {
        hl,
        ..Default::default()
    };
    // Make DMG optional to allow provider-level entries without a DMG segment
    let (rest, dmg_opt) = opt(DMG::parse).parse(input)?;
    output.dmg = dmg_opt.unwrap_or_default();
    let (rest, loop_2100d) = parse_loop_2100_d(rest)?;
    output.loop_2100d = loop_2100d;
    let (rest, loop_2200d) = many0(parse_loop_2200_d).parse(rest)?;
    output.loop_2200d = loop_2200d;
    let res = Ok((rest, _276Generic2000Loop::D(output)));
    trace!("exit parse_loop_2000D");
    res
}

fn parse_loop_2100_d(input: &str) -> IResult<&str, _276Loop2100D> {
    trace!("enter parse_loop_2100D input: \"{input}\"");
    let (rest, nm1) = NM1::parse(input)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let res = Ok((rest, _276Loop2100D { nm1, r#ref: rref }));
    trace!("exit parse_loop_2100D");
    res
}

fn parse_loop_2200_d(input: &str) -> IResult<&str, _276Loop2200D> {
    trace!("enter parse_loop_2200D input: \"{input}\"");
    let (rest, trn) = TRN::parse(input)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, amt) = many0(AMT::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    let (rest, svc) = opt(SVC::parse).parse(rest)?;
    trace!("exit parse_loop_2200D");
    Ok((
        rest,
        _276Loop2200D {
            trn,
            r#ref: rref,
            amt,
            dtp,
            svc,
        },
    ))
}

// Parse Loop 2000E
fn parse_loop_2000_e(hl: HL, input: &str) -> IResult<&str, _276Generic2000Loop> {
    trace!("enter parse_loop_2000E input: \"{input}\"");
    let mut output = _276Loop2000E {
        hl,
        ..Default::default()
    };
    let (rest, dmg) = DMG::parse(input)?;
    output.dmg = dmg;
    let (rest, loop_2100e) = parse_loop_2100_e(rest)?;
    output.loop_2100e = loop_2100e;
    let (rest, loop_2200e) = many0(parse_loop_2200_e).parse(rest)?;
    output.loop_2200e = loop_2200e;
    let res = Ok((rest, _276Generic2000Loop::E(output)));
    trace!("exit parse_loop_2000E");
    res
}

fn parse_loop_2100_e(input: &str) -> IResult<&str, _276Loop2100E> {
    trace!("enter parse_loop_2100E input: \"{input}\"");
    let (rest, nm1) = NM1::parse(input)?;
    let (rest, per) = opt(PER::parse).parse(rest)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    let (rest, svc_opt) = opt(SVC::parse).parse(rest)?;
    let svc = svc_opt.unwrap_or_default();
    let res = Ok((
        rest,
        _276Loop2100E {
            nm1,
            per,
            r#ref: rref,
            dtp,
            svc,
        },
    ));
    trace!("exit parse_loop_2100E");
    res
}

fn parse_loop_2200_e(input: &str) -> IResult<&str, _276Loop2200E> {
    trace!("enter parse_loop_2200E input: \"{input}\"");
    let (rest, trn) = TRN::parse(input)?;
    let (rest, rref) = many0(REF::parse).parse(rest)?;
    let (rest, svc) = opt(SVC::parse).parse(rest)?;
    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
    let res = Ok((
        rest,
        _276Loop2200E {
            trn,
            r#ref: rref,
            dtp,
            svc,
        },
    ));
    trace!("exit parse_loop_2200E");
    res
}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    static INIT: Once = Once::new();
    pub fn initialize() {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });
    }
    use super::*;
    use log::debug;

    const CLAIM_REQUEST: &str = "ST*276*0001*005010X212~BHT*0010*13*ABC276XXX*20050915*1425~HL*1**20*1~NM1*PR*2*ABC INSURANCE*****PI*12345~HL*2*1*21*1~NM1*41*2*XYZ SERVICE*****46*X67E~HL*3*2*19*1~NM1*1P*2*HOME HOSPITAL*****XX*1666666661~HL*4*3*22*0~DMG*D8*19301210*M~NM1*IL*1*SMITH*FRED****MI*123456789A~TRN*1*ABCXYZ1~REF*BLT*111~REF*EJ*SM123456~AMT*T3*8513.88~DTP*472*RD8*20050831-20050906~HL*5*3*22*0~DMG*D8*19301115*F~NM1*IL*1*JONES*MARY****MI*234567890A~TRN*1*ABCXYZ2~REF*BLT*111~REF*EJ*JO234567~AMT*T3*7599~DTP*472*RD8*20050731-20050809~HL*6*2*19*1~NM1*1P*2*HOME HOSPITAL PHYSICIANS*****XX*1666666666~HL*7*6*22*1~NM1*IL*1*MANN*JOHN****MI*345678901~HL*8*7*23~DMG*D8*19951101*M~NM1*QC*1*MANN*JOSEPH~TRN*1*ABCXYZ3~REF*EJ*MA345678~SVC*HC:99203*150*****1~DTP*472*D8*20050501~SE*36*0001~GE*1*20213~IEA*1*000010216~";
    const CLAIM_NCPDP_REQUEST: &str = "ST*276*0001*005010X212~BHT*0010*13*ABC276XXX*20060415*1425~HL*1**20*1~NM1*PR*2*ABC INSURANCE*****PI*12345~HL*2*1*21*1~NM1*41*2*XYZ SERVICE*****46*X67E~HL*3*2*19*1~NM1*1P*2*HOME HOSPITAL PHARMACY*****XX*1666666662~HL*4*3*22*0~DMG*D8*19301210*M~NM1*IL*1*SMITH*FRED****MI*123456789012~TRN*1*ABCXYZ1~REF*XZ*7654321~AMT*T3*85~DTP*472*D8*20060301~SE*16*0001~GE*1*20213~IEA*1*000010216~";
    const INFO_RECEIVER_REQUEST: &str = "ST*276*0001*005010X212~BHT*0010*13*ABC276XXX*20050915*1425~HL*1**20*1~NM1*PR*2*ABC INSURANCE*****PI*12345~HL*2*1*21*1~NM1*41*2*XYZ SERVICE*****46*X67E~HL*3*2*19*1~NM1*1P*2*HOME HOSPITAL*****XX*1666666661~HL*4*3*22*0~DMG*D8*19301210*M~NM1*IL*1*SMITH*FRED****MI*123456789A~TRN*1*ABCXYZ1~REF*BLT*111~REF*EJ*SM123456~AMT*T3*8513.88~DTP*472*RD8*20050831-20050906~SE*17*0001~GE*1*20213~IEA*1*000010216~";
    const PROVIDER_REQUEST: &str = "ST*276*0001*005010X212~BHT*0010*13*ABC276XXX*20050915*1425~HL*1**20*1~NM1*PR*2*ABC INSURANCE*****PI*12345~HL*2*1*21*1~NM1*41*2*XYZ SERVICE*****46*X67E~HL*3*2*19*1~NM1*1P*2*HOME HOSPITAL*****XX*1666666661~HL*4*3*22*0~DMG*D8*19301210*M~NM1*IL*1*SMITH*FRED****MI*123456789A~TRN*1*ABCXYZ1~REF*BLT*111~REF*EJ*SM123456~AMT*T3*8513.88~DTP*472*RD8*20050831-20050906~HL*5*2*19*1~NM1*1P*2*HOME HOSPITAL PHYSICIANS*****XX*6166666666~HL*6*5*22*1~NM1*IL*1*MANN*JOHN****MI*345678901~HL*7*6*23~DMG*D8*19951101*M~NM1*QC*1*MANN*JOSEPH~TRN*1*ABCXYZ3~REF*EJ*MA345678~SVC*HC:99203*150*****1~DTP*472*D8*20050501~SE*28*0001~GE*1*20213~IEA*1*000010216~";

    #[test]
    fn parse_claim_request() {
        initialize();
        let (_, doc) = parse_276(CLAIM_REQUEST).expect("failed to parse claim-request");
        debug!("Parsed: {doc:#?}");
        // Header
        assert_eq!(doc.st._01, "276");
        assert_eq!(doc.bht._02, "13");
        // Loop counts
        assert_eq!(doc.loop_2000a.len(), 1);
        assert_eq!(doc.loop_2000b.len(), 1);
        assert_eq!(doc.loop_2000c.len(), 2);
        assert_eq!(doc.loop_2000d.len(), 3);
        assert_eq!(doc.loop_2000e.len(), 1);
        // 2000A - Payer
        let payer = &doc.loop_2000a[0];
        assert_eq!(payer.loop_2100a.nm1._03.as_deref(), Some("ABC INSURANCE"));
        assert!(payer.loop_2100a.per.is_none());
        assert!(payer.loop_2200a.is_empty());
        // 2000C loops - Service Providers
        let sp1 = &doc.loop_2000c[0];
        assert!(sp1.loop_2100c.dmg.is_none());
        assert_eq!(sp1.loop_2100c.nm1._03.as_deref(), Some("HOME HOSPITAL"));
        assert!(sp1.loop_2200c.is_empty());
        let sp2 = &doc.loop_2000c[1];
        assert!(sp2.loop_2100c.dmg.is_none());
        assert_eq!(
            sp2.loop_2100c.nm1._03.as_deref(),
            Some("HOME HOSPITAL PHYSICIANS")
        );
        assert!(sp2.loop_2200c.is_empty());
        // 2000D - Subscriber
        let sub = &doc.loop_2000d[0];
        assert_eq!(sub.dmg._01.as_deref(), Some("D8"));
        assert_eq!(sub.loop_2100d.nm1._03.as_deref(), Some("SMITH"));
        let sub_stat = &sub.loop_2200d[0];
        assert_eq!(sub_stat.trn._02, "ABCXYZ1");
        assert_eq!(sub_stat.r#ref[0]._01, "BLT");
        // 2000E – Dependent Service Details
        let dep = &doc.loop_2000e[0];
        let svc = dep.loop_2200e[0].svc.as_ref().unwrap();
        assert_eq!(svc._01, "HC:99203");
    }

    #[test]
    fn parse_claim_ncpdp_request() {
        initialize();
        let (_, doc) = parse_276(CLAIM_NCPDP_REQUEST).unwrap();
        assert_eq!(
            doc.loop_2000c[0].loop_2100c.nm1._03.as_deref(),
            Some("HOME HOSPITAL PHARMACY")
        );
        let sub = &doc.loop_2000d[0];
        assert_eq!(sub.dmg._01.as_deref(), Some("D8"));
        assert_eq!(sub.loop_2100d.nm1._03.as_deref(), Some("SMITH"));
        let stat = &sub.loop_2200d[0];
        assert!(!stat.amt.is_empty());
        assert_eq!(stat.amt[0]._02.as_str(), "85");
        assert_eq!(stat.trn._02, "ABCXYZ1");
    }

    #[test]
    fn parse_info_receiver_request() {
        initialize();
        let (_, doc) = parse_276(INFO_RECEIVER_REQUEST).unwrap();
        assert!(doc.loop_2000c[0].loop_2200c.is_empty());
        assert_eq!(doc.loop_2000e.len(), 0);
    }

    #[test]
    fn parse_provider_request() {
        initialize();
        let (_, doc) = parse_276(PROVIDER_REQUEST).unwrap();
        assert_eq!(doc.loop_2000d.len(), 2);
        assert_eq!(doc.loop_2000d[1].dmg._01.as_deref(), None);
        // verify there is exactly one dependent loop with service details
        assert_eq!(doc.loop_2000e.len(), 1);
        // 2000E – Dependent Service Details
        let dep = &doc.loop_2000e[0];
        let svc = dep.loop_2200e[0].svc.as_ref().unwrap();
        assert_eq!(svc._01, "HC:99203");
    }
}
