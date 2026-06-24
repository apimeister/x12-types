use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 811 - Consolidated Service Invoice/Statement
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Consolidated Service Invoice/Statement Transaction Set (811) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide for the bill or statement of services rendered, e.g., usage-sensitive billing for utilities, telecommunications and other services.
///
/// Heading: ST, BIG, NTE, CUR, REF, PER, ITD, DTM, TXI, then the N1 and FA1 loops.
/// Detail LOOP HL: the hierarchical loop carrying an LX service loop (with a QTY loop), an
///   NM1 party loop, an ITA allowance loop, an IT1 item loop (with AMT, QTY, ITA and NM1
///   sub-loops), an SLN subline loop (with QTY and NM1 sub-loops), a TCD loop, a USD
///   usage-sensitive loop, an III loop (with an LQ sub-loop) and an FA1 loop.
/// Summary: TDS, an ITA loop, a BAL loop, an N1 loop (with BAL, ITA and LX sub-loops, the
///   LX loop carrying AMT and ITA sub-loops), CTT, SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811 {
    pub st: ST,
    pub big: BIG,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nte: Vec<NTE>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur: Option<CUR>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub per: Vec<PER>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub itd: Vec<ITD>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub txi: Vec<TXI>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_811LoopN1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_811LoopFa1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "HL")]
    pub loop_hl: Vec<_811LoopHl>,
    pub tds: TDS,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "ITA")]
    pub loop_ita: Vec<_811LoopItaSum>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "BAL")]
    pub loop_bal: Vec<_811LoopBal>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_n1_sum: Vec<_811LoopN1Sum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

/// Heading party loop (N1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dmg: Option<DMG>,
}

/// Reusable financial-accounting loop (FA1 + FA2).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

/// Reusable quantity loop (QTY + SI), used in the LX, IT1, SLN, TCD and USD loops.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopQty {
    pub qty: QTY,
    pub si: Option<SI>,
}

/// Reusable allowance loop (ITA + DTM + TXI), used at the HL and IT1 levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopIta {
    pub ita: ITA,
    pub dtm: Option<DTM>,
    pub txi: Vec<TXI>,
}

/// Detail hierarchical loop (HL).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopHl {
    pub hl: HL,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_811LoopLx>,
    #[x12(loop_trigger = "NM1")]
    pub loop_nm1: Vec<_811LoopHlNm1>,
    #[x12(loop_trigger = "ITA")]
    pub loop_ita: Vec<_811LoopIta>,
    #[x12(loop_trigger = "IT1")]
    pub loop_it1: Vec<_811LoopIt1>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_811LoopSln>,
    #[x12(loop_trigger = "TCD")]
    pub loop_tcd: Vec<_811LoopTcd>,
    #[x12(loop_trigger = "USD")]
    pub loop_usd: Vec<_811LoopUsd>,
    #[x12(loop_trigger = "III")]
    pub loop_iii: Vec<_811LoopIii>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_811LoopFa1>,
}

/// Service loop (LX) nested in the HL loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopLx {
    pub lx: LX,
    pub veh: Option<VEH>,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub r#ref: Vec<REF>,
    pub amt: Vec<AMT>,
    pub dtm: Vec<DTM>,
    pub txi: Vec<TXI>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_811LoopQty>,
}

/// Party loop (NM1) nested directly in the HL loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopHlNm1 {
    pub nm1: NM1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub txi: Vec<TXI>,
    pub dmg: Option<DMG>,
}

/// Item loop (IT1) nested in the HL loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopIt1 {
    pub it1: IT1,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub inc: Option<INC>,
    pub txi: Vec<TXI>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub msg: Vec<MSG>,
    pub cad: Option<CAD>,
    pub ynq: Vec<YNQ>,
    pub lq: Vec<LQ>,
    pub lcd: Vec<LCD>,
    #[x12(loop_trigger = "AMT")]
    pub loop_amt: Vec<_811LoopAmt>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_811LoopQty>,
    #[x12(loop_trigger = "ITA")]
    pub loop_ita: Vec<_811LoopIta>,
    #[x12(loop_trigger = "NM1")]
    pub loop_nm1: Vec<_811LoopIt1Nm1>,
}

/// Amount loop (AMT + CUR) nested in the IT1 loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopAmt {
    pub amt: AMT,
    pub cur: Option<CUR>,
}

/// Party loop (NM1) nested in the IT1 loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopIt1Nm1 {
    pub nm1: NM1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
    pub nx2: Vec<NX2>,
    pub dmg: Option<DMG>,
    pub r#ref: Vec<REF>,
    pub lcd: Vec<LCD>,
}

/// Subline loop (SLN) nested in the HL loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopSln {
    pub sln: SLN,
    pub si: Vec<SI>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub cur: Option<CUR>,
    pub inc: Option<INC>,
    pub ita: Vec<ITA>,
    pub txi: Vec<TXI>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dtm: Vec<DTM>,
    pub amt: Vec<AMT>,
    pub msg: Vec<MSG>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_811LoopQty>,
    #[x12(loop_trigger = "NM1")]
    pub loop_nm1: Vec<_811LoopSlnNm1>,
}

/// Party loop (NM1) nested in the SLN loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopSlnNm1 {
    pub nm1: NM1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dmg: Option<DMG>,
}

/// Transportation/charge loop (TCD) nested in the HL loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopTcd {
    pub tcd: TCD,
    pub si: Vec<SI>,
    pub txi: Vec<TXI>,
    pub ita: Vec<ITA>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_811LoopQty>,
}

/// Usage-sensitive loop (USD) nested in the HL loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopUsd {
    pub usd: USD,
    pub si: Vec<SI>,
    pub ita: Vec<ITA>,
    pub trf: Vec<TRF>,
    #[x12(loop_trigger = "QTY")]
    pub loop_qty: Vec<_811LoopQty>,
}

/// Information loop (III) nested in the HL loop, with an LQ sub-loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopIii {
    pub iii: III,
    pub dtp: Vec<DTP>,
    pub amt: Vec<AMT>,
    pub pct: Vec<PCT>,
    #[x12(loop_trigger = "LQ")]
    pub loop_lq: Vec<_811LoopLqIii>,
}

/// Code loop (LQ) nested in the III loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopLqIii {
    pub lq: LQ,
    pub amt: Vec<AMT>,
    pub pct: Vec<PCT>,
}

/// Summary allowance loop (ITA + DTM + REF).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopItaSum {
    pub ita: ITA,
    pub dtm: Option<DTM>,
    pub r#ref: Vec<REF>,
}

/// Reusable balance loop (BAL + DTM), used at the summary and summary-N1 levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopBal {
    pub bal: BAL,
    pub dtm: Option<DTM>,
}

/// Summary party loop (N1) with balance, allowance and line-number sub-loops.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopN1Sum {
    pub n1: N1,
    #[x12(loop_trigger = "BAL")]
    pub loop_bal: Vec<_811LoopBal>,
    #[x12(loop_trigger = "ITA")]
    pub loop_ita: Vec<_811LoopItaN1Sum>,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_811LoopLxSum>,
}

/// Allowance loop (ITA) nested in the summary N1 loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopItaN1Sum {
    pub ita: ITA,
    pub dtm: Vec<DTM>,
    pub amt: Option<AMT>,
    pub si: Vec<SI>,
    pub r#ref: Vec<REF>,
    pub cur: Option<CUR>,
}

/// Line-number loop (LX) nested in the summary N1 loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopLxSum {
    pub lx: LX,
    pub r#ref: Option<REF>,
    #[x12(loop_trigger = "AMT")]
    pub loop_amt: Vec<_811LoopAmtSum>,
    #[x12(loop_trigger = "ITA")]
    pub loop_ita: Vec<_811LoopItaLxSum>,
}

/// Amount loop (AMT + DTM) nested in the summary LX loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopAmtSum {
    pub amt: AMT,
    pub dtm: Option<DTM>,
}

/// Allowance loop (ITA + DTM) nested in the summary LX loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _811LoopItaLxSum {
    pub ita: ITA,
    pub dtm: Option<DTM>,
}
