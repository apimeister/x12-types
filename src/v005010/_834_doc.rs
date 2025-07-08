use super::segment::*;
use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

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
