use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 823 - Lockbox
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Lockbox Transaction Set (823) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a bank (depository) to report the detail and summary of lockbox deposits, payments and remittance information to a depositor.
///
/// Heading: ST, the N1 party loop, TRN, DTM.
/// Detail LOOP DEP: deposit (DEP/AMT/QTY/REF/DTM) with a BAT batch loop; each BAT carries a
///   BPR payment loop with ADX adjustment, N1 party, RMR remittance, TXP tax-payment, DED
///   deduction and LX (NM1 -> AIN/PEN -> INV) sub-loops. The ADX, RMR and IT1 detail loops
///   carry the recurring REF/SAC/SLN reference structures.
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823 {
    pub st: ST,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_823LoopN1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trn: Option<TRN>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "DEP")]
    pub loop_dep: Vec<_823LoopDep>,
    pub se: SE,
}

/// Reusable party loop (N1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
}

/// Reusable reference loop (REF + DTM).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopRef {
    pub r#ref: REF,
    pub dtm: Vec<DTM>,
}

/// Reusable allowance/charge loop (SAC + TXI).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopSac {
    pub sac: SAC,
    pub txi: Vec<TXI>,
}

/// Reusable subline loop (SLN) with REF and SAC sub-loops.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopSln {
    pub sln: SLN,
    #[x12(loop_trigger = "REF")]
    pub loop_ref: Vec<_823LoopRef>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_823LoopSac>,
}

/// Reusable item loop (IT1) with REF, SAC and SLN sub-loops.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopIt1 {
    pub it1: IT1,
    #[x12(loop_trigger = "REF")]
    pub loop_ref: Vec<_823LoopRef>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_823LoopSac>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_823LoopSln>,
}

/// Reusable adjustment loop (ADX) with REF and IT1 sub-loops.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopAdx {
    pub adx: ADX,
    pub nte: Vec<NTE>,
    pub per: Vec<PER>,
    pub dtm: Option<DTM>,
    #[x12(loop_trigger = "REF")]
    pub loop_ref: Vec<_823LoopRef>,
    #[x12(loop_trigger = "IT1")]
    pub loop_it1: Vec<_823LoopIt1>,
}

/// Deposit loop (DEP).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopDep {
    pub dep: DEP,
    pub amt: AMT,
    pub qty: Vec<QTY>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[x12(loop_trigger = "BAT")]
    pub loop_bat: Vec<_823LoopBat>,
}

/// Batch loop (BAT) nested in the deposit loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopBat {
    pub bat: BAT,
    pub ava: Vec<AVA>,
    pub amt: Option<AMT>,
    pub qty: Option<QTY>,
    pub dtm: Vec<DTM>,
    #[x12(loop_trigger = "BPR")]
    pub loop_bpr: Vec<_823LoopBpr>,
}

/// Payment loop (BPR) nested in the batch loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopBpr {
    pub bpr: BPR,
    pub trn: Option<TRN>,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub ava: Option<AVA>,
    #[x12(loop_trigger = "ADX")]
    pub loop_adx: Vec<_823LoopAdx>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_823LoopN1>,
    #[x12(loop_trigger = "RMR")]
    pub loop_rmr: Vec<_823LoopRmr>,
    #[x12(loop_trigger = "TXP")]
    pub loop_txp: Vec<_823LoopTxp>,
    #[x12(loop_trigger = "DED")]
    pub loop_ded: Vec<_823LoopDed>,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_823LoopLx>,
}

/// Remittance loop (RMR) with IT1 and ADX sub-loops.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopRmr {
    pub rmr: RMR,
    pub n1: Option<N1>,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[x12(loop_trigger = "IT1")]
    pub loop_it1: Vec<_823LoopIt1>,
    #[x12(loop_trigger = "ADX")]
    pub loop_adx: Vec<_823LoopAdx>,
}

/// Tax-payment loop (TXP + TXI).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopTxp {
    pub txp: TXP,
    pub txi: Vec<TXI>,
}

/// Deduction loop (DED).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopDed {
    pub ded: DED,
}

/// Line-number loop (LX) with an NM1 party sub-loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopLx {
    pub lx: LX,
    pub r#ref: Vec<REF>,
    pub trn: Vec<TRN>,
    #[x12(loop_trigger = "NM1")]
    pub loop_nm1: Vec<_823LoopNm1>,
}

/// Party loop (NM1) nested in the LX loop, with AIN and PEN sub-loops.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopNm1 {
    pub nm1: NM1,
    pub r#ref: Vec<REF>,
    pub g53: Vec<G53>,
    #[x12(loop_trigger = "AIN")]
    pub loop_ain: Vec<_823LoopAin>,
    #[x12(loop_trigger = "PEN")]
    pub loop_pen: Vec<_823LoopPen>,
}

/// Account-identification loop (AIN).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopAin {
    pub ain: AIN,
    pub qty: Vec<QTY>,
    pub dtp: Vec<DTP>,
}

/// Penalty loop (PEN) with an INV sub-loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopPen {
    pub pen: PEN,
    pub amt: Vec<AMT>,
    pub dtp: Vec<DTP>,
    #[x12(loop_trigger = "INV")]
    pub loop_inv: Vec<_823LoopInv>,
}

/// Investment-detail loop (INV + DTP).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _823LoopInv {
    pub inv: INV,
    pub dtp: Vec<DTP>,
}
