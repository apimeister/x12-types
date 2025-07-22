use super::{AMT, BHT, DTP, HL, N3, N4, NM1, PER, QTY, REF, SE, ST, STC, SVC, TRN};
use crate::util::Parser;
use log::{error, trace};
use nom::{
    combinator::{opt, peek},
    multi::{many0, many1},
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 277 - Health Care Claim Status
///
/// This represents the entire 277 transaction. The loops 2000A–2000E appear in a
/// hierarchical (HL) structure. Each loop can contain sub-loops for claim status
/// detail. Typical segment ordering at the top level: ST, BHT, hierarchical loops,
/// then SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277 {
    /// ST - Transaction Set Header
    pub st: ST,

    /// BHT - Beginning of Hierarchical Transaction
    pub bht: BHT,

    /// 2000A - Information Source Loop (required, typically the payer)
    pub loop_2000a: _277Loop2000A,

    /// 2000B - Information Receiver Loops
    pub loop_2000b: Vec<_277Loop2000B>,

    /// 2000C - Service Provider Loops
    pub loop_2000c: Vec<_277Loop2000C>,

    /// 2000D - Subscriber Loops
    pub loop_2000d: Vec<_277Loop2000D>,

    /// 2000E - Dependent Loops
    pub loop_2000e: Vec<_277Loop2000E>,

    /// SE - Transaction Set Trailer
    pub se: SE,
}

/// 2000A - Information Source Loop
///
/// Identifies the entity providing the claim status (e.g. a payer).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2000A {
    /// HL - Hierarchical Level
    pub hl: HL,

    /// 2100A - Payer Information
    pub loop_2100a: _277Loop2100A,
}

/// 2100A - Payer Information
///
/// Commonly includes the payer name (NM1), plus optional address and contact segments.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2100A {
    /// NM1 - Payer Name
    pub nm1: NM1,

    // non-standard, but shows up for some legacy payers
    pub trn: Option<TRN>,
    // non-standard, but shows up for some legacy payers
    pub dtp: Vec<DTP>,

    /// N3 - Payer Address
    pub n3: Option<N3>,

    /// N4 - Payer City/State/ZIP
    pub n4: Option<N4>,

    /// REF - Additional Payer Identifiers
    pub r#ref: Vec<REF>,

    /// PER - Payer Contact Information
    pub per: Vec<PER>,
}

/// 2000B - Information Receiver Loop
///
/// Entity receiving the 277 (e.g. clearinghouse or provider if direct).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2000B {
    /// HL - Hierarchical Level
    pub hl: HL,

    /// 2100B - Information Receiver Name
    pub loop_2100b: _277Loop2100B,

    /// 2200B - Information Receiver Claim Status Tracking
    pub loop_2200b: Vec<_277Loop2200B>,
}

/// 2100B - Information Receiver Name
///
/// Identifies who is receiving this claim status.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2100B {
    /// NM1 - Receiver Name
    pub nm1: NM1,

    /// N3 - Receiver Address (optional)
    pub n3: Option<N3>,

    /// N4 - Receiver City/State/ZIP (optional)
    pub n4: Option<N4>,

    /// REF - Additional Receiver Identifiers
    pub r#ref: Vec<REF>,

    /// PER - Receiver Contact Information
    pub per: Vec<PER>,
}

/// 2200B - Information Receiver Claim Status
///
/// Typical 277 ordering: TRN, STC (0+), REF (0+), DTP (0+), then next loop if any.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2200B {
    /// TRN - Trace Number
    pub trn: TRN,

    /// STC - Claim/Line Status
    pub stc: Vec<STC>,

    pub qty: Vec<QTY>,
    pub amt: Vec<AMT>,

    /// REF - Additional Identifiers
    pub r#ref: Vec<REF>,

    /// DTP - Dates
    pub dtp: Vec<DTP>,
    // If there were service line details at this level, they'd be loop 2220B,
    // but typically 277 doesn't use that here.
}

/// 2000C - Service Provider Loop
///
/// If the 277 identifies a specific service provider for the claims in question.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2000C {
    /// HL - Hierarchical Level
    pub hl: HL,

    /// 2100C - Service Provider Name
    pub loop_2100c: _277Loop2100C,

    /// 2200C - Service Provider Claim Status
    pub loop_2200c: Vec<_277Loop2200C>,
}

/// 2100C - Service Provider Name
///
/// Identifies the provider entity referenced at this loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2100C {
    /// NM1 - Provider Name
    pub nm1: NM1,

    /// N3 - Provider Address
    pub n3: Option<N3>,

    /// N4 - Provider City/State/ZIP
    pub n4: Option<N4>,

    pub trn: Vec<TRN>,
    pub stc: Vec<STC>,
    pub qty: Vec<QTY>,
    pub amt: Vec<AMT>,

    /// REF - Additional Provider Identifiers
    pub r#ref: Vec<REF>,

    /// PER - Provider Contact Information
    pub per: Vec<PER>,
}

/// 2200C - Service Provider Claim Status
///
/// Typical segment order: TRN, STC, REF, DTP, etc.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2200C {
    /// TRN - Tracking Number
    pub trn: Option<TRN>,

    /// STC - Status (repeatable)
    pub stc: Vec<STC>,

    /// QTY - Claim level quantities
    pub qty: Vec<QTY>,

    /// AMT - Claim-level amounts
    pub amt: Vec<AMT>,

    /// REF - Additional Identifiers
    pub r#ref: Vec<REF>,

    /// DTP - Date(s)
    pub dtp: Vec<DTP>,
    // If service lines appear at this level, they'd be loop 2220C,
    // but typically 277 uses line-level loops in the subscriber/dependent section.
}

/// 2000D - Subscriber Loop
///
/// The subscriber for these claims. If the patient is the subscriber, no separate 2000E loop is needed.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2000D {
    /// HL - Hierarchical Level
    pub hl: HL,

    /// 2100D - Subscriber Name
    pub loop_2100d: _277Loop2100D,

    /// 2200D - Subscriber Claim Status
    ///
    /// Typically can repeat for multiple claims at the subscriber level.
    pub loop_2200d: Vec<_277Loop2200D>,
}

/// 2100D - Subscriber Name
///
/// Provides subscriber’s identifying details.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2100D {
    /// NM1 - Subscriber Name
    pub nm1: NM1,

    /// N3 - Subscriber Address (optional)
    pub n3: Option<N3>,

    /// N4 - Subscriber City/State/ZIP
    pub n4: Option<N4>,

    /// REF - Additional Subscriber Identifiers
    pub r#ref: Vec<REF>,

    /// PER - Subscriber Contact Info (rare)
    pub per: Vec<PER>,
}

/// 2200D - Subscriber Claim Status
///
/// Typical ordering: TRN, STC (0+), REF (0+), DTP (0+), then line-level loop 2220D if present.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2200D {
    /// TRN - Claim Status Tracking Number
    pub trn: Vec<TRN>,

    /// STC - Claim Status(es)
    pub stc: Vec<STC>,

    /// REF - Additional Identifiers
    pub r#ref: Vec<REF>,

    /// DTP - Claim-level Dates
    pub dtp: Vec<DTP>,

    /// 2220D - Service Line Detail
    pub svc: Vec<_277Loop2220D>,
}

/// 2220D - Subscriber Service Line Information
///
/// Typical ordering: SVC, STC (0+), REF (0+), DTP (0+).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2220D {
    /// SVC - Service Line(s)
    pub svc: Vec<SVC>,

    /// STC - Service Line Status(es)
    pub stc: Vec<STC>,

    /// REF - Additional Line Item Identifiers
    pub r#ref: Vec<REF>,

    /// DTP - Service Line Dates
    pub dtp: Vec<DTP>,
}

/// 2000E - Dependent Loop
///
/// If the patient is a dependent (different from subscriber), claims or lines appear here.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2000E {
    /// HL - Hierarchical Level
    pub hl: HL,

    /// 2100E - Dependent Name
    pub loop_2100e: _277Loop2100E,

    /// 2200E - Dependent Claim Status
    ///
    /// Typically can repeat if multiple claims exist for this dependent.
    pub loop_2200e: Vec<_277Loop2200E>,
}

/// 2100E - Dependent Name
///
/// Identifies dependent if different from subscriber.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2100E {
    /// NM1 - Dependent Name
    pub nm1: NM1,

    /// N3 - Dependent Address (optional)
    pub n3: Option<N3>,

    /// N4 - Dependent City/State/ZIP (optional)
    pub n4: Option<N4>,

    /// REF - Additional Dependent Identifiers
    pub r#ref: Vec<REF>,

    /// PER - Dependent Contact Info
    pub per: Vec<PER>,
}

/// 2200E - Dependent Claim Status
///
/// Typical ordering: TRN, STC (0+), REF (0+), DTP (0+), then line-level 2220E if present.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2200E {
    /// TRN - Claim Status Tracking Number
    pub trn: TRN,

    /// STC - Claim Status(es)
    pub stc: Vec<STC>,

    /// REF - Additional Identifiers
    pub r#ref: Vec<REF>,

    /// DTP - Claim-level Dates
    pub dtp: Vec<DTP>,

    /// 2220E - Dependent Service Line Details
    pub svc: Vec<_277Loop2220E>,
}

/// 2220E - Dependent Service Line Information
///
/// Typical ordering: SVC, STC, REF, DTP.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _277Loop2220E {
    /// SVC - Service Line(s)
    pub svc: Vec<SVC>,

    /// STC - Service Line Status(es)
    pub stc: Vec<STC>,

    /// REF - Additional Line Identifiers
    pub r#ref: Vec<REF>,

    /// DTP - Service Line Dates
    pub dtp: Vec<DTP>,
}

///////////////////////////////////////////////////////////
// Top-level 277 parser
///////////////////////////////////////////////////////////

impl<'a> Parser<&'a str, _277, nom::error::Error<&'a str>> for _277 {
    fn parse(input: &'a str) -> IResult<&'a str, _277> {
        parse_277(input)
    }
}
/// A generic enum for any 2000-level loop we parse.
pub enum _277Generic2000Loop {
    A(_277Loop2000A),
    B(_277Loop2000B),
    C(_277Loop2000C),
    D(_277Loop2000D),
    E(_277Loop2000E),
}

/// Parse an entire 277 transaction from `input`.
/// Returns the remaining unparsed input and the constructed `_277` on success.
fn parse_277(input: &str) -> IResult<&str, _277> {
    // parse ST, BHT first
    let (rest, st) = ST::parse(input)?;

    if st._01 != "277" {
        error!(
            "ST segment declares {} document instead of expected 277",
            st._01
        );
        return Err(nom::Err::Failure(nom::error::Error::new(
            "ST segment does not declare an EDI 277",
            nom::error::ErrorKind::Fail,
        )));
    }

    let (rest, bht) = BHT::parse(rest)?;

    let mut doc = _277 {
        st,
        bht,
        ..Default::default()
    };

    // Now parse as many 2000-level loops as we find
    let mut rest = rest;
    loop {
        // Make sure the next segment doesn't start a new 277 document.
        if peek(opt(ST::parse)).parse(rest)?.1.is_some() {
            break;
        }
        // Try parse one 2000 loop
        match parse_2000_any(rest) {
            Ok((rest2, loop_obj)) => {
                match loop_obj {
                    _277Generic2000Loop::A(a) => {
                        // if we haven't assigned loop_2000a yet, store it
                        doc.loop_2000a = a;
                    }
                    _277Generic2000Loop::B(b) => {
                        doc.loop_2000b.push(b);
                    }
                    _277Generic2000Loop::C(c) => {
                        doc.loop_2000c.push(c);
                    }
                    _277Generic2000Loop::D(d) => {
                        doc.loop_2000d.push(d);
                    }
                    _277Generic2000Loop::E(e) => {
                        doc.loop_2000e.push(e);
                    }
                }
                rest = rest2;
            }
            Err(_) => {
                // no more 2000 loops, break
                break;
            }
        }
    }

    // parse SE at the end
    let (rest, se) = SE::parse(rest)?;
    doc.se = se;

    Ok((rest, doc))
}

/// Parse exactly one 2000-level loop, by reading HL03, then dispatching.
fn parse_2000_any(input: &str) -> IResult<&str, _277Generic2000Loop> {
    // parse the HL segment

    let (rest, hl_seg) = HL::parse(input)?;
    trace!("Parsed HL: {hl_seg}");
    match hl_seg._03.as_str() {
        "20" => parse_loop_2000_a(hl_seg, rest),
        "21" => parse_loop_2000_b(hl_seg, rest),
        "19" => parse_loop_2000_c(hl_seg, rest),
        "22" | "PT" => parse_loop_2000_d(hl_seg, rest),
        "23" => parse_loop_2000_e(hl_seg, rest),
        _ => {
            trace!("No recognized 2000 loops");
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::NoneOf,
            )))
        }
    }
}

///////////////////////////////////////////////////////////
// 2000A - Info Source
///////////////////////////////////////////////////////////

fn parse_loop_2000_a(hl: HL, input: &str) -> IResult<&str, _277Generic2000Loop> {
    let mut output = _277Loop2000A::default();

    trace!("--> 2000A");

    // HL
    //let (rest, hl) = HL::parse(input)?;
    output.hl = hl;

    // 2100A
    let (rest, loop_2100a) = parse_loop_2100_a(input)?;
    output.loop_2100a = loop_2100a;

    trace!("<-- 2000A");
    Ok((rest, _277Generic2000Loop::A(output)))
}

fn parse_loop_2100_a(input: &str) -> IResult<&str, _277Loop2100A> {
    let mut output = _277Loop2100A::default();
    trace!("--> 2100A");

    // NM1
    let (rest, nm1) = NM1::parse(input)?;
    output.nm1 = nm1;

    // Not standard placement, but some senders place this in the 2100A loop
    let (rest, maybe_trn) = opt(TRN::parse).parse(rest)?;
    output.trn = maybe_trn;

    // Not standard placement, but some senders place this in the 2100A loop
    let (rest, dtp_vec) = many0(DTP::parse).parse(rest)?;
    output.dtp = dtp_vec;

    // N3 (opt) -> cast parse to function pointer
    let (rest, n3) = opt(N3::parse).parse(rest)?;
    output.n3 = n3;

    // N4 (opt)
    let (rest, n4) = opt(N4::parse).parse(rest)?;
    output.n4 = n4;

    // REF (0+)
    let (rest, ref_vec) = many0(REF::parse).parse(rest)?;
    output.r#ref = ref_vec;

    // PER (0+)
    let (rest, per_vec) = many0(PER::parse).parse(rest)?;
    output.per = per_vec;
    trace!("<-- 2100A");

    Ok((rest, output))
}

///////////////////////////////////////////////////////////
// 2000B - Info Receiver
///////////////////////////////////////////////////////////

fn parse_loop_2000_b(hl: HL, input: &str) -> IResult<&str, _277Generic2000Loop> {
    let mut output = _277Loop2000B {
        hl,
        ..Default::default()
    };

    // parse single 2100B
    let (rest, loop_2100b) = parse_loop_2100_b(input)?;
    output.loop_2100b = loop_2100b;

    // parse 2200B in a loop, break if HL or SE
    let mut rest2 = rest;
    let mut list_2200b = Vec::new();
    loop {
        // KEY: check HL or SE
        if peek(opt(HL::parse)).parse(rest2)?.1.is_some()
            || peek(opt(SE::parse)).parse(rest2)?.1.is_some()
        {
            break;
        }
        match parse_loop_2200_b(rest2) {
            Ok((r3, sub_b)) => {
                list_2200b.push(sub_b);
                rest2 = r3;
            }
            Err(_) => break,
        }
    }
    output.loop_2200b = list_2200b;

    Ok((rest2, _277Generic2000Loop::B(output)))
}

fn parse_loop_2100_b(input: &str) -> IResult<&str, _277Loop2100B> {
    let mut output = _277Loop2100B::default();

    let (rest, nm1) = NM1::parse(input)?;
    output.nm1 = nm1;

    let (rest, n3) = opt(N3::parse).parse(rest)?;
    output.n3 = n3;

    let (rest, n4) = opt(N4::parse).parse(rest)?;
    output.n4 = n4;

    let (rest, rrefs) = many0(REF::parse).parse(rest)?;
    output.r#ref = rrefs;

    let (rest, pers) = many0(PER::parse).parse(rest)?;
    output.per = pers;

    Ok((rest, output))
}

fn parse_loop_2200_b(input: &str) -> IResult<&str, _277Loop2200B> {
    let mut output = _277Loop2200B::default();

    // TRN
    let (rest, trn) = TRN::parse(input)?;
    output.trn = trn;

    // STC (0+)
    let (rest, stcs) = many0(STC::parse).parse(rest)?;
    output.stc = stcs;

    // QTY (0+)
    let (rest, qtys) = many0(QTY::parse).parse(rest)?;
    output.qty = qtys;

    // AMT (0+)
    let (rest, amts) = many0(AMT::parse).parse(rest)?;
    output.amt = amts;

    // REF (0+)
    let (rest, refs) = many0(REF::parse).parse(rest)?;
    output.r#ref = refs;

    // DTP (0+)
    let (rest, dtp_vec) = many0(DTP::parse).parse(rest)?;
    output.dtp = dtp_vec;

    Ok((rest, output))
}

///////////////////////////////////////////////////////////
// 2000C - Service Provider
///////////////////////////////////////////////////////////

fn parse_loop_2000_c(hl: HL, input: &str) -> IResult<&str, _277Generic2000Loop> {
    let mut output = _277Loop2000C {
        hl,
        ..Default::default()
    };

    // parse single 2100C
    let (rest, loop_2100c) = parse_loop_2100_c(input)?;
    output.loop_2100c = loop_2100c;

    // parse repeated 2200C
    let mut rest2 = rest;
    let mut list_2200c = Vec::new();
    loop {
        // KEY: check HL or SE
        if peek(opt(HL::parse)).parse(rest2)?.1.is_some()
            || peek(opt(SE::parse)).parse(rest2)?.1.is_some()
        {
            break;
        }
        match parse_loop_2200_c(rest2) {
            Ok((r3, c_sub)) => {
                list_2200c.push(c_sub);
                rest2 = r3;
            }
            Err(_) => break,
        }
    }
    output.loop_2200c = list_2200c;

    Ok((rest2, _277Generic2000Loop::C(output)))
}

fn parse_loop_2100_c(input: &str) -> IResult<&str, _277Loop2100C> {
    trace!("--> 2100C");
    let mut output = _277Loop2100C::default();

    trace!("nm1");
    let (rest, nm1) = NM1::parse(input)?;
    output.nm1 = nm1;

    trace!("n3");
    let (rest, n3) = opt(N3::parse).parse(rest)?;
    output.n3 = n3;

    trace!("n4");
    let (rest, n4) = opt(N4::parse).parse(rest)?;
    output.n4 = n4;

    trace!("trn");
    let (rest, trn) = many0(TRN::parse).parse(rest)?;
    output.trn = trn;

    trace!("stc");
    let (rest, stc) = many0(STC::parse).parse(rest)?;
    output.stc = stc;

    trace!("qty");
    let (rest, qty) = many0(QTY::parse).parse(rest)?;
    output.qty = qty;

    trace!("amt");
    let (rest, amt) = many0(AMT::parse).parse(rest)?;
    output.amt = amt;

    trace!("rrefs");
    let (rest, rrefs) = many0(REF::parse).parse(rest)?;
    output.r#ref = rrefs;

    trace!("pers");
    let (rest, pers) = many0(PER::parse).parse(rest)?;
    output.per = pers;

    trace!("<-- 2100C");

    Ok((rest, output))
}

fn parse_loop_2200_c(input: &str) -> IResult<&str, _277Loop2200C> {
    let mut output = _277Loop2200C::default();

    trace!("--> 2200C");

    // TRN (optional in some 277 usage)
    trace!("trn");
    let (rest, trn_opt) = opt(TRN::parse).parse(input)?;
    output.trn = trn_opt;

    // STC (0+)
    trace!("stc");
    let (rest, stcs) = many0(STC::parse).parse(rest)?;
    output.stc = stcs;

    // QTY/AMT are non-standard in this loop, but show up sometimes

    // QTY
    trace!("qty");
    let (rest, qty) = many0(QTY::parse).parse(rest)?;
    output.qty = qty;
    // AMT
    trace!("amt");
    let (rest, amt) = many0(AMT::parse).parse(rest)?;
    output.amt = amt;

    // REF (0+)
    trace!("ref");
    let (rest, rrefs) = many0(REF::parse).parse(rest)?;
    output.r#ref = rrefs;

    // DTP (0+)
    trace!("dtp");
    let (rest, dtps) = many0(DTP::parse).parse(rest)?;
    output.dtp = dtps;

    trace!("<-- 2200C");
    Ok((rest, output))
}

///////////////////////////////////////////////////////////
// 2000D - Subscriber
///////////////////////////////////////////////////////////

fn parse_loop_2000_d(hl: HL, input: &str) -> IResult<&str, _277Generic2000Loop> {
    let mut output = _277Loop2000D {
        hl,
        ..Default::default()
    };

    // parse single 2100D
    let (rest, loop_2100d) = parse_loop_2100_d(input)?;
    output.loop_2100d = loop_2100d;

    // parse repeated 2200D
    let mut rest2 = rest;
    let mut list_2200d = Vec::new();
    loop {
        // KEY: check HL or SE
        if peek(opt(HL::parse)).parse(rest2)?.1.is_some()
            || peek(opt(SE::parse)).parse(rest2)?.1.is_some()
        {
            break;
        }
        match parse_loop_2200_d(rest2) {
            Ok((r3, d_sub)) => {
                list_2200d.push(d_sub);
                rest2 = r3;
            }
            Err(_) => break,
        }
    }
    output.loop_2200d = list_2200d;

    Ok((rest2, _277Generic2000Loop::D(output)))
}

fn parse_loop_2100_d(input: &str) -> IResult<&str, _277Loop2100D> {
    let mut output = _277Loop2100D::default();

    trace!("--> 2100D");

    trace!("nm1");
    let (rest, nm1) = NM1::parse(input)?;
    output.nm1 = nm1;

    trace!("n3");
    let (rest, n3) = opt(N3::parse).parse(rest)?;
    output.n3 = n3;

    trace!("n4");
    let (rest, n4) = opt(N4::parse).parse(rest)?;
    output.n4 = n4;

    trace!("rrefs");
    let (rest, rrefs) = many0(REF::parse).parse(rest)?;
    output.r#ref = rrefs;

    trace!("pers");
    let (rest, pers) = many0(PER::parse).parse(rest)?;
    output.per = pers;

    trace!("<-- 2100D");

    Ok((rest, output))
}

fn parse_loop_2200_d(input: &str) -> IResult<&str, _277Loop2200D> {
    let mut output = _277Loop2200D::default();
    trace!("--> 2200D");

    trace!("trn");
    let (rest, trn) = many0(TRN::parse).parse(input)?;
    output.trn = trn;

    trace!("stc");
    let (rest, stc_vec) = many0(STC::parse).parse(rest)?;
    output.stc = stc_vec;

    trace!("ref");
    let (rest, ref_vec) = many0(REF::parse).parse(rest)?;
    output.r#ref = ref_vec;

    trace!("dtp");
    let (rest, dtp_vec) = many0(DTP::parse).parse(rest)?;
    output.dtp = dtp_vec;

    trace!("svc");
    let (rest, svc_vec) = many0(parse_loop_2220_d).parse(rest)?;
    output.svc = svc_vec;

    trace!("<-- 2200D");
    Ok((rest, output))
}

fn parse_loop_2220_d(input: &str) -> IResult<&str, _277Loop2220D> {
    let mut output = _277Loop2220D::default();
    trace!("--> 2220D");

    // SVC (1+)
    trace!("svc");
    let (rest, svc_list) = many1(SVC::parse).parse(input)?;
    output.svc = svc_list;

    // STC (0+)
    trace!("stc");
    let (rest, stc_list) = many0(STC::parse).parse(rest)?;
    output.stc = stc_list;

    // REF (0+)
    trace!("ref");
    let (rest, ref_list) = many0(REF::parse).parse(rest)?;
    output.r#ref = ref_list;

    // DTP (0+)
    trace!("dtp");
    let (rest, dtp_list) = many0(DTP::parse).parse(rest)?;
    output.dtp = dtp_list;

    trace!("<-- 2220D");
    Ok((rest, output))
}

///////////////////////////////////////////////////////////
// 2000E - Dependent
///////////////////////////////////////////////////////////

fn parse_loop_2000_e(hl: HL, input: &str) -> IResult<&str, _277Generic2000Loop> {
    let mut output = _277Loop2000E {
        hl,
        ..Default::default()
    };

    // parse single 2100E
    let (rest, loop_2100e) = parse_loop_2100_e(input)?;
    output.loop_2100e = loop_2100e;

    // parse repeated 2200E
    let mut rest2 = rest;
    let mut list_2200e = Vec::new();
    loop {
        // KEY: check HL or SE
        if peek(opt(HL::parse)).parse(rest2)?.1.is_some()
            || peek(opt(SE::parse)).parse(rest2)?.1.is_some()
        {
            break;
        }
        match parse_loop_2200_e(rest2) {
            Ok((r3, e_sub)) => {
                list_2200e.push(e_sub);
                rest2 = r3;
            }
            Err(_) => break,
        }
    }
    output.loop_2200e = list_2200e;

    Ok((rest2, _277Generic2000Loop::E(output)))
}

fn parse_loop_2100_e(input: &str) -> IResult<&str, _277Loop2100E> {
    let mut output = _277Loop2100E::default();

    let (rest, nm1) = NM1::parse(input)?;
    output.nm1 = nm1;

    let (rest, n3) = opt(N3::parse).parse(rest)?;
    output.n3 = n3;

    let (rest, n4) = opt(N4::parse).parse(rest)?;
    output.n4 = n4;

    let (rest, refs) = many0(REF::parse).parse(rest)?;
    output.r#ref = refs;

    let (rest, pers) = many0(PER::parse).parse(rest)?;
    output.per = pers;

    Ok((rest, output))
}

fn parse_loop_2200_e(input: &str) -> IResult<&str, _277Loop2200E> {
    let mut output = _277Loop2200E::default();

    let (rest, trn) = TRN::parse(input)?;
    output.trn = trn;

    let (rest, stc_list) = many0(STC::parse).parse(rest)?;
    output.stc = stc_list;

    let (rest, ref_list) = many0(REF::parse).parse(rest)?;
    output.r#ref = ref_list;

    let (rest, dtp_list) = many0(DTP::parse).parse(rest)?;
    output.dtp = dtp_list;

    let (rest, svc_list) = many0(parse_loop_2220_e).parse(rest)?;
    output.svc = svc_list;

    Ok((rest, output))
}

fn parse_loop_2220_e(input: &str) -> IResult<&str, _277Loop2220E> {
    let mut output = _277Loop2220E::default();

    // SVC (1+)
    let (rest, svc_vec) = many1(SVC::parse).parse(input)?;
    output.svc = svc_vec;

    // STC (0+)
    let (rest, stc_vec) = many0(STC::parse).parse(rest)?;
    output.stc = stc_vec;

    // REF (0+)
    let (rest, ref_vec) = many0(REF::parse).parse(rest)?;
    output.r#ref = ref_vec;

    // DTP (0+)
    let (rest, dtp_vec) = many0(DTP::parse).parse(rest)?;
    output.dtp = dtp_vec;

    Ok((rest, output))
}
