//! v005030 repesents all entities of the 005030 specification.

use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
pub use segment::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use x12_types_macros::DisplayX12;

mod segment;

#[cfg(test)]
mod test_404;
#[cfg(test)]
mod test_segments;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Transmission<T> {
    pub isa: ISA,
    pub functional_group: Vec<FunctionalGroup<T>>,
    pub iea: IEA,
}

impl<'a, T: Default + Parser<&'a str, T, nom::error::Error<&'a str>>>
    Parser<&'a str, Transmission<T>, nom::error::Error<&'a str>> for Transmission<T>
{
    fn parse(input: &'a str) -> IResult<&'a str, Transmission<T>> {
        let mut output = Transmission::default();
        let (input, obj) = ISA::parse(input)?;
        output.isa = obj;
        // functional group
        let (input, gs) = GS::parse(input)?;
        let (input, t_obj) = T::parse(input)?;
        let (input, ge) = GE::parse(input)?;
        let fg = FunctionalGroup {
            gs,
            segments: vec![t_obj],
            ge,
        };
        output.functional_group.push(fg);
        let (input, obj) = IEA::parse(input)?;
        output.iea = obj;
        Ok((input, output))
    }
}

impl<T: Display> Display for Transmission<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![];
        lines.push(format!("{}", self.isa));
        for fg in &self.functional_group {
            lines.push(format!("{}", fg.gs));
            for segment in &fg.segments {
                lines.push(format!("{segment}"));
            }
            lines.push(format!("{}", fg.ge));
        }
        lines.push(format!("{}", self.iea));
        let all = lines.join("");
        write!(f, "{all}")
    }
}

/// 404 - Rail Carrier Shipment Information
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404 {
    pub st: ST,
    pub zc1: Option<ZC1>,
    pub bx: Option<BX>,
    pub bnx: Option<BNX>,
    pub m3: M3,
    pub n9: Vec<N9>,
    pub cm: Vec<CM>,
    pub m1: Option<M1>,
    pub dtm: Option<DTM>,
    pub loop_n7: Vec<_404LoopN7>,
    pub na: Option<NA>,
    pub f9: F9,
    pub d9: D9,
    pub loop_n1: Vec<_404LoopN1>,
    pub loop_s1: Vec<_404LoopS1>,
    pub r2: Vec<R2>,
    pub r9: Option<R9>,
    pub loop_e1: Vec<_404LoopE1>,
    pub h3: Vec<H3>,
    pub ps: Vec<PS>,
    pub loop_lx: Vec<_404LoopLX>,
    pub loop_t1: Vec<_404LoopT1>,
    pub l3: Option<L3>,
    pub ls: Option<LS>,
    pub loop_lh1: Vec<_404LoopLH1>,
    pub le: Option<LE>,
    pub per: Option<PER>,
    pub lh2: Option<LH2>,
    pub lhr: Option<LHR>,
    pub lh6: Option<LH6>,
    pub xh: Option<XH>,
    pub x7: Option<X7>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _404, nom::error::Error<&'a str>> for _404 {
    fn parse(input: &'a str) -> IResult<&'a str, _404> {
        let mut output = _404::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = opt(ZC1::parse).parse(rest)?;
        output.zc1 = obj;
        let (rest, obj) = opt(BX::parse).parse(rest)?;
        output.bx = obj;
        let (rest, obj) = opt(BNX::parse).parse(rest)?;
        output.bnx = obj;
        let (rest, obj) = M3::parse(rest)?;
        output.m3 = obj;
        let (rest, obj) = many0(N9::parse).parse(rest)?;
        output.n9 = obj;
        let (rest, obj) = many0(CM::parse).parse(rest)?;
        output.cm = obj;
        let (rest, obj) = opt(M1::parse).parse(rest)?;
        output.m1 = obj;
        let (rest, obj) = opt(DTM::parse).parse(rest)?;
        output.dtm = obj;
        // loop n7
        let mut loop_n7 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N7::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n7) = N7::parse(loop_rest)?;
            let (rest, em) = opt(EM::parse).parse(rest)?;
            let (rest, m7) = opt(M7::parse).parse(rest)?;
            let (rest, n5) = opt(N5::parse).parse(rest)?;
            let (rest, ic) = opt(IC::parse).parse(rest)?;
            let (rest, im) = opt(IM::parse).parse(rest)?;
            let (rest, m12) = opt(M12::parse).parse(rest)?;
            let (rest, ga) = opt(GA::parse).parse(rest)?;
            loop_rest = rest;
            loop_n7.push(_404LoopN7 {
                n7,
                em,
                loop_vc: vec![],
                m7,
                n5,
                ic,
                im,
                m12,
                loop_e1: vec![],
                ga,
                loop_ref: vec![],
            });
        }
        let rest = loop_rest;
        output.loop_n7 = loop_n7;
        let (rest, obj) = opt(NA::parse).parse(rest)?;
        output.na = obj;
        let (rest, obj) = F9::parse(rest)?;
        output.f9 = obj;
        let (rest, obj) = D9::parse(rest)?;
        output.d9 = obj;
        // loop n1
        let mut loop_n1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = opt(N2::parse).parse(rest)?;
            let (rest, n3) = opt(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, r#ref) = opt(REF::parse).parse(rest)?;
            let (rest, per) = opt(PER::parse).parse(rest)?;
            let (rest, bl) = opt(BL::parse).parse(rest)?;
            loop_rest = rest;
            loop_n1.push(_404LoopN1 {
                n1,
                n2,
                n3,
                n4,
                r#ref,
                per,
                bl,
            });
        }
        let rest = loop_rest;
        output.loop_n1 = loop_n1;
        let (rest, obj) = many0(R2::parse).parse(rest)?;
        output.r2 = obj;
        let (rest, obj) = opt(R9::parse).parse(rest)?;
        output.r9 = obj;
        let (rest, obj) = many0(H3::parse).parse(rest)?;
        output.h3 = obj;
        let (rest, obj) = many0(PS::parse).parse(rest)?;
        output.ps = obj;
        // loop lx
        let mut loop_lx = vec![];
        let mut loop_rest = rest;
        while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, lx) = LX::parse(loop_rest)?;
            let (rest, l5) = L5::parse(rest)?;
            let (rest, x1) = opt(X1::parse).parse(rest)?;
            loop_rest = rest;
            // loop l0
            let mut loop_l0 = vec![];
            while peek(opt(L0::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, l0) = opt(L0::parse).parse(loop_rest)?;
                let (rest, mea) = opt(MEA::parse).parse(rest)?;
                let (rest, l1) = opt(L1::parse).parse(rest)?;
                loop_rest = rest;
                // loop pi
                let mut loop_pi = vec![];
                while peek(opt(PI::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, pi) = opt(PI::parse).parse(loop_rest)?;
                    let (rest, cd) = many0(CD::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_pi.push(_404LoopL0PI { pi, cd });
                }
                loop_l0.push(_404LoopL0 {
                    l0,
                    mea,
                    l1,
                    loop_pi,
                });
            }
            loop_lx.push(_404LoopLX {
                lx,
                l5,
                loop_l0,
                x1,
            });
        }
        output.loop_lx = loop_lx;
        let rest = loop_rest;
        let (rest, obj) = opt(L3::parse).parse(rest)?;
        output.l3 = obj;
        let (rest, obj) = opt(LS::parse).parse(rest)?;
        output.ls = obj;
        // loop lh1
        let mut loop_lh1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(LH1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, lh1) = opt(LH1::parse).parse(loop_rest)?;
            let (rest, lh2) = many0(LH2::parse).parse(rest)?;
            let (rest, lh3) = many0(LH3::parse).parse(rest)?;
            let (rest, lfh) = many0(LFH::parse).parse(rest)?;
            let (rest, lep) = opt(LEP::parse).parse(rest)?;
            let (rest, lh4) = opt(LH4::parse).parse(rest)?;
            let (rest, lht) = opt(LHT::parse).parse(rest)?;
            let (rest, lhr) = opt(LHR::parse).parse(rest)?;
            let (rest, per) = opt(PER::parse).parse(rest)?;
            loop_rest = rest;
            // loop n1
            let mut loop_n1 = vec![];
            while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, n1) = opt(N1::parse).parse(loop_rest)?;
                let (rest, n3) = many0(N3::parse).parse(rest)?;
                let (rest, n4) = opt(N4::parse).parse(rest)?;
                let (rest, per) = many0(PER::parse).parse(rest)?;
                loop_n1.push(_404LoopLh1N1 { n1, n3, n4, per });
                loop_rest = rest;
            }
            loop_lh1.push(_404LoopLH1 {
                lh1,
                lh2,
                lh3,
                lfh,
                lep,
                lh4,
                lht,
                lhr,
                per,
                loop_n1,
            });
        }
        output.loop_lh1 = loop_lh1;
        let rest = loop_rest;
        let (rest, obj) = opt(LE::parse).parse(rest)?;
        output.le = obj;
        let (rest, obj) = opt(PER::parse).parse(rest)?;
        output.per = obj;
        let (rest, obj) = opt(LH2::parse).parse(rest)?;
        output.lh2 = obj;
        let (rest, obj) = opt(LHR::parse).parse(rest)?;
        output.lhr = obj;
        let (rest, obj) = opt(LH6::parse).parse(rest)?;
        output.lh6 = obj;
        let (rest, obj) = opt(XH::parse).parse(rest)?;
        output.xh = obj;
        let (rest, obj) = opt(X7::parse).parse(rest)?;
        output.x7 = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopN7 {
    pub n7: N7,
    pub em: Option<EM>,
    pub loop_vc: Vec<_404LoopVC>,
    pub m7: Option<M7>,
    pub n5: Option<N5>,
    pub ic: Option<IC>,
    pub im: Option<IM>,
    pub m12: Option<M12>,
    pub loop_e1: Vec<_404LoopN7E1>,
    pub ga: Option<GA>,
    pub loop_ref: Vec<_404LoopN7Ref>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopN7Ref {
    pub _ref: Option<REF>,
    pub n10: Option<N10>,
    pub loop_n1: Vec<_404LoopN7RefN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopN7RefN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopVC {
    pub vc: Option<VC>,
    pub loop_n1: Vec<_404LoopVcN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopVcN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub h3: Option<H3>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub r#ref: Option<REF>,
    pub per: Option<PER>,
    pub bl: Option<BL>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopS1 {
    pub s1: Option<S1>,
    pub s2: Option<S2>,
    pub s9: Option<S9>,
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Option<PER>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopN7E1 {
    pub e1: E1,
    pub e4: Option<E4>,
    pub e5: Option<E5>,
    pub pi: Option<PI>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopE1 {
    pub e1: E1,
    pub e4: Option<E4>,
    pub e5: Option<E5>,
    pub pi: Option<PI>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopL0 {
    pub l0: Option<L0>,
    pub mea: Option<MEA>,
    pub l1: Option<L1>,
    pub loop_pi: Vec<_404LoopL0PI>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopL0PI {
    pub pi: Option<PI>,
    pub cd: Vec<CD>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopT1 {
    pub t1: Option<T1>,
    pub t2: Option<T2>,
    pub t3: Option<T3>,
    pub t6: Option<T6>,
    pub t8: Option<T8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopLH1 {
    pub lh1: Option<LH1>,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Option<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Option<LHT>,
    pub lhr: Option<LHR>,
    pub per: Option<PER>,
    pub loop_n1: Vec<_404LoopLh1N1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopLh1N1 {
    pub n1: Option<N1>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopLX {
    pub lx: LX,
    pub l5: L5,
    pub loop_l0: Vec<_404LoopL0>,
    pub x1: Option<X1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopRef {
    pub _ref: Option<REF>,
    pub n10: Option<N10>,
    pub loop_n1: Vec<_404LoopRefN1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopRefN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct FunctionalGroup<T> {
    pub gs: GS,
    pub segments: Vec<T>,
    pub ge: GE,
}
