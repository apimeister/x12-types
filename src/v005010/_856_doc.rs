use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 856 - Ship Notice/Manifest
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Ship Notice/Manifest Transaction Set (856) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to notify a trading partner that a shipment has been or will be sent. The transaction set enables the sender to describe the contents and configuration of a shipment in various levels of detail and provides an organized flexibility that allows both the sender and receiver to carry out automated business processes.
///
/// The detail is a single hierarchical (HL) loop that repeats for each level of the
/// shipment/order/tare/pack/item hierarchy. Each HL level carries item, packaging,
/// carrier, hazardous-material, load, party, and charge detail via the segments and
/// sub-loops below; the level itself is distinguished by the HL segment's level code.
///
/// Heading: 0100 ST, 0200 BSN, 0400 DTM.
/// Detail LOOP HL (200000): HL, LIN, SN1, SLN, PRF, PO4, PID, MEA, PWK, PKG, TD1, TD5,
///   LOOP TD3 (TD3, AT9), TD4, TSD, REF, PER, LOOP LH1 (LH1, LH2, LH3, LFH, LEP, LH4, LHT,
///   LHR, PER, LHE), LOOP CLD (CLD, REF, DTP), MAN, DTM, FOB, PAL,
///   LOOP N1 (N1, N2, N3, N4, REF, PER, FOB), SDQ, ETD, CUR, LOOP SAC (SAC, CUR), GF, YNQ,
///   LOOP LM (LM, LQ), LOOP V1 (V1, R4, DTM).
/// Summary: CTT, SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856 {
    pub st: ST,
    pub bsn: BSN,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "HL")]
    pub loop_hl: Vec<_856LoopHL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctt: Option<CTT>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856LoopHL {
    pub hl: HL,
    pub lin: Option<LIN>,
    pub sn1: Option<SN1>,
    pub sln: Vec<SLN>,
    pub prf: Option<PRF>,
    pub po4: Option<PO4>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    #[x12(loop_trigger = "TD3")]
    pub loop_td3: Vec<_856LoopTD3>,
    pub td4: Vec<TD4>,
    pub tsd: Option<TSD>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    #[x12(loop_trigger = "LH1")]
    pub loop_lh1: Vec<_856LoopLH1>,
    #[x12(loop_trigger = "CLD")]
    pub loop_cld: Vec<_856LoopCLD>,
    pub man: Vec<MAN>,
    pub dtm: Vec<DTM>,
    pub fob: Option<FOB>,
    pub pal: Option<PAL>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_856LoopN1>,
    pub sdq: Vec<SDQ>,
    pub etd: Option<ETD>,
    pub cur: Option<CUR>,
    #[x12(loop_trigger = "SAC")]
    pub loop_sac: Vec<_856LoopSAC>,
    pub gf: Option<GF>,
    pub ynq: Vec<YNQ>,
    #[x12(loop_trigger = "LM")]
    pub loop_lm: Vec<_856LoopLM>,
    #[x12(loop_trigger = "V1")]
    pub loop_v1: Vec<_856LoopV1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856LoopTD3 {
    pub td3: TD3,
    pub at9: Option<AT9>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856LoopLH1 {
    pub lh1: LH1,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Vec<LEP>,
    pub lh4: Vec<LH4>,
    pub lht: Vec<LHT>,
    pub lhr: Vec<LHR>,
    pub per: Vec<PER>,
    pub lhe: Option<LHE>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856LoopCLD {
    pub cld: CLD,
    pub r#ref: Vec<REF>,
    pub dtp: Option<DTP>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub fob: Option<FOB>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856LoopSAC {
    pub sac: SAC,
    pub cur: Option<CUR>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856LoopLM {
    pub lm: LM,
    pub lq: Vec<LQ>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856LoopV1 {
    pub v1: V1,
    pub r4: Vec<R4>,
    pub dtm: Vec<DTM>,
}
