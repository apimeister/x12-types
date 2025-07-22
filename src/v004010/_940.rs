use crate::util::Parser;
use crate::v004010::*;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::multi::many0;
use nom::IResult;
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 940 - Warehouse Shipping Order
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Warehouse Shipping Order Transaction Set (940) for use within the context of an Electronic Data Interchange (EDI) environment. This transaction set can be used to enable the depositor to advise a warehouse to make a shipment, confirm a shipment, or modify or cancel a previously submitted shipping order, or to report the status of the current shipping order to the depositor.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _940 {
    pub st: ST,
    pub w05: W05,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n1_loop: Option<Vec<N1Loop940>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w66: Option<W66>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub w01_loop: Vec<W01Loop940>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w76: Option<W76>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _940, nom::error::Error<&'a str>> for _940 {
    fn parse(input: &'a str) -> IResult<&'a str, _940> {
        let mut output = _940::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = W05::parse(rest)?;
        output.w05 = obj;
        
        // N1 loop
        let mut n1_loop = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = opt(N2::parse).parse(rest)?;
            let (rest, n3) = opt(many0(N3::parse)).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, ref_loop) = opt(many0(RefLoop940::parse)).parse(rest)?;
            let (rest, per) = opt(many0(PER::parse)).parse(rest)?;
            loop_rest = rest;
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
        let rest = loop_rest;
        
        let (rest, obj) = opt(W66::parse).parse(rest)?;
        output.w66 = obj;
        
        // W01 loop
        let mut w01_loop = vec![];
        let mut loop_rest = rest;
        while peek(opt(W01::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, w01) = W01::parse(loop_rest)?;
            let (rest, g69) = opt(G69::parse).parse(rest)?;
            let (rest, n9_loop) = opt(many0(N9Loop940::parse)).parse(rest)?;
            let (rest, g62) = opt(many0(G62::parse)).parse(rest)?;
            let (rest, nte) = opt(many0(NTE::parse)).parse(rest)?;
            let (rest, man_loop) = opt(many0(ManLoop940::parse)).parse(rest)?;
            let (rest, w76_loop) = opt(many0(W76Loop940::parse)).parse(rest)?;
            let (rest, lx_loop) = opt(many0(LxLoop940::parse)).parse(rest)?;
            loop_rest = rest;
            w01_loop.push(W01Loop940 {
                w01,
                g69,
                n9_loop,
                g62,
                nte,
                man_loop,
                w76_loop,
                lx_loop,
            });
        }
        output.w01_loop = w01_loop;
        let rest = loop_rest;
        
        let (rest, obj) = opt(W76::parse).parse(rest)?;
        output.w76 = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct N1Loop940 {
    pub n1: N1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n2: Option<N2>,
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct RefLoop940 {
    pub ref_: REF,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtm: Option<DTM>,
}

impl<'a> Parser<&'a str, RefLoop940, nom::error::Error<&'a str>> for RefLoop940 {
    fn parse(input: &'a str) -> IResult<&'a str, RefLoop940> {
        let (rest, ref_) = REF::parse(input)?;
        let (rest, dtm) = opt(DTM::parse).parse(rest)?;
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
    pub g62: Option<Vec<G62>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nte: Option<Vec<NTE>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub man_loop: Option<Vec<ManLoop940>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w76_loop: Option<Vec<W76Loop940>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lx_loop: Option<Vec<LxLoop940>>,
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

impl<'a> Parser<&'a str, N9Loop940, nom::error::Error<&'a str>> for N9Loop940 {
    fn parse(input: &'a str) -> IResult<&'a str, N9Loop940> {
        let (rest, n9) = N9::parse(input)?;
        let (rest, dtm) = opt(DTM::parse).parse(rest)?;
        let (rest, msg) = opt(many0(MSG::parse)).parse(rest)?;
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

impl<'a> Parser<&'a str, ManLoop940, nom::error::Error<&'a str>> for ManLoop940 {
    fn parse(input: &'a str) -> IResult<&'a str, ManLoop940> {
        let (rest, man) = MAN::parse(input)?;
        let (rest, amt) = opt(many0(AMT::parse)).parse(rest)?;
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

impl<'a> Parser<&'a str, W76Loop940, nom::error::Error<&'a str>> for W76Loop940 {
    fn parse(input: &'a str) -> IResult<&'a str, W76Loop940> {
        let (rest, w76) = W76::parse(input)?;
        let (rest, qty) = opt(many0(QTY::parse)).parse(rest)?;
        let (rest, lm_loop) = opt(many0(LmLoop940::parse)).parse(rest)?;
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
        let (rest, lm) = LM::parse(input)?;
        let (rest, lq) = many0(LQ::parse).parse(rest)?;
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
    pub l5_loop: Option<Vec<L5Loop940>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ls_loop: Option<Vec<LsLoop940>>,
}

impl<'a> Parser<&'a str, LxLoop940, nom::error::Error<&'a str>> for LxLoop940 {
    fn parse(input: &'a str) -> IResult<&'a str, LxLoop940> {
        let (rest, lx) = LX::parse(input)?;
        let (rest, l5_loop) = opt(many0(L5Loop940::parse)).parse(rest)?;
        let (rest, ls_loop) = opt(many0(LsLoop940::parse)).parse(rest)?;
        Ok((
            rest,
            LxLoop940 {
                lx,
                l5_loop,
                ls_loop,
            },
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct L5Loop940 {
    pub l5: L5,
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

impl<'a> Parser<&'a str, LsLoop940, nom::error::Error<&'a str>> for LsLoop940 {
    fn parse(input: &'a str) -> IResult<&'a str, LsLoop940> {
        let (rest, ls) = LS::parse(input)?;
        let (rest, fa1_loop) = many0(Fa1Loop940::parse).parse(rest)?;
        let (rest, le) = LE::parse(rest)?;
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
        let (rest, fa1) = FA1::parse(input)?;
        let (rest, fa2) = many0(FA2::parse).parse(rest)?;
        Ok((
            rest,
            Fa1Loop940 {
                fa1,
                fa2,
            },
        ))
    }
} 