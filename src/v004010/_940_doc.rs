use crate::util::Parser;
use crate::v004010::*;
use nom::combinator::opt;
use nom::multi::many0;
use nom::IResult;
use serde::{Deserialize, Serialize};

pub use segment::*;

/// 940 - Warehouse Shipping Order
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Warehouse Shipping Order Transaction Set (940) for use within the context of an Electronic Data Interchange (EDI) environment. This transaction set can be used to enable the depositor to advise a warehouse to make a shipment, confirm a shipment, or modify or cancel a previously submitted shipping order, or to report the status of the current shipping order to the depositor.
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _940 {
    pub _010: ST,
    pub _020: W05,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0100: Vec<_940Loop100>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _090: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _100: Vec<G61>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _110: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _120: Vec<NTE>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _130: Option<W09>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _140: Option<W66>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _150: Option<W6>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _153: Vec<R2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _156: Option<BNX>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0200_loop: Vec<_940Loop200>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0300_loop: Vec<_940Loop300>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _0400: Option<W76>,
    pub _0500: SE,
}

impl<'a> Parser<&'a str, _940, nom::error::Error<&'a str>> for _940 {
    fn parse(input: &'a str) -> IResult<&'a str, _940> {
        let (mut rest, st) = ST::parse(input)?;
        let (rest_w05, w05) = W05::parse(rest)?;
        let (rest_loop100, loop100) = many0(_940Loop100::parse).parse(rest_w05)?;
        let (rest_n9, n9) = many0(N9::parse).parse(rest_loop100)?;
        let (rest_g61, g61) = many0(G61::parse).parse(rest_n9)?;
        let (rest_g62, g62) = many0(G62::parse).parse(rest_g61)?;
        let (rest_nte, nte) = many0(NTE::parse).parse(rest_g62)?;
        let (rest_w09, w09) = opt(W09::parse).parse(rest_nte)?;
        let (rest_w66, w66) = opt(W66::parse).parse(rest_w09)?;
        let (rest_w6, w6) = opt(W6::parse).parse(rest_w66)?;
        let (rest_r2, r2) = many0(R2::parse).parse(rest_w6)?;
        let (rest_bnx, bnx) = opt(BNX::parse).parse(rest_r2)?;
        let (rest_loop200, loop200) = many0(_940Loop200::parse).parse(rest_bnx)?;
        let (rest_loop300, loop300) = many0(_940Loop300::parse).parse(rest_loop200)?;
        let (rest_w76, w76) = opt(W76::parse).parse(rest_loop300)?;
        let (rest_se, se) = SE::parse(rest_w76)?;
        rest = rest_se;
        Ok((
            rest,
            _940 {
                _010: st,
                _020: w05,
                _0100: loop100,
                _090: n9,
                _100: g61,
                _110: g62,
                _120: nte,
                _130: w09,
                _140: w66,
                _150: w6,
                _153: r2,
                _156: bnx,
                _0200_loop: loop200,
                _0300_loop: loop300,
                _0400: w76,
                _0500: se,
            },
        ))
    }
}

// Loop Structures for 940

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _940Loop100 {
    pub _040: N1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _050: Vec<N2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _060: Vec<N3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _070: Option<N4>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _080: Vec<PER>,
}

impl<'a> Parser<&'a str, _940Loop100, nom::error::Error<&'a str>> for _940Loop100 {
    fn parse(input: &'a str) -> IResult<&'a str, _940Loop100> {
        let (mut rest, n1) = N1::parse(input)?;
        let (rest_n2, n2) = many0(N2::parse).parse(rest)?;
        let (rest_n3, n3) = many0(N3::parse).parse(rest_n2)?;
        let (rest_n4, n4) = opt(N4::parse).parse(rest_n3)?;
        let (rest_per, per) = many0(PER::parse).parse(rest_n4)?;
        rest = rest_per;
        Ok((
            rest,
            _940Loop100 {
                _040: n1,
                _050: n2,
                _060: n3,
                _070: n4,
                _080: per,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _940Loop200 {
    pub _160: LM,
    pub _170: Vec<LQ>,
}

impl<'a> Parser<&'a str, _940Loop200, nom::error::Error<&'a str>> for _940Loop200 {
    fn parse(input: &'a str) -> IResult<&'a str, _940Loop200> {
        let (rest_lm, lm) = LM::parse(input)?;
        let (rest_lq, lq) = many0(LQ::parse).parse(rest_lm)?;
        Ok((rest_lq, _940Loop200 { _160: lm, _170: lq }))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _940Loop300 {
    pub _005: LX,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _010: Vec<MAN>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _015: Vec<SDQ>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _016: Option<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _017: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0310_loop: Vec<_940Loop310>,
}

impl<'a> Parser<&'a str, _940Loop300, nom::error::Error<&'a str>> for _940Loop300 {
    fn parse(input: &'a str) -> IResult<&'a str, _940Loop300> {
        let (mut rest, lx) = LX::parse(input)?;
        let (rest_man, man) = many0(MAN::parse).parse(rest)?;
        let (rest_sdq, sdq) = many0(SDQ::parse).parse(rest_man)?;
        let (rest_n1, n1) = opt(N1::parse).parse(rest_sdq)?;
        let (rest_g62, g62) = many0(G62::parse).parse(rest_n1)?;
        let (rest_0310, loop_0310) = many0(_940Loop310::parse).parse(rest_g62)?;
        rest = rest_0310;

        Ok((
            rest,
            _940Loop300 {
                _005: lx,
                _010: man,
                _015: sdq,
                _016: n1,
                _017: g62,
                _0310_loop: loop_0310,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _940Loop310 {
    pub _020: W01,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _030: Vec<G69>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _040: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _045: Vec<NTE>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _050: Vec<W20>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _070: Vec<QTY>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _080: Option<AMT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _090: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _100: Option<G66>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _110: Vec<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _112: Vec<PER>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _114: Vec<LH2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _116: Option<LHR>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _118: Vec<LH6>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0320_loop: Vec<_940Loop320>,
}

impl<'a> Parser<&'a str, _940Loop310, nom::error::Error<&'a str>> for _940Loop310 {
    fn parse(input: &'a str) -> IResult<&'a str, _940Loop310> {
        let (mut rest, w01) = W01::parse(input)?;
        let (rest_g69, g69) = many0(G69::parse).parse(rest)?;
        let (rest_n9, n9) = many0(N9::parse).parse(rest_g69)?;
        let (rest_nte, nte) = many0(NTE::parse).parse(rest_n9)?;
        let (rest_w20, w20) = many0(W20::parse).parse(rest_nte)?;
        let (rest_qty, qty) = many0(QTY::parse).parse(rest_w20)?;
        let (rest_amt, amt) = opt(AMT::parse).parse(rest_qty)?;
        let (rest_g62, g62) = many0(G62::parse).parse(rest_amt)?;
        let (rest_g66, g66) = opt(G66::parse).parse(rest_g62)?;
        let (rest_n1, n1) = many0(N1::parse).parse(rest_g66)?;
        let (rest_per, per) = many0(PER::parse).parse(rest_n1)?;
        let (rest_lh2, lh2) = many0(LH2::parse).parse(rest_per)?;
        let (rest_lhr, lhr) = opt(LHR::parse).parse(rest_lh2)?;
        let (rest_lh6, lh6) = many0(LH6::parse).parse(rest_lhr)?;
        let (rest_loop, loop320) = many0(_940Loop320::parse).parse(rest_lh6)?;
        rest = rest_loop;
        Ok((
            rest,
            _940Loop310 {
                _020: w01,
                _030: g69,
                _040: n9,
                _045: nte,
                _050: w20,
                _070: qty,
                _080: amt,
                _090: g62,
                _100: g66,
                _110: n1,
                _112: per,
                _114: lh2,
                _116: lhr,
                _118: lh6,
                _0320_loop: loop320,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _940Loop320 {
    pub _120: LM,
    #[serde(default)]
    pub _130: Vec<LQ>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _135: Option<LS>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0330_loop: Vec<_940Loop330>,
}

impl<'a> Parser<&'a str, _940Loop320, nom::error::Error<&'a str>> for _940Loop320 {
    fn parse(input: &'a str) -> IResult<&'a str, _940Loop320> {
        let (mut rest, lm) = LM::parse(input)?;
        let (rest_lq, lq) = many0(LQ::parse).parse(rest)?;
        let (rest_ls, ls) = opt(LS::parse).parse(rest_lq)?;
        let (rest_loop, loop330) = many0(_940Loop330::parse).parse(rest_ls)?;
        rest = rest_loop;
        Ok((
            rest,
            _940Loop320 {
                _120: lm,
                _130: lq,
                _135: ls,
                _0330_loop: loop330,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _940Loop330 {
    pub _140: LX,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _150: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _160: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _170: Option<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _175: Vec<SDQ>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0331_loop: Vec<_940Loop331>,
}

impl<'a> Parser<&'a str, _940Loop330, nom::error::Error<&'a str>> for _940Loop330 {
    fn parse(input: &'a str) -> IResult<&'a str, _940Loop330> {
        let (mut rest, lx) = LX::parse(input)?;
        let (rest_n9, n9) = many0(N9::parse).parse(rest)?;
        let (rest_g62, g62) = many0(G62::parse).parse(rest_n9)?;
        let (rest_n1, n1) = opt(N1::parse).parse(rest_g62)?;
        let (rest_sdq, sdq) = many0(SDQ::parse).parse(rest_n1)?;
        let (rest_loop, loop331) = many0(_940Loop331::parse).parse(rest_sdq)?;
        rest = rest_loop;
        Ok((
            rest,
            _940Loop330 {
                _140: lx,
                _150: n9,
                _160: g62,
                _170: n1,
                _175: sdq,
                _0331_loop: loop331,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _940Loop331 {
    pub _180: LM,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _190: Vec<LQ>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0332_loop: Vec<_940Loop332>,
}

impl<'a> Parser<&'a str, _940Loop331, nom::error::Error<&'a str>> for _940Loop331 {
    fn parse(input: &'a str) -> IResult<&'a str, _940Loop331> {
        let (mut rest, lm) = LM::parse(input)?;
        let (rest_lq, lq) = many0(LQ::parse).parse(rest)?;
        let (rest_loop, loop332) = many0(_940Loop332::parse).parse(rest_lq)?;
        rest = rest_loop;
        Ok((
            rest,
            _940Loop331 {
                _180: lm,
                _190: lq,
                _0332_loop: loop332,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _940Loop332 {
    pub _200: LH1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _210: Vec<LH2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _220: Vec<LH3>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _230: Vec<LFH>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _240: Vec<LEP>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _250: Option<LH4>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _260: Vec<LHT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _270: Vec<LHR>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _280: Vec<PER>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _285: Option<LE>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _340_loop: Vec<_940Loop340>,
}

impl<'a> Parser<&'a str, _940Loop332, nom::error::Error<&'a str>> for _940Loop332 {
    fn parse(input: &'a str) -> IResult<&'a str, _940Loop332> {
        let (mut rest, lh1) = LH1::parse(input)?;
        let (rest_lh2, lh2) = many0(LH2::parse).parse(rest)?;
        let (rest_lh3, lh3) = many0(LH3::parse).parse(rest_lh2)?;
        let (rest_lfh, lfh) = many0(LFH::parse).parse(rest_lh3)?;
        let (rest_lep, lep) = many0(LEP::parse).parse(rest_lfh)?;
        let (rest_lh4, lh4) = opt(LH4::parse).parse(rest_lep)?;
        let (rest_lht, lht) = many0(LHT::parse).parse(rest_lh4)?;
        let (rest_lhr, lhr) = many0(LHR::parse).parse(rest_lht)?;
        let (rest_per, per) = many0(PER::parse).parse(rest_lhr)?;
        let (rest_le, le) = opt(LE::parse).parse(rest_per)?;
        let (rest_loop, loop340) = many0(_940Loop340::parse).parse(rest_le)?;
        rest = rest_loop;
        Ok((
            rest,
            _940Loop332 {
                _200: lh1,
                _210: lh2,
                _220: lh3,
                _230: lfh,
                _240: lep,
                _250: lh4,
                _260: lht,
                _270: lhr,
                _280: per,
                _285: le,
                _340_loop: loop340,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _940Loop340 {
    pub _290: FA1,
    pub _300: Vec<FA2>,
}

impl<'a> Parser<&'a str, _940Loop340, nom::error::Error<&'a str>> for _940Loop340 {
    fn parse(input: &'a str) -> IResult<&'a str, _940Loop340> {
        let (mut rest, fa1) = FA1::parse(input)?;
        let (rest_fa2, fa2) = many0(FA2::parse).parse(rest)?;
        rest = rest_fa2;
        Ok((
            rest,
            _940Loop340 {
                _290: fa1,
                _300: fa2,
            },
        ))
    }
}
