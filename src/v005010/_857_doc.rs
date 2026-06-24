use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 857 - Shipment and Billing Notice
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Shipment and Billing Notice Transaction Set (857) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set combines the functions of a ship notice (856) and an invoice (810) into a single hierarchical transaction.
///
/// Heading: ST, BHT.
/// Detail LOOP HL: one hierarchical loop carrying, by level, a BS1 shipment loop (G05), a
///   BS2 order loop (TDS), a BS3 tare loop (PAL), a BS4 pack loop (LX) and a BS5 item loop
///   (IT1), each with its own party/allowance/code sub-loops.
/// Summary: SE.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857 {
    pub st: ST,
    pub bht: BHT,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "HL")]
    pub loop_hl: Vec<_857LoopHl>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopHl {
    pub hl: HL,
    #[x12(loop_trigger = "G05")]
    pub loop_bs1: Vec<_857LoopBs1>,
    #[x12(loop_trigger = "TDS")]
    pub loop_bs2: Vec<_857LoopBs2>,
    #[x12(loop_trigger = "PAL")]
    pub loop_bs3: Vec<_857LoopBs3>,
    #[x12(loop_trigger = "LX")]
    pub loop_bs4: Vec<_857LoopBs4>,
    #[x12(loop_trigger = "IT1")]
    pub loop_bs5: Vec<_857LoopBs5>,
}

/// Reusable code-source loop (LM + LQ).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopLm {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

/// Reusable financial-accounting loop (FA1 + FA2).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopFa1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

/// Reusable allowance/charge loop (SAC + TXI).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopSac {
    pub sac: SAC,
    pub txi: Vec<TXI>,
}

/// BS1 - shipment-level loop (G05).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopBs1 {
    pub g05: G05,
    pub td1: Vec<TD1>,
    pub td3: Vec<TD3>,
    pub td4: Vec<TD4>,
    pub td5: Vec<TD5>,
    pub fob: Option<FOB>,
    pub dtm: Vec<DTM>,
    pub n9: Vec<N9>,
    pub per: Vec<PER>,
    pub cur: Option<CUR>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_857LoopBsN1>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_857LoopLm>,
}

/// Party loop (N1) used at the shipment and order levels.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopBsN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
}

/// BS2 - order-level loop (TDS).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopBs2 {
    pub tds: TDS,
    pub prf: Option<PRF>,
    pub n9: Vec<N9>,
    pub dtm: Vec<DTM>,
    pub itd: Vec<ITD>,
    pub txi: Vec<TXI>,
    pub sn1: Option<SN1>,
    pub iss: Option<ISS>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_857LoopSac>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_857LoopBsN1>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_857LoopFa1>,
}

/// BS3 - tare/pallet-level loop (PAL).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopBs3 {
    pub pal: PAL,
    pub sn1: Option<SN1>,
    pub man: Vec<MAN>,
}

/// BS4 - pack/subpack-level loop (LX).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopBs4 {
    pub lx: LX,
    pub n9: Vec<N9>,
    pub sn1: Option<SN1>,
    pub po4: Option<PO4>,
    pub mea: Vec<MEA>,
    pub pkg: Vec<PKG>,
    pub man: Vec<MAN>,
}

/// BS5 - item-level loop (IT1).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopBs5 {
    pub it1: IT1,
    pub it3: Option<IT3>,
    pub po4: Option<PO4>,
    pub td4: Vec<TD4>,
    pub tc2: Vec<TC2>,
    pub txi: Vec<TXI>,
    pub ctp: Vec<CTP>,
    pub n9: Vec<N9>,
    pub mea: Vec<MEA>,
    pub dtm: Vec<DTM>,
    pub itd: Vec<ITD>,
    #[x12(loop_trigger = "PID")]
    pub loop_pid: Vec<_857LoopPid>,
    #[x12(loop_trigger = "SLN")]
    pub loop_sln: Vec<_857LoopSln>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_857LoopSac>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_857LoopLm>,
    #[x12(loop_trigger = "FA1")]
    pub loop_fa1: Vec<_857LoopFa1>,
}

/// Description loop (PID + MEA) nested in the item loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopPid {
    pub pid: PID,
    pub mea: Vec<MEA>,
}

/// Subline loop (SLN + PID) nested in the item loop.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _857LoopSln {
    pub sln: SLN,
    pub pid: Vec<PID>,
}
