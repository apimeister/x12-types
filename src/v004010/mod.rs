//! v004010 repesents all entities von the 004010 specification.

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
mod segment;
pub use segment::*;

#[cfg(test)]
mod test;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Transmission {
    pub isa: ISA,
    pub functional_group: Vec<FunctionalGroup>,
    pub iea: IEA,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct FunctionalGroup {
    pub gs: GS,
    pub segments: Vec<Segments>,
    pub ge: GE,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Segments {
    _204(_204),
    _315(_315),
    _322(_322),
    _404(_404),
    _997(_997),
    _998(_998),
}

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
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204 {
    pub st: ST,
    pub b2: B2,
    pub b2a: B2A,
    pub l11: Option<L11>,
    pub g62: Option<G62>,
    pub ms3: Option<MS3>,
    pub at5: Option<AT5>,
    pub pld: Option<PLD>,
    pub lh6: Option<LH6>,
    pub nte: Option<NTE>,
    pub loop_100: Vec<_204Loop100>,
    pub loop_200: Vec<_204Loop200>,
    pub loop_300: Vec<_204Loop300>,
    pub l3: Option<L3>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop100 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Option<L11>,
    pub g61: Vec<G61>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop200 {
    pub n7: Option<N7>,
    pub n7a: Option<N7A>,
    pub n7b: Option<N7B>,
    pub mea: Option<MEA>,
    pub m7: Option<M7>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop300 {
    pub s5: S5,
    pub l11: Vec<L11>,
    pub g62: Vec<G62>,
    pub at8: Option<AT8>,
    pub lad: Vec<LAD>,
    pub at5: Vec<AT5>,
    pub pld: Option<PLD>,
    pub nte: Vec<NTE>,
    pub loop_310: Vec<_204Loop310>,
    pub loop_320: Vec<_204Loop320>,
    pub loop_350: Vec<_204Loop350>,
    pub loop_380: Vec<_204Loop380>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop310 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g61: Vec<G61>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop320 {
    pub l5: Option<L5>,
    pub at8: Option<AT8>,
    pub loop_325: Vec<_204Loop325>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop325 {
    pub g61: Option<G61>,
    pub l11: Vec<L11>,
    pub lh6: Option<LH6>,
    pub loop_330: Vec<_204Loop330>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop330 {
    pub lh1: Option<LH1>,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Option<LFH>,
    pub lep: Vec<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Vec<LHT>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop350 {
    pub oid: Option<OID>,
    pub g62: Vec<G62>,
    pub lad: Vec<LAD>,
    pub loop_360: Vec<_204Loop360>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop360 {
    pub l5: Option<L5>,
    pub at8: Option<AT8>,
    pub loop_365: Vec<_204Loop365>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop365 {
    pub g61: Option<G61>,
    pub l11: Vec<L11>,
    pub lh6: Vec<LH6>,
    pub loop_370: Vec<_204Loop370>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop370 {
    pub lh1: Option<LH1>,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Vec<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Vec<LHT>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _204Loop380 {
    pub n7: Option<N7>,
    pub n7a: Option<N7A>,
    pub n7b: Option<N7B>,
    pub mea: Option<MEA>,
    pub m7: Option<M7>,
}

/// 315 - Status Details (Ocean)
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Status Details (Ocean) Transaction Set (315) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide all the information necessary to report status or event details for selected shipments or containers. It is intended to accommodate the details for one status or event associated with many shipments or containers, as well as more than one status or event for one shipment or container.
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1
/// 0020 | B4 | Beginning Segment for Inquiry or Reply | M | 1
/// 0030 | N9 | Reference Identification | O | 30
/// 0040 | Q2 | Status Details (Ocean) | O | 1
/// 0050 | SG | Shipment Status | O | 15
/// LOOP ID - R4 | 20
/// R4 -> 0060 | R4 | Port or Terminal | M | 1
/// R4 -> 0070 | DTM | Date/Time Reference | O | 15
/// 0080 | V9 | Event Detail | O | 10
/// 0090 | SE | Transaction Set Trailer | M | 1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _315 {
    pub st: ST,
    pub b4: B4,
    pub n9: Vec<N9>,
    pub q2: Option<Q2>,
    pub sg: Vec<SG>,
    pub loop_r4: Vec<_315LoopR4>,
    pub v9: Option<V9>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _315LoopR4 {
    pub r4: R4,
    pub dtm: Option<DTM>,
}

/// 322 - Terminal Operations and Intermodal Ramp Activity
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Terminal Operations and Intermodal Ramp Activity Transaction Set (322) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide all the information necessary for a terminal operation, port authority or intermodal ramp to communicate terminal and intermodal ramp activities (e.g., "ingates" and "outgates") to authorized parties to a shipment.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1 |   |  
/// 0015 | ZC1 | Beginning Segment For Data Correction Or Change | O | 1 |   |  
/// 0016 | Q5 | Status Details | M | 1 |   |  
/// LOOP ID - N7 | 1000
/// N7 -> 0020 | N7 | Equipment Details | M | 1 |   |  
/// N7 -> 0030 | V4 | Cargo Location Reference | O | 1 |   |  
/// N7 -> 0040 | DTM | Date/Time Reference | O | 2 |   |  
/// N7 -> 0050 | M7 | Seal Numbers | O | 5 |   |  
/// N7 -> 0060 | W09 | Equipment and Temperature | O | 1 |   |  
/// N7 -> 0070 | W2 | Equipment Identification | O | 1 |   |  
/// N7 -> 0080 | NA | Cross-Reference Equipment | O | 30 |   |  
/// N7 -> 0085 | GR5 | Loading Details | O | 10 |   |  
/// N7 -> 0100 | Y7 | Priority | O | 1 |   |  
/// N7 -> 0110 | V1 | Vessel Identification | O | 1 |   |  
/// N7 -> LOOP ID - R4 | 20 |  
/// N7 -> R4 -> 0120 | R4 | Port or Terminal | M | 1 |   |  
/// N7 -> R4 -> 0130 | DTM | Date/Time Reference | O | 15 |   |  
/// N7 -> 0140 | H3 | Special Handling Instructions | O | 6 |   |  
/// N7 -> LOOP ID - N1 | 10 |  
/// N7 -> N1 -> 0150 | N1 | Name | O | 1 |   |  
/// N7 -> N1 -> 0153 | N3 | Address Information | O | 2 |   |  
/// N7 -> N1 -> 0156 | N4 | Geographic Location | O | 1 |   |  
/// N7 -> 0160 | K1 | Remarks | O | 2 |   |  
/// N7 -> 0170 | N9 | Reference Identification | O | 10 |   |  
/// N7 -> LOOP ID - L0 | 999 |  
/// N7 -> L0 -> 0180 | L0 | Line Item - Quantity and Weight | O | 1 |   |  
/// N7 -> L0 -> 0190 | L5 | Description, Marks and Numbers | O | 1 |   |  
/// N7 -> L0 -> 0200 | H1 | Hazardous Material | O | 3 |   |  
/// N7 -> 0210 | L3 | Total Weight and Charges | O | 2 |   |  
/// 0220 | SE | Transaction Set Trailer | M | 1 |   |
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _322 {
    pub st: ST,
    pub zc1: Option<ZC1>,
    pub q5: Q5,
    pub loop_n7: Vec<_322LoopN7>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _322LoopN7 {
    pub n7: N7,
    pub v4: Option<V4>,
    pub dtm: Option<DTM>,
    pub m7: Option<M7>,
    pub w09: Option<W09>,
    pub w2: Option<W2>,
    pub na: Option<NA>,
    pub gr5: Option<GR5>,
    pub y7: Option<Y7>,
    pub v1: Option<V1>,
    pub loop_r4: Vec<_322LoopR4>,
    pub h3: Vec<H3>,
    pub loop_n1: Vec<_322LoopN1>,
    pub k1: Vec<K1>,
    pub n9: Vec<N9>,
    pub loop_l0: Vec<_322LoopL0>,
    pub l3: Vec<L3>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _322LoopR4 {
    r4: R4,
    dtm: Vec<DTM>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _322LoopN1 {
    n1: Option<N1>,
    n3: Vec<N3>,
    n4: Option<N4>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _322LoopL0 {
    l0: Option<L0>,
    l5: Option<L5>,
    h1: Vec<H1>,
}

/// 404 - Rail Carrier Shipment Information
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Rail Carrier Shipment Information Transaction Set (404) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to transmit rail-carrier-specific bill of lading information to a railroad. It is the initial tender of a shipment between a consignor and a rail carrier and can be used as notification of equipment release and/or a legal bill of lading.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1 |   |   |  
/// 0020 | ZC1 | Beginning Segment For Data Correction Or Change | O | 1 |   |   |  
/// 0030 | BX | General Shipment Information | O | 1 |   |   |  
/// 0040 | BNX | Rail Shipment Information | O | 1 |   |   |  
/// 0050 | M3 | Release | M | 1 |   |   |  
/// 0060 | N9 | Reference Identification | M | 30 |   |   |  
/// 0070 | CM | Cargo Manifest | O | 2 |   |   |  
/// 0080 | M1 | Insurance | O | 1 |   |   |  
/// 0090 | DTM | Date/Time Reference | O | 5 |   |   |  
/// LOOP ID - N7 | 500
/// N7 -> 0100 | N7 | Equipment Details | M | 1 |   |   |  
/// N7 -> 0101 | EM | Equipment Characteristics | O | 1 |   |   |  
/// N7 -> LOOP ID - VC | 21 |  
/// N7 -> VC -> 0110 | VC | Motor Vehicle Control | O | 1 |   |   |  
/// N7 -> VC -> LOOP ID - N1 | 2 |   |  
/// N7 -> VC -> N1 -> 0112 | N1 | Name | O | 1 |   |   |  
/// N7 -> VC -> N1 -> 0114 | N3 | Address Information | O | 2 |   |   |  
/// N7 -> VC -> N1 -> 0116 | N4 | Geographic Location | O | 1 |   |   |  
/// N7 -> VC -> N1 -> 0118 | H3 | Special Handling Instructions | O | 1 |   |   |  
/// N7 -> 0130 | M7 | Seal Numbers | O | 5 |   |   |  
/// N7 -> 0140 | N5 | Equipment Ordered | O | 1 |   |   |  
/// N7 -> 0150 | IC | Intermodal Chassis Equipment | O | 1 |   |   |  
/// N7 -> 0160 | IM | Intermodal Movement Information | O | 1 |   |   |  
/// N7 -> 0170 | M12 | In-bond Identifying Information | O | 2 |   |   |  
/// N7 -> LOOP ID - E1 | 2 |  
/// N7 -> E1 -> 0171 | E1 | Empty Car Disposition - Pended Destination Consignee | O | 1 |   |   |  
/// N7 -> E1 -> 0172 | E4 | Empty Car Disposition - Pended Destination City | O | 1 |   |   |  
/// N7 -> E1 -> 0173 | E5 | Empty Car Disposition - Pended Destination Route | O | 13 |   |   |  
/// N7 -> E1 -> 0174 | PI | Price Authority Identification | O | 1 |   |   |  
/// N7 -> 0175 | GA | Canadian Grain Information | O | 15 |   |   |  
/// N7 -> LOOP ID - REF | 99 |  
/// N7 -> REF -> 0177 | REF | Reference Identification | O | 1 |   |   |  
/// N7 -> REF -> 0178 | N10 | Quantity and Description | O | 15 |   |   |  
/// N7 -> REF -> LOOP ID - N1 | 5 |   |  
/// N7 -> REF -> N1 -> 0179 | N1 | Name | O | 1 |   |   |  
/// N7 -> REF -> N1 -> 0180 | N3 | Address Information | O | 1 |   |   |  
/// N7 -> REF -> N1 -> 0182 | N4 | Geographic Location | O | 1 |   |   |  
/// 0185 | NA | Cross-Reference Equipment | O | 10 |   |   |  
/// 0190 | F9 | Origin Station | M | 1 |   |   |  
/// 0200 | D9 | Destination Station | M | 1 |   |   |  
/// LOOP ID - N1 | 10
/// N1 -> 0210 | N1 | Name | M | 1 |   |   |  
/// N1 -> 0215 | N2 | Additional Name Information | O | 2 |   |   |  
/// N1 -> 0220 | N3 | Address Information | O | 2 |   |   |  
/// N1 -> 0230 | N4 | Geographic Location | O | 1 |   |   |  
/// N1 -> 0235 | REF | Reference Identification | O | 2 |   |   |  
/// N1 -> 0240 | PER | Administrative Communications Contact | O | 2 |   |   |  
/// N1 -> 0252 | BL | Billing Information | O | 12 |   |   |  
/// LOOP ID - S1 | 12
/// S1 -> 0430 | S1 | Stop-off Name | O | 1 |   |   |  
/// S1 -> 0440 | S2 | Stop-off Address | O | 2 |   |   |  
/// S1 -> 0448 | S9 | Stop-off Station | O | 1 |   |   |  
/// S1 -> 0449 | N1 | Name | O | 1 |   |   |  
/// S1 -> 0450 | N2 | Additional Name Information | O | 1 |   |   |  
/// S1 -> 0451 | N3 | Address Information | O | 1 |   |   |  
/// S1 -> 0452 | N4 | Geographic Location | O | 1 |   |   |  
/// S1 -> 0453 | PER | Administrative Communications Contact | O | 1 |   |   |  
/// 0460 | R2 | Route Information | O | 13 |   |   |  
/// 0480 | R9 | Route Code | O | 1 |   |   |  
/// LOOP ID - E1 | 2
/// E1 -> 0490 | E1 | Empty Car Disposition - Pended Destination Consignee | O | 1 |   |   |  
/// E1 -> 0500 | E4 | Empty Car Disposition - Pended Destination City | O | 1 |   |   |  
/// E1 -> 0510 | E5 | Empty Car Disposition - Pended Destination Route | O | 13 |   |   |  
/// E1 -> 0511 | PI | Price Authority Identification | O | 1 |   |   |  
/// 0520 | H3 | Special Handling Instructions | O | 20 |   |   |  
/// 0530 | PS | Protective Service Instructions | O | 5 |   |   |  
/// LOOP ID - LX | 25
/// LX -> 0540 | LX | Assigned Number | M | 1 |   |   |  
/// LX -> 0550 | L5 | Description, Marks and Numbers | M | 15 |   |   |  
/// LX -> LOOP ID - L0 | 25 |  
/// LX -> L0 -> 0570 | L0 | Line Item - Quantity and Weight | O | 1 |   |   |  
/// LX -> L0 -> 0575 | MEA | Measurements | O | 3 |   |   |  
/// LX -> L0 -> 0580 | L1 | Rate and Charges | O | 10 |   |   |  
/// LX -> L0 -> 0590 | PI | Price Authority Identification | O | 30 |   |   |  
/// LX -> 0600 | X1 | Export License | O | 6 |   |   |  
/// LOOP ID - T1 | 64
/// T1 -> 0610 | T1 | Transit Inbound Origin | O | 1 |   |   |  
/// T1 -> 0620 | T2 | Transit Inbound Lading | O | 30 |   |   |  
/// T1 -> 0630 | T3 | Transit Inbound Route | O | 12 |   |   |  
/// T1 -> 0640 | T6 | Transit Inbound Rates | O | 1 |   |   |  
/// T1 -> 0650 | T8 | Free-form Transit Data | O | 99 |   |   |  
/// 0660 | L3 | Total Weight and Charges | O | 1 |   |   |  
/// 0670 | LS | Loop Header | O | 1 |   |   |  
/// LOOP ID - LH1 | 100
/// LH1 -> 0680 | LH1 | Hazardous Identification Information | O | 1 |   |   |  
/// LH1 -> 0690 | LH2 | Hazardous Classification Information | O | 4 |   |   |  
/// LH1 -> 0700 | LH3 | Hazardous Material Shipping Name | O | 10 |   |   |  
/// LH1 -> 0710 | LFH | Freeform Hazardous Material Information | O | 20 |   |   |  
/// LH1 -> 0720 | LEP | EPA Required Data | O | 3 |   |   |  
/// LH1 -> 0730 | LH4 | Canadian Dangerous Requirements | O | 1 |   |   |  
/// LH1 -> 0740 | LHT | Transborder Hazardous Requirements | O | 3 |   |   |  
/// LH1 -> 0750 | LHR | Hazardous Material Identifying Reference Numbers | O | 5 |   |   |  
/// LH1 -> 0755 | PER | Administrative Communications Contact | O | 5 |   |   |  
/// 0760 | LE | Loop Trailer | O | 1 |   |   |  
/// 0770 | PER | Administrative Communications Contact | O | 5 |   |   |  
/// 0780 | LH2 | Hazardous Classification Information | O | 6 |   |   |  
/// 0790 | LHR | Hazardous Material Identifying Reference Numbers | O | 1 |   |   |  
/// 0800 | LH6 | Hazardous Certification | O | 5 |   |   |  
/// 0810 | XH | Pro Forma - B13 Information | O | 1 |   |   |  
/// 0820 | X7 | Customs Information | O | 10 |   |   |  
/// 0840 | SE | Transaction Set Trailer | M | 1
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404 {
    pub _010: ST,
    pub _020: Option<ZC1>,
    pub _030: Option<BX>,
    pub _040: Option<BNX>,
    pub _050: M3,
    pub _060: N9,
    pub _070: Option<CM>,
    pub _080: Option<M1>,
    pub _090: Option<DTM>,
    pub loop_n7: Vec<_404LoopN7>,
    pub na: Option<NA>,
    pub f9: F9,
    pub d9: D9,
    pub loop_n1: Vec<_404LoopN1>,
    pub loop_s1: Vec<_404LoopS1>,
    pub r2: Option<R2>,
    pub r9: Option<R9>,
    pub loop_e1: Vec<_404LoopE1>,
    pub h3: Option<H3>,
    pub ps: Option<PS>,
    pub loop_lx: Vec<_404LoopLX>,
    pub loop_t1: Vec<_404LoopT1>,
    pub l3: Option<L3>,
    pub ls: Option<LS>,
    pub loop_lh1: Vec<_404LoopLH1>,
    pub le: Option<LE>,
    pub per: Option<PER>,
    pub lh2: Option<LH2>,
    pub lhr: Option<LHR>,
    pub lh6: Option<LH6>,
    pub xh: Option<XH>,
    pub x7: Option<X7>,
    pub se: SE,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopN7{
    pub n7: N7,
    pub em: Option<EM>,
    pub loop_vc: Vec<_404LoopVC>,
    pub m7: Option<M7>,
    pub n5: Option<N5>,
    pub ic: Option<IC>,
    pub im: Option<IM>,
    pub m12: Option<M12>,
    pub loop_e1: Vec<_404LoopN7E1>,
    pub ga: Option<GA>,
    pub loop_ref: Vec<_404LoopN7Ref>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopN7Ref{
    pub _ref: Option<REF>,
    pub n10: Option<N10>,
    pub loop_n1: Vec<_404LoopN7RefN1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopN7RefN1{
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopVC{
    pub vc: Option<VC>,
    pub loop_n1: Vec<_404LoopVcN1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopVcN1{
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub h3: Option<H3>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopN1{
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub _ref: Option<REF>,
    pub per: Option<PER>,
    pub bl: Option<BL>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopS1 {
    pub s1: Option<S1>,
    pub s2: Option<S2>,
    pub s9: Option<S9>,
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per: Option<PER>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopN7E1{
    pub e1: E1,
    pub e4: Option<E4>,
    pub e5: Option<E5>,
    pub pi: Option<PI>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopE1 {
    pub e1: E1,
    pub e4: Option<E4>,
    pub e5: Option<E5>,
    pub pi: Option<PI>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopL0 {
    pub l0: Option<L0>,
    pub mea: Option<MEA>,
    pub l1: Option<L1>,
    pub pi: Option<PI>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopT1{
    pub t1: Option<T1>,
    pub t2: Option<T2>,
    pub t3: Option<T3>,
    pub t6: Option<T6>,
    pub t8: Option<T8>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopLH1{
    pub lh1: Option<LH1>,
    pub lh2: Option<LH2>,
    pub lh3: Option<LH3>,
    pub lfh: Option<LFH>,
    pub lep: Option<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Option<LHT>,
    pub lhr: Option<LHR>,
    pub per: Option<PER>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopLX{
    pub lx: LX,
    pub l5: L5,
    pub loop_l0: Vec<_404LoopL0>,
    pub x1: Option<X1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopRef{
    pub _ref: Option<REF>,
    pub n10: Option<N10>,
    pub loop_n1: Vec<_404LoopRefN1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopRefN1{
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}

/// 997 - Functional Acknowledgment
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Functional Acknowledgment Transaction Set (997) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to define the control structures for a set of acknowledgments to indicate the results of the syntactical analysis of the electronically encoded documents. The encoded documents are the transaction sets, which are grouped in functional groups, used in defining transactions for business data interchange. This standard does not cover the semantic meaning of the information encoded in the transaction sets.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1 |   |  
/// 0020 | AK1 | Functional Group Response Header | M | 1 |   |  
/// LOOP ID - AK2 | 999999
/// AK2 -> 0030 | AK2 | Transaction Set Response Header | O | 1 |   |  
/// AK2 -> LOOP ID - AK3 | 999999 |  
/// AK2 -> AK3 -> 0040 | AK3 | Data Segment Note | O | 1 |   |  
/// AK2 -> AK3 -> 0050 | AK4 | Data Element Note | O | 99 |   |  
/// AK2 -> 0060 | AK5 | Transaction Set Response Trailer | M | 1 |   |  
/// 0070 | AK9 | Functional Group Response Trailer | M | 1 |   |  
/// 0080 | SE | Transaction Set Trailer | M | 1 |  
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _997 {
    pub st: ST,
    pub ak1: AK1,
    pub loop_ak2: Vec<_997LoopAk2>,
    pub ak9: AK9,
    pub se: SE,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _997LoopAk2{
    pub ak2: AK2,
    pub loop_ak3: Vec<_997LoopAk3>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _997LoopAk3{
    pub ak3: Option<AK3>,
    pub ak4: Vec<AK4>,
}

/// 998 - Set Cancellation
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Set Cancellation Transaction Set (998) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to request the deletion of a previously transmitted transaction set and will indicate the reason for this action, such as diversion or cancelled bill.
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0100 | ST | Transaction Set Header | M | 1
/// 0200 | ZD | Transaction Set Deletion - ID, Reason, and Source | M | 1
/// 0300 | SE | Transaction Set Trailer | M | 1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _998 {
    pub st: ST,
    pub zd: ZD,
    pub se: SE,
}
