use super::segment::*;
use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 850 - Purchase Order
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850 {
    pub st: ST,
    pub beg: BEG,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub tax: Vec<TAX>,
    pub fob: Vec<FOB>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub csh: Vec<CSH>,
    pub tc2: Vec<TC2>,
    pub loop_sac: Vec<_850LoopSac>,
    pub itd: Vec<ITD>,
    pub dis: Vec<DIS>,
    pub inc: Option<INC>,
    pub dtm: Vec<DTM>,
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
    pub pct: Vec<PCT>,
    pub ctb: Vec<CTB>,
    pub txi: Vec<TXI>,
    pub loop_ldt: Vec<_850LoopLdt>,
    pub loop_amt: Vec<_850LoopAmt>,
    pub loop_n9: Vec<_850LoopN9>,
    pub loop_n1: Vec<_850LoopN1>,
    pub loop_po1: Vec<_850LoopPo1>,
    pub ctt: Option<CTT>,
    pub amt: Option<AMT>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _850, nom::error::Error<&'a str>> for _850 {
    fn parse(input: &'a str) -> IResult<&'a str, _850> {
        let mut output = _850::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = BEG::parse(rest)?;
        output.beg = obj;
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
        let (rest, obj) = many0(CSH::parse).parse(rest)?;
        output.csh = obj;
        let (rest, obj) = many0(TC2::parse).parse(rest)?;
        output.tc2 = obj;

        // loop sac
        let mut loop_sac = vec![];
        let mut loop_rest = rest;
        while peek(opt(SAC::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, sac) = SAC::parse(loop_rest)?;
            let (rest, cur) = opt(CUR::parse).parse(rest)?;
            loop_rest = rest;
            loop_sac.push(_850LoopSac { sac, cur });
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
        let (rest, obj) = many0(PCT::parse).parse(rest)?;
        output.pct = obj;
        let (rest, obj) = many0(CTB::parse).parse(rest)?;
        output.ctb = obj;
        let (rest, obj) = many0(TXI::parse).parse(rest)?;
        output.txi = obj;

        // loop ldt
        let mut loop_ldt = vec![];
        let mut loop_rest = rest;
        while peek(opt(LDT::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, ldt) = LDT::parse(loop_rest)?;
            let (rest, qty) = many0(QTY::parse).parse(rest)?;
            let (rest, mtx) = many0(MTX::parse).parse(rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            loop_rest = rest;
            loop_ldt.push(_850LoopLdt {
                ldt,
                qty,
                mtx,
                r#ref,
            });
        }
        let rest = loop_rest;
        output.loop_ldt = loop_ldt;

        // loop amt
        let mut loop_amt = vec![];
        let mut loop_rest = rest;
        while peek(opt(AMT::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, amt) = AMT::parse(loop_rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            let (rest, dtm) = opt(DTM::parse).parse(rest)?;
            let (rest, pct) = many0(PCT::parse).parse(rest)?;

            // loop fa1
            let mut loop_fa1 = vec![];
            let mut loop_fa1_rest = rest;
            while peek(opt(FA1::parse)).parse(loop_fa1_rest)?.1.is_some() {
                let (rest, fa1) = FA1::parse(loop_fa1_rest)?;
                let (rest, fa2) = many0(FA2::parse).parse(rest)?;
                loop_fa1_rest = rest;
                loop_fa1.push(_850LoopFa1 { fa1, fa2 });
            }
            let rest = loop_fa1_rest;
            loop_rest = rest;
            loop_amt.push(_850LoopAmt {
                amt,
                r#ref,
                dtm,
                pct,
                loop_fa1,
            });
        }
        let rest = loop_rest;
        output.loop_amt = loop_amt;

        // loop n9
        let mut loop_n9 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N9::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n9) = N9::parse(loop_rest)?;
            let (rest, dtm) = many0(DTM::parse).parse(rest)?;
            let (rest, mtx) = many0(MTX::parse).parse(rest)?;
            let (rest, pwk) = many0(PWK::parse).parse(rest)?;
            let (rest, efi) = many0(EFI::parse).parse(rest)?;
            loop_rest = rest;
            loop_n9.push(_850LoopN9 {
                n9,
                dtm,
                mtx,
                pwk,
                efi,
            });
        }
        let rest = loop_rest;
        output.loop_n9 = loop_n9;

        // loop n1
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
            let (rest, td1) = opt(TD1::parse).parse(rest)?;
            let (rest, td3) = opt(TD3::parse).parse(rest)?;
            let (rest, td4) = opt(TD4::parse).parse(rest)?;
            let (rest, td5) = many0(TD5::parse).parse(rest)?;
            let (rest, pkg) = many0(PKG::parse).parse(rest)?;
            let (rest, ldt) = opt(LDT::parse).parse(rest)?;
            loop_rest = rest;
            loop_n1.push(_850LoopN1 {
                n1,
                n2,
                n3,
                n4,
                r#ref,
                per,
                si,
                fob,
                td1,
                td3,
                td4,
                td5,
                pkg,
                ldt,
            });
        }
        let rest = loop_rest;
        output.loop_n1 = loop_n1;

        // loop po1
        let mut loop_po1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(PO1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, po1) = PO1::parse(loop_rest)?;
            let (rest, lin) = many0(LIN::parse).parse(rest)?;
            let (rest, si) = many0(SI::parse).parse(rest)?;
            let (rest, cur) = opt(CUR::parse).parse(rest)?;
            let (rest, po3) = many0(PO3::parse).parse(rest)?;
            let (rest, ctp) = many0(CTP::parse).parse(rest)?;
            let (rest, pam) = many0(PAM::parse).parse(rest)?;
            let (rest, mea) = many0(MEA::parse).parse(rest)?;

            // loop_pid
            let mut loop_pid = vec![];
            let mut loop_pid_rest = rest;
            while peek(opt(PID::parse)).parse(loop_pid_rest)?.1.is_some() {
                let (rest, pid) = PID::parse(loop_pid_rest)?;
                let (rest, mea) = many0(MEA::parse).parse(rest)?;
                loop_pid_rest = rest;
                loop_pid.push(_850LoopPid { pid, mea });
            }
            let rest = loop_pid_rest;

            let (rest, pwk) = many0(PWK::parse).parse(rest)?;
            let (rest, po4) = opt(PO4::parse).parse(rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            let (rest, per) = many0(PER::parse).parse(rest)?;

            // loop_sac
            let mut loop_sac = vec![];
            let mut loop_sac_rest = rest;
            while peek(opt(SAC::parse)).parse(loop_sac_rest)?.1.is_some() {
                let (rest, sac) = SAC::parse(loop_sac_rest)?;
                let (rest, cur) = opt(CUR::parse).parse(rest)?;
                loop_sac_rest = rest;
                loop_sac.push(_850LoopSac { sac, cur });
            }
            let rest = loop_sac_rest;

            let (rest, itd) = many0(ITD::parse).parse(rest)?;
            let (rest, dis) = many0(DIS::parse).parse(rest)?;
            let (rest, inc) = opt(INC::parse).parse(rest)?;
            let (rest, dtm) = many0(DTM::parse).parse(rest)?;
            let (rest, tax) = many0(TAX::parse).parse(rest)?;
            let (rest, fob) = many0(FOB::parse).parse(rest)?;
            let (rest, sdq) = many0(SDQ::parse).parse(rest)?;
            let (rest, man) = many0(MAN::parse).parse(rest)?;
            let (rest, td1) = opt(TD1::parse).parse(rest)?;
            let (rest, td5) = many0(TD5::parse).parse(rest)?;
            let (rest, td3) = many0(TD3::parse).parse(rest)?;
            let (rest, td4) = many0(TD4::parse).parse(rest)?;
            let (rest, pct) = many0(PCT::parse).parse(rest)?;
            let (rest, ctb) = many0(CTB::parse).parse(rest)?;
            let (rest, txi) = many0(TXI::parse).parse(rest)?;

            // loop_ldt
            let mut loop_ldt = vec![];
            let mut loop_ldt_rest = rest;
            while peek(opt(LDT::parse)).parse(loop_ldt_rest)?.1.is_some() {
                let (rest, ldt) = LDT::parse(loop_ldt_rest)?;
                let (rest, qty) = many0(QTY::parse).parse(rest)?;
                let (rest, mtx) = many0(MTX::parse).parse(rest)?;
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                loop_ldt_rest = rest;
                loop_ldt.push(_850LoopLdt {
                    ldt,
                    qty,
                    mtx,
                    r#ref,
                });
            }
            let rest = loop_ldt_rest;

            // loop_qty
            let mut loop_qty = vec![];
            let mut loop_qty_rest = rest;
            while peek(opt(QTY::parse)).parse(loop_qty_rest)?.1.is_some() {
                let (rest, qty) = QTY::parse(loop_qty_rest)?;
                let (rest, si) = many0(SI::parse).parse(rest)?;
                loop_qty_rest = rest;
                loop_qty.push(_850LoopQty { qty, si });
            }
            let rest = loop_qty_rest;

            // loop_sch
            let mut loop_sch = vec![];
            let mut loop_sch_rest = rest;
            while peek(opt(SCH::parse)).parse(loop_sch_rest)?.1.is_some() {
                let (rest, sch) = SCH::parse(loop_sch_rest)?;
                let (rest, td1) = many0(TD1::parse).parse(rest)?;
                let (rest, td5) = many0(TD5::parse).parse(rest)?;
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                let (rest, ldt) = opt(LDT::parse).parse(rest)?;
                loop_sch_rest = rest;
                loop_sch.push(_850LoopSch {
                    sch,
                    td1,
                    td5,
                    r#ref,
                    ldt,
                });
            }
            let rest = loop_sch_rest;

            // loop_pkg
            let mut loop_pkg = vec![];
            let mut loop_pkg_rest = rest;
            while peek(opt(PKG::parse)).parse(loop_pkg_rest)?.1.is_some() {
                let (rest, pkg) = PKG::parse(loop_pkg_rest)?;
                let (rest, mea) = many0(MEA::parse).parse(rest)?;
                loop_pkg_rest = rest;
                loop_pkg.push(_850LoopPkg { pkg, mea });
            }
            let rest = loop_pkg_rest;

            // loop_n9
            let mut loop_n9 = vec![];
            let mut loop_n9_rest = rest;
            while peek(opt(N9::parse)).parse(loop_n9_rest)?.1.is_some() {
                let (rest, n9) = N9::parse(loop_n9_rest)?;
                let (rest, dtm) = many0(DTM::parse).parse(rest)?;
                let (rest, mtx) = many0(MTX::parse).parse(rest)?;
                loop_n9_rest = rest;
                loop_n9.push(_850LoopN9Detail { n9, dtm, mtx });
            }
            let rest = loop_n9_rest;

            // loop_n1
            let mut loop_n1 = vec![];
            let mut loop_n1_rest = rest;
            while peek(opt(N1::parse)).parse(loop_n1_rest)?.1.is_some() {
                let (rest, n1) = N1::parse(loop_n1_rest)?;
                let (rest, n2) = many0(N2::parse).parse(rest)?;
                let (rest, n3) = many0(N3::parse).parse(rest)?;
                let (rest, n4) = opt(N4::parse).parse(rest)?;
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                let (rest, per) = many0(PER::parse).parse(rest)?;
                let (rest, dmg) = opt(DMG::parse).parse(rest)?;
                loop_n1_rest = rest;
                loop_n1.push(_850LoopN1Detail {
                    n1,
                    n2,
                    n3,
                    n4,
                    r#ref,
                    per,
                    dmg,
                });
            }
            let rest = loop_n1_rest;

            // loop_sln_detail
            let mut loop_sln_detail = vec![];
            let mut loop_sln_rest = rest;
            while peek(opt(SLN::parse)).parse(loop_sln_rest)?.1.is_some() {
                let (rest, obj) = _850LoopSln::parse(loop_sln_rest)?;
                loop_sln_rest = rest;
                loop_sln_detail.push(obj);
            }
            let rest = loop_sln_rest;

            // loop_adv_detail
            let mut loop_adv_detail = vec![];
            let mut loop_adv_rest = rest;
            while peek(opt(ADV::parse)).parse(loop_adv_rest)?.1.is_some() {
                let (rest, obj) = _850LoopAdv::parse(loop_adv_rest)?;
                loop_adv_rest = rest;
                loop_adv_detail.push(obj);
            }
            let rest = loop_adv_rest;

            // loop_lm
            let mut loop_lm = vec![];
            let mut loop_lm_rest = rest;
            while peek(opt(LM::parse)).parse(loop_lm_rest)?.1.is_some() {
                let (rest, lm) = LM::parse(loop_lm_rest)?;
                let (rest, lq) = many0(LQ::parse).parse(rest)?;
                loop_lm_rest = rest;
                loop_lm.push(_850LoopLm { lm, lq });
            }
            let rest = loop_lm_rest;

            // loop_spi
            let mut loop_spi = vec![];
            let mut loop_spi_rest = rest;
            while peek(opt(SPI::parse)).parse(loop_spi_rest)?.1.is_some() {
                let (rest, spi) = SPI::parse(loop_spi_rest)?;
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                let (rest, dtm) = many0(DTM::parse).parse(rest)?;
                let (rest, msg) = many0(MSG::parse).parse(rest)?;
                // loop_n1 for SPI
                let mut loop_n1_spi = vec![];
                let mut loop_n1_spi_rest = rest;
                while peek(opt(N1::parse)).parse(loop_n1_spi_rest)?.1.is_some() {
                    let (rest, n1) = N1::parse(loop_n1_spi_rest)?;
                    let (rest, n3) = many0(N3::parse).parse(rest)?;
                    let (rest, n4) = opt(N4::parse).parse(rest)?;
                    let (rest, cb1) = opt(CB1::parse).parse(rest)?;
                    loop_n1_spi_rest = rest;
                    loop_n1_spi.push(_850LoopN1Spi { n1, n3, n4, cb1 });
                }
                let rest = loop_n1_spi_rest;
                loop_spi_rest = rest;
                loop_spi.push(_850LoopSpi {
                    spi,
                    r#ref,
                    dtm,
                    msg,
                    loop_n1: loop_n1_spi,
                });
            }
            let rest = loop_spi_rest;

            // loop_adv
            let mut loop_adv = vec![];
            let mut loop_adv_rest = rest;
            while peek(opt(ADV::parse)).parse(loop_adv_rest)?.1.is_some() {
                let (rest, adv) = ADV::parse(loop_adv_rest)?;
                let (rest, dtm) = many0(DTM::parse).parse(rest)?;
                let (rest, mtx) = many0(MTX::parse).parse(rest)?;
                loop_adv_rest = rest;
                loop_adv.push(_850LoopAdv { adv, dtm, mtx });
            }
            let rest = loop_adv_rest;

            // loop_sln
            let mut loop_sln = vec![];
            let mut loop_sln_rest = rest;
            while peek(opt(SLN::parse)).parse(loop_sln_rest)?.1.is_some() {
                let (rest, obj) = _850LoopSln::parse(loop_sln_rest)?;
                loop_sln_rest = rest;
                loop_sln.push(obj);
            }
            let rest = loop_sln_rest;

            loop_rest = rest;
            loop_po1.push(_850LoopPo1 {
                po1,
                lin,
                si,
                cur,
                po3,
                ctp,
                pam,
                mea,
                loop_pid,
                pwk,
                po4,
                r#ref,
                per,
                loop_sac,
                itd,
                dis,
                inc,
                dtm,
                tax,
                fob,
                sdq,
                man,
                td1,
                td5,
                td3,
                td4,
                pct,
                ctb,
                txi,
                loop_ldt,
                loop_qty,
                loop_sch,
                loop_pkg,
                loop_n9,
                loop_n1,
                loop_spi,
                loop_adv: loop_adv_detail,
                loop_lm,
                loop_sln: loop_sln_detail,
            });
        }
        let rest = loop_rest;
        output.loop_po1 = loop_po1;

        let (rest, obj) = opt(CTT::parse).parse(rest)?;
        output.ctt = obj;
        let (rest, obj) = opt(AMT::parse).parse(rest)?;
        output.amt = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopSac {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopLdt {
    pub ldt: LDT,
    pub qty: Vec<QTY>,
    pub mtx: Vec<MTX>,
    pub r#ref: Vec<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopAmt {
    pub amt: AMT,
    pub r#ref: Vec<REF>,
    pub dtm: Option<DTM>,
    pub pct: Vec<PCT>,
    pub loop_fa1: Vec<_850LoopFa1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopN9 {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
    pub pwk: Vec<PWK>,
    pub efi: Vec<EFI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub si: Vec<SI>,
    pub fob: Option<FOB>,
    pub td1: Option<TD1>,
    pub td3: Option<TD3>,
    pub td4: Option<TD4>,
    pub td5: Vec<TD5>,
    pub pkg: Vec<PKG>,
    pub ldt: Option<LDT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopPid {
    pub pid: PID,
    pub mea: Vec<MEA>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopQty {
    pub qty: QTY,
    pub si: Vec<SI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopSch {
    pub sch: SCH,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub r#ref: Vec<REF>,
    pub ldt: Option<LDT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopPkg {
    pub pkg: PKG,
    pub mea: Vec<MEA>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopN9Detail {
    pub n9: N9,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopN1Detail {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dmg: Option<DMG>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopN1Spi {
    pub n1: N1,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub cb1: Option<CB1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopSpi {
    pub spi: SPI,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub msg: Vec<MSG>,
    pub loop_n1: Vec<_850LoopN1Spi>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopAdv {
    pub adv: ADV,
    pub dtm: Vec<DTM>,
    pub mtx: Vec<MTX>,
}

impl<'a> Parser<&'a str, _850LoopAdv, nom::error::Error<&'a str>> for _850LoopAdv {
    fn parse(input: &'a str) -> IResult<&'a str, _850LoopAdv> {
        let mut output = _850LoopAdv::default();
        let (rest, obj) = ADV::parse(input)?;
        output.adv = obj;
        let (rest, obj) = many0(DTM::parse).parse(rest)?;
        output.dtm = obj;
        let (rest, obj) = many0(MTX::parse).parse(rest)?;
        output.mtx = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopPo1 {
    pub po1: PO1,
    pub lin: Vec<LIN>,
    pub si: Vec<SI>,
    pub cur: Option<CUR>,
    pub po3: Vec<PO3>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub mea: Vec<MEA>,
    pub loop_pid: Vec<_850LoopPid>,
    pub pwk: Vec<PWK>,
    pub po4: Option<PO4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub loop_sac: Vec<_850LoopSac>,
    pub itd: Vec<ITD>,
    pub dis: Vec<DIS>,
    pub inc: Option<INC>,
    pub dtm: Vec<DTM>,
    pub tax: Vec<TAX>,
    pub fob: Vec<FOB>,
    pub sdq: Vec<SDQ>,
    pub man: Vec<MAN>,
    pub td1: Option<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub pct: Vec<PCT>,
    pub ctb: Vec<CTB>,
    pub txi: Vec<TXI>,
    pub loop_ldt: Vec<_850LoopLdt>,
    pub loop_qty: Vec<_850LoopQty>,
    pub loop_sch: Vec<_850LoopSch>,
    pub loop_pkg: Vec<_850LoopPkg>,
    pub loop_n9: Vec<_850LoopN9Detail>,
    pub loop_n1: Vec<_850LoopN1Detail>,
    pub loop_spi: Vec<_850LoopSpi>,
    pub loop_sln: Vec<_850LoopSln>,
    pub loop_adv: Vec<_850LoopAdv>,
    pub loop_lm: Vec<_850LoopLm>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _850LoopSln {
    pub sln: SLN,
    pub mtx: Vec<MTX>,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
}

impl<'a> Parser<&'a str, _850LoopSln, nom::error::Error<&'a str>> for _850LoopSln {
    fn parse(input: &'a str) -> IResult<&'a str, _850LoopSln> {
        let mut output = _850LoopSln::default();
        let (rest, obj) = SLN::parse(input)?;
        output.sln = obj;
        let (rest, obj) = many0(MTX::parse).parse(rest)?;
        output.mtx = obj;
        let (rest, obj) = many0(SI::parse).parse(rest)?;
        output.si = obj;
        let (rest, obj) = many0(PID::parse).parse(rest)?;
        output.pid = obj;
        Ok((rest, output))
    }
}
