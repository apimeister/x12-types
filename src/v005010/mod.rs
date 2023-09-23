use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult,
};
pub use segment::*;
use serde::{Deserialize, Serialize};

use crate::util::Parser;

mod segment;

#[cfg(test)]
mod test_834;
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
        while peek(opt(ST::parse))(loop_rest)?.1.is_some() {
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct FunctionalGroup<T> {
    pub gs: GS,
    pub segments: Vec<T>,
    pub ge: GE,
}

/// 834 - Benefit Enrollment and Maintenance
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
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
        let (rest, obj) = many0(REF::parse)(rest)?;
        output.r#ref = obj;
        let (rest, obj) = many0(DTP::parse)(rest)?;
        output.dtp = obj;
        let (rest, obj) = many0(AMT::parse)(rest)?;
        output.amt = obj;
        let (rest, obj) = many0(QTY::parse)(rest)?;
        output.qty = obj;
        // loop 1000
        let mut loop_1000 = vec![];
        let mut loop_rest = rest.clone();
        while peek(opt(N1::parse))(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = many0(N2::parse)(rest)?;
            let (rest, n3) = many0(N3::parse)(rest)?;
            let (rest, n4) = opt(N4::parse)(rest)?;
            let (rest, per) = many0(PER::parse)(rest)?;
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
        let mut loop_rest = rest.clone();
        while peek(opt(INS::parse))(loop_rest)?.1.is_some() {
            let (rest, ins) = opt(INS::parse)(loop_rest)?;
            let (rest, r#ref) = many0(REF::parse)(rest)?;
            let (rest, dtp) = many0(DTP::parse)(rest)?;
            loop_rest = rest;
            // loop 2100
            let mut loop_2100 = vec![];
            while peek(opt(NM1::parse))(loop_rest)?.1.is_some() {
                let (rest, nm1) = opt(NM1::parse)(loop_rest)?;
                let (rest, per) = opt(PER::parse)(rest)?;
                let (rest, n3) = opt(N3::parse)(rest)?;
                let (rest, n4) = opt(N4::parse)(rest)?;
                let (rest, dmg) = opt(DMG::parse)(rest)?;
                let (rest, pm) = opt(PM::parse)(rest)?;
                let (rest, ec) = many0(EC::parse)(rest)?;
                let (rest, icm) = opt(ICM::parse)(rest)?;
                let (rest, amt) = many0(AMT::parse)(rest)?;
                let (rest, hlh) = opt(HLH::parse)(rest)?;
                let (rest, hi) = many0(HI::parse)(rest)?;
                let (rest, lui) = many0(LUI::parse)(rest)?;
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
            while peek(opt(HD::parse))(loop_rest)?.1.is_some() {
                let (rest, hd) = opt(HD::parse)(loop_rest)?;
                let (rest, dtp) = many0(DTP::parse)(rest)?;
                let (rest, amt) = many0(AMT::parse)(rest)?;
                let (rest, r#ref) = many0(REF::parse)(rest)?;
                let (rest, idc) = many0(IDC::parse)(rest)?;
                loop_rest = rest;
                // loop 2310
                let mut loop_2310 = vec![];
                while peek(opt(LX::parse))(loop_rest)?.1.is_some()
                    || peek(opt(NM1::parse))(loop_rest)?.1.is_some()
                {
                    let (rest, lx) = opt(LX::parse)(loop_rest)?;
                    let (rest, nm1) = opt(NM1::parse)(rest)?;
                    let (rest, n1) = many0(N1::parse)(rest)?;
                    let (rest, n2) = opt(N2::parse)(rest)?;
                    let (rest, n3) = many0(N3::parse)(rest)?;
                    let (rest, n4) = opt(N4::parse)(rest)?;
                    let (rest, per) = many0(PER::parse)(rest)?;
                    let (rest, prv) = opt(PRV::parse)(rest)?;
                    let (rest, dtp) = many0(DTP::parse)(rest)?;
                    let (rest, pla) = opt(PLA::parse)(rest)?;
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
                while peek(opt(COB::parse))(loop_rest)?.1.is_some()
                    || peek(opt(REF::parse))(loop_rest)?.1.is_some()
                    || peek(opt(DTP::parse))(loop_rest)?.1.is_some()
                {
                    let (rest, cob) = opt(COB::parse)(loop_rest)?;
                    let (rest, r#ref) = opt(REF::parse)(rest)?;
                    let (rest, dtp) = opt(DTP::parse)(rest)?;
                    loop_rest = rest;
                    // loop 2330
                    let mut loop_2330 = vec![];
                    while peek(opt(NM1::parse))(loop_rest)?.1.is_some() {
                        let (rest, nm1) = opt(NM1::parse)(loop_rest)?;
                        let (rest, n2) = opt(N2::parse)(rest)?;
                        let (rest, n3) = many0(N3::parse)(rest)?;
                        let (rest, n4) = opt(N4::parse)(rest)?;
                        let (rest, per) = opt(PER::parse)(rest)?;
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
            while peek(opt(RP::parse))(loop_rest)?.1.is_some() {
                let (rest, rp) = opt(RP::parse)(loop_rest)?;
                let (rest, dtp) = many0(DTP::parse)(rest)?;
                let (rest, r#ref) = many0(REF::parse)(rest)?;
                let (rest, inv) = many0(INV::parse)(rest)?;
                let (rest, amt) = many0(AMT::parse)(rest)?;
                let (rest, qty) = many0(QTY::parse)(rest)?;
                let (rest, k3) = many0(K3::parse)(rest)?;
                let (rest, rel) = opt(REL::parse)(rest)?;
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
            let (rest, ls) = opt(LS::parse)(rest)?;
            loop_rest = rest;
            // loop 2700
            let mut loop_2700 = vec![];
            while peek(opt(LX::parse))(loop_rest)?.1.is_some() {
                let (rest, lx) = opt(LX::parse)(loop_rest)?;
                loop_rest = rest;
                // loop 2750
                let mut loop_2750 = vec![];
                while peek(opt(N1::parse))(loop_rest)?.1.is_some() {
                    let (rest, n1) = N1::parse(loop_rest)?;
                    let (rest, r#ref) = REF::parse(rest)?;
                    let (rest, dtp) = opt(DTP::parse)(rest)?;
                    loop_rest = rest;
                    loop_2750.push(_834Loop2750 { n1, r#ref, dtp });
                }
                loop_2700.push(_834Loop2700 { lx, loop_2750 });
            }
            let rest = loop_rest;
            let (rest, le) = opt(LE::parse)(rest)?;
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop1000 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub loop_1100: Vec<_834Loop1100>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop1100 {
    pub act: Option<ACT>,
    pub r#ref: Vec<REF>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub dtp: Option<DTP>,
    pub amt: Option<AMT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2100 {
    nm1: Option<NM1>,
    per: Option<PER>,
    n3: Option<N3>,
    n4: Option<N4>,
    dmg: Option<DMG>,
    pm: Option<PM>,
    ec: Vec<EC>,
    icm: Option<ICM>,
    amt: Vec<AMT>,
    hlh: Option<HLH>,
    hi: Vec<HI>,
    lui: Vec<LUI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2200 {
    dsb: Option<DSB>,
    dtp: Vec<DTP>,
    ad1: Vec<AD1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2300 {
    hd: Option<HD>,
    dtp: Vec<DTP>,
    amt: Vec<AMT>,
    r#ref: Vec<REF>,
    idc: Vec<IDC>,
    loop_2310: Vec<_834Loop2310>,
    loop_2320: Vec<_834Loop2320>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2310 {
    lx: Option<LX>,
    nm1: Option<NM1>,
    n1: Vec<N1>,
    n2: Option<N2>,
    n3: Vec<N3>,
    n4: Option<N4>,
    per: Vec<PER>,
    prv: Option<PRV>,
    dtp: Vec<DTP>,
    pla: Option<PLA>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2320 {
    cob: Option<COB>,
    r#ref: Option<REF>,
    dtp: Option<DTP>,
    loop_2330: Vec<_834Loop2330>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2330 {
    nm1: Option<NM1>,
    n2: Option<N2>,
    n3: Vec<N3>,
    n4: Option<N4>,
    per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2400 {}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2500 {}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2600 {
    rp: Option<RP>,
    dtp: Vec<DTP>,
    r#ref: Vec<REF>,
    inv: Vec<INV>,
    amt: Vec<AMT>,
    qty: Vec<QTY>,
    k3: Vec<K3>,
    rel: Option<REL>,
    loop_2610: Vec<_834Loop2610>,
    loop_2630: Vec<_834Loop2630>,
    loop_2650: Vec<_834Loop2650>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2610 {}
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2630 {}
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2650 {}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2700 {
    lx: Option<LX>,
    loop_2750: Vec<_834Loop2750>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _834Loop2750 {
    n1: N1,
    r#ref: REF,
    dtp: Option<DTP>,
}
