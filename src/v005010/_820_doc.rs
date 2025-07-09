use super::segment::*;
use crate::util::Parser;
use nom::{combinator::opt, multi::many0, IResult, Parser as _};
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

/// 820 - Payment Order/Remittance Advice
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820 {
    pub st: ST,
    pub bpr: BPR,
    pub nte: Vec<NTE>,
    pub trn: Option<TRN>,
    pub cur: Option<CUR>,
    pub ref_segments: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub loop_n1: Vec<_820LoopN1>,
    pub loop_ent: Vec<_820LoopENT>,
    pub loop_rmr: Vec<_820LoopRMR>,
    pub loop_txp: Vec<_820LoopTXP>,
    pub loop_ded: Vec<_820LoopDED>,
    pub loop_lx: Vec<_820LoopLX>,
    pub loop_n9: Option<_820LoopN9>,
    pub loop_ryl: Vec<_820LoopRYL>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _820, nom::error::Error<&'a str>> for _820 {
    fn parse(input: &'a str) -> IResult<&'a str, _820> {
        let mut output = _820::default();
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
        output.ref_segments = obj;
        let (rest, obj) = many0(DTM::parse).parse(rest)?;
        output.dtm = obj;
        let (rest, obj) = many0(_820LoopN1::parse).parse(rest)?;
        output.loop_n1 = obj;
        let (rest, obj) = many0(_820LoopENT::parse).parse(rest)?;
        output.loop_ent = obj;
        let (rest, obj) = many0(_820LoopRMR::parse).parse(rest)?;
        output.loop_rmr = obj;
        let (rest, obj) = many0(_820LoopTXP::parse).parse(rest)?;
        output.loop_txp = obj;
        let (rest, obj) = many0(_820LoopDED::parse).parse(rest)?;
        output.loop_ded = obj;
        let (rest, obj) = many0(_820LoopLX::parse).parse(rest)?;
        output.loop_lx = obj;
        let (rest, obj) = opt(_820LoopN9::parse).parse(rest)?;
        output.loop_n9 = obj;
        let (rest, obj) = many0(_820LoopRYL::parse).parse(rest)?;
        output.loop_ryl = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

/// Loop N1 - Party Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub ref_segments: Vec<REF>,
    pub per: Vec<PER>,
    pub rdm: Option<RDM>,
    pub dtm: Option<DTM>,
}

impl<'a> Parser<&'a str, _820LoopN1, nom::error::Error<&'a str>> for _820LoopN1 {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopN1> {
        let mut output = _820LoopN1::default();
        let (rest, obj) = N1::parse(input)?;
        output.n1 = obj;
        let (rest, obj) = many0(N2::parse).parse(rest)?;
        output.n2 = obj;
        let (rest, obj) = many0(N3::parse).parse(rest)?;
        output.n3 = obj;
        let (rest, obj) = opt(N4::parse).parse(rest)?;
        output.n4 = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.ref_segments = obj;
        let (rest, obj) = many0(PER::parse).parse(rest)?;
        output.per = obj;
        let (rest, obj) = opt(RDM::parse).parse(rest)?;
        output.rdm = obj;
        let (rest, obj) = opt(DTM::parse).parse(rest)?;
        output.dtm = obj;
        Ok((rest, output))
    }
}

/// Loop ENT - Entity
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopENT {
    pub ent: ENT,
    pub loop_fa1: Vec<_820LoopFA1>,
    pub loop_nm1: Vec<_820LoopNM1>,
    pub loop_adx: Vec<_820LoopADX>,
    pub loop_rmr: Vec<_820LoopRMR>,
    pub loop_adx_nested: Vec<_820LoopADXNested>,
    pub loop_fa1_nested: Vec<_820LoopFA1>,
}

impl<'a> Parser<&'a str, _820LoopENT, nom::error::Error<&'a str>> for _820LoopENT {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopENT> {
        let mut output = _820LoopENT::default();
        let (rest, obj) = ENT::parse(input)?;
        output.ent = obj;
        let (rest, obj) = many0(_820LoopFA1::parse).parse(rest)?;
        output.loop_fa1 = obj;
        let (rest, obj) = many0(_820LoopNM1::parse).parse(rest)?;
        output.loop_nm1 = obj;
        let (rest, obj) = many0(_820LoopADX::parse).parse(rest)?;
        output.loop_adx = obj;
        let (rest, obj) = many0(_820LoopRMR::parse).parse(rest)?;
        output.loop_rmr = obj;
        let (rest, obj) = many0(_820LoopADXNested::parse).parse(rest)?;
        output.loop_adx_nested = obj;
        let (rest, obj) = many0(_820LoopFA1::parse).parse(rest)?;
        output.loop_fa1_nested = obj;
        Ok((rest, output))
    }
}

/// Loop FA1 - Type of Financial Accounting Data
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopFA1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

impl<'a> Parser<&'a str, _820LoopFA1, nom::error::Error<&'a str>> for _820LoopFA1 {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopFA1> {
        let mut output = _820LoopFA1::default();
        let (rest, obj) = FA1::parse(input)?;
        output.fa1 = obj;
        let (rest, obj) = many0(FA2::parse).parse(rest)?;
        output.fa2 = obj;
        Ok((rest, output))
    }
}

/// Loop NM1 - Individual or Organizational Name
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopNM1 {
    pub nm1: NM1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub ref_segments: Vec<REF>,
    pub per: Vec<PER>,
}

impl<'a> Parser<&'a str, _820LoopNM1, nom::error::Error<&'a str>> for _820LoopNM1 {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopNM1> {
        let mut output = _820LoopNM1::default();
        let (rest, obj) = NM1::parse(input)?;
        output.nm1 = obj;
        let (rest, obj) = many0(N2::parse).parse(rest)?;
        output.n2 = obj;
        let (rest, obj) = many0(N3::parse).parse(rest)?;
        output.n3 = obj;
        let (rest, obj) = opt(N4::parse).parse(rest)?;
        output.n4 = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.ref_segments = obj;
        let (rest, obj) = many0(PER::parse).parse(rest)?;
        output.per = obj;
        Ok((rest, output))
    }
}

/// Loop ADX - Adjustment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopADX {
    pub adx: ADX,
    pub nte: Vec<NTE>,
    pub per: Vec<PER>,
    pub dtm: Option<DTM>,
    pub loop_ref: Vec<_820LoopREF>,
    pub loop_it1: Vec<_820LoopIT1>,
    pub loop_fa1: Vec<_820LoopFA1>,
}

impl<'a> Parser<&'a str, _820LoopADX, nom::error::Error<&'a str>> for _820LoopADX {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopADX> {
        let mut output = _820LoopADX::default();
        let (rest, obj) = ADX::parse(input)?;
        output.adx = obj;
        let (rest, obj) = many0(NTE::parse).parse(rest)?;
        output.nte = obj;
        let (rest, obj) = many0(PER::parse).parse(rest)?;
        output.per = obj;
        let (rest, obj) = opt(DTM::parse).parse(rest)?;
        output.dtm = obj;
        let (rest, obj) = many0(_820LoopREF::parse).parse(rest)?;
        output.loop_ref = obj;
        let (rest, obj) = many0(_820LoopIT1::parse).parse(rest)?;
        output.loop_it1 = obj;
        let (rest, obj) = many0(_820LoopFA1::parse).parse(rest)?;
        output.loop_fa1 = obj;
        Ok((rest, output))
    }
}

/// Loop REF - Reference Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopREF {
    pub ref_segment: REF,
    pub dtm: Vec<DTM>,
}

impl<'a> Parser<&'a str, _820LoopREF, nom::error::Error<&'a str>> for _820LoopREF {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopREF> {
        let mut output = _820LoopREF::default();
        let (rest, obj) = REF::parse(input)?;
        output.ref_segment = obj;
        let (rest, obj) = many0(DTM::parse).parse(rest)?;
        output.dtm = obj;
        Ok((rest, output))
    }
}

/// Loop IT1 - Baseline Item Data (Invoice)
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopIT1 {
    pub it1: IT1,
    pub rpa: Option<RPA>,
    pub qty: Option<QTY>,
    pub loop_ref: Vec<_820LoopREF>,
    pub loop_sac: Vec<_820LoopSAC>,
    pub loop_sln: Vec<_820LoopSLN>,
}

impl<'a> Parser<&'a str, _820LoopIT1, nom::error::Error<&'a str>> for _820LoopIT1 {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopIT1> {
        let mut output = _820LoopIT1::default();
        let (rest, obj) = IT1::parse(input)?;
        output.it1 = obj;
        let (rest, obj) = opt(RPA::parse).parse(rest)?;
        output.rpa = obj;
        let (rest, obj) = opt(QTY::parse).parse(rest)?;
        output.qty = obj;
        let (rest, obj) = many0(_820LoopREF::parse).parse(rest)?;
        output.loop_ref = obj;
        let (rest, obj) = many0(_820LoopSAC::parse).parse(rest)?;
        output.loop_sac = obj;
        let (rest, obj) = many0(_820LoopSLN::parse).parse(rest)?;
        output.loop_sln = obj;
        Ok((rest, output))
    }
}

/// Loop SAC - Service, Promotion, Allowance, or Charge Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopSAC {
    pub sac: SAC,
    pub txi: Vec<TXI>,
    pub dtm: Vec<DTM>,
}

impl<'a> Parser<&'a str, _820LoopSAC, nom::error::Error<&'a str>> for _820LoopSAC {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopSAC> {
        let mut output = _820LoopSAC::default();
        let (rest, obj) = SAC::parse(input)?;
        output.sac = obj;
        let (rest, obj) = many0(TXI::parse).parse(rest)?;
        output.txi = obj;
        let (rest, obj) = many0(DTM::parse).parse(rest)?;
        output.dtm = obj;
        Ok((rest, output))
    }
}

/// Loop SLN - Subline Item Detail
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopSLN {
    pub sln: SLN,
    pub loop_ref: Vec<_820LoopREF>,
    pub loop_sac: Vec<_820LoopSAC>,
}

impl<'a> Parser<&'a str, _820LoopSLN, nom::error::Error<&'a str>> for _820LoopSLN {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopSLN> {
        let mut output = _820LoopSLN::default();
        let (rest, obj) = SLN::parse(input)?;
        output.sln = obj;
        let (rest, obj) = many0(_820LoopREF::parse).parse(rest)?;
        output.loop_ref = obj;
        let (rest, obj) = many0(_820LoopSAC::parse).parse(rest)?;
        output.loop_sac = obj;
        Ok((rest, output))
    }
}

/// Loop RMR - Remittance Advice Accounts Receivable Open Item Reference
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopRMR {
    pub rmr: RMR,
    pub nte: Vec<NTE>,
    pub ref_segments: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub veh: Option<VEH>,
    pub loop_it1: Vec<_820LoopIT1>,
    pub loop_adx: Vec<_820LoopADXNested>,
    pub loop_fa1: Vec<_820LoopFA1>,
}

impl<'a> Parser<&'a str, _820LoopRMR, nom::error::Error<&'a str>> for _820LoopRMR {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopRMR> {
        let mut output = _820LoopRMR::default();
        let (rest, obj) = RMR::parse(input)?;
        output.rmr = obj;
        let (rest, obj) = many0(NTE::parse).parse(rest)?;
        output.nte = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.ref_segments = obj;
        let (rest, obj) = many0(DTM::parse).parse(rest)?;
        output.dtm = obj;
        let (rest, obj) = opt(VEH::parse).parse(rest)?;
        output.veh = obj;
        let (rest, obj) = many0(_820LoopIT1::parse).parse(rest)?;
        output.loop_it1 = obj;
        let (rest, obj) = many0(_820LoopADXNested::parse).parse(rest)?;
        output.loop_adx = obj;
        let (rest, obj) = many0(_820LoopFA1::parse).parse(rest)?;
        output.loop_fa1 = obj;
        Ok((rest, output))
    }
}

/// Loop ADX Nested - Adjustment (nested within RMR)
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopADXNested {
    pub adx: ADX,
    pub nte: Vec<NTE>,
    pub per: Vec<PER>,
    pub loop_ref: Vec<_820LoopREF>,
    pub loop_it1: Vec<_820LoopIT1>,
}

impl<'a> Parser<&'a str, _820LoopADXNested, nom::error::Error<&'a str>> for _820LoopADXNested {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopADXNested> {
        let mut output = _820LoopADXNested::default();
        let (rest, obj) = ADX::parse(input)?;
        output.adx = obj;
        let (rest, obj) = many0(NTE::parse).parse(rest)?;
        output.nte = obj;
        let (rest, obj) = many0(PER::parse).parse(rest)?;
        output.per = obj;
        let (rest, obj) = many0(_820LoopREF::parse).parse(rest)?;
        output.loop_ref = obj;
        let (rest, obj) = many0(_820LoopIT1::parse).parse(rest)?;
        output.loop_it1 = obj;
        Ok((rest, output))
    }
}

/// Loop TXP - Tax Payment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopTXP {
    pub txp: TXP,
    pub txi: Vec<TXI>,
    pub ref_segments: Vec<REF>,
    pub dtm: Vec<DTM>,
}

impl<'a> Parser<&'a str, _820LoopTXP, nom::error::Error<&'a str>> for _820LoopTXP {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopTXP> {
        let mut output = _820LoopTXP::default();
        let (rest, obj) = TXP::parse(input)?;
        output.txp = obj;
        let (rest, obj) = many0(TXI::parse).parse(rest)?;
        output.txi = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.ref_segments = obj;
        let (rest, obj) = many0(DTM::parse).parse(rest)?;
        output.dtm = obj;
        Ok((rest, output))
    }
}

/// Loop DED - Deductions
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopDED {
    pub ded: DED,
}

impl<'a> Parser<&'a str, _820LoopDED, nom::error::Error<&'a str>> for _820LoopDED {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopDED> {
        let mut output = _820LoopDED::default();
        let (rest, obj) = DED::parse(input)?;
        output.ded = obj;
        Ok((rest, output))
    }
}

/// Loop LX - Transaction Set Line Number
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopLX {
    pub lx: LX,
    pub ref_segments: Vec<REF>,
    pub trn: Vec<TRN>,
    pub loop_nm1: Vec<_820LoopNM1>,
}

impl<'a> Parser<&'a str, _820LoopLX, nom::error::Error<&'a str>> for _820LoopLX {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopLX> {
        let mut output = _820LoopLX::default();
        let (rest, obj) = LX::parse(input)?;
        output.lx = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.ref_segments = obj;
        let (rest, obj) = many0(TRN::parse).parse(rest)?;
        output.trn = obj;
        let (rest, obj) = many0(_820LoopNM1::parse).parse(rest)?;
        output.loop_nm1 = obj;
        Ok((rest, output))
    }
}

/// Loop N9 - Extended Reference Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopN9 {
    pub n9: N9,
    pub ref_segments: Vec<REF>,
    pub loop_amt: Vec<_820LoopAMT>,
    pub loop_n1: Vec<_820LoopN1>,
}

impl<'a> Parser<&'a str, _820LoopN9, nom::error::Error<&'a str>> for _820LoopN9 {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopN9> {
        let mut output = _820LoopN9::default();
        let (rest, obj) = N9::parse(input)?;
        output.n9 = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.ref_segments = obj;
        let (rest, obj) = many0(_820LoopAMT::parse).parse(rest)?;
        output.loop_amt = obj;
        let (rest, obj) = many0(_820LoopN1::parse).parse(rest)?;
        output.loop_n1 = obj;
        Ok((rest, output))
    }
}

/// Loop AMT - Monetary Amount Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopAMT {
    pub amt: AMT,
    pub ref_segments: Vec<REF>,
}

impl<'a> Parser<&'a str, _820LoopAMT, nom::error::Error<&'a str>> for _820LoopAMT {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopAMT> {
        let mut output = _820LoopAMT::default();
        let (rest, obj) = AMT::parse(input)?;
        output.amt = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.ref_segments = obj;
        Ok((rest, output))
    }
}

/// Loop RYL - Royalty Payment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopRYL {
    pub ryl: RYL,
    pub loop_nm1: Vec<_820LoopNM1>,
    pub loop_asm: Option<_820LoopASM>,
}

impl<'a> Parser<&'a str, _820LoopRYL, nom::error::Error<&'a str>> for _820LoopRYL {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopRYL> {
        let mut output = _820LoopRYL::default();
        let (rest, obj) = RYL::parse(input)?;
        output.ryl = obj;
        let (rest, obj) = many0(_820LoopNM1::parse).parse(rest)?;
        output.loop_nm1 = obj;
        let (rest, obj) = opt(_820LoopASM::parse).parse(rest)?;
        output.loop_asm = obj;
        Ok((rest, output))
    }
}

/// Loop ASM - Amount and Settlement Method
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820LoopASM {
    pub asm: ASM,
    pub adx: Option<ADX>,
}

impl<'a> Parser<&'a str, _820LoopASM, nom::error::Error<&'a str>> for _820LoopASM {
    fn parse(input: &'a str) -> IResult<&'a str, _820LoopASM> {
        let mut output = _820LoopASM::default();
        let (rest, obj) = ASM::parse(input)?;
        output.asm = obj;
        let (rest, obj) = opt(ADX::parse).parse(rest)?;
        output.adx = obj;
        Ok((rest, output))
    }
}
