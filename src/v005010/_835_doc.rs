use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 835 - Health Care Claim Payment/Advice
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _835 {
    pub st: ST,
    pub bpr: BPR,
    pub nte: Vec<NTE>,
    pub trn: Option<TRN>,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub dtm: Vec<DTM>,
    #[x12(loop_trigger = "N1")]
    pub loop_1000: Vec<_835Loop1000>,
    #[x12(loop_trigger = "LX")]
    pub loop_2000: Vec<_835Loop2000>,
    pub plb: Vec<PLB>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _835Loop2000 {
    pub lx: LX,
    pub ts3: Option<TS3>,
    pub ts2: Option<TS2>,
    #[x12(loop_trigger = "CLP")]
    pub loop_2100: Vec<_835Loop2100>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
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
    #[x12(loop_trigger = "SVC")]
    pub loop_2110: Vec<_835Loop2110>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _835Loop2110 {
    pub svc: SVC,
    pub dtm: Vec<DTM>,
    pub cas: Vec<CAS>,
    pub r#ref: Vec<REF>,
    pub amt: Vec<AMT>,
    pub qty: Vec<QTY>,
    pub lq: Vec<LQ>,
}
