use super::segment::*;
use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 855 - Purchase Order Acknowledgment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855 {
    pub st: ST,
    pub bak: BAK,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub tax: Vec<TAX>,
    pub fob: Vec<FOB>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub csh: Option<CSH>,
    pub loop_sac: Vec<_855LoopSac>,
    pub itd: Vec<ITD>,
    pub dis: Vec<DIS>,
    pub inc: Option<INC>,
    pub dtm: Vec<DTM>,
    pub ldt: Vec<LDT>,
    pub lin: Vec<LIN>,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub man: Vec<MAN>,
    pub txi: Vec<TXI>,
    pub ctb: Vec<CTB>,
    pub loop_n9: Vec<_855LoopN9>,
    pub loop_n1: Vec<_855LoopN1>,
    pub loop_po1: Vec<_855LoopPo1>,
    pub loop_ctt: Vec<_855LoopCtt>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _855, nom::error::Error<&'a str>> for _855 {
    fn parse(input: &'a str) -> IResult<&'a str, _855> {
        let mut output = _855::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = BAK::parse(rest)?;
        output.bak = obj;
        let (rest, obj) = opt(CUR::parse).parse(rest)?;
        output.cur = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.r#ref = obj;
        let (rest, obj) = many0(PER::parse).parse(rest)?;
        output.per = obj;
        let (rest, obj) = many0(TAX::parse).parse(rest)?;
        output.tax = obj;
        let (rest, obj) = many0(FOB::parse).parse(rest)?;
        output.fob = obj;
        let (rest, obj) = many0(CTP::parse).parse(rest)?;
        output.ctp = obj;
        let (rest, obj) = many0(PAM::parse).parse(rest)?;
        output.pam = obj;
        let (rest, obj) = opt(CSH::parse).parse(rest)?;
        output.csh = obj;

        // loop sac
        let mut loop_sac = vec![];
        let mut loop_rest = rest;
        while peek(opt(SAC::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, sac) = SAC::parse(loop_rest)?;
            let (rest, cur) = opt(CUR::parse).parse(rest)?;
            loop_rest = rest;
            loop_sac.push(_855LoopSac { sac, cur });
        }
        let rest = loop_rest;
        output.loop_sac = loop_sac;

        let (rest, obj) = many0(ITD::parse).parse(rest)?;
        output.itd = obj;
        let (rest, obj) = many0(DIS::parse).parse(rest)?;
        output.dis = obj;
        let (rest, obj) = opt(INC::parse).parse(rest)?;
        output.inc = obj;
        let (rest, obj) = many0(DTM::parse).parse(rest)?;
        output.dtm = obj;
        let (rest, obj) = many0(LDT::parse).parse(rest)?;
        output.ldt = obj;
        let (rest, obj) = many0(LIN::parse).parse(rest)?;
        output.lin = obj;
        let (rest, obj) = many0(SI::parse).parse(rest)?;
        output.si = obj;
        let (rest, obj) = many0(PID::parse).parse(rest)?;
        output.pid = obj;
        let (rest, obj) = many0(MEA::parse).parse(rest)?;
        output.mea = obj;
        let (rest, obj) = many0(PWK::parse).parse(rest)?;
        output.pwk = obj;
        let (rest, obj) = many0(PKG::parse).parse(rest)?;
        output.pkg = obj;
        let (rest, obj) = many0(TD1::parse).parse(rest)?;
        output.td1 = obj;
        let (rest, obj) = many0(TD5::parse).parse(rest)?;
        output.td5 = obj;
        let (rest, obj) = many0(TD3::parse).parse(rest)?;
        output.td3 = obj;
        let (rest, obj) = many0(TD4::parse).parse(rest)?;
        output.td4 = obj;
        let (rest, obj) = many0(MAN::parse).parse(rest)?;
        output.man = obj;
        let (rest, obj) = many0(TXI::parse).parse(rest)?;
        output.txi = obj;
        let (rest, obj) = many0(CTB::parse).parse(rest)?;
        output.ctb = obj;

        // loop_n9
        let mut loop_n9 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N9::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n9) = N9::parse(loop_rest)?;
            let (rest, dtm) = many0(DTM::parse).parse(rest)?;
            let (rest, mtx) = many0(MTX::parse).parse(rest)?;
            let (rest, pwk) = many0(PWK::parse).parse(rest)?;
            let (rest, efi) = many0(EFI::parse).parse(rest)?;
            loop_rest = rest;
            loop_n9.push(_855LoopN9 {
                n9,
                dtm,
                mtx,
                pwk,
                efi,
            });
        }
        let rest = loop_rest;
        output.loop_n9 = loop_n9;

        // loop_n1
        let mut loop_n1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = many0(N2::parse).parse(rest)?;
            let (rest, n3) = many0(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            let (rest, per) = many0(PER::parse).parse(rest)?;
            let (rest, si) = many0(SI::parse).parse(rest)?;
            let (rest, fob) = opt(FOB::parse).parse(rest)?;
            let (rest, dtm) = opt(DTM::parse).parse(rest)?;
            let (rest, ldt) = opt(LDT::parse).parse(rest)?;
            loop_rest = rest;
            loop_n1.push(_855LoopN1 {
                n1,
                n2,
                n3,
                n4,
                r#ref,
                per,
                si,
                fob,
                dtm,
                ldt,
            });
        }
        let rest = loop_rest;
        output.loop_n1 = loop_n1;

        // loop_po1
        let mut loop_po1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(PO1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, obj) = _855LoopPo1::parse(loop_rest)?;
            loop_rest = rest;
            loop_po1.push(obj);
        }
        let rest = loop_rest;
        output.loop_po1 = loop_po1;

        // loop_ctt
        let mut loop_ctt = vec![];
        let mut loop_rest = rest;
        while peek(opt(CTT::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, ctt) = CTT::parse(loop_rest)?;
            let (rest, amt) = opt(AMT::parse).parse(rest)?;
            loop_rest = rest;
            loop_ctt.push(_855LoopCtt { ctt, amt });
        }
        let rest = loop_rest;
        output.loop_ctt = loop_ctt;

        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855LoopSac {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855LoopN9 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub pwk: Vec<PWK>,
    pub efi: Vec<EFI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub si: Vec<SI>,
    pub fob: Option<FOB>,
    pub dtm: Option<DTM>,
    pub ldt: Option<LDT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855LoopPo1 {
    pub po1: PO1,
    pub lin: Vec<LIN>,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub po4: Option<PO4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub loop_sac: Vec<_855LoopSacPo1>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub ack: Vec<ACK>,
    pub dtm: Vec<DTM>,
    pub ctb: Vec<CTB>,
    pub txi: Vec<TXI>,
    pub ldt: Vec<LDT>,
    pub man: Vec<MAN>,
    pub sdq: Vec<SDQ>,
    pub loop_sch: Vec<_855LoopSch>,
    pub loop_n9: Vec<_855LoopN9Po1>,
    pub loop_n1: Vec<_855LoopN1Po1>,
    pub loop_lm: Vec<_855LoopLm>,
}

impl<'a> Parser<&'a str, _855LoopPo1, nom::error::Error<&'a str>> for _855LoopPo1 {
    fn parse(input: &'a str) -> IResult<&'a str, _855LoopPo1> {
        let mut output = _855LoopPo1::default();
        let (rest, obj) = PO1::parse(input)?;
        output.po1 = obj;
        let (rest, obj) = many0(LIN::parse).parse(rest)?;
        output.lin = obj;
        let (rest, obj) = many0(SI::parse).parse(rest)?;
        output.si = obj;
        let (rest, obj) = many0(PID::parse).parse(rest)?;
        output.pid = obj;
        let (rest, obj) = many0(MEA::parse).parse(rest)?;
        output.mea = obj;
        let (rest, obj) = many0(PWK::parse).parse(rest)?;
        output.pwk = obj;
        let (rest, obj) = many0(PKG::parse).parse(rest)?;
        output.pkg = obj;
        let (rest, obj) = opt(PO4::parse).parse(rest)?;
        output.po4 = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.r#ref = obj;
        let (rest, obj) = many0(PER::parse).parse(rest)?;
        output.per = obj;

        // loop_sac
        let mut loop_sac = vec![];
        let mut loop_rest = rest;
        while peek(opt(SAC::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, sac) = SAC::parse(loop_rest)?;
            let (rest, cur) = opt(CUR::parse).parse(rest)?;
            loop_rest = rest;
            loop_sac.push(_855LoopSacPo1 { sac, cur });
        }
        let rest = loop_rest;
        output.loop_sac = loop_sac;

        let (rest, obj) = many0(CTP::parse).parse(rest)?;
        output.ctp = obj;
        let (rest, obj) = many0(PAM::parse).parse(rest)?;
        output.pam = obj;
        let (rest, obj) = many0(ACK::parse).parse(rest)?;
        output.ack = obj;
        let (rest, obj) = many0(DTM::parse).parse(rest)?;
        output.dtm = obj;
        let (rest, obj) = many0(CTB::parse).parse(rest)?;
        output.ctb = obj;
        let (rest, obj) = many0(TXI::parse).parse(rest)?;
        output.txi = obj;
        let (rest, obj) = many0(LDT::parse).parse(rest)?;
        output.ldt = obj;
        let (rest, obj) = many0(MAN::parse).parse(rest)?;
        output.man = obj;
        let (rest, obj) = many0(SDQ::parse).parse(rest)?;
        output.sdq = obj;

        // loop_sch
        let mut loop_sch = vec![];
        let mut loop_rest = rest;
        while peek(opt(SCH::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, sch) = SCH::parse(loop_rest)?;
            let (rest, td1) = many0(TD1::parse).parse(rest)?;
            let (rest, td5) = many0(TD5::parse).parse(rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            loop_rest = rest;
            loop_sch.push(_855LoopSch {
                sch,
                td1,
                td5,
                r#ref,
            });
        }
        let rest = loop_rest;
        output.loop_sch = loop_sch;

        // loop_n9
        let mut loop_n9 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N9::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n9) = N9::parse(loop_rest)?;
            let (rest, dtm) = many0(DTM::parse).parse(rest)?;
            let (rest, mtx) = many0(MTX::parse).parse(rest)?;
            loop_rest = rest;
            loop_n9.push(_855LoopN9Po1 { n9, dtm, mtx });
        }
        let rest = loop_rest;
        output.loop_n9 = loop_n9;

        // loop_n1
        let mut loop_n1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = many0(N2::parse).parse(rest)?;
            let (rest, n3) = many0(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            let (rest, per) = many0(PER::parse).parse(rest)?;
            loop_rest = rest;
            loop_n1.push(_855LoopN1Po1 {
                n1,
                n2,
                n3,
                n4,
                r#ref,
                per,
            });
        }
        let rest = loop_rest;
        output.loop_n1 = loop_n1;

        // loop_lm
        let mut loop_lm = vec![];
        let mut loop_rest = rest;
        while peek(opt(LM::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, lm) = LM::parse(loop_rest)?;
            let (rest, lq) = many0(LQ::parse).parse(rest)?;
            loop_rest = rest;
            loop_lm.push(_855LoopLm { lm, lq });
        }
        let rest = loop_rest;
        output.loop_lm = loop_lm;

        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855LoopSacPo1 {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855LoopSch {
    pub sch: SCH,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub r#ref: Vec<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855LoopN9Po1 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855LoopN1Po1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _855LoopCtt {
    pub ctt: CTT,
    pub amt: Option<AMT>,
}
