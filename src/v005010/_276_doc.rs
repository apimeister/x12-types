pub use super::segment::*;
use serde::{Deserialize, Serialize};
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

// Helper functions to manage the SE segment count. The caller is going to
// have a hard time calculating this, so the library should help out.
// It'd be good if all of the doc types had something like this, and better
// if it could be maintained automatically.
impl _276 {
    // Need a better way to do this. This is a hokey implementation for now.
    // It'd be better if we could calculate this without doing so much work.
    pub fn get_segment_count(&self) -> usize {
        let str_276 = self.to_string();
        let segments: Vec<&str> = dbg!(str_276
            .split("~")
            .filter(|seg| seg.trim().len() > 0)
            .collect());
        segments.len()
    }

    /// Set the SE01 segment count. Normally pass None for count
    /// to allow counting automatically and setting to calculated count.
    /// If the count is failing for some reason, you can pass one in.
    pub fn set_se_segment_count(&mut self, count: Option<usize>) {
        let count = if let Some(cnt) = count {
            cnt
        } else {
            self.get_segment_count()
        };

        self.se._01 = count.to_string()
    }
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
