use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 820 - Payment Order/Remittance Advice
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
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

/// Loop N1 - Party Identification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
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

/// Loop ENT - Entity
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopENT {
    pub ent: ENT,
    pub loop_fa1: Vec<_820LoopFA1>,
    pub loop_nm1: Vec<_820LoopNM1>,
    pub loop_adx: Vec<_820LoopADX>,
    pub loop_rmr: Vec<_820LoopRMR>,
    pub loop_adx_nested: Vec<_820LoopADXNested>,
    pub loop_fa1_nested: Vec<_820LoopFA1>,
}

/// Loop FA1 - Type of Financial Accounting Data
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopFA1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

/// Loop NM1 - Individual or Organizational Name
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopNM1 {
    pub nm1: NM1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub ref_segments: Vec<REF>,
    pub per: Vec<PER>,
}

/// Loop ADX - Adjustment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopADX {
    pub adx: ADX,
    pub nte: Vec<NTE>,
    pub per: Vec<PER>,
    pub dtm: Option<DTM>,
    pub loop_ref: Vec<_820LoopREF>,
    pub loop_it1: Vec<_820LoopIT1>,
    pub loop_fa1: Vec<_820LoopFA1>,
}

/// Loop REF - Reference Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopREF {
    pub ref_segment: REF,
    pub dtm: Vec<DTM>,
}

/// Loop IT1 - Baseline Item Data (Invoice)
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopIT1 {
    pub it1: IT1,
    pub rpa: Option<RPA>,
    pub qty: Option<QTY>,
    pub loop_ref: Vec<_820LoopREF>,
    pub loop_sac: Vec<_820LoopSAC>,
    pub loop_sln: Vec<_820LoopSLN>,
}

/// Loop SAC - Service, Promotion, Allowance, or Charge Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopSAC {
    pub sac: SAC,
    pub txi: Vec<TXI>,
    pub dtm: Vec<DTM>,
}

/// Loop SLN - Subline Item Detail
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopSLN {
    pub sln: SLN,
    pub loop_ref: Vec<_820LoopREF>,
    pub loop_sac: Vec<_820LoopSAC>,
}

/// Loop RMR - Remittance Advice Accounts Receivable Open Item Reference
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
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

/// Loop ADX Nested - Adjustment (nested within RMR)
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopADXNested {
    pub adx: ADX,
    pub nte: Vec<NTE>,
    pub per: Vec<PER>,
    pub loop_ref: Vec<_820LoopREF>,
    pub loop_it1: Vec<_820LoopIT1>,
}

/// Loop TXP - Tax Payment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopTXP {
    pub txp: TXP,
    pub txi: Vec<TXI>,
    pub ref_segments: Vec<REF>,
    pub dtm: Vec<DTM>,
}

/// Loop DED - Deductions
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopDED {
    pub ded: DED,
}

/// Loop LX - Transaction Set Line Number
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopLX {
    pub lx: LX,
    pub ref_segments: Vec<REF>,
    pub trn: Vec<TRN>,
    pub loop_nm1: Vec<_820LoopNM1>,
}

/// Loop N9 - Extended Reference Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopN9 {
    pub n9: N9,
    pub ref_segments: Vec<REF>,
    pub loop_amt: Vec<_820LoopAMT>,
    pub loop_n1: Vec<_820LoopN1>,
}

/// Loop AMT - Monetary Amount Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopAMT {
    pub amt: AMT,
    pub ref_segments: Vec<REF>,
}

/// Loop RYL - Royalty Payment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopRYL {
    pub ryl: RYL,
    pub loop_nm1: Vec<_820LoopNM1>,
    pub loop_asm: Option<_820LoopASM>,
}

/// Loop ASM - Amount and Settlement Method
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _820LoopASM {
    pub asm: ASM,
    pub adx: Option<ADX>,
}
