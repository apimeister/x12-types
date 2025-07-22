use crate::util::Parser;
use crate::v004010::*;
use nom::combinator::opt;
use nom::multi::many0;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;

/// 940 - Warehouse Shipping Order
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Warehouse Shipping Order Transaction Set (940) for use within the context of an Electronic Data Interchange (EDI) environment. This transaction set can be used to enable the depositor to advise a warehouse to make a shipment, confirm a shipment, or modify or cancel a previously submitted shipping order, or to report the status of the current shipping order to the depositor.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _940 {
    pub _010: ST,
    pub _020: W05,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0100: Vec<_940_Loop100>,
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
    pub _0200_loop: Vec<_940_Loop200>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0300_loop: Vec<_940_Loop300>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _0400: Option<W76>,
    pub _0500: SE,
}

impl fmt::Display for _940 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self._010, self._020)?;
        for loop_item in &self._0100 {
            write!(f, "{loop_item}")?;
        }
        for n9_item in &self._090 {
            write!(f, "{n9_item}")?;
        }
        for g61_item in &self._100 {
            write!(f, "{g61_item}")?;
        }
        for g62_item in &self._110 {
            write!(f, "{g62_item}")?;
        }
        for nte_item in &self._120 {
            write!(f, "{nte_item}")?;
        }
        if let Some(ref w09_item) = self._130 {
            write!(f, "{w09_item}")?;
        }
        if let Some(ref w66_item) = self._140 {
            write!(f, "{w66_item}")?;
        }
        if let Some(ref w6_item) = self._150 {
            write!(f, "{w6_item}")?;
        }
        for r2_item in &self._153 {
            write!(f, "{r2_item}")?;
        }
        if let Some(ref bnx_item) = self._156 {
            write!(f, "{bnx_item}")?;
        }
        for loop_item in &self._0200_loop {
            write!(f, "{loop_item}")?;
        }
        for loop_item in &self._0300_loop {
            write!(f, "{loop_item}")?;
        }
        if let Some(ref w76_item) = self._0400 {
            write!(f, "{w76_item}")?;
        }
        write!(f, "{}", self._0500)?;
        Ok(())
    }
}

impl<'a> Parser<&'a str, _940, nom::error::Error<&'a str>> for _940 {
    fn parse(input: &'a str) -> IResult<&'a str, _940> {
        let mut output = _940::default();
        let (mut rest, obj) = ST::parse(input)?;
        output._010 = obj;
        let (rest_temp, obj) = W05::parse(rest)?;
        output._020 = obj;
        rest = rest_temp;

        // N1 loop (0100)
        let (rest_n1, n1_loop) = many0(_940_Loop100::parse).parse(rest)?;
        output._0100 = n1_loop;
        rest = rest_n1;

        let (rest_n9, n9_heading) = many0(N9::parse).parse(rest)?;
        output._090 = n9_heading;
        rest = rest_n9;

        let (rest_g61, g61) = many0(G61::parse).parse(rest)?;
        output._100 = g61;
        rest = rest_g61;

        let (rest_g62, g62_heading) = many0(G62::parse).parse(rest)?;
        output._110 = g62_heading;
        rest = rest_g62;

        let (rest_nte, nte_heading) = many0(NTE::parse).parse(rest)?;
        output._120 = nte_heading;
        rest = rest_nte;

        let (rest_w09, w09) = opt(W09::parse).parse(rest)?;
        output._130 = w09;
        rest = rest_w09;

        let (rest_w66, w66) = opt(W66::parse).parse(rest)?;
        output._140 = w66;
        rest = rest_w66;

        let (rest_w6, w6) = opt(W6::parse).parse(rest)?;
        output._150 = w6;
        rest = rest_w6;

        let (rest_r2, r2) = many0(R2::parse).parse(rest)?;
        output._153 = r2;
        rest = rest_r2;

        let (rest_bnx, bnx) = opt(BNX::parse).parse(rest)?;
        output._156 = bnx;
        rest = rest_bnx;

        // LM loop (0200)
        let (rest_lm_loop, lm_loop_heading) = many0(_940_Loop200::parse).parse(rest)?;
        output._0200_loop = lm_loop_heading;
        rest = rest_lm_loop;

        // LX loop (detail section) (0300)
        let (rest_lx_loop, lx_loop) = many0(_940_Loop300::parse).parse(rest)?;
        output._0300_loop = lx_loop;
        rest = rest_lx_loop;

        let (rest_w76, w76) = opt(W76::parse).parse(rest)?;
        output._0400 = w76;
        rest = rest_w76;

        let (rest_se, se) = SE::parse(rest)?;
        output._0500 = se;
        rest = rest_se;

        Ok((rest, output))
    }
}

// Loop Structures for 940

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _940_Loop100 {
    pub n1: N1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n2: Vec<N2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n3: Vec<N3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n4: Option<N4>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ref_loop: Vec<_940_0100_Ref_Loop>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
}

impl fmt::Display for _940_Loop100 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.n1)?;
        for n2 in &self.n2 {
            write!(f, "{n2}")?;
        }
        for n3 in &self.n3 {
            write!(f, "{n3}")?;
        }
        if let Some(ref n4) = self.n4 {
            write!(f, "{n4}")?;
        }
        for ref_loop in &self.ref_loop {
            write!(f, "{ref_loop}")?;
        }
        for per in &self.per {
            write!(f, "{per}")?;
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, _940_Loop100, nom::error::Error<&'a str>> for _940_Loop100 {
    fn parse(input: &'a str) -> IResult<&'a str, _940_Loop100> {
        let (mut rest, n1) = N1::parse(input)?;
        let (rest_n2, n2) = many0(N2::parse).parse(rest)?;
        let (rest_n3, n3) = many0(N3::parse).parse(rest_n2)?;
        let (rest_n4, n4) = opt(N4::parse).parse(rest_n3)?;
        let (rest_ref, ref_loop) = many0(_940_0100_Ref_Loop::parse).parse(rest_n4)?;
        let (rest_per, per) = many0(PER::parse).parse(rest_ref)?;
        rest = rest_per;
        Ok((
            rest,
            _940_Loop100 {
                n1,
                n2,
                n3,
                n4,
                ref_loop,
                per,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _940_0100_Ref_Loop {
    pub ref_: REF,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtm: Option<DTM>,
}

impl fmt::Display for _940_0100_Ref_Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ref_)?;
        if let Some(ref dtm) = self.dtm {
            write!(f, "{dtm}")?;
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, _940_0100_Ref_Loop, nom::error::Error<&'a str>> for _940_0100_Ref_Loop {
    fn parse(input: &'a str) -> IResult<&'a str, _940_0100_Ref_Loop> {
        let (mut rest, ref_) = REF::parse(input)?;
        let (rest_dtm, dtm) = opt(DTM::parse).parse(rest)?;
        rest = rest_dtm;
        Ok((
            rest,
            _940_0100_Ref_Loop {
                ref_,
                dtm,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _940_Loop300 {
    pub w01: W01,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g69: Option<G69>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9_loop: Vec<_940_0300_W01_N9_Loop>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w20: Option<W20>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub qty: Vec<QTY>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<AMT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g66: Option<G66>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n1_loop: Vec<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lh2: Vec<LH2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lhr: Option<LHR>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lh6: Vec<LH6>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lm_loop: Vec<_940_Loop200>, // Re-using LM Loop (0200)
}

impl fmt::Display for _940_Loop300 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.w01)?;
        if let Some(ref g69) = self.g69 {
            write!(f, "{g69}")?;
        }
        for n9_loop in &self.n9_loop {
            write!(f, "{n9_loop}")?;
        }
        for nte in &self.nte {
            write!(f, "{nte}")?;
        }
        if let Some(ref w20) = self.w20 {
            write!(f, "{w20}")?;
        }
        for qty in &self.qty {
            write!(f, "{qty}")?;
        }
        if let Some(ref amt) = self.amt {
            write!(f, "{amt}")?;
        }
        for g62 in &self.g62 {
            write!(f, "{g62}")?;
        }
        if let Some(ref g66) = self.g66 {
            write!(f, "{g66}")?;
        }
        for n1_loop in &self.n1_loop {
            write!(f, "{n1_loop}")?;
        }
        for per in &self.per {
            write!(f, "{per}")?;
        }
        for lh2 in &self.lh2 {
            write!(f, "{lh2}")?;
        }
        if let Some(ref lhr) = self.lhr {
            write!(f, "{lhr}")?;
        }
        for lh6 in &self.lh6 {
            write!(f, "{lh6}")?;
        }
        for lm_loop in &self.lm_loop {
            write!(f, "{lm_loop}")?;
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, _940_Loop300, nom::error::Error<&'a str>> for _940_Loop300 {
    fn parse(input: &'a str) -> IResult<&'a str, _940_Loop300> {
        let (mut rest, w01) = W01::parse(input)?;
        let (rest_g69, g69) = opt(G69::parse).parse(rest)?;
        let (rest_n9, n9_loop) = many0(_940_0300_W01_N9_Loop::parse).parse(rest_g69)?;
        let (rest_nte, nte) = many0(NTE::parse).parse(rest_n9)?;
        let (rest_w20, w20) = opt(W20::parse).parse(rest_nte)?;
        let (rest_qty, qty) = many0(QTY::parse).parse(rest_w20)?;
        let (rest_amt, amt) = opt(AMT::parse).parse(rest_qty)?;
        let (rest_g62, g62) = many0(G62::parse).parse(rest_amt)?;
        let (rest_g66, g66) = opt(G66::parse).parse(rest_g62)?;
        let (rest_n1, n1_loop) = many0(N1::parse).parse(rest_g66)?;
        let (rest_per, per) = many0(PER::parse).parse(rest_n1)?;
        let (rest_lh2, lh2) = many0(LH2::parse).parse(rest_per)?;
        let (rest_lhr, lhr) = opt(LHR::parse).parse(rest_lh2)?;
        let (rest_lh6, lh6) = many0(LH6::parse).parse(rest_lhr)?;
        let (rest_lm, lm_loop) = many0(_940_Loop200::parse).parse(rest_lh6)?;
        rest = rest_lm;

        Ok((
            rest,
            _940_Loop300 {
                w01,
                g69,
                n9_loop,
                nte,
                w20,
                qty,
                amt,
                g62,
                g66,
                n1_loop,
                per,
                lh2,
                lhr,
                lh6,
                lm_loop,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _940_0300_W01_N9_Loop {
    pub n9: N9,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtm: Option<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub msg: Vec<MSG>,
}

impl fmt::Display for _940_0300_W01_N9_Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.n9)?;
        if let Some(ref dtm) = self.dtm {
            write!(f, "{dtm}")?;
        }
        for msg in &self.msg {
            write!(f, "{msg}")?;
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, _940_0300_W01_N9_Loop, nom::error::Error<&'a str>> for _940_0300_W01_N9_Loop {
    fn parse(input: &'a str) -> IResult<&'a str, _940_0300_W01_N9_Loop> {
        let (mut rest, n9) = N9::parse(input)?;
        let (rest_dtm, dtm) = opt(DTM::parse).parse(rest)?;
        let (rest_msg, msg) = many0(MSG::parse).parse(rest_dtm)?;
        rest = rest_msg;
        Ok((
            rest,
            _940_0300_W01_N9_Loop {
                n9,
                dtm,
                msg,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _940_Loop200 {
    pub lm: LM,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lq: Vec<LQ>,
}

impl fmt::Display for _940_Loop200 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lm)?;
        for lq in &self.lq {
            write!(f, "{lq}")?;
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, _940_Loop200, nom::error::Error<&'a str>> for _940_Loop200 {
    fn parse(input: &'a str) -> IResult<&'a str, _940_Loop200> {
        let (mut rest, lm) = LM::parse(input)?;
        let (rest_lq, lq) = many0(LQ::parse).parse(rest)?;
        rest = rest_lq;
        Ok((
            rest,
            _940_Loop200 {
                lm,
                lq,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _940_0300_Loop {
    pub lx: LX,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub man: Vec<MAN>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sdq: Vec<SDQ>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n1_detail: Vec<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62_detail: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub w01_loop: Vec<_940_Loop300>, // Nested loop 0310
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ls_loop: Vec<_940_0300_Ls_Loop>, // Nested loop 0330
}

impl fmt::Display for _940_0300_Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lx)?;
        for man in &self.man {
            write!(f, "{man}")?;
        }
        for sdq in &self.sdq {
            write!(f, "{sdq}")?;
        }
        for n1_detail in &self.n1_detail {
            write!(f, "{n1_detail}")?;
        }
        for g62_detail in &self.g62_detail {
            write!(f, "{g62_detail}")?;
        }
        for w01_loop in &self.w01_loop {
            write!(f, "{w01_loop}")?;
        }
        for ls_loop in &self.ls_loop {
            write!(f, "{ls_loop}")?;
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, _940_0300_Loop, nom::error::Error<&'a str>> for _940_0300_Loop {
    fn parse(input: &'a str) -> IResult<&'a str, _940_0300_Loop> {
        let (mut rest, lx) = LX::parse(input)?;
        let (rest_man, man) = many0(MAN::parse).parse(rest)?;
        let (rest_sdq, sdq) = many0(SDQ::parse).parse(rest_man)?;
        let (rest_n1, n1_detail) = many0(N1::parse).parse(rest_sdq)?;
        let (rest_g62, g62_detail) = many0(G62::parse).parse(rest_n1)?;

        // Nested loop for W01
        let (rest_w01, w01_loop) = many0(_940_Loop300::parse).parse(rest_g62)?;

        // Nested loop for LS
        let (final_rest, ls_loop) = many0(_940_0300_Ls_Loop::parse).parse(rest_w01)?;

        rest = final_rest;
        Ok((
            rest,
            _940_0300_Loop {
                lx,
                man,
                sdq,
                n1_detail,
                g62_detail,
                w01_loop,
                ls_loop,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _940_0300_Ls_Loop {
    pub ls: LS,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fa1_loop: Vec<_940_0300_Ls_Fa1_Loop>,
    pub le: LE,
}

impl fmt::Display for _940_0300_Ls_Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.ls, self.le)?;
        for fa1_loop in &self.fa1_loop {
            write!(f, "{fa1_loop}")?;
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, _940_0300_Ls_Loop, nom::error::Error<&'a str>> for _940_0300_Ls_Loop {
    fn parse(input: &'a str) -> IResult<&'a str, _940_0300_Ls_Loop> {
        let (mut rest, ls) = LS::parse(input)?;
        let (rest_fa1, fa1_loop) = many0(_940_0300_Ls_Fa1_Loop::parse).parse(rest)?;
        let (rest_le, le) = LE::parse(rest_fa1)?;
        rest = rest_le;
        Ok((
            rest,
            _940_0300_Ls_Loop {
                ls,
                fa1_loop,
                le,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _940_0300_Ls_Fa1_Loop {
    pub fa1: FA1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fa2: Vec<FA2>,
}

impl fmt::Display for _940_0300_Ls_Fa1_Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fa1)?;
        for fa2 in &self.fa2 {
            write!(f, "{fa2}")?;
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, _940_0300_Ls_Fa1_Loop, nom::error::Error<&'a str>> for _940_0300_Ls_Fa1_Loop {
    fn parse(input: &'a str) -> IResult<&'a str, _940_0300_Ls_Fa1_Loop> {
        let (mut rest, fa1) = FA1::parse(input)?;
        let (rest_fa2, fa2) = many0(FA2::parse).parse(rest)?;
        rest = rest_fa2;
        Ok((
            rest,
            _940_0300_Ls_Fa1_Loop {
                fa1,
                fa2,
            },
        ))
    }
} 