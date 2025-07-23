use crate::util::Parser;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::multi::many0;
use nom::IResult;
use nom::Parser as _;
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;
use super::segment::*;

/// 945 - Warehouse Shipping Advice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Warehouse Shipping Advice Transaction Set (945) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by the warehouse to advise the depositor that shipment was made. It is used to reconcile order quantities with shipment quantities.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0100 | ST | Transaction Set Header | M | 1
/// 0200 | W06 | Warehouse Shipment Identification | M | 1
/// 0300 | N1 | Name | O | 1 | Loop 100
/// 0310 | N2 | Additional Name Information | O | 2
/// 0320 | N3 | Address Information | O | 2
/// 0330 | N4 | Geographic Location | O | 1
/// 0400 | N9 | Reference Identification | O | 10
/// 0500 | G62 | Date/Time | O | 10
/// 0600 | W27 | Carrier Detail | O | 1
/// 0700 | LX | Assigned Number | O | 1 | Loop 200
/// 0710 | W12 | Warehouse Item Detail | M | 1
/// 0720 | N9 | Reference Identification | O | 10
/// 0800 | W03 | Total Shipment Information | O | 1
/// 0900 | SE | Transaction Set Trailer | M | 1
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
        let mut loop_n1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = many0(N2::parse).parse(rest)?;
            let (rest, n3) = many0(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            loop_rest = rest;
            loop_n1.push(_945LoopN1 { n1, n2, n3, n4 });
        }
        output.loop_n1 = loop_n1;
        let rest = loop_rest;

        let (rest, obj) = many0(N9::parse).parse(rest)?;
        output.n9 = obj;
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

        // loop lx (line item details)
        let mut loop_lx = vec![];
        let mut loop_rest = rest;
        while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, lx) = LX::parse(loop_rest)?;
            let (rest, w12) = W12::parse(rest)?;
            let (rest, g69) = many0(G69::parse).parse(rest)?;
            let (rest, n9) = many0(N9::parse).parse(rest)?;
            loop_rest = rest;
            loop_lx.push(_945LoopLX { lx, w12, g69, n9 });
        }
        output.loop_lx = loop_lx;
        let rest = loop_rest;

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
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _945LoopLX {
    pub lx: LX,
    pub w12: W12,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g69: Vec<G69>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
}
