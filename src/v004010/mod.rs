//! v004010 repesents all entities of the 004010 specification.

use nom::character::complete::newline;
use nom::combinator::opt;
use nom::IResult;
use serde::{Deserialize, Serialize};
use serde_x12::Path;
use serde_x12::PathItem;
use serde_x12::PathOperation;
use serde_x12::Reflect;
use std::fmt::Debug;
mod segment;
use nom::Parser;
pub use segment::*;

#[cfg(test)]
mod test_204;
#[cfg(test)]
mod test_301;
#[cfg(test)]
mod test_310;
#[cfg(test)]
mod test_315;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Transmission<T> {
    pub isa: ISA,
    pub functional_group: Vec<FunctionalGroup<T>>,
    pub iea: IEA,
}

pub fn parse_transmission<T: Default, Parser>(input: &str) -> IResult<&str, Transmission<T>> {
    let mut output = Transmission::default();
    let (input, obj) = parse_isa(input)?;
    output.isa = obj;
    let o = T::default();
    
    let t_obj = opt(o.parse)(input)?;
    // let (input, obj) = parse_functional_group(input)?;
    // output.b2 = obj;
    let (input, obj) = parse_iea(input)?;
    output.iea = obj;
    Ok((input, output))
}

impl<T: Reflect> Reflect for Transmission<T> {
    fn get_path(current_path: &Path, next_segment: &str, last_path: &Path) -> Path {
        match next_segment {
            "ISA" => Path {
                elem: vec![PathItem {
                    name: "isa".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            "IEA" => Path {
                elem: vec![PathItem {
                    name: "iea".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            "GS" => current_path
                .push("functional_group".to_string(), Some(0), false)
                .push("gs".to_string(), None, true)
                .next_op(PathOperation::Push(PathItem {
                    name: "segments".to_string(),
                    vec_position: Some(0),
                    leaf: false,
                })),
            "GE" => current_path.pop().push("ge".to_string(), None, true),
            _ => T::get_path(current_path, next_segment, last_path),
        }
    }

    fn get_type_name() -> String {
        "Transmission".to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct FunctionalGroup<T> {
    pub gs: GS,
    pub segments: Vec<T>,
    pub ge: GE,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l11: Option<L11>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g62: Option<G62>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms3: Option<MS3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at5: Option<AT5>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pld: Option<PLD>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lh6: Option<LH6>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nte: Option<NTE>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_100: Vec<_204Loop100>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_200: Vec<_204Loop200>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_300: Vec<_204Loop300>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l3: Option<L3>,
    pub se: SE,
}

pub fn parse_204(input: &str) -> IResult<&str, _204> {
    let mut output = _204::default();
    let (input, obj) = parse_st(input)?;
    output.st = obj;
    let (input, obj) = parse_b2(input)?;
    output.b2 = obj;
    let (input, obj) = parse_b2a(input)?;
    output.b2a = obj;
    let (input, obj) = opt(parse_l11)(input)?;
    output.l11 = obj;
    let (input, obj) = opt(parse_g62)(input)?;
    output.g62 = obj;
    let (input, obj) = opt(parse_ms3)(input)?;
    output.ms3 = obj;
    let (input, obj) = opt(parse_at5)(input)?;
    output.at5 = obj;
    let (input, obj) = opt(parse_pld)(input)?;
    output.pld = obj;
    let (input, obj) = opt(parse_lh6)(input)?;
    output.lh6 = obj;
    let (input, obj) = opt(parse_nte)(input)?;
    output.nte = obj;
    let (input, obj) = opt(parse_l3)(input)?;
    output.l3 = obj;
    let (input, obj) = parse_se(input)?;
    output.se = obj;
    Ok((input, output))
}

impl Reflect for _204 {
    fn get_type_name() -> String {
        "_204".to_string()
    }

    fn get_path(current_path: &Path, next_segment: &str, last_path: &Path) -> Path {
        match next_segment {
            "ST" => current_path.push("st".to_string(), None, true),
            "B2" => current_path.push("b2".to_string(), None, true),
            "B2A" => current_path.push("b2a".to_string(), None, true),
            "L11" => current_path.push("l11".to_string(), None, true),
            "G62" => current_path.push("g62".to_string(), None, true),
            "MS3" => current_path.push("ms3".to_string(), None, true),
            "AT5" => current_path.push("at5".to_string(), None, true),
            "PLD" => current_path.push("pld".to_string(), None, true),
            "LH6" => current_path.push("lh6".to_string(), None, true),
            "NTE" => current_path.push("nte".to_string(), None, true),
            "L3" => current_path.push("l3".to_string(), None, true),
            "SE" => current_path.push("se".to_string(), None, true),
            _ => last_path.clone(),
        }
    }
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

/// 214 - Transportation Carrier Shipment Status Message
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Transportation Carrier Shipment Status Message Transaction Set (214) for use within the context of an Electronic Data Interchange (EDI) environment. This transaction set can be used by a transportation carrier to provide shippers, consignees, and their agents with the status of shipments in terms of dates, times, locations, route, identifying numbers, and conveyance.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// 0010 | ST | Transaction Set Header | M | 1
/// 0020 | B10 | Beginning Segment for Transportation Carrier Shipment Status Message | M | 1
/// 0030 | L11 | Business Instructions and Reference Number | O | 300
/// 0035 | MAN | Marks and Numbers | O | 9999
/// 0040 | K1 | Remarks | O | 10
/// LOOP ID - 0100 | 10
/// 0100 -> 0050 | N1 | Name | O | 1
/// 0100 -> 0060 | N2 | Additional Name Information | O | 1
/// 0100 -> 0070 | N3 | Address Information | O | 2
/// 0100 -> 0080 | N4 | Geographic Location | O | 1
/// 0100 -> 0090 | G61 | Contact | O | 1
/// 0100 -> 0100 | G62 | Date/Time | O | 1
/// 0100 -> 0110 | L11 | Business Instructions and Reference Number | O | 10
/// 0120 | MS3 | Interline Information | O | 12
/// LOOP ID - 0200 | 999999
/// 0200 -> 0130 | LX | Assigned Number | O | 1
/// 0200 -> LOOP ID - 0205 | 10
/// 0200 -> 0205 -> 0140 | AT7 | Shipment Status Details | O | 1
/// 0200 -> 0205 -> 0143 | MS1 | Equipment, Shipment, or Real Property Location | O | 1
/// 0200 -> 0205 -> 0146 | MS2 | Equipment or Container Owner and Type | O | 1
/// 0200 -> 0150 | L11 | Business Instructions and Reference Number | O | 10
/// 0200 -> 0155 | MAN | Marks and Numbers | O | 9999
/// 0200 -> 0160 | Q7 | Lading Exception Code | O | 10
/// 0200 -> 0170 | K1 | Remarks | O | 10
/// 0200 -> 0180 | AT5 | Bill of Lading Handling Requirements | O | 10
/// 0200 -> 0200 | AT8 | Shipment Weight, Packaging and Quantity Data | O | 10
/// 0200 -> LOOP ID - 0210 | 999999
/// 0200 -> 0210 -> 0210 | CD3 | Carton (Package) Detail | O | 1
/// 0200 -> 0210 -> 0220 | L11 | Business Instructions and Reference Number | O | 20
/// 0200 -> 0210 -> LOOP ID - 0215 | 10
/// 0200 -> 0210 -> 0215 -> 0230 | AT7 | Shipment Status Details | O | 1
/// 0200 -> 0210 -> 0215 -> 0233 | MS1 | Equipment, Shipment, or Real Property Location | O | 1
/// 0200 -> 0210 -> 0215 -> 0236 | MS2 | Equipment or Container Owner and Type | O | 1
/// 0200 -> 0210 -> 0240 | NM1 | Individual or Organizational Name | O | 1
/// 0200 -> 0210 -> 0250 | Q7 | Lading Exception Code | O | 10
/// 0200 -> 0210 -> 0260 | AT8 | Shipment Weight, Packaging and Quantity Data | O | 1
/// 0200 -> 0210 -> 0265 | MAN | Marks and Numbers | O | 9999
/// 0200 -> 0210 -> LOOP ID - 0220 | 999999
/// 0200 -> 0210 -> 0220 -> 0270 | N1 | Name | O | 1
/// 0200 -> 0210 -> 0220 -> 0280 | N2 | Additional Name Information | O | 1
/// 0200 -> 0210 -> 0220 -> 0290 | N3 | Address Information | O | 3
/// 0200 -> 0210 -> 0220 -> 0300 | N4 | Geographic Location | O | 1
/// 0200 -> 0210 -> 0220 -> 0310 | L11 | Business Instructions and Reference Number | O | 10
/// 0200 -> LOOP ID - 0230 | 999999
/// 0200 -> 0230 -> 0320 | PRF | Purchase Order Reference | O | 1
/// 0200 -> 0230 -> LOOP ID - 0231 | 999999
/// 0200 -> 0230 -> 0231 -> 0330 | N1 | Name | O | 1
/// 0200 -> 0230 -> 0231 -> 0340 | N2 | Additional Name Information | O | 1
/// 0200 -> 0230 -> 0231 -> 0350 | N3 | Address Information | O | 2
/// 0200 -> 0230 -> 0231 -> 0360 | N4 | Geographic Location | O | 1
/// 0200 -> 0230 -> 0231 -> 0370 | L11 | Business Instructions and Reference Number | O | 10
/// 0200 -> 0230 -> LOOP ID - 0233 | 999999
/// 0200 -> 0230 -> 0233 -> 0380 | CD3 | Carton (Package) Detail | O | 1
/// 0200 -> 0230 -> 0233 -> 0390 | L11 | Business Instructions and Reference Number | O | 20
/// 0200 -> 0230 -> 0233 -> LOOP ID - 0240 | 10
/// 0200 -> 0230 -> 0233 -> 0240 -> 0400 | AT7 | Shipment Status Details | O | 1
/// 0200 -> 0230 -> 0233 -> 0240 -> 0402 | MS1 | Equipment, Shipment, or Real Property Location | O | 1
/// 0200 -> 0230 -> 0233 -> 0240 -> 0404 | MS2 | Equipment or Container Owner and Type | O | 1
/// 0200 -> 0230 -> 0233 -> 0405 | MAN | Marks and Numbers | O | 9999
/// 0200 -> LOOP ID - 0250 | 999999
/// 0200 -> 0250 -> 0410 | SPO | Shipment Purchase Order Detail | O | 1
/// 0200 -> 0250 -> 0420 | SDQ | Destination Quantity | O | 10
/// 0200 -> LOOP ID - 0260 | >1
/// 0200 -> 0260 -> 0423 | EFI | Electronic Format Identification | O | 1
/// 0200 -> 0260 -> 0426 | BIN | Binary Data | M | 1
/// 0610 | SE | Transaction Set Trailer | M | 1
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214 {
    pub st: ST,
    pub b10: B10,
    pub l11: Vec<L11>,
    pub man: Vec<MAN>,
    pub k1: Vec<K1>,
    pub loop_0100: Vec<_214Loop0100>,
    pub ms3: Vec<MS3>,
    pub loop_0200: Vec<_214Loop0200>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0100 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g61: Option<G61>,
    pub g62: Option<G62>,
    pub l11: Vec<L11>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200 {
    pub lx: Option<LX>,
    pub loop_0205: Vec<_214Loop0200Loop0205>,
    pub l11: Vec<L11>,
    pub man: Vec<MAN>,
    pub q7: Vec<Q7>,
    pub k1: Vec<K1>,
    pub at5: Vec<AT5>,
    pub at8: Vec<AT8>,
    pub loop_0210: Vec<_214Loop0200Loop0210>,
    pub loop_0230: Vec<_214Loop0200Loop0230>,
    pub loop_0250: Vec<_214Loop0200Loop0250>,
    pub loop_0260: Vec<_214Loop0200Loop0260>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200Loop0205 {
    pub at7: Option<AT7>,
    pub ms1: Option<MS1>,
    pub ms2: Option<MS2>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200Loop0210 {
    pub cd3: Option<CD3>,
    pub l11: Vec<L11>,
    pub loop_0215: Vec<_214Loop0200Loop0210Loop0215>,
    pub nm1: Option<NM1>,
    pub q7: Vec<Q7>,
    pub at8: Option<AT8>,
    pub man: Vec<MAN>,
    pub loop_0220: Vec<_214Loop0200Loop0210Loop0220>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200Loop0210Loop0215 {
    pub at7: Option<AT7>,
    pub ms1: Option<MS1>,
    pub ms2: Option<MS2>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200Loop0210Loop0220 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200Loop0230 {
    pub prf: Option<PRF>,
    pub loop_0231: Vec<_214Loop0200Loop0230Loop0231>,
    pub loop_0233: Vec<_214Loop0200Loop0230Loop0233>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200Loop0230Loop0231 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200Loop0230Loop0233 {
    pub cd3: Option<CD3>,
    pub l11: Vec<L11>,
    pub loop_0240: Vec<_214Loop0200Loop0230Loop0233Loop0240>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200Loop0230Loop0233Loop0240 {
    pub at7: Option<AT7>,
    pub ms1: Option<MS1>,
    pub ms2: Option<MS2>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200Loop0250 {
    pub spo: Option<SPO>,
    pub sdq: Option<SDQ>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _214Loop0200Loop0260 {
    pub efi: Option<EFI>,
    pub bin: BIN,
}

/// 301 Confirmation (Ocean)
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _301 {
    pub st: ST,
    pub b1: B1,
    pub y3: Y3,
    pub loop_y4: Vec<_301LoopY4>,
    pub n9: Vec<N9>,
    pub r2a: Vec<R2A>,
    pub loop_n1: Vec<_301LoopN1>,
    pub loop_r4: Vec<_301LoopR4>,
    pub w09: Option<W09>,
    pub h3: Option<H3>,
    pub ea: Vec<EA>,
    pub loop_lx: Vec<_301LoopLx>,
    pub v1: Vec<V1>,
    pub v9: Vec<V9>,
    pub se: SE,
}

pub fn parse_301(input: &str) -> IResult<&str, _301> {
    let mut output = _301::default();
    let (input, obj) = parse_st(input)?;
    output.st = obj;
    let (input, _) = opt(newline)(input)?;
    let (input, obj) = parse_b1(input)?;
    output.b1 = obj;
    let (input, _) = opt(newline)(input)?;
    let (input, obj) = parse_y3(input)?;
    let (input, _) = opt(newline)(input)?;
    output.y3 = obj;
    let (input, obj) = parse_y4(input)?;
    output.loop_y4.push(_301LoopY4 {
        y4: Some(obj),
        w09: None,
    });
    let (input, _) = opt(newline)(input)?;
    let (input, obj) = parse_n9(input)?;
    output.n9.push(obj);
    let (input, _) = opt(newline)(input)?;
    let (input, obj_n1) = parse_n1(input)?;
    let (input, _) = opt(newline)(input)?;
    let (input, obj_n3) = parse_n3(input)?;
    let (input, _) = opt(newline)(input)?;
    let (input, obj_n4) = parse_n4(input)?;
    let (input, _) = opt(newline)(input)?;
    output.loop_n1.push(_301LoopN1 {
        n1: Some(obj_n1),
        n2: None,
        n3: Some(obj_n3),
        n4: Some(obj_n4),
        g61: None,
    });
    let (input, obj) = parse_r4(input)?;
    output.loop_r4.push(_301LoopR4 {
        r4: obj,
        ..Default::default()
    });
    let (input, _) = opt(newline)(input)?;
    let (input, obj_r4) = parse_r4(input)?;
    let (input, _) = opt(newline)(input)?;
    let (input, dtm_obj) = parse_dtm(input)?;
    let (input, _) = opt(newline)(input)?;
    output.loop_r4.push(_301LoopR4 {
        r4: obj_r4,
        dtm: vec![dtm_obj],
    });
    let (input, obj_r4) = parse_r4(input)?;
    output.loop_r4.push(_301LoopR4 {
        r4: obj_r4,
        dtm: vec![],
    });
    let (input, _) = opt(newline)(input)?;
    let (input, obj_lx) = parse_lx(input)?;
    let (input, _) = opt(newline)(input)?;
    let (input, obj_l0) = parse_l0(input)?;
    let (input, _) = opt(newline)(input)?;
    let (input, obj_l5) = parse_l5(input)?;
    let (input, _) = opt(newline)(input)?;
    output.loop_lx.push(_301LoopLx {
        lx: obj_lx,
        n7: None,
        w09: None,
        k1: vec![],
        l0: Some(obj_l0),
        l5: Some(obj_l5),
        l4: None,
        l1: None,
        loop_h1: vec![],
    });
    let (input, obj) = parse_v1(input)?;
    output.v1 = vec![obj];
    let (input, _) = opt(newline)(input)?;
    let (input, obj) = parse_se(input)?;
    output.se = obj;
    Ok((input, output))
}

impl Reflect for _301 {
    fn get_path(current_path: &Path, next_segment: &str, last_path: &Path) -> Path {
        match next_segment {
            "ST" => current_path.push("st".to_string(), None, true),
            "B1" => current_path.push("b1".to_string(), None, true),
            "Y3" => current_path.push("y3".to_string(), None, true),
            "Y4" => {
                println!("get_path: Y4: current_path: {current_path}, next_segment: {next_segment}, last_path: {last_path}");
                //must be part of _301LoopY4 loop
                let new_path = current_path.push("loop_y4".to_string(), Some(0), false);
                let x = _301LoopR4::get_path(&new_path, next_segment, last_path);
                x.next_op(PathOperation::Pop)

                // let counter = match last_path.elem.last().unwrap().vec_position {
                //     Some(count) => count + 1,
                //     None => 0,
                // };
                // let new_path = current_path.push("loop_y4".to_string(), Some(counter), false);
                // let x = _301LoopY4::get_path(&new_path, next_segment, last_path);
                // x.next_op(PathOperation::Pop)
            }
            "N9" => {
                let counter = match last_path.elem.last().unwrap().vec_position {
                    Some(count) => count + 1,
                    None => 0,
                };
                current_path.push("N9".to_string(), Some(counter), true)
            }
            "N1" => {
                let counter = match last_path.elem.last().unwrap().vec_position {
                    Some(count) => count + 1,
                    None => 0,
                };
                current_path.push("N1".to_string(), Some(counter), true)
            }
            "H3" => current_path.push("h3".to_string(), None, true),
            "SE" => current_path.push("se".to_string(), None, true),
            _ => {
                //must be part of _301LoopR4 loop
                let v = last_path.pop();
                if !v.elem.is_empty() {
                    let last_elem = v.elem.last().unwrap();
                    let counter = if last_elem.name == "loop_r4" {
                        match last_path.pop().elem.last().unwrap().vec_position {
                            Some(count) => count + 1,
                            None => 0,
                        }
                    } else {
                        0
                    };
                    let new_path = current_path.push("loop_r4".to_string(), Some(counter), false);
                    let x = _301LoopR4::get_path(&new_path, next_segment, last_path);
                    x.next_op(PathOperation::Pop)
                } else {
                    last_path.clone()
                }
            }
        }
    }

    fn get_type_name() -> String {
        "_301".to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _301LoopLx {
    pub lx: LX,
    pub n7: Option<N7>,
    pub w09: Option<W09>,
    pub k1: Vec<K1>,
    pub l0: Option<L0>,
    pub l5: Option<L5>,
    pub l4: Option<L4>,
    pub l1: Option<L1>,
    pub loop_h1: Vec<_301LoopLxLoopH1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _301LoopY4 {
    pub y4: Option<Y4>,
    pub w09: Option<W09>,
}

impl Reflect for _301LoopY4 {
    fn get_type_name() -> String {
        "_301LoopY4".to_string()
    }
    fn get_path(current_path: &Path, next_segment: &str, last_path: &Path) -> Path {
        match next_segment {
            "Y4" => current_path.push("y4".to_string(), None, true),
            "W09" => last_path.pop().push("w09".to_string(), Some(0), true),
            _ => Path::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _301LoopN1 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub g61: Option<G61>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _301LoopLxLoopH1 {
    pub h1: Option<H1>,
    pub h2: Vec<H2>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _301LoopR4 {
    pub r4: R4,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
}

impl Reflect for _301LoopR4 {
    fn get_type_name() -> String {
        "_301LoopR4".to_string()
    }
    fn get_path(current_path: &Path, next_segment: &str, last_path: &Path) -> Path {
        match next_segment {
            "R4" => current_path.push("r4".to_string(), None, true),
            "DTM" => last_path.pop().push("dtm".to_string(), Some(0), true),
            _ => Path::default(),
        }
    }
}

/// 309 - U.S. Customs Manifest
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the U.S. Customs Manifest Transaction Set (309) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used by carriers, terminal operators, port authorities, or service centers to provide U.S. Customs with manifest data on cargo arriving in or departing from the U.S. on oceangoing vessels, railroad trains, or other types of conveyances. The transaction set can be also used by carriers to provide terminal operators, port authorities, or service centers with manifest data on cargo arriving at their facilities via the conveyances mentioned above.
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1
/// 0020 | M10 | Manifest Identifying Information | M | 1
/// LOOP ID - P4 | 20
/// P4 -> 0040 | P4 | U.S. Port Information | M | 1
/// P4 -> LOOP ID - LX | 9999 |  
/// P4 -> LX -> 0060 | LX | Assigned Number | M | 1
/// P4 -> LX -> 0070 | M13 | Manifest Amendment Details | O | 1
/// P4 -> LX -> 0080 | M11 | Manifest Bill of Lading Details | O | 1
/// P4 -> LX -> 0085 | N9 | Reference Identification | O | 999
/// P4 -> LX -> LOOP ID - N1 | 5 |   |  
/// P4 -> LX -> N1 -> 0100 | N1 | Name | O | 1
/// P4 -> LX -> N1 -> 0110 | N3 | Address Information | O | 2
/// P4 -> LX -> N1 -> 0120 | N4 | Geographic Location | O | 1
/// P4 -> LX -> N1 -> 0123 | DTM | Date/Time Reference | O | 1
/// P4 -> LX -> N1 -> 0125 | PER | Administrative Communications Contact | O | 1
/// P4 -> LX -> LOOP ID - M12 | 1 |   |  
/// P4 -> LX -> M12 -> 0130 | M12 | In-bond Identifying Information | O | 1
/// P4 -> LX -> M12 -> 0135 | P5 | Port Information | O | 5
/// P4 -> LX -> LOOP ID - VID | 999 |   |  
/// P4 -> LX -> VID -> 0150 | VID | Conveyance Identification | O | 1
/// P4 -> LX -> VID -> 0155 | VC | Motor Vehicle Control | O | 21
/// P4 -> LX -> VID -> LOOP ID - N10 | 999 |   |   |  
/// P4 -> LX -> VID -> N10 -> 0160 | N10 | Quantity and Description | O | 1
/// P4 -> LX -> VID -> N10 -> LOOP ID - H1 | 10 |   |   |   |  
/// P4 -> LX -> VID -> N10 -> H1 -> 0165 | H1 | Hazardous Material | O | 1
/// P4 -> LX -> VID -> N10 -> H1 -> 0166 | H2 | Additional Hazardous Material Description | O | 99
/// 0200 | SE | Transaction Set Trailer | M | 1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _309 {
    pub st: ST,
    pub m10: M10,
    pub loop_p4: Vec<_309LoopP4>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _309LoopP4 {
    pub p4: P4,
    pub loop_lx: Vec<_309LoopLX>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _309LoopLX {
    pub lx: LX,
    pub m13: M13,
    pub m11: M11,
    pub n9: Vec<N9>,
    pub loop_n1: Vec<_309LoopN1>,
    pub loop_m12: Vec<_309LoopM12>,
    pub loop_vid: Vec<_309LoopVID>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _309LoopN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dtm: Option<DTM>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _309LoopM12 {
    pub m12: Option<M12>,
    pub p5: Vec<P5>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _309LoopVID {
    pub vid: Option<VID>,
    pub vc: Vec<VC>,
    pub loop_n10: Vec<_309LoopN10>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _309LoopN10 {
    pub n10: Option<N10>,
    pub loop_h1: Vec<_309LoopH1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _309LoopH1 {
    pub h1: Option<H1>,
    pub h2: Vec<H2>,
}

/// 310 - Freight Receipt and Invoice (Ocean)
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Freight Receipt and Invoice (Ocean) Transaction Set (310) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide ocean bill of lading information. It is sent by ocean carriers to interested parties and can be used as the receipt for the shipment; to substitute for a paper bill of lading where the parties have agreed that a paper bill of lading is not necessary; to allow shipper or forwarder to verify bill of lading information before an original is printed and released; for information purposes, i.e., as a bill of lading copy; by the carrier to convey manifest information to a terminal operator; and as an invoice for freight.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 010 | ST | Transaction Set Header | M | 1
/// 020 | B3 | Beginning Segment for Carrier's Invoice | M | 1
/// 030 | B2A | Set Purpose | O | 1
/// 040 | Y6 | Authentication | O | 2
/// 050 | G3 | Compensation Information | O | 1
/// 060 | N9 | Reference Identification | O | 15
/// 070 | V1 | Vessel Identification | M | 2
/// 080 | M0 | Letter of Credit Reference | O | 1
/// 090 | M1 | Insurance | O | 5
/// 100 | C2 | Bank ID | O | 1
/// 110 | C3 | Currency | O | 1
/// 120 | Y2 | Container Details | O | 10
/// LOOP ID - N1 | 10
/// N1 -> 130 | N1 | Name | M | 1
/// N1 -> 140 | N2 | Additional Name Information | O | 1
/// N1 -> 150 | N3 | Address Information | O | 2
/// N1 -> 160 | N4 | Geographic Location | O | 1
/// 170 | G61 | Contact | O | 3
/// LOOP ID - R4 | 20
/// R4 -> 180 | R4 | Port or Terminal | M | 1
/// R4 -> 190 | DTM | Date/Time Reference | O | 15
/// 199 | R2A | Route Information with Preference | O | 25
/// 200 | R2 | Route Information | O | 13
/// 210 | K1 | Remarks | O | 12
/// 220 | H3 | Special Handling Instructions | O | 6
/// 230 | L5 | Description, Marks and Numbers | O | 1 |
/// LOOP ID - C8 | 20
/// C8 -> 240 | C8 | Certifications and Clauses | O | 1
/// C8 -> 250 | C8C| Certifications Clauses Continuation | O | 5
/// LOOP ID - LX | 999
/// LX -> 010 | LX | Assigned Number | M | 1 |
/// LX -> LOOP ID - N7 | 999
/// LX -> N7 -> 020 | N7 | Equipment Details | O | 1 |
/// LX -> N7 -> 025 | QTY | Quantity | O | 1 |
/// LX -> N7 -> 030 | V4 | Cargo Location Reference | O | 1 |
/// LX -> N7 -> 040 | N12 | Equipment Environment | O | 1 |
/// LX -> N7 -> 050 | M7 | Seal Numbers | O | 5 |
/// LX -> N7 -> 060 | W09 | Equipment and Temperature | O | 1 |
/// LX -> N7 -> LOOP ID - L1 | 20
/// LX -> N7 -> L1 -> 070 | L1 | Rate and Charges | O | 1 |
/// LX -> N7 -> L1 -> 080 | C3 | Currency | O | 1 |
/// LX -> N7 -> 090 | L7 | Tariff Reference | O | 1 |
/// LX -> N7 -> 100 | X1 | Export License | O | 1 |
/// LX -> N7 -> 110 | X2 | Import License | O | 1 |
/// LX -> N7 -> 120 | N9 | Reference Identification | O | 3 |
/// LX -> N7 -> LOOP ID - H1 | 10
/// LX -> N7 -> H1 -> 130 | H1 | Hazardous Material | O | 1 |
/// LX -> N7 -> H1 -> 140 | H2 | Additional Hazardous Material Description | O | 10 |
/// LX -> LOOP ID - L0 | 120
/// LX -> L0 -> 150 | L0 | Line Item - Quantity and Weight | O | 1 |
/// LX -> L0 -> 160 | L5 | Description, Marks and Numbers | O | 999 |
/// LX -> L0 ->  |  | LOOP ID - L1 | 20
/// LX -> L0 -> L1 -> 170 | L1 | Rate and Charges | O | 1 |
/// LX -> L0 -> L1 -> 180 | C3 | Currency | O | 1 |
/// LX -> L0 -> 190 | L7 | Tariff Reference | O | 1 |
/// LX -> L0 -> 200 | X1 | Export License | O | 1 |
/// LX -> L0 -> 210 | X2 | Import License | O | 1 |
/// LX -> L0 -> LOOP ID - C8 | 20
/// LX -> L0 -> C8 -> 220 | C8 | Certifications and Clauses | O | 1 |
/// LX -> L0 -> C8C -> 221 | C8C | Certifications Clauses Continuation | O | 5 |
/// LX -> L0 -> LOOP ID - H1 | 10
/// LX -> L0 -> H1 -> 230 | H1 | Hazardous Material | O | 1 |
/// LX -> L0 -> H1 -> 240 | H2 | Additional Hazardous Material Description | O | 10
/// 010 | L3 | Total Weight and Charges | M | 1  
/// 020 | PWK | Paperwork | O | 25  
/// LOOP ID - L1 | 20
/// L1 -> 030 | L1 | Rate and Charges | O | 1  
/// L1 -> 040 | C3 | Currency | O | 1
/// 050 | V9 | Event Detail | O | 10  
/// 055 | C8 | Certifications and Clauses | O | 20  
/// 060 | K1 | Remarks | O | 999  
/// 070 | L11 | Business Instructions and Reference Number | O | 1  
/// 080 | SE | Transaction Set Trailer | M | 1 |
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _310 {
    pub st: ST,
    pub b3: B3,
    pub b2a: Option<B2A>,
    #[serde(default)]
    pub y6: Vec<Y6>,
    pub g3: Option<G3>,
    #[serde(default)]
    pub n9: Vec<N9>,
    #[serde(default)]
    pub v1: Vec<V1>,
    pub m0: Option<M0>,
    #[serde(default)]
    pub m1: Vec<M1>,
    pub c2: Option<C2>,
    pub c3: Option<C3>,
    #[serde(default)]
    pub y2: Vec<Y2>,
    #[serde(default)]
    pub loop_n1: Vec<_310LoopN1>,
    #[serde(default)]
    pub g61: Vec<G61>,
    #[serde(default)]
    pub loop_r4: Vec<_310LoopR4>,
    #[serde(default)]
    pub r2a: Vec<R2A>,
    #[serde(default)]
    pub r2: Vec<R2>,
    /// heading remarks
    #[serde(default)]
    pub k1: Vec<K1>,
    #[serde(default)]
    pub h3: Vec<H3>,
    pub l5: Option<L5>,
    #[serde(default)]
    pub loop_c8: Vec<_310LoopC8>,
    #[serde(default)]
    pub loop_lx: Vec<_310LoopLX>,
    pub l3: L3,
    #[serde(default)]
    pub pwk: Vec<PWK>,
    #[serde(default)]
    pub loop_l1: Vec<_310LoopL1>,
    pub v9: Vec<V9>,
    pub c8: Vec<C8>,
    ///TODO summary remarks
    pub k1_2: Vec<K1>,
    pub l11: Option<L11>,
    pub se: SE,
}

impl Reflect for _310 {
    fn get_path(current_path: &Path, next_segment: &str, last_path: &Path) -> Path {
        match next_segment {
            "ST" => current_path.push("st".to_string(), None, true),
            "B3" => current_path.push("b3".to_string(), None, true),
            "B2A" => current_path.push("b2a".to_string(), None, true),
            "Y6" => {
                let counter = match last_path.elem.last().unwrap().vec_position {
                    Some(count) => count + 1,
                    None => 0,
                };
                current_path.push("y6".to_string(), Some(counter), true)
            }
            _ => Path::default(),
        }
    }

    fn get_type_name() -> String {
        "_310".to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _310LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _310LoopR4 {
    pub r4: R4,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtm: Option<DTM>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _310LoopC8 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c8: Option<C8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c8c: Option<C8C>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _310LoopLX {
    pub lx: LX,
    pub loop_n7: Vec<_310LoopN7>,
    pub loop_l0: Vec<_310LoopL0>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _310LoopN7 {
    pub n7: Option<N7>,
    pub qty: Option<QTY>,
    pub v4: Option<V4>,
    pub n12: Option<N12>,
    pub m7: Vec<M7>,
    pub w09: Option<W09>,
    pub loop_l1: Vec<_310LoopL1>,
    pub l7: Option<L7>,
    pub x1: Option<X1>,
    pub x2: Option<X2>,
    pub n9: Vec<N9>,
    pub loop_h1: Vec<_310LoopH1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _310LoopL0 {
    pub l0: Option<L0>,
    pub l5: Option<L5>,
    pub loop_l1: Vec<_310LoopL1>,
    pub l7: Option<L7>,
    pub x1: Option<X1>,
    pub x2: Option<X2>,
    pub loop_c8: Vec<_310LoopC8>,
    pub loop_h1: Vec<_310LoopH1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _310LoopL1 {
    pub l1: Option<L1>,
    pub c3: Option<C3>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _310LoopH1 {
    pub h1: Option<H1>,
    pub h2: Option<H2>,
}

/// 315 - Status Details (Ocean)
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Status Details (Ocean) Transaction Set (315) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide all the information necessary to report status or event details for selected shipments or containers. It is intended to accommodate the details for one status or event associated with many shipments or containers, as well as more than one status or event for one shipment or container.
///
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _315 {
    pub st: ST,
    pub b4: B4,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q2: Option<Q2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sg: Vec<SG>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loop_r4: Vec<_315LoopR4>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v9: Option<V9>,
    pub se: SE,
}

impl Reflect for _315 {
    fn get_path(current_path: &Path, next_segment: &str, last_path: &Path) -> Path {
        match next_segment {
            "ST" => current_path.push("st".to_string(), None, true),
            "B4" => current_path.push("b4".to_string(), None, true),
            "N9" => {
                let counter = match last_path.elem.last().unwrap().vec_position {
                    Some(count) => count + 1,
                    None => 0,
                };
                current_path.push("n9".to_string(), Some(counter), true)
            }
            "Q2" => current_path.push("q2".to_string(), None, true),
            "SG" => current_path.push("sg".to_string(), Some(0), true),
            "V9" => current_path.push("v9".to_string(), None, true),
            "SE" => current_path.push("se".to_string(), None, true),
            _ => {
                //must be part of _315LoopR4 loop
                let v = last_path.pop();
                if !v.elem.is_empty() {
                    let last_elem = v.elem.last().unwrap();
                    let counter = if last_elem.name == "loop_r4" {
                        match last_path.pop().elem.last().unwrap().vec_position {
                            Some(count) => count + 1,
                            None => 0,
                        }
                    } else {
                        0
                    };
                    let new_path = current_path.push("loop_r4".to_string(), Some(counter), false);
                    let x = _315LoopR4::get_path(&new_path, next_segment, last_path);
                    x.next_op(PathOperation::Pop)
                } else {
                    last_path.clone()
                }
            }
        }
    }

    fn get_type_name() -> String {
        "_315".to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct _315LoopR4 {
    pub r4: R4,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
}

impl Reflect for _315LoopR4 {
    fn get_type_name() -> String {
        "_315LoopR4".to_string()
    }
    fn get_path(current_path: &Path, next_segment: &str, last_path: &Path) -> Path {
        match next_segment {
            "R4" => current_path.push("r4".to_string(), None, true),
            "DTM" => last_path.pop().push("dtm".to_string(), Some(0), true),
            _ => Path::default(),
        }
    }
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
    #[serde(default)]
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
pub struct _404LoopN7 {
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
pub struct _404LoopN7Ref {
    pub _ref: Option<REF>,
    pub n10: Option<N10>,
    pub loop_n1: Vec<_404LoopN7RefN1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopN7RefN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopVC {
    pub vc: Option<VC>,
    pub loop_n1: Vec<_404LoopVcN1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopVcN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub h3: Option<H3>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopN1 {
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
pub struct _404LoopN7E1 {
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
pub struct _404LoopT1 {
    pub t1: Option<T1>,
    pub t2: Option<T2>,
    pub t3: Option<T3>,
    pub t6: Option<T6>,
    pub t8: Option<T8>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopLH1 {
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
pub struct _404LoopLX {
    pub lx: LX,
    pub l5: L5,
    pub loop_l0: Vec<_404LoopL0>,
    pub x1: Option<X1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopRef {
    pub _ref: Option<REF>,
    pub n10: Option<N10>,
    pub loop_n1: Vec<_404LoopRefN1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct _404LoopRefN1 {
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
pub struct _997LoopAk2 {
    pub ak2: AK2,
    pub loop_ak3: Vec<_997LoopAk3>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _997LoopAk3 {
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
