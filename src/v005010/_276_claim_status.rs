pub use super::segment::*;
use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult, Parser as _,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use x12_types_macros::DisplayX12;

/// 276 - Health Claim Status Request
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276 {
    pub st: ST,
    pub bht: BHT,
    /// Payer Information
    pub loop_2100a: _276Loop2100A,
    /// Receiver Information
    pub loop_2100b: _276Loop2100B,
    /// Service Provider Detail
    pub loop_2100c: _276Loop2100C,
    /// Subscriber Information
    pub loop_2000d: _276Loop2000D,
    /// Payer Claim Control Number - Loop 2200D
    pub loop_2200d: _276Loop2200D,

    pub se: SE,
}

/// Payer Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2100A {
    pub hl: HL,
    pub nm1: NM1,
}

/// Receiver Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2100B {
    pub hl: HL,
    pub nm1: NM1,
}

/// Service Provider Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2100C {
    pub hl: HL,
    pub nm1: NM1,
}

/// Subscriber Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2000D {
    pub hl: HL,
    /// Date of Birth
    pub nm1: NM1,
    pub dmg: Option<DMG>,
    /// Subscriber Name - loop 2100D
    pub trn: TRN,
}

/// Claim Search Information
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _276Loop2200D {
    /// Payer Claim Control number
    pub r#ref: Vec<REF>,
    /// Total claim charge amount
    pub amt: Option<AMT>,
    /// Claim Service Date
    pub dtp: DTP,
    /// Service line Information - Loop 2210D
    pub svc: Vec<SVC>,
}
