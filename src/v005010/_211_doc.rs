use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 211 - Motor Carrier Bill of Lading
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Motor Carrier Bill of Lading Transaction Set (211) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by a shipper to provide a carrier with the data required to create a bill of lading. It can include the line item, packaging, rates, charges, marks and numbers, and hazardous material detail associated with the shipment.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0100 | ST | Transaction Set Header | M | 1
/// 0200 | BOL | Beginning Segment for the Motor Carrier Bill of Lading | M | 1
/// 0300 | B2A | Set Purpose | M | 1
/// 0400 | MS3 | Interline Information | O | 12
/// 0500 | MS2 | Equipment or Container Owner and Type | O | 1
/// 0600 | L11 | Business Instructions and Reference Number | O | 100
/// 0700 | G62 | Date/Time | O | 6
/// 0800 | AT5 | Bill of Lading Handling Requirements | O | 50
/// 0900 | K1 | Remarks | O | 10
/// LOOP ID - 0100 | 10
/// 0100 -> 1000 | N1 | Party Identification | O | 1
/// 0100 -> 1100 | N2 | Additional Name Information | O | 1
/// 0100 -> 1200 | N3 | Party Location | O | 2
/// 0100 -> 1300 | N4 | Geographic Location | O | 1
/// 0100 -> 1400 | G61 | Contact | O | 3
/// LOOP ID - 0200 | 9999
/// 0200 -> 0100 | AT1 | Bill of Lading Line Item Number | M | 1
/// 0200 -> 0200 | L11 | Business Instructions and Reference Number | O | 100
/// 0200 -> 0300 | AT3 | Bill of Lading Rates and Charges | O | 1
/// 0200 -> 0400 | AT4 | Bill of Lading Description | O | 99
/// 0200 -> LOOP ID - 0210 | 1
/// 0200 -> 0210 -> 0500 | AT2 | Bill of Lading Line Item Detail | M | 1
/// 0200 -> 0210 -> 0600 | MAN | Marks and Numbers Information | O | 999999
/// 0200 -> 0210 -> 0700 | OID | Order Information Detail | O | 999999
/// 0200 -> 0210 -> 0705 | L4 | Measurement | O | 1
/// 0200 -> LOOP ID - 0220 | 999999
/// 0200 -> 0220 -> 0800 | LX | Assigned Number | M | 1
/// 0200 -> 0220 -> 0900 | MAN | Marks and Numbers Information | O | 999999
/// 0200 -> 0220 -> 1000 | OID | Order Information Detail | O | 999999
/// 0200 -> LOOP ID - 0230 | 2
/// 0200 -> 0230 -> 1100 | G61 | Contact | M | 1
/// 0200 -> 0230 -> 1200 | L11 | Business Instructions and Reference Number | O | 5
/// 0200 -> 0230 -> 1300 | LH6 | Hazardous Certification | O | 6
/// 0200 -> 0230 -> LOOP ID - 0231 | 25
/// 0200 -> 0230 -> 0231 -> 1400 | LH1 | Hazardous Identification Information | M | 1
/// 0200 -> 0230 -> 0231 -> 1500 | LH2 | Hazardous Classification Information | O | 4
/// 0200 -> 0230 -> 0231 -> 1600 | LH3 | Hazardous Material Shipping Name | O | 10
/// 0200 -> 0230 -> 0231 -> 1700 | LFH | Freeform Hazardous Material Information | O | 20
/// 0200 -> 0230 -> 0231 -> 1800 | LEP | EPA Required Data | O | 3
/// 0200 -> 0230 -> 0231 -> 1900 | LH4 | Canadian Dangerous Requirements | O | 1
/// 0200 -> 0230 -> 0231 -> 2000 | LHT | Transborder Hazardous Requirements | O | 3
/// 0200 -> 0230 -> 0231 -> 2100 | L11 | Business Instructions and Reference Number | O | 5
/// 2200 | SE | Transaction Set Trailer | M | 1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _211 {
    pub st: ST,
    pub bol: BOL,
    pub b2a: B2A,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ms3: Vec<MS3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms2: Option<MS2>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub l11: Vec<L11>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub g62: Vec<G62>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub at5: Vec<AT5>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub k1: Vec<K1>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_100: Vec<_211Loop100>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "AT1")]
    pub loop_200: Vec<_211Loop200>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _211Loop100 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g61: Vec<G61>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _211Loop200 {
    pub at1: AT1,
    pub l11: Vec<L11>,
    pub at3: Option<AT3>,
    pub at4: Vec<AT4>,
    #[x12(loop_trigger = "AT2")]
    pub loop_210: Vec<_211Loop210>,
    #[x12(loop_trigger = "LX")]
    pub loop_220: Vec<_211Loop220>,
    #[x12(loop_trigger = "G61")]
    pub loop_230: Vec<_211Loop230>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _211Loop210 {
    pub at2: AT2,
    pub man: Vec<MAN>,
    pub oid: Vec<OID>,
    pub l4: Option<L4>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _211Loop220 {
    pub lx: LX,
    pub man: Vec<MAN>,
    pub oid: Vec<OID>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _211Loop230 {
    pub g61: G61,
    pub l11: Vec<L11>,
    pub lh6: Vec<LH6>,
    #[x12(loop_trigger = "LH1")]
    pub loop_231: Vec<_211Loop231>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _211Loop231 {
    pub lh1: LH1,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Vec<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Vec<LHT>,
    pub l11: Vec<L11>,
}
