use super::segment::*;
use crate::util::Parser;
use nom::combinator::opt;
use nom::multi::many0;
use nom::IResult;
use nom::Parser as _;
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 945 - Warehouse Shipping Advice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Warehouse Shipping Advice Transaction Set (945) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by the warehouse to advise the depositor that shipment was made. It is used to reconcile order quantities with shipment quantities.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _945 {
    pub st: ST,
    pub w06: W06,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_n1: Vec<_945LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g61: Vec<G61>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w27: Option<W27>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w6: Option<W6>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w28: Option<W28>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub w10: Vec<W10>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g72: Vec<G72>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lm: Vec<_945LoopLM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lx: Vec<_945LoopLX>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w03: Option<W03>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _945, nom::error::Error<&'a str>> for _945 {
    fn parse(input: &'a str) -> IResult<&'a str, _945> {
        let mut output = _945::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = W06::parse(rest)?;
        output.w06 = obj;

        // loop n1 (name/address information)
        let (rest, loop_n1) = many0(|input| {
            let (rest, n1) = N1::parse(input)?;
            let (rest, n2) = many0(N2::parse).parse(rest)?;
            let (rest, n3) = many0(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, per) = many0(PER::parse).parse(rest)?;
            Ok((
                rest,
                _945LoopN1 {
                    n1,
                    n2,
                    n3,
                    n4,
                    per,
                },
            ))
        })
        .parse(rest)?;
        output.loop_n1 = loop_n1;

        let (rest, obj) = many0(N9::parse).parse(rest)?;
        output.n9 = obj;
        let (rest, obj) = many0(G61::parse).parse(rest)?;
        output.g61 = obj;
        let (rest, obj) = many0(G62::parse).parse(rest)?;
        output.g62 = obj;
        let (rest, obj) = many0(NTE::parse).parse(rest)?;
        output.nte = obj;
        let (rest, obj) = opt(W27::parse).parse(rest)?;
        output.w27 = obj;
        let (rest, obj) = opt(W6::parse).parse(rest)?;
        output.w6 = obj;
        let (rest, obj) = opt(W28::parse).parse(rest)?;
        output.w28 = obj;
        let (rest, obj) = many0(W10::parse).parse(rest)?;
        output.w10 = obj;
        let (rest, obj) = many0(G72::parse).parse(rest)?;
        output.g72 = obj;

        // loop lm (code source information)
        let (rest, loop_lm) = many0(|input| {
            let (rest, lm) = LM::parse(input)?;
            let (rest, lq) = many0(LQ::parse).parse(rest)?;
            Ok((rest, _945LoopLM { lm, lq }))
        })
        .parse(rest)?;
        output.loop_lm = loop_lm;

        // loop lx (assigned number)
        let (rest, loop_lx) = many0(|input| {
            let (rest, lx) = LX::parse(input)?;
            let (rest, man) = many0(MAN::parse).parse(rest)?;
            let (rest, pal) = opt(PAL::parse).parse(rest)?;
            let (rest, n9) = many0(N9::parse).parse(rest)?;
            let (rest, loop_w12) = many0(|input| {
                let (rest, w12) = W12::parse(input)?;
                let (rest, g69) = many0(G69::parse).parse(rest)?;
                let (rest, n9) = many0(N9::parse).parse(rest)?;
                let (rest, g62) = many0(G62::parse).parse(rest)?;
                let (rest, qty) = many0(QTY::parse).parse(rest)?;
                let (rest, mea) = many0(MEA::parse).parse(rest)?;
                let (rest, amt) = opt(AMT::parse).parse(rest)?;
                let (rest, r4) = many0(R4::parse).parse(rest)?;
                let (rest, w27) = opt(W27::parse).parse(rest)?;
                let (rest, n1) = many0(N1::parse).parse(rest)?;
                let (rest, g72) = many0(G72::parse).parse(rest)?;
                let (rest, loop_lm) = many0(|input| {
                    let (rest, lm) = LM::parse(input)?;
                    let (rest, lq) = many0(LQ::parse).parse(rest)?;
                    Ok((rest, _945LoopLM { lm, lq }))
                })
                .parse(rest)?;
                let (rest, ls) = opt(LS::parse).parse(rest)?;
                let (rest, loop_lx_detail) = many0(|input| {
                    let (rest, lx) = LX::parse(input)?;
                    let (rest, n9) = many0(N9::parse).parse(rest)?;
                    let (rest, g62) = many0(G62::parse).parse(rest)?;
                    let (rest, n1) = opt(N1::parse).parse(rest)?;
                    let (rest, loop_lm) = many0(|input| {
                        let (rest, lm) = LM::parse(input)?;
                        let (rest, lq) = many0(LQ::parse).parse(rest)?;
                        Ok((rest, _945LoopLM { lm, lq }))
                    })
                    .parse(rest)?;
                    Ok((
                        rest,
                        _945LoopLXDetail {
                            lx,
                            n9,
                            g62,
                            n1,
                            loop_lm,
                        },
                    ))
                })
                .parse(rest)?;
                let (rest, le) = opt(LE::parse).parse(rest)?;
                let (rest, loop_fa1) = many0(|input| {
                    let (rest, fa1) = FA1::parse(input)?;
                    let (rest, fa2) = many0(FA2::parse).parse(rest)?;
                    Ok((rest, _945LoopFA1 { fa1, fa2 }))
                })
                .parse(rest)?;
                Ok((
                    rest,
                    _945LoopW12 {
                        w12,
                        g69,
                        n9,
                        g62,
                        qty,
                        mea,
                        amt,
                        r4,
                        w27,
                        n1,
                        g72,
                        loop_lm,
                        ls,
                        loop_lx_detail,
                        le,
                        loop_fa1,
                    },
                ))
            })
            .parse(rest)?;
            Ok((
                rest,
                _945LoopLX {
                    lx,
                    man,
                    pal,
                    n9,
                    loop_w12,
                },
            ))
        })
        .parse(rest)?;
        output.loop_lx = loop_lx;

        let (rest, obj) = opt(W03::parse).parse(rest)?;
        output.w03 = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _945LoopN1 {
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
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _945LoopLM {
    pub lm: LM,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lq: Vec<LQ>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _945LoopLX {
    pub lx: LX,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub man: Vec<MAN>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pal: Option<PAL>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_w12: Vec<_945LoopW12>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _945LoopW12 {
    pub w12: W12,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g69: Vec<G69>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub qty: Vec<QTY>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mea: Vec<MEA>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<AMT>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r4: Vec<R4>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w27: Option<W27>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n1: Vec<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g72: Vec<G72>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lm: Vec<_945LoopLM>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ls: Option<LS>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lx_detail: Vec<_945LoopLXDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub le: Option<LE>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_fa1: Vec<_945LoopFA1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _945LoopLXDetail {
    pub lx: LX,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n1: Option<N1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lm: Vec<_945LoopLM>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _945LoopFA1 {
    pub fa1: FA1,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fa2: Vec<FA2>,
}
