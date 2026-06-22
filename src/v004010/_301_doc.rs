use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 301 Confirmation (Ocean)
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _301 {
    pub st: ST,
    pub b1: B1,
    pub y3: Y3,
    #[x12(loop_trigger = "Y4|W09")]
    pub loop_y4: Vec<_301LoopY4>,
    pub n9: Vec<N9>,
    pub r2a: Vec<R2A>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_301LoopN1>,
    #[x12(loop_trigger = "R4")]
    pub loop_r4: Vec<_301LoopR4>,
    pub w09: Option<W09>,
    pub h3: Option<H3>,
    pub ea: Vec<EA>,
    #[x12(loop_trigger = "LX|W09")]
    pub loop_lx: Vec<_301LoopLx>,
    pub v1: Vec<V1>,
    pub v9: Vec<V9>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _301LoopLx {
    pub lx: LX,
    pub n7: Option<N7>,
    pub w09: Option<W09>,
    pub k1: Vec<K1>,
    pub l0: Option<L0>,
    pub l5: Option<L5>,
    pub l4: Option<L4>,
    pub l1: Option<L1>,
    #[x12(loop_trigger = "H1")]
    pub loop_h1: Vec<_301LoopLxLoopH1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _301LoopY4 {
    pub y4: Option<Y4>,
    pub w09: Option<W09>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _301LoopN1 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub g61: Option<G61>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _301LoopLxLoopH1 {
    pub h1: Option<H1>,
    pub h2: Vec<H2>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12, ParseX12)]
pub struct _301LoopR4 {
    pub r4: R4,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
}
