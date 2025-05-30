//! v005010 repesents all entities of the 005010 specification.

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

mod _276_doc;
pub use _276_doc::*;

mod _277_doc;
pub use _277_doc::*;

mod segment;

#[cfg(test)]
mod test_834;
#[cfg(test)]
mod test_835;
#[cfg(test)]
mod test_837;
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
        let mut loop_rest = input;
        let mut obj_arr = vec![];
        while peek(opt(ST::parse)).parse(loop_rest)?.1.is_some() {
            let (input, t_obj) = T::parse(loop_rest)?;
            loop_rest = input;
            obj_arr.push(t_obj);
        }
        let input = loop_rest;
        let (input, ge) = GE::parse(input)?;
        let fg = FunctionalGroup {
            gs,
            segments: obj_arr,
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
                lines.push(format!("{}", segment));
            }
            lines.push(format!("{}", fg.ge));
        }
        lines.push(format!("{}", self.iea));
        let all = lines.join("");
        write!(f, "{all}")
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct FunctionalGroup<T> {
    pub gs: GS,
    pub segments: Vec<T>,
    pub ge: GE,
}

/// 834 - Benefit Enrollment and Maintenance
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834 {
    pub st: ST,
    pub bgn: BGN,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub amt: Vec<AMT>,
    pub qty: Vec<QTY>,
    pub loop_1000: Vec<_834Loop1000>,
    pub loop_2000: Vec<_834Loop2000>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _834, nom::error::Error<&'a str>> for _834 {
    fn parse(input: &'a str) -> IResult<&'a str, _834> {
        let mut output = _834::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = BGN::parse(rest)?;
        output.bgn = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.r#ref = obj;
        let (rest, obj) = many0(DTP::parse).parse(rest)?;
        output.dtp = obj;
        let (rest, obj) = many0(AMT::parse).parse(rest)?;
        output.amt = obj;
        let (rest, obj) = many0(QTY::parse).parse(rest)?;
        output.qty = obj;
        // loop 1000
        let mut loop_1000 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = many0(N2::parse).parse(rest)?;
            let (rest, n3) = many0(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, per) = many0(PER::parse).parse(rest)?;
            loop_rest = rest;
            loop_1000.push(_834Loop1000 {
                n1,
                n2,
                n3,
                n4,
                per,
                loop_1100: vec![],
            });
        }
        let rest = loop_rest;
        output.loop_1000 = loop_1000;
        // loop 2000
        let mut loop_2000 = vec![];
        let mut loop_rest = rest;
        while peek(opt(INS::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, ins) = opt(INS::parse).parse(loop_rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            let (rest, dtp) = many0(DTP::parse).parse(rest)?;
            loop_rest = rest;
            // loop 2100
            let mut loop_2100 = vec![];
            while peek(opt(NM1::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, nm1) = opt(NM1::parse).parse(loop_rest)?;
                let (rest, per) = opt(PER::parse).parse(rest)?;
                let (rest, n3) = opt(N3::parse).parse(rest)?;
                let (rest, n4) = opt(N4::parse).parse(rest)?;
                let (rest, dmg) = opt(DMG::parse).parse(rest)?;
                let (rest, pm) = opt(PM::parse).parse(rest)?;
                let (rest, ec) = many0(EC::parse).parse(rest)?;
                let (rest, icm) = opt(ICM::parse).parse(rest)?;
                let (rest, amt) = many0(AMT::parse).parse(rest)?;
                let (rest, hlh) = opt(HLH::parse).parse(rest)?;
                let (rest, hi) = many0(HI::parse).parse(rest)?;
                let (rest, lui) = many0(LUI::parse).parse(rest)?;
                loop_rest = rest;
                loop_2100.push(_834Loop2100 {
                    nm1,
                    per,
                    n3,
                    n4,
                    dmg,
                    pm,
                    ec,
                    icm,
                    amt,
                    hlh,
                    hi,
                    lui,
                });
            }
            // loop 2300
            let mut loop_2300 = vec![];
            while peek(opt(HD::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, hd) = opt(HD::parse).parse(loop_rest)?;
                let (rest, dtp) = many0(DTP::parse).parse(rest)?;
                let (rest, amt) = many0(AMT::parse).parse(rest)?;
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                let (rest, idc) = many0(IDC::parse).parse(rest)?;
                loop_rest = rest;
                // loop 2310
                let mut loop_2310 = vec![];
                while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some()
                    || peek(opt(NM1::parse)).parse(loop_rest)?.1.is_some()
                {
                    let (rest, lx) = opt(LX::parse).parse(loop_rest)?;
                    let (rest, nm1) = opt(NM1::parse).parse(rest)?;
                    let (rest, n1) = many0(N1::parse).parse(rest)?;
                    let (rest, n2) = opt(N2::parse).parse(rest)?;
                    let (rest, n3) = many0(N3::parse).parse(rest)?;
                    let (rest, n4) = opt(N4::parse).parse(rest)?;
                    let (rest, per) = many0(PER::parse).parse(rest)?;
                    let (rest, prv) = opt(PRV::parse).parse(rest)?;
                    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
                    let (rest, pla) = opt(PLA::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_2310.push(_834Loop2310 {
                        lx,
                        nm1,
                        n1,
                        n2,
                        n3,
                        n4,
                        per,
                        prv,
                        dtp,
                        pla,
                    });
                }
                // loop 2320
                let mut loop_2320 = vec![];
                while peek(opt(COB::parse)).parse(loop_rest)?.1.is_some()
                    || peek(opt(REF::parse)).parse(loop_rest)?.1.is_some()
                    || peek(opt(DTP::parse)).parse(loop_rest)?.1.is_some()
                {
                    let (rest, cob) = opt(COB::parse).parse(loop_rest)?;
                    let (rest, r#ref) = opt(REF::parse).parse(rest)?;
                    let (rest, dtp) = opt(DTP::parse).parse(rest)?;
                    loop_rest = rest;
                    // loop 2330
                    let mut loop_2330 = vec![];
                    while peek(opt(NM1::parse)).parse(loop_rest)?.1.is_some() {
                        let (rest, nm1) = opt(NM1::parse).parse(loop_rest)?;
                        let (rest, n2) = opt(N2::parse).parse(rest)?;
                        let (rest, n3) = many0(N3::parse).parse(rest)?;
                        let (rest, n4) = opt(N4::parse).parse(rest)?;
                        let (rest, per) = opt(PER::parse).parse(rest)?;
                        loop_rest = rest;
                        loop_2330.push(_834Loop2330 {
                            nm1,
                            n2,
                            n3,
                            n4,
                            per,
                        });
                    }
                    loop_2320.push(_834Loop2320 {
                        cob,
                        r#ref,
                        dtp,
                        loop_2330,
                    });
                }
                loop_2300.push(_834Loop2300 {
                    hd,
                    dtp,
                    amt,
                    r#ref,
                    idc,
                    loop_2310,
                    loop_2320,
                });
            }
            // loop 2600
            let mut loop_2600 = vec![];
            while peek(opt(RP::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, rp) = opt(RP::parse).parse(loop_rest)?;
                let (rest, dtp) = many0(DTP::parse).parse(rest)?;
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                let (rest, inv) = many0(INV::parse).parse(rest)?;
                let (rest, amt) = many0(AMT::parse).parse(rest)?;
                let (rest, qty) = many0(QTY::parse).parse(rest)?;
                let (rest, k3) = many0(K3::parse).parse(rest)?;
                let (rest, rel) = opt(REL::parse).parse(rest)?;
                loop_rest = rest;
                loop_2600.push(_834Loop2600 {
                    rp,
                    dtp,
                    r#ref,
                    inv,
                    amt,
                    qty,
                    k3,
                    rel,
                    loop_2610: vec![],
                    loop_2630: vec![],
                    loop_2650: vec![],
                });
            }
            let rest = loop_rest;
            let (rest, ls) = opt(LS::parse).parse(rest)?;
            loop_rest = rest;
            // loop 2700
            let mut loop_2700 = vec![];
            while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, lx) = opt(LX::parse).parse(loop_rest)?;
                loop_rest = rest;
                // loop 2750
                let mut loop_2750 = vec![];
                while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, n1) = N1::parse(loop_rest)?;
                    let (rest, r#ref) = REF::parse(rest)?;
                    let (rest, dtp) = opt(DTP::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_2750.push(_834Loop2750 { n1, r#ref, dtp });
                }
                loop_2700.push(_834Loop2700 { lx, loop_2750 });
            }
            let rest = loop_rest;
            let (rest, le) = opt(LE::parse).parse(rest)?;
            loop_rest = rest;
            loop_2000.push(_834Loop2000 {
                ins,
                r#ref,
                dtp,
                loop_2100,
                loop_2200: vec![],
                loop_2300,
                loop_2400: vec![],
                loop_2500: vec![],
                loop_2600,
                ls,
                loop_2700,
                le,
            });
        }
        let rest = loop_rest;
        output.loop_2000 = loop_2000;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop1000 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub loop_1100: Vec<_834Loop1100>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop1100 {
    pub act: Option<ACT>,
    pub r#ref: Vec<REF>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub dtp: Option<DTP>,
    pub amt: Option<AMT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2000 {
    pub ins: Option<INS>,
    pub r#ref: Vec<REF>,
    pub dtp: Vec<DTP>,
    pub loop_2100: Vec<_834Loop2100>,
    pub loop_2200: Vec<_834Loop2200>,
    pub loop_2300: Vec<_834Loop2300>,
    pub loop_2400: Vec<_834Loop2400>,
    pub loop_2500: Vec<_834Loop2500>,
    pub loop_2600: Vec<_834Loop2600>,
    pub ls: Option<LS>,
    pub loop_2700: Vec<_834Loop2700>,
    pub le: Option<LE>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2100 {
    pub nm1: Option<NM1>,
    pub per: Option<PER>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dmg: Option<DMG>,
    pub pm: Option<PM>,
    pub ec: Vec<EC>,
    pub icm: Option<ICM>,
    pub amt: Vec<AMT>,
    pub hlh: Option<HLH>,
    pub hi: Vec<HI>,
    pub lui: Vec<LUI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2200 {
    pub dsb: Option<DSB>,
    pub dtp: Vec<DTP>,
    pub ad1: Vec<AD1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2300 {
    pub hd: Option<HD>,
    pub dtp: Vec<DTP>,
    pub amt: Vec<AMT>,
    pub r#ref: Vec<REF>,
    pub idc: Vec<IDC>,
    pub loop_2310: Vec<_834Loop2310>,
    pub loop_2320: Vec<_834Loop2320>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2310 {
    pub lx: Option<LX>,
    pub nm1: Option<NM1>,
    pub n1: Vec<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub prv: Option<PRV>,
    pub dtp: Vec<DTP>,
    pub pla: Option<PLA>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2320 {
    pub cob: Option<COB>,
    pub r#ref: Option<REF>,
    pub dtp: Option<DTP>,
    pub loop_2330: Vec<_834Loop2330>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2330 {
    pub nm1: Option<NM1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2400 {
    pub lc: Option<LC>,
    pub amt: Vec<AMT>,
    pub dtp: Vec<DTP>,
    pub r#ref: Vec<REF>,
    pub loop_2410: Vec<_834Loop2410>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2410 {
    pub ben: Option<BEN>,
    pub nm1: Option<NM1>,
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dmg: Option<DMG>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2500 {
    pub fsa: Option<FSA>,
    pub amt: Vec<AMT>,
    pub dtp: Vec<DTP>,
    pub r#ref: Vec<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2600 {
    pub rp: Option<RP>,
    pub dtp: Vec<DTP>,
    pub r#ref: Vec<REF>,
    pub inv: Vec<INV>,
    pub amt: Vec<AMT>,
    pub qty: Vec<QTY>,
    pub k3: Vec<K3>,
    pub rel: Option<REL>,
    pub loop_2610: Vec<_834Loop2610>,
    pub loop_2630: Vec<_834Loop2630>,
    pub loop_2650: Vec<_834Loop2650>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2610 {
    pub nm1: Option<NM1>,
    pub n2: Option<N2>,
    pub dmg: Option<DMG>,
    pub ben: Option<BEN>,
    pub r#ref: Vec<REF>,
    pub loop_2620: Vec<_834Loop2620>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2620 {
    pub nx1: Option<NX1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dtp: Vec<DTP>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2630 {
    pub fc: Option<FC>,
    pub dtp: Vec<DTP>,
    pub loop_2640: Vec<_834Loop2640>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2640 {
    pub inv: Option<INV>,
    pub dtp: Vec<DTP>,
    pub qty: Vec<QTY>,
    pub ent: Vec<ENT>,
    pub r#ref: Vec<REF>,
    pub amt: Vec<AMT>,
    pub k3: Vec<K3>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2650 {
    pub ain: Option<AIN>,
    pub qty: Vec<QTY>,
    pub dtp: Vec<DTP>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2700 {
    pub lx: Option<LX>,
    pub loop_2750: Vec<_834Loop2750>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _834Loop2750 {
    pub n1: N1,
    pub r#ref: REF,
    pub dtp: Option<DTP>,
}

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

/// 837 - Health Care Claim
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837 {
    pub st: ST,
    pub bht: BHT,
    pub r#ref: Vec<REF>,
    pub loop_1000: Vec<_837Loop1000>,
    pub loop_2000: Vec<_837Loop2000>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _837, nom::error::Error<&'a str>> for _837 {
    fn parse(input: &'a str) -> IResult<&'a str, _837> {
        let mut output = _837::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = BHT::parse(rest)?;
        output.bht = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.r#ref = obj;
        // loop 1000
        let mut loop_1000 = vec![];
        let mut loop_rest = rest;
        while peek(opt(NM1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, nm1) = NM1::parse(loop_rest)?;
            let (rest, n2) = many0(N2::parse).parse(rest)?;
            let (rest, n3) = many0(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            let (rest, per) = many0(PER::parse).parse(rest)?;
            loop_rest = rest;
            loop_1000.push(_837Loop1000 {
                nm1,
                n2,
                n3,
                n4,
                r#ref,
                per,
            });
        }
        let rest = loop_rest;
        output.loop_1000 = loop_1000;
        // loop 2000
        let mut loop_2000 = vec![];
        let mut loop_rest = rest;
        while peek(opt(HL::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, hl) = HL::parse(loop_rest)?;
            let (rest, prv) = opt(PRV::parse).parse(rest)?;
            let (rest, sbr) = opt(SBR::parse).parse(rest)?;
            let (rest, pat) = opt(PAT::parse).parse(rest)?;
            let (rest, dtp) = opt(DTP::parse).parse(rest)?;
            let (rest, cur) = opt(CUR::parse).parse(rest)?;
            loop_rest = rest;
            // loop 2010
            let mut loop_2010 = vec![];
            while peek(opt(NM1::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, nm1) = NM1::parse(loop_rest)?;
                let (rest, n2) = opt(N2::parse).parse(rest)?;
                let (rest, n3) = opt(N3::parse).parse(rest)?;
                let (rest, n4) = opt(N4::parse).parse(rest)?;
                let (rest, dmg) = opt(DMG::parse).parse(rest)?;
                let (rest, r#ref) = opt(REF::parse).parse(rest)?;
                let (rest, per) = opt(PER::parse).parse(rest)?;
                loop_rest = rest;
                loop_2010.push(_837Loop2010 {
                    nm1,
                    n2,
                    n3,
                    n4,
                    dmg,
                    r#ref,
                    per,
                });
            }
            // loop 2300
            let mut loop_2300 = vec![];
            while peek(opt(CLM::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, clm) = CLM::parse(loop_rest)?;
                let (rest, dtp) = opt(DTP::parse).parse(rest)?;
                let (rest, cl1) = opt(CL1::parse).parse(rest)?;
                let (rest, dn1) = opt(DN1::parse).parse(rest)?;
                let (rest, dn2) = opt(DN2::parse).parse(rest)?;
                let (rest, pwk) = opt(PWK::parse).parse(rest)?;
                let (rest, cn1) = opt(CN1::parse).parse(rest)?;
                let (rest, dsb) = opt(DSB::parse).parse(rest)?;
                let (rest, ur) = opt(UR::parse).parse(rest)?;
                let (rest, amt) = opt(AMT::parse).parse(rest)?;
                let (rest, r#ref) = opt(REF::parse).parse(rest)?;
                let (rest, k3) = opt(K3::parse).parse(rest)?;
                let (rest, nte) = opt(NTE::parse).parse(rest)?;
                let (rest, cr1) = opt(CR1::parse).parse(rest)?;
                let (rest, cr2) = opt(CR2::parse).parse(rest)?;
                let (rest, cr3) = opt(CR3::parse).parse(rest)?;
                let (rest, cr4) = opt(CR4::parse).parse(rest)?;
                let (rest, cr5) = opt(CR5::parse).parse(rest)?;
                let (rest, cr6) = opt(CR6::parse).parse(rest)?;
                let (rest, cr8) = opt(CR8::parse).parse(rest)?;
                let (rest, crc) = opt(CRC::parse).parse(rest)?;
                let (rest, hi) = opt(HI::parse).parse(rest)?;
                let (rest, qty) = opt(QTY::parse).parse(rest)?;
                let (rest, hcp) = opt(HCP::parse).parse(rest)?;
                loop_rest = rest;
                // loop 2310
                let mut loop_2310 = vec![];
                while peek(opt(NM1::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, nm1) = NM1::parse(loop_rest)?;
                    let (rest, prv) = opt(PRV::parse).parse(rest)?;
                    let (rest, n2) = opt(N2::parse).parse(rest)?;
                    let (rest, n3) = opt(N3::parse).parse(rest)?;
                    let (rest, n4) = opt(N4::parse).parse(rest)?;
                    let (rest, r#ref) = opt(REF::parse).parse(rest)?;
                    let (rest, per) = opt(PER::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_2310.push(_837Loop2310 {
                        nm1,
                        prv,
                        n2,
                        n3,
                        n4,
                        r#ref,
                        per,
                    });
                }
                // loop 2320
                let mut loop_2320 = vec![];
                while peek(opt(SBR::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, sbr) = SBR::parse(loop_rest)?;
                    let (rest, cas) = opt(CAS::parse).parse(rest)?;
                    let (rest, amt) = many0(AMT::parse).parse(rest)?;
                    let (rest, dmg) = opt(DMG::parse).parse(rest)?;
                    let (rest, oi) = opt(OI::parse).parse(rest)?;
                    let (rest, mia) = opt(MIA::parse).parse(rest)?;
                    let (rest, moa) = opt(MOA::parse).parse(rest)?;
                    loop_rest = rest;
                    // loop 2330
                    let mut loop_2330 = vec![];
                    while peek(opt(NM1::parse)).parse(loop_rest)?.1.is_some() {
                        let (rest, nm1) = NM1::parse(loop_rest)?;
                        let (rest, n2) = opt(N2::parse).parse(rest)?;
                        let (rest, n3) = opt(N3::parse).parse(rest)?;
                        let (rest, n4) = opt(N4::parse).parse(rest)?;
                        let (rest, per) = opt(PER::parse).parse(rest)?;
                        let (rest, dtp) = opt(DTP::parse).parse(rest)?;
                        let (rest, r#ref) = opt(REF::parse).parse(rest)?;
                        loop_rest = rest;
                        loop_2330.push(_837Loop2330 {
                            nm1,
                            n2,
                            n3,
                            n4,
                            per,
                            dtp,
                            r#ref,
                        });
                    }
                    loop_2320.push(_837Loop2320 {
                        sbr,
                        cas,
                        amt,
                        dmg,
                        oi,
                        mia,
                        moa,
                        loop_2330,
                    });
                }
                // loop 2400
                let mut loop_2400 = vec![];
                while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, lx) = LX::parse(loop_rest)?;
                    let (rest, sv1) = opt(SV1::parse).parse(rest)?;
                    let (rest, sv2) = opt(SV2::parse).parse(rest)?;
                    let (rest, sv3) = opt(SV3::parse).parse(rest)?;
                    let (rest, too) = opt(TOO::parse).parse(rest)?;
                    let (rest, sv4) = opt(SV4::parse).parse(rest)?;
                    let (rest, sv5) = opt(SV5::parse).parse(rest)?;
                    let (rest, sv6) = opt(SV6::parse).parse(rest)?;
                    let (rest, sv7) = opt(SV7::parse).parse(rest)?;
                    let (rest, hi) = opt(HI::parse).parse(rest)?;
                    let (rest, pwk) = opt(PWK::parse).parse(rest)?;
                    let (rest, cr1) = opt(CR1::parse).parse(rest)?;
                    let (rest, cr2) = opt(CR2::parse).parse(rest)?;
                    let (rest, cr3) = opt(CR3::parse).parse(rest)?;
                    let (rest, cr4) = opt(CR4::parse).parse(rest)?;
                    let (rest, cr5) = opt(CR5::parse).parse(rest)?;
                    let (rest, crc) = opt(CRC::parse).parse(rest)?;
                    let (rest, dtp) = opt(DTP::parse).parse(rest)?;
                    let (rest, qty) = opt(QTY::parse).parse(rest)?;
                    let (rest, mea) = opt(MEA::parse).parse(rest)?;
                    let (rest, cn1) = opt(CN1::parse).parse(rest)?;
                    let (rest, r#ref) = opt(REF::parse).parse(rest)?;
                    let (rest, amt) = many0(AMT::parse).parse(rest)?;
                    let (rest, k3) = many0(K3::parse).parse(rest)?;
                    let (rest, nte) = opt(NTE::parse).parse(rest)?;
                    let (rest, ps1) = opt(PS1::parse).parse(rest)?;
                    let (rest, imm) = opt(IMM::parse).parse(rest)?;
                    let (rest, hsd) = opt(HSD::parse).parse(rest)?;
                    let (rest, hcp) = opt(HCP::parse).parse(rest)?;
                    loop_rest = rest;
                    // loop 2420
                    let mut loop_2420 = vec![];
                    while peek(opt(NM1::parse)).parse(loop_rest)?.1.is_some() {
                        let (rest, nm1) = NM1::parse(loop_rest)?;
                        let (rest, prv) = opt(PRV::parse).parse(rest)?;
                        let (rest, n2) = opt(N2::parse).parse(rest)?;
                        let (rest, n3) = opt(N3::parse).parse(rest)?;
                        let (rest, n4) = opt(N4::parse).parse(rest)?;
                        let (rest, r#ref) = opt(REF::parse).parse(rest)?;
                        let (rest, per) = opt(PER::parse).parse(rest)?;
                        loop_rest = rest;
                        loop_2420.push(_837Loop2420 {
                            nm1,
                            prv,
                            n2,
                            n3,
                            n4,
                            r#ref,
                            per,
                        });
                    }
                    // loop 2430
                    let mut loop_2430 = vec![];
                    while peek(opt(SVD::parse)).parse(loop_rest)?.1.is_some() {
                        let (rest, svd) = SVD::parse(loop_rest)?;
                        let (rest, cas) = many0(CAS::parse).parse(rest)?;
                        let (rest, dtp) = opt(DTP::parse).parse(rest)?;
                        let (rest, amt) = opt(AMT::parse).parse(rest)?;
                        loop_rest = rest;
                        loop_2430.push(_837Loop2430 { svd, cas, dtp, amt });
                    }
                    loop_2400.push(_837Loop2400 {
                        lx,
                        sv1,
                        sv2,
                        sv3,
                        too,
                        sv4,
                        sv5,
                        sv6,
                        sv7,
                        hi,
                        pwk,
                        cr1,
                        cr2,
                        cr3,
                        cr4,
                        cr5,
                        crc,
                        dtp,
                        qty,
                        mea,
                        cn1,
                        r#ref,
                        amt,
                        k3,
                        nte,
                        ps1,
                        imm,
                        hsd,
                        hcp,
                        loop_2410: vec![],
                        loop_2420,
                        loop_2430: vec![],
                        loop_2440: vec![],
                    });
                }
                loop_2300.push(_837Loop2300 {
                    clm,
                    dtp,
                    cl1,
                    dn1,
                    dn2,
                    pwk,
                    cn1,
                    dsb,
                    ur,
                    amt,
                    r#ref,
                    k3,
                    nte,
                    cr1,
                    cr2,
                    cr3,
                    cr4,
                    cr5,
                    cr6,
                    cr8,
                    crc,
                    hi,
                    qty,
                    hcp,
                    loop_2305: vec![],
                    loop_2310,
                    loop_2320,
                    loop_2400,
                });
            }
            loop_2000.push(_837Loop2000 {
                hl,
                prv,
                sbr,
                pat,
                dtp,
                cur,
                loop_2010,
                loop_2300,
            });
        }
        let rest = loop_rest;
        output.loop_2000 = loop_2000;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop1000 {
    pub nm1: NM1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2000 {
    pub hl: HL,
    pub prv: Option<PRV>,
    pub sbr: Option<SBR>,
    pub pat: Option<PAT>,
    pub dtp: Option<DTP>,
    pub cur: Option<CUR>,
    pub loop_2010: Vec<_837Loop2010>,
    pub loop_2300: Vec<_837Loop2300>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2010 {
    pub nm1: NM1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dmg: Option<DMG>,
    pub r#ref: Option<REF>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2300 {
    pub clm: CLM,
    pub dtp: Option<DTP>,
    pub cl1: Option<CL1>,
    pub dn1: Option<DN1>,
    pub dn2: Option<DN2>,
    pub pwk: Option<PWK>,
    pub cn1: Option<CN1>,
    pub dsb: Option<DSB>,
    pub ur: Option<UR>,
    pub amt: Option<AMT>,
    pub r#ref: Option<REF>,
    pub k3: Option<K3>,
    pub nte: Option<NTE>,
    pub cr1: Option<CR1>,
    pub cr2: Option<CR2>,
    pub cr3: Option<CR3>,
    pub cr4: Option<CR4>,
    pub cr5: Option<CR5>,
    pub cr6: Option<CR6>,
    pub cr8: Option<CR8>,
    pub crc: Option<CRC>,
    pub hi: Option<HI>,
    pub qty: Option<QTY>,
    pub hcp: Option<HCP>,
    pub loop_2305: Vec<_837Loop2305>,
    pub loop_2310: Vec<_837Loop2310>,
    pub loop_2320: Vec<_837Loop2320>,
    pub loop_2400: Vec<_837Loop2400>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2305 {
    pub cr7: CR7,
    pub hsd: Vec<HSD>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2310 {
    pub nm1: NM1,
    pub prv: Option<PRV>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub r#ref: Option<REF>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2320 {
    pub sbr: SBR,
    pub cas: Option<CAS>,
    pub amt: Vec<AMT>,
    pub dmg: Option<DMG>,
    pub oi: Option<OI>,
    pub mia: Option<MIA>,
    pub moa: Option<MOA>,
    pub loop_2330: Vec<_837Loop2330>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2330 {
    pub nm1: NM1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Option<PER>,
    pub dtp: Option<DTP>,
    pub r#ref: Option<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2400 {
    pub lx: LX,
    pub sv1: Option<SV1>,
    pub sv2: Option<SV2>,
    pub sv3: Option<SV3>,
    pub too: Option<TOO>,
    pub sv4: Option<SV4>,
    pub sv5: Option<SV5>,
    pub sv6: Option<SV6>,
    pub sv7: Option<SV7>,
    pub hi: Option<HI>,
    pub pwk: Option<PWK>,
    pub cr1: Option<CR1>,
    pub cr2: Option<CR2>,
    pub cr3: Option<CR3>,
    pub cr4: Option<CR4>,
    pub cr5: Option<CR5>,
    pub crc: Option<CRC>,
    pub dtp: Option<DTP>,
    pub qty: Option<QTY>,
    pub mea: Option<MEA>,
    pub cn1: Option<CN1>,
    pub r#ref: Option<REF>,
    pub amt: Vec<AMT>,
    pub k3: Vec<K3>,
    pub nte: Option<NTE>,
    pub ps1: Option<PS1>,
    pub imm: Option<IMM>,
    pub hsd: Option<HSD>,
    pub hcp: Option<HCP>,
    pub loop_2410: Vec<_837Loop2410>,
    pub loop_2420: Vec<_837Loop2420>,
    pub loop_2430: Vec<_837Loop2430>,
    pub loop_2440: Vec<_837Loop2440>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2410 {
    pub lin: LIN,
    pub ctp: Option<CTP>,
    pub r#ref: Option<REF>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2420 {
    pub nm1: NM1,
    pub prv: Option<PRV>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub r#ref: Option<REF>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2430 {
    pub svd: SVD,
    pub cas: Vec<CAS>,
    pub dtp: Option<DTP>,
    pub amt: Option<AMT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2440 {
    pub lq: LQ,
    pub frm: FRM,
}
