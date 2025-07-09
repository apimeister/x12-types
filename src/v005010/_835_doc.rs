use super::segment::*;
use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 835 - Health Care Claim Payment/Advice
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _835 {
    pub st: ST,
    pub bpr: BPR,
    pub nte: Vec<NTE>,
    pub trn: Option<TRN>,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub loop_1000: Vec<_835Loop1000>,
    pub loop_2000: Vec<_835Loop2000>,
    pub plb: Vec<PLB>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _835, nom::error::Error<&'a str>> for _835 {
    fn parse(input: &'a str) -> IResult<&'a str, _835> {
        let mut output = _835::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = BPR::parse(rest)?;
        output.bpr = obj;
        let (rest, obj) = many0(NTE::parse).parse(rest)?;
        output.nte = obj;
        let (rest, obj) = opt(TRN::parse).parse(rest)?;
        output.trn = obj;
        let (rest, obj) = opt(CUR::parse).parse(rest)?;
        output.cur = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.r#ref = obj;
        let (rest, obj) = many0(DTM::parse).parse(rest)?;
        output.dtm = obj;
        // loop 1000
        let mut loop_1000 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = many0(N2::parse).parse(rest)?;
            let (rest, n3) = many0(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            let (rest, per) = many0(PER::parse).parse(rest)?;
            let (rest, rdm) = opt(RDM::parse).parse(rest)?;
            let (rest, dtm) = opt(DTM::parse).parse(rest)?;
            loop_rest = rest;
            loop_1000.push(_835Loop1000 {
                n1,
                n2,
                n3,
                n4,
                r#ref,
                per,
                rdm,
                dtm,
            });
        }
        output.loop_1000 = loop_1000;
        // loop 2000
        let mut loop_2000 = vec![];
        while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, lx) = LX::parse(loop_rest)?;
            let (rest, ts3) = opt(TS3::parse).parse(rest)?;
            let (rest, ts2) = opt(TS2::parse).parse(rest)?;
            loop_rest = rest;
            // loop 2100
            let mut loop_2100 = vec![];
            while peek(opt(CLP::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, clp) = CLP::parse(loop_rest)?;
                let (rest, cas) = many0(CAS::parse).parse(rest)?;
                let (rest, nm1) = many0(NM1::parse).parse(rest)?;
                let (rest, mia) = opt(MIA::parse).parse(rest)?;
                let (rest, moa) = opt(MOA::parse).parse(rest)?;
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                let (rest, dtm) = many0(DTM::parse).parse(rest)?;
                let (rest, per) = many0(PER::parse).parse(rest)?;
                let (rest, amt) = many0(AMT::parse).parse(rest)?;
                let (rest, qty) = many0(QTY::parse).parse(rest)?;
                loop_rest = rest;
                // loop 2110
                let mut loop_2110 = vec![];
                while peek(opt(SVC::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, svc) = SVC::parse(loop_rest)?;
                    let (rest, dtm) = many0(DTM::parse).parse(rest)?;
                    let (rest, cas) = many0(CAS::parse).parse(rest)?;
                    let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                    let (rest, amt) = many0(AMT::parse).parse(rest)?;
                    let (rest, qty) = many0(QTY::parse).parse(rest)?;
                    let (rest, lq) = many0(LQ::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_2110.push(_835Loop2110 {
                        svc,
                        dtm,
                        cas,
                        r#ref,
                        amt,
                        qty,
                        lq,
                    });
                }
                loop_2100.push(_835Loop2100 {
                    clp,
                    cas,
                    nm1,
                    mia,
                    moa,
                    r#ref,
                    dtm,
                    per,
                    amt,
                    qty,
                    loop_2110,
                });
            }
            loop_2000.push(_835Loop2000 {
                lx,
                ts3,
                ts2,
                loop_2100,
            });
        }
        output.loop_2000 = loop_2000;
        let rest = loop_rest;
        let (rest, obj) = many0(PLB::parse).parse(rest)?;
        output.plb = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _835Loop1000 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub rdm: Option<RDM>,
    pub dtm: Option<DTM>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _835Loop2000 {
    pub lx: LX,
    pub ts3: Option<TS3>,
    pub ts2: Option<TS2>,
    pub loop_2100: Vec<_835Loop2100>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _835Loop2100 {
    pub clp: CLP,
    pub cas: Vec<CAS>,
    pub nm1: Vec<NM1>,
    pub mia: Option<MIA>,
    pub moa: Option<MOA>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub per: Vec<PER>,
    pub amt: Vec<AMT>,
    pub qty: Vec<QTY>,
    pub loop_2110: Vec<_835Loop2110>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _835Loop2110 {
    pub svc: SVC,
    pub dtm: Vec<DTM>,
    pub cas: Vec<CAS>,
    pub r#ref: Vec<REF>,
    pub amt: Vec<AMT>,
    pub qty: Vec<QTY>,
    pub lq: Vec<LQ>,
}
