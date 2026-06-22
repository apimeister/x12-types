use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 999 - Implementation Acknowledgment
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _999 {
    pub st: ST,
    pub ak1: AK1,
    #[x12(loop_trigger = "AK2")]
    pub loop_ak2: Vec<_999LoopAK2>,
    pub ak9: AK9,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _999LoopAK2 {
    pub ak2: AK2,
    #[x12(loop_trigger = "IK3")]
    pub loop_ik3: Vec<_999LoopIK3>,
    pub ik5: IK5,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _999LoopIK3 {
    pub ik3: IK3,
    pub ctx: Vec<CTX>,
    #[x12(loop_trigger = "IK4")]
    pub loop_ik4: Vec<_999LoopIK4>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _999LoopIK4 {
    pub ik4: IK4,
    pub ctx: Vec<CTX>,
}
