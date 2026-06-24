use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 204 - Motor Carrier Load Tender
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Motor Carrier Load Tender Transaction Set (204) for use within the context of an Electronic Data Interchange (EDI) environment. This transaction set can be used to allow shippers or other interested parties to offer (tender) a shipment to a full load (truckload) motor carrier including detailed scheduling, equipment requirements, commodities, and shipping instructions pertinent to a load tender. It is not to be used to provide a motor carrier with data relative to a Less-than-Truckload bill of lading, pick-up notification, or manifest.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1
/// 0020 | B2 | Beginning Segment for Shipment Information Transaction | M | 1
/// 0030 | B2A | Set Purpose | M | 1
/// 0080 | L11 | Business Instructions and Reference Number | O | 50
/// 0090 | G62 | Date/Time | O | 1
/// 0100 | MS3 | Interline Information | O | 1
/// 0110 | AT5 | Bill of Lading Handling Requirements | O | 6
/// 0120 | PLD | Pallet Information | O | 1
/// 0125 | LH6 | Hazardous Certification | O | 6
/// 0130 | NTE | Note/Special Instruction | O | 10
/// LOOP ID - 0100 | 5
/// 0100 -> 0140 | N1 | Name | O | 1
/// 0100 -> 0150 | N2 | Additional Name Information | O | 1
/// 0100 -> 0160 | N3 | Address Information | O | 2
/// 0100 -> 0170 | N4 | Geographic Location | O | 1
/// 0100 -> 0180 | L11 | Business Instructions and Reference Number | O | 1
/// 0100 -> 0190 | G61 | Contact | O | 3
/// LOOP ID - 0200 | 10
/// 0200 -> 0200 | N7 | Equipment Details | O | 1
/// 0200 -> 0203 | N7A | Accessorial Equipment Details | O | 1
/// 0200 -> 0205 | N7B | Additional Equipment Details | O | 1
/// 0200 -> 0208 | MEA | Measurements | O | 1
/// 0200 -> 0210 | M7 | Seal Numbers | O | 2
/// LOOP ID - 0300 | 999
/// 0300 -> 0010 | S5 | Stop Off Details | M | 1
/// 0300 -> 0020 | L11 | Business Instructions and Reference Number | O | 50
/// 0300 -> 0030 | G62 | Date/Time | O | 2
/// 0300 -> 0040 | AT8 | Shipment Weight, Packaging and Quantity Data | O | 1
/// 0300 -> 0050 | LAD | Lading Detail | O | 999
/// 0300 -> 0060 | AT5 | Bill of Lading Handling Requirements | O | 6
/// 0300 -> 0063 | PLD | Pallet Information | O | 1
/// 0300 -> 0065 | NTE | Note/Special Instruction | O | 20
/// 0300 -> LOOP ID - 0310 | 1 |  
/// 0300 -> 0310 -> 0070 | N1 | Name | O | 1
/// 0300 -> 0310 -> 0080 | N2 | Additional Name Information | O | 1
/// 0300 -> 0310 -> 0090 | N3 | Address Information | O | 2
/// 0300 -> 0310 -> 0100 | N4 | Geographic Location | O | 1
/// 0300 -> 0310 -> 0120 | G61 | Contact | O | 3
/// 0300 -> LOOP ID - 0320 | 99 |  
/// 0300 -> 0320 -> 0130 | L5 | Description, Marks and Numbers | O | 1
/// 0300 -> 0320 -> 0135 | AT8 | Shipment Weight, Packaging and Quantity Data | O | 1
/// 0300 -> 0320 -> LOOP ID - 0325 | 99 |   |  
/// 0300 -> 0320 -> 0325 -> 0140 | G61 | Contact | O | 1
/// 0300 -> 0320 -> 0325 -> 0141 | L11 | Business Instructions and Reference Number | O | 5
/// 0300 -> 0320 -> 0325 -> 0142 | LH6 | Hazardous Certification | O | 6
/// 0300 -> 0320 -> 0325 -> LOOP ID - 0330 | 25 |   |   |  
/// 0300 -> 0320 -> 0325 -> 0330 -> 0143 | LH1 | Hazardous Identification Information | O | 1
/// 0300 -> 0320 -> 0325 -> 0330 -> 0144 | LH2 | Hazardous Classification Information | O | 4
/// 0300 -> 0320 -> 0325 -> 0330 -> 0145 | LH3 | Hazardous Material Shipping Name | O | 10
/// 0300 -> 0320 -> 0325 -> 0330 -> 0146 | LFH | Freeform Hazardous Material Information | O | 20
/// 0300 -> 0320 -> 0325 -> 0330 -> 0147 | LEP | EPA Required Data | O | 3
/// 0300 -> 0320 -> 0325 -> 0330 -> 0148 | LH4 | Canadian Dangerous Requirements | O | 1
/// 0300 -> 0320 -> 0325 -> 0330 -> 0149 | LHT | Transborder Hazardous Requirements | O | 3
/// 0300 -> LOOP ID - 0350 | 999 |  
/// 0300 -> 0350 -> 0150 | OID | Order Identification Detail | O | 1
/// 0300 -> 0350 -> 0160 | G62 | Date/Time | O | 2
/// 0300 -> 0350 -> 0180 | LAD | Lading Detail | O | 999
/// 0300 -> 0350 -> LOOP ID - 0360 | 99 |   |  
/// 0300 -> 0350 -> 0360 -> 0190 | L5 | Description, Marks and Numbers | O | 1
/// 0300 -> 0350 -> 0360 -> 0195 | AT8 | Shipment Weight, Packaging and Quantity Data | O | 1
/// 0300 -> 0350 -> 0360 -> LOOP ID - 0365 | 99 |   |   |  
/// 0300 -> 0350 -> 0360 -> 0365 -> 0200 | G61 | Contact | O | 1
/// 0300 -> 0350 -> 0360 -> 0365 -> 0201 | L11 | Business Instructions and Reference Number | O | 5
/// 0300 -> 0350 -> 0360 -> 0365 -> 0202 | LH6 | Hazardous Certification | O | 6
/// 0300 -> 0350 -> 0360 -> 0365 -> LOOP ID - 0370 | 25 |   |   |   |  
/// 0300 -> 0350 -> 0360 -> 0365 -> 0370 -> 0203 | LH1 | Hazardous Identification Information | O | 1
/// 0300 -> 0350 -> 0360 -> 0365 -> 0370 -> 0204 | LH2 | Hazardous Classification Information | O | 4
/// 0300 -> 0350 -> 0360 -> 0365 -> 0370 -> 0205 | LH3 | Hazardous Material Shipping Name | O | 10
/// 0300 -> 0350 -> 0360 -> 0365 -> 0370 -> 0206 | LFH | Freeform Hazardous Material Information | O | 20
/// 0300 -> 0350 -> 0360 -> 0365 -> 0370 -> 0207 | LEP | EPA Required Data | O | 3
/// 0300 -> 0350 -> 0360 -> 0365 -> 0370 -> 0208 | LH4 | Canadian Dangerous Requirements | O | 1
/// 0300 -> 0350 -> 0360 -> 0365 -> 0370 -> 0209 | LHT | Transborder Hazardous Requirements | O | 3
/// 0300 -> LOOP ID - 0380 | 10 |  
/// 0300 -> 0380 -> 0210 | N7 | Equipment Details | O | 1
/// 0300 -> 0380 -> 0220 | N7A | Accessorial Equipment Details | O | 1
/// 0300 -> 0380 -> 0230 | N7B | Additional Equipment Details | O | 1
/// 0300 -> 0380 -> 0240 | MEA | Measurements | O | 1
/// 0300 -> 0380 -> 0250 | M7 | Seal Numbers | O | 2
/// 9010 | L3 | Total Weight and Charges | O | 1
/// 9020 | SE | Transaction Set Trailer | M | 1
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204 {
    pub st: ST,
    pub b2: B2,
    pub b2a: B2A,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub l11: Vec<L11>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g62: Option<G62>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ms3: Option<MS3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at5: Option<AT5>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pld: Option<PLD>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lh6: Vec<LH6>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nte: Option<NTE>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N1")]
    pub loop_100: Vec<_204Loop100>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "N7")]
    pub loop_200: Vec<_204Loop200>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[x12(loop_trigger = "S5")]
    pub loop_300: Vec<_204Loop300>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l3: Option<L3>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop100 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Option<L11>,
    pub g61: Vec<G61>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop200 {
    pub n7: Option<N7>,
    pub n7a: Option<N7A>,
    pub n7b: Option<N7B>,
    pub mea: Option<MEA>,
    pub m7: Option<M7>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop300 {
    pub s5: S5,
    pub l11: Vec<L11>,
    pub g62: Vec<G62>,
    pub at8: Option<AT8>,
    pub lad: Vec<LAD>,
    pub at5: Vec<AT5>,
    pub pld: Option<PLD>,
    pub nte: Vec<NTE>,
    #[x12(loop_trigger = "N1")]
    pub loop_310: Vec<_204Loop310>,
    #[x12(loop_trigger = "L5|LH1")]
    pub loop_320: Vec<_204Loop320>,
    #[x12(loop_trigger = "OID")]
    pub loop_350: Vec<_204Loop350>,
    #[x12(loop_trigger = "N7")]
    pub loop_380: Vec<_204Loop380>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop310 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g61: Vec<G61>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop320 {
    pub l5: Option<L5>,
    pub at8: Option<AT8>,
    #[x12(loop_trigger = "G61|LH1")]
    pub loop_325: Vec<_204Loop325>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop325 {
    pub g61: Option<G61>,
    pub l11: Vec<L11>,
    pub lh6: Option<LH6>,
    #[x12(loop_trigger = "LH1")]
    pub loop_330: Vec<_204Loop330>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop330 {
    pub lh1: Option<LH1>,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Vec<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Vec<LHT>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop350 {
    pub oid: Option<OID>,
    pub g62: Vec<G62>,
    pub lad: Vec<LAD>,
    #[x12(loop_trigger = "L5|LH1")]
    pub loop_360: Vec<_204Loop360>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop360 {
    pub l5: Option<L5>,
    pub at8: Option<AT8>,
    #[x12(loop_trigger = "G61|LH1")]
    pub loop_365: Vec<_204Loop365>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop365 {
    pub g61: Option<G61>,
    pub l11: Vec<L11>,
    pub lh6: Vec<LH6>,
    #[x12(loop_trigger = "LH1")]
    pub loop_370: Vec<_204Loop370>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop370 {
    pub lh1: Option<LH1>,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Vec<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Vec<LHT>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12, ParseX12)]
pub struct _204Loop380 {
    pub n7: Option<N7>,
    pub n7a: Option<N7A>,
    pub n7b: Option<N7B>,
    pub mea: Option<MEA>,
    pub m7: Option<M7>,
}
