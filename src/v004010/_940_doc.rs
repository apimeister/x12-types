use crate::util::Parser;
use crate::v004010::*;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::multi::many0;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;
use x12_types_macros::DisplayX12;

/// 940 - Warehouse Shipping Order
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Warehouse Shipping Order Transaction Set (940) for use within the context of an Electronic Data Interchange (EDI) environment. This transaction set can be used to enable the depositor to advise a warehouse to make a shipment, confirm a shipment, or modify or cancel a previously submitted shipping order, or to report the status of the current shipping order to the depositor.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _940 {
    pub _010: ST,
    pub _020: W05,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0100: Vec<_940_0100_Loop>,
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
    pub _0200_loop: Vec<_940_0200_Loop>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _0300_loop: Vec<_940_0300_Loop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _0400: Option<W76>,
    pub _0500: SE,
}

impl fmt::Display for _940 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.st)?;
        write!(f, "{}", self.w05)?;
        if let Some(ref n1_loop_vec) = self.n1_loop {
            for n1_loop in n1_loop_vec {
                write!(f, "{n1_loop}")?;
            }
        }
        if let Some(ref n9_heading_vec) = self.n9_heading {
            for n9 in n9_heading_vec {
                write!(f, "{n9}")?;
            }
        }
        if let Some(ref g61_vec) = self.g61 {
            for g61 in g61_vec {
                write!(f, "{g61}")?;
            }
        }
        if let Some(ref g62_heading_vec) = self.g62_heading {
            for g62 in g62_heading_vec {
                write!(f, "{g62}")?;
            }
        }
        if let Some(ref nte_heading_vec) = self.nte_heading {
            for nte in nte_heading_vec {
                write!(f, "{nte}")?;
            }
        }
        if let Some(ref w09) = self.w09 {
            write!(f, "{w09}")?;
        }
        if let Some(ref w66) = self.w66 {
            write!(f, "{w66}")?;
        }
        if let Some(ref r2_vec) = self.r2 {
            for r2 in r2_vec {
                write!(f, "{r2}")?;
            }
        }
        if let Some(ref bnx) = self.bnx {
            write!(f, "{bnx}")?;
        }
        if let Some(ref lm_loop_heading_vec) = self.lm_loop_heading {
            for lm_loop in lm_loop_heading_vec {
                write!(f, "{lm_loop}")?;
            }
        }
        for lx_loop in &self.lx_loop {
            write!(f, "{lx_loop}")?;
        }
        if let Some(ref w76) = self.w76 {
            write!(f, "{w76}")?;
        }
        write!(f, "{}", self.se)?;
        Ok(())
    }
}

impl<'a> Parser<&'a str, _940, nom::error::Error<&'a str>> for _940 {
    fn parse(input: &'a str) -> IResult<&'a str, _940> {
        let mut output = _940::default();
        let (mut rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest_temp, obj) = W05::parse(rest)?;
        output.w05 = obj;
        rest = rest_temp;

        // N1 loop
        let mut n1_loop = vec![];
        let mut loop_rest = rest;
        while peek(N1::parse).parse(loop_rest)?.1.is_some() {
            let (rest_n1, n1) = N1::parse(loop_rest)?;
            let (rest_n2, n2) = opt(many0(N2::parse)).parse(rest_n1)?;
            let (rest_n3, n3) = opt(many0(N3::parse)).parse(rest_n2)?;
            let (rest_n4, n4) = opt(N4::parse).parse(rest_n3)?;
            let (rest_ref, ref_loop) = opt(many0(RefLoop940::parse)).parse(rest_n4)?;
            let (rest_per, per) = opt(many0(PER::parse)).parse(rest_ref)?;
            loop_rest = rest_per;
            n1_loop.push(N1Loop940 {
                n1,
                n2,
                n3,
                n4,
                ref_loop,
                per,
            });
        }
        output.n1_loop = if n1_loop.is_empty() { None } else { Some(n1_loop) };
        rest = loop_rest;

        let (rest_n9, n9_heading) = opt(many0(N9::parse)).parse(rest)?;
        output.n9_heading = n9_heading;
        rest = rest_n9;

        let (rest_g61, g61) = opt(many0(G61::parse)).parse(rest)?;
        output.g61 = g61;
        rest = rest_g61;

        let (rest_g62, g62_heading) = opt(many0(G62::parse)).parse(rest)?;
        output.g62_heading = g62_heading;
        rest = rest_g62;

        let (rest_nte, nte_heading) = opt(many0(NTE::parse)).parse(rest)?;
        output.nte_heading = nte_heading;
        rest = rest_nte;

        let (rest_w09, w09) = opt(W09::parse).parse(rest)?;
        output.w09 = w09;
        rest = rest_w09;

        let (rest_w66, w66) = opt(W66::parse).parse(rest)?;
        output.w66 = w66;
        rest = rest_w66;

        let (rest_r2, r2) = opt(many0(R2::parse)).parse(rest)?;
        output.r2 = r2;
        rest = rest_r2;

        let (rest_bnx, bnx) = opt(BNX::parse).parse(rest)?;
        output.bnx = bnx;
        rest = rest_bnx;

        let (rest_lm_loop, lm_loop_heading) = opt(many0(LmLoop940::parse)).parse(rest)?;
        output.lm_loop_heading = lm_loop_heading;
        rest = rest_lm_loop;

        // LX loop (detail section)
        let mut lx_loop = vec![];
        let mut loop_rest = rest;
        while peek(LX::parse).parse(loop_rest)?.1.is_some() {
            let (rest_lx, lx) = LX::parse(loop_rest)?;
            let (rest_man_loop, man_loop) = opt(many0(ManLoop940::parse)).parse(rest_lx)?;
            let (rest_sdq, sdq) = opt(many0(SDQ::parse)).parse(rest_man_loop)?;
            let (rest_n1_detail, n1_detail) = opt(many0(N1::parse)).parse(rest_sdq)?;
            let (rest_g62_detail, g62_detail) = opt(many0(G62::parse)).parse(rest_n1_detail)?;

            // Nested loop for W01 (0310)
            let (rest_w01_items, w01_items) = many0(W01Loop940::parse).parse(rest_g62_detail)?;

            // Nested loop for LS (0330)
            let (final_rest, ls_loop) = opt(many0(LsLoop940::parse)).parse(rest_w01_items)?;

            loop_rest = final_rest;

            lx_loop.push(LxLoop940 {
                lx,
                man: if man_loop.is_empty() { None } else { Some(man_loop) },
                sdq: if sdq.is_empty() { None } else { Some(sdq) },
                n1_detail: if n1_detail.is_empty() { None } else { Some(n1_detail) },
                g62_detail: if g62_detail.is_empty() { None } else { Some(g62_detail) },
                w01_loop: w01_items,
                ls_loop: if ls_loop.is_empty() { None } else { Some(ls_loop) },
            });
        }
        output.lx_loop = lx_loop;
        rest = loop_rest;

        let (rest_w76, w76) = opt(W76::parse).parse(rest)?;
        output.w76 = w76;
        rest = rest_w76;

        let (rest_se, se) = SE::parse(rest)?;
        output.se = se;
        rest = rest_se;

        Ok((rest, output))
    }
}

// Loop Structures for 940

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct N1Loop940 {
    pub n1: N1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n2: Option<Vec<N2>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n3: Option<Vec<N3>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n4: Option<N4>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_loop: Option<Vec<RefLoop940>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per: Option<Vec<PER>>,
}

impl fmt::Display for N1Loop940 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.n1)?;
        if let Some(ref n2_vec) = self.n2 {
            for n2 in n2_vec {
                write!(f, "{n2}")?;
            }
        }
        if let Some(ref n3_vec) = self.n3 {
            for n3 in n3_vec {
                write!(f, "{n3}")?;
            }
        }
        if let Some(ref n4) = self.n4 {
            write!(f, "{n4}")?;
        }
        if let Some(ref ref_loop_vec) = self.ref_loop {
            for ref_loop in ref_loop_vec {
                write!(f, "{ref_loop}")?;
            }
        }
        if let Some(ref per_vec) = self.per {
            for per in per_vec {
                write!(f, "{per}")?;
            }
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, N1Loop940, nom::error::Error<&'a str>> for N1Loop940 {
    fn parse(input: &'a str) -> IResult<&'a str, N1Loop940> {
        let (mut rest, n1) = N1::parse(input)?;
        let (rest_n2, n2) = opt(many0(N2::parse)).parse(rest)?;
        let (rest_n3, n3) = opt(many0(N3::parse)).parse(rest_n2)?;
        let (rest_n4, n4) = opt(N4::parse).parse(rest_n3)?;
        let (rest_ref, ref_loop) = opt(many0(RefLoop940::parse)).parse(rest_n4)?;
        let (rest_per, per) = opt(many0(PER::parse)).parse(rest_ref)?;
        rest = rest_per;
        Ok((
            rest,
            N1Loop940 {
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct RefLoop940 {
    pub ref_: REF,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtm: Option<DTM>,
}

impl<'a> Parser<&'a str, RefLoop940, nom::error::Error<&'a str>> for RefLoop940 {
    fn parse(input: &'a str) -> IResult<&'a str, RefLoop940> {
        let (mut rest, ref_) = REF::parse(input)?;
        let (rest_dtm, dtm) = opt(DTM::parse).parse(rest)?;
        rest = rest_dtm;
        Ok((
            rest,
            RefLoop940 {
                ref_,
                dtm,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct W01Loop940 {
    pub w01: W01,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g69: Option<G69>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n9_loop: Option<Vec<N9Loop940>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nte: Option<Vec<NTE>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w20: Option<W20>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<Vec<QTY>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<AMT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g62: Option<Vec<G62>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g66: Option<G66>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n1_loop: Option<Vec<N1>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per: Option<Vec<PER>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lh2: Option<Vec<LH2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lhr: Option<LHR>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lh6: Option<Vec<LH6>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lm_loop: Option<Vec<LmLoop940>>,
}

impl fmt::Display for W01Loop940 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.w01)?;
        if let Some(ref g69) = self.g69 {
            write!(f, "{g69}")?;
        }
        if let Some(ref n9_loop_vec) = self.n9_loop {
            for n9_loop in n9_loop_vec {
                write!(f, "{n9_loop}")?;
            }
        }
        if let Some(ref nte_vec) = self.nte {
            for nte in nte_vec {
                write!(f, "{nte}")?;
            }
        }
        if let Some(ref w20) = self.w20 {
            write!(f, "{w20}")?;
        }
        if let Some(ref qty_vec) = self.qty {
            for qty in qty_vec {
                write!(f, "{qty}")?;
            }
        }
        if let Some(ref amt) = self.amt {
            write!(f, "{amt}")?;
        }
        if let Some(ref g62_vec) = self.g62 {
            for g62 in g62_vec {
                write!(f, "{g62}")?;
            }
        }
        if let Some(ref g66) = self.g66 {
            write!(f, "{g66}")?;
        }
        if let Some(ref n1_loop_vec) = self.n1_loop {
            for n1 in n1_loop_vec {
                write!(f, "{n1}")?;
            }
        }
        if let Some(ref per_vec) = self.per {
            for per in per_vec {
                write!(f, "{per}")?;
            }
        }
        if let Some(ref lh2_vec) = self.lh2 {
            for lh2 in lh2_vec {
                write!(f, "{lh2}")?;
            }
        }
        if let Some(ref lhr) = self.lhr {
            write!(f, "{lhr}")?;
        }
        if let Some(ref lh6_vec) = self.lh6 {
            for lh6 in lh6_vec {
                write!(f, "{lh6}")?;
            }
        }
        if let Some(ref lm_loop_vec) = self.lm_loop {
            for lm_loop in lm_loop_vec {
                write!(f, "{lm_loop}")?;
            }
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, W01Loop940, nom::error::Error<&'a str>> for W01Loop940 {
    fn parse(input: &'a str) -> IResult<&'a str, W01Loop940> {
        let (mut rest, w01) = W01::parse(input)?;
        let (rest_g69, g69) = opt(G69::parse).parse(rest)?;
        let (rest_n9, n9_loop) = opt(many0(N9Loop940::parse)).parse(rest_g69)?;
        let (rest_nte, nte) = opt(many0(NTE::parse)).parse(rest_n9)?;
        let (rest_w20, w20) = opt(W20::parse).parse(rest_nte)?;
        let (rest_qty, qty) = opt(many0(QTY::parse)).parse(rest_w20)?;
        let (rest_amt, amt) = opt(AMT::parse).parse(rest_qty)?;
        let (rest_g62, g62) = opt(many0(G62::parse)).parse(rest_amt)?;
        let (rest_g66, g66) = opt(G66::parse).parse(rest_g62)?;
        let (rest_n1, n1_loop) = opt(many0(N1::parse)).parse(rest_g66)?;
        let (rest_per, per) = opt(many0(PER::parse)).parse(rest_n1)?;
        let (rest_lh2, lh2) = opt(many0(LH2::parse)).parse(rest_per)?;
        let (rest_lhr, lhr) = opt(LHR::parse).parse(rest_lh2)?;
        let (rest_lh6, lh6) = opt(many0(LH6::parse)).parse(rest_lhr)?;
        let (rest_lm, lm_loop) = opt(many0(LmLoop940::parse)).parse(rest_lh6)?;
        rest = rest_lm;

        Ok((
            rest,
            W01Loop940 {
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
pub struct N9Loop940 {
    pub n9: N9,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtm: Option<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<Vec<MSG>>,
}

impl fmt::Display for N9Loop940 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.n9)?;
        if let Some(ref dtm) = self.dtm {
            write!(f, "{dtm}")?;
        }
        if let Some(ref msg_vec) = self.msg {
            for msg in msg_vec {
                write!(f, "{msg}")?;
            }
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, N9Loop940, nom::error::Error<&'a str>> for N9Loop940 {
    fn parse(input: &'a str) -> IResult<&'a str, N9Loop940> {
        let (mut rest, n9) = N9::parse(input)?;
        let (rest_dtm, dtm) = opt(DTM::parse).parse(rest)?;
        let (rest_msg, msg) = opt(many0(MSG::parse)).parse(rest_dtm)?;
        rest = rest_msg;
        Ok((
            rest,
            N9Loop940 {
                n9,
                dtm,
                msg,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ManLoop940 {
    pub man: MAN,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<Vec<AMT>>,
}

impl fmt::Display for ManLoop940 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.man)?;
        if let Some(ref amt_vec) = self.amt {
            for amt in amt_vec {
                write!(f, "{amt}")?;
            }
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, ManLoop940, nom::error::Error<&'a str>> for ManLoop940 {
    fn parse(input: &'a str) -> IResult<&'a str, ManLoop940> {
        let (mut rest, man) = MAN::parse(input)?;
        let (rest_amt, amt) = opt(many0(AMT::parse)).parse(rest)?;
        rest = rest_amt;
        Ok((
            rest,
            ManLoop940 {
                man,
                amt,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct W76Loop940 {
    pub w76: W76,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<Vec<QTY>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lm_loop: Option<Vec<LmLoop940>>,
}

impl fmt::Display for W76Loop940 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.w76)?;
        if let Some(ref qty_vec) = self.qty {
            for qty in qty_vec {
                write!(f, "{qty}")?;
            }
        }
        if let Some(ref lm_loop_vec) = self.lm_loop {
            for lm_loop in lm_loop_vec {
                write!(f, "{lm_loop}")?;
            }
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, W76Loop940, nom::error::Error<&'a str>> for W76Loop940 {
    fn parse(input: &'a str) -> IResult<&'a str, W76Loop940> {
        let (mut rest, w76) = W76::parse(input)?;
        let (rest_qty, qty) = opt(many0(QTY::parse)).parse(rest)?;
        let (rest_lm, lm_loop) = opt(many0(LmLoop940::parse)).parse(rest_qty)?;
        rest = rest_lm;
        Ok((
            rest,
            W76Loop940 {
                w76,
                qty,
                lm_loop,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct LmLoop940 {
    pub lm: LM,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lq: Vec<LQ>,
}

impl<'a> Parser<&'a str, LmLoop940, nom::error::Error<&'a str>> for LmLoop940 {
    fn parse(input: &'a str) -> IResult<&'a str, LmLoop940> {
        let (mut rest, lm) = LM::parse(input)?;
        let (rest_lq, lq) = many0(LQ::parse).parse(rest)?;
        rest = rest_lq;
        Ok((
            rest,
            LmLoop940 {
                lm,
                lq,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LxLoop940 {
    pub lx: LX,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub man: Option<Vec<MAN>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdq: Option<Vec<SDQ>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n1_detail: Option<Vec<N1>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g62_detail: Option<Vec<G62>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub w01_loop: Vec<W01Loop940>, // Nested loop 0310
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ls_loop: Option<Vec<LsLoop940>>, // Nested loop 0330
}

impl fmt::Display for LxLoop940 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lx)?;
        if let Some(ref man_vec) = self.man {
            for man in man_vec {
                write!(f, "{man}")?;
            }
        }
        if let Some(ref sdq_vec) = self.sdq {
            for sdq in sdq_vec {
                write!(f, "{sdq}")?;
            }
        }
        if let Some(ref n1_detail_vec) = self.n1_detail {
            for n1 in n1_detail_vec {
                write!(f, "{n1}")?;
            }
        }
        if let Some(ref g62_detail_vec) = self.g62_detail {
            for g62 in g62_detail_vec {
                write!(f, "{g62}")?;
            }
        }
        for w01_loop in &self.w01_loop {
            write!(f, "{w01_loop}")?;
        }
        if let Some(ref ls_loop_vec) = self.ls_loop {
            for ls_loop in ls_loop_vec {
                write!(f, "{ls_loop}")?;
            }
        }
        Ok(())
    }
}

impl<'a> Parser<&'a str, LxLoop940, nom::error::Error<&'a str>> for LxLoop940 {
    fn parse(input: &'a str) -> IResult<&'a str, LxLoop940> {
        let (mut rest, lx) = LX::parse(input)?;
        let (rest_man, man) = opt(many0(MAN::parse)).parse(rest)?;
        let (rest_sdq, sdq) = opt(many0(SDQ::parse)).parse(rest_man)?;
        let (rest_n1, n1_detail) = opt(many0(N1::parse)).parse(rest_sdq)?;
        let (rest_g62, g62_detail) = opt(many0(G62::parse)).parse(rest_n1)?;

        // Nested loop for W01
        let (rest_w01, w01_loop) = many0(W01Loop940::parse).parse(rest_g62)?;

        // Nested loop for LS
        let (final_rest, ls_loop) = opt(many0(LsLoop940::parse)).parse(rest_w01)?;

        rest = final_rest;
        Ok((
            rest,
            LxLoop940 {
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
pub struct L5Loop940 {
    pub l5: L5,
}

impl fmt::Display for L5Loop940 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.l5)?;
        Ok(())
    }
}

impl<'a> Parser<&'a str, L5Loop940, nom::error::Error<&'a str>> for L5Loop940 {
    fn parse(input: &'a str) -> IResult<&'a str, L5Loop940> {
        let (rest, l5) = L5::parse(input)?;
        Ok((
            rest,
            L5Loop940 {
                l5,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LsLoop940 {
    pub ls: LS,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fa1_loop: Vec<Fa1Loop940>,
    pub le: LE,
}

impl fmt::Display for LsLoop940 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ls)?;
        for fa1_loop in &self.fa1_loop {
            write!(f, "{fa1_loop}")?;
        }
        write!(f, "{}", self.le)?;
        Ok(())
    }
}

impl<'a> Parser<&'a str, LsLoop940, nom::error::Error<&'a str>> for LsLoop940 {
    fn parse(input: &'a str) -> IResult<&'a str, LsLoop940> {
        let (mut rest, ls) = LS::parse(input)?;
        let (rest_fa1, fa1_loop) = many0(Fa1Loop940::parse).parse(rest)?;
        let (rest_le, le) = LE::parse(rest_fa1)?;
        rest = rest_le;
        Ok((
            rest,
            LsLoop940 {
                ls,
                fa1_loop,
                le,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct Fa1Loop940 {
    pub fa1: FA1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fa2: Vec<FA2>,
}

impl<'a> Parser<&'a str, Fa1Loop940, nom::error::Error<&'a str>> for Fa1Loop940 {
    fn parse(input: &'a str) -> IResult<&'a str, Fa1Loop940> {
        let (mut rest, fa1) = FA1::parse(input)?;
        let (rest_fa2, fa2) = many0(FA2::parse).parse(rest)?;
        rest = rest_fa2;
        Ok((
            rest,
            Fa1Loop940 {
                fa1,
                fa2,
            },
        ))
    }
} 