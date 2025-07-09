use super::segment::*;
use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

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
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
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
                let (rest, dtp) = many0(DTP::parse).parse(rest)?;
                let (rest, cl1) = opt(CL1::parse).parse(rest)?;
                let (rest, dn1) = opt(DN1::parse).parse(rest)?;
                let (rest, dn2) = opt(DN2::parse).parse(rest)?;
                let (rest, pwk) = opt(PWK::parse).parse(rest)?;
                let (rest, cn1) = opt(CN1::parse).parse(rest)?;
                let (rest, dsb) = opt(DSB::parse).parse(rest)?;
                let (rest, ur) = opt(UR::parse).parse(rest)?;
                let (rest, amt) = opt(AMT::parse).parse(rest)?;
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
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
                let (rest, hi) = many0(HI::parse).parse(rest)?;
                let (rest, qty) = opt(QTY::parse).parse(rest)?;
                let (rest, hcp) = opt(HCP::parse).parse(rest)?;
                loop_rest = rest;
                // loop 2305
                let mut loop_2305 = vec![];
                while peek(opt(CR7::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, cr7) = CR7::parse(loop_rest)?;
                    let (rest, hsd) = many0(HSD::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_2305.push(_837Loop2305 { cr7, hsd });
                }
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
                    let (rest, dtp) = many0(DTP::parse).parse(rest)?;
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
                    // loop 2410
                    let mut loop_2410 = vec![];
                    while peek(opt(LIN::parse)).parse(loop_rest)?.1.is_some() {
                        let (rest, lin) = LIN::parse(loop_rest)?;
                        let (rest, ctp) = opt(CTP::parse).parse(rest)?;
                        let (rest, r#ref) = opt(REF::parse).parse(rest)?;
                        loop_rest = rest;
                        loop_2410.push(_837Loop2410 { lin, ctp, r#ref });
                    }
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
                    // loop 2440
                    let mut loop_2440 = vec![];
                    while peek(opt(LQ::parse)).parse(loop_rest)?.1.is_some() {
                        let (rest, lq) = LQ::parse(loop_rest)?;
                        let (rest, frm) = many0(FRM::parse).parse(rest)?;
                        loop_rest = rest;
                        loop_2440.push(_837Loop2440 { lq, frm });
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
                        loop_2410,
                        loop_2420,
                        loop_2430,
                        loop_2440,
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
                    loop_2305,
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
    pub r#ref: Vec<REF>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _837Loop2300 {
    pub clm: CLM,
    pub dtp: Vec<DTP>,
    pub cl1: Option<CL1>,
    pub dn1: Option<DN1>,
    pub dn2: Option<DN2>,
    pub pwk: Option<PWK>,
    pub cn1: Option<CN1>,
    pub dsb: Option<DSB>,
    pub ur: Option<UR>,
    pub amt: Option<AMT>,
    pub r#ref: Vec<REF>,
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
    pub hi: Vec<HI>,
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
    pub dtp: Vec<DTP>,
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
    pub frm: Vec<FRM>,
}
