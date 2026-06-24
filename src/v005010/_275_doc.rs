use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 275 - Patient Information
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Patient Information Transaction Set (275) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to convey patient information and to transmit additional information (including attachments) to support a health care claim or encounter between providers, payers, and other interested parties.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0100 | ST | Transaction Set Header | M | 1
/// 0200 | BGN | Beginning Segment | O | 1
/// 0300 | DTM | Date/Time Reference | O | 3
/// 0400 | TRN | Trace | O | 5
/// LOOP ID - NM1 | >1
/// 0500 | NM1 | Individual or Organizational Name | M | 1
/// 0600 | IN1 | Individual Identification | O | 1
/// 0700 | DMG | Demographic Information | O | 3
/// 0800 | PRV | Provider Information | O | 1
/// 0900 | PER | Administrative Communications Contact | O | 2
/// 1000 | REF | Reference Information | O | 5
/// 1050 | DTP | Date or Time or Period | O | 1
/// NM1 -> LOOP ID - NX1 | 5
/// 1100 | NX1 | Property or Entity Identification | M | 1
/// 1200 | N3 | Party Location | O | 1
/// 1300 | N4 | Geographic Location | O | 1
/// LOOP ID - LX | >1
/// 0100 | LX | Transaction Set Line Number | M | 1
/// 0150 | TRN | Trace | O | 1
/// 0175 | STC | Status Information | O | 1
/// 0200 | NM1 | Individual or Organizational Name | O | 1
/// 0300 | PRV | Provider Information | O | 1
/// 0400 | PER | Administrative Communications Contact | O | 1
/// 0500 | REF | Reference Information | O | 5
/// LX -> LOOP ID - DTP | >1
/// 0600 | DTP | Date or Time or Period | M | 1
/// 0700 | CAT | Category of Patient Information Service | O | 1
/// 0800 | PID | Product/Item Description | O | 1
/// DTP -> LOOP ID - EFI | 1
/// 0900 | EFI | Electronic Format Identification | M | 1
/// 1000 | BIN | Binary Data Segment | M | 1
/// 1100 | SE | Transaction Set Trailer | M | 1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _275 {
    pub st: ST,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bgn: Option<BGN>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trn: Vec<TRN>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_nm1: Vec<_275LoopNm1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_lx: Vec<_275LoopLx>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _275LoopNm1 {
    pub nm1: NM1,
    pub in1: Option<IN1>,
    pub dmg: Vec<DMG>,
    pub prv: Option<PRV>,
    pub per: Vec<PER>,
    pub r#ref: Vec<REF>,
    pub dtp: Option<DTP>,
    pub loop_nx1: Vec<_275LoopNx1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _275LoopNx1 {
    pub nx1: NX1,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _275LoopLx {
    pub lx: LX,
    pub trn: Option<TRN>,
    pub stc: Option<STC>,
    pub nm1: Option<NM1>,
    pub prv: Option<PRV>,
    pub per: Option<PER>,
    pub r#ref: Vec<REF>,
    pub loop_dtp: Vec<_275LoopDtp>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _275LoopDtp {
    pub dtp: DTP,
    pub cat: Option<CAT>,
    pub pid: Option<PID>,
    pub loop_efi: Vec<_275LoopEfi>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _275LoopEfi {
    pub efi: EFI,
    pub bin: BIN,
}
