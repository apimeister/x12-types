//! v004010 repesents all entities of the 004010 specification.

use crate::util::Parser;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::multi::many0;
use nom::IResult;
use nom::Parser as _;
pub use segment::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Display;
use x12_types_macros::DisplayX12;

mod segment;

#[cfg(test)]
mod test_204;
#[cfg(test)]
mod test_214;
#[cfg(test)]
mod test_301;
#[cfg(test)]
mod test_309;
#[cfg(test)]
mod test_310;
#[cfg(test)]
mod test_315;
#[cfg(test)]
mod test_322;
#[cfg(test)]
mod test_404;
#[cfg(test)]
mod test_810;
#[cfg(test)]
mod test_997;
#[cfg(test)]
mod test_998;
#[cfg(test)]
mod test_segments;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Transmission<T> {
    pub isa: ISA,
    pub functional_group: Vec<FunctionalGroup<T>>,
    pub iea: IEA,
}

impl<'a, T: Default + Parser<&'a str, T, nom::error::Error<&'a str>>>
    Parser<&'a str, Transmission<T>, nom::error::Error<&'a str>> for Transmission<T>
{
    fn parse(input: &'a str) -> IResult<&'a str, Transmission<T>> {
        let mut output = Transmission::default();
        let (input, obj) = ISA::parse(input)?;
        output.isa = obj;
        // functional group
        let (input, gs) = GS::parse(input)?;
        let (input, t_obj) = many0(T::parse).parse(input)?;
        // let (input, t_obj) = T::parse(input)?;
        let (input, ge) = GE::parse(input)?;
        let fg = FunctionalGroup {
            gs,
            segments: t_obj,
            // segments: vec![t_obj],
            ge,
        };
        output.functional_group.push(fg);
        let (input, obj) = IEA::parse(input)?;
        output.iea = obj;
        Ok((input, output))
    }
}

impl<T: Display> Display for Transmission<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![];
        lines.push(format!("{}", self.isa));
        for fg in &self.functional_group {
            lines.push(format!("{}", fg.gs));
            for segment in &fg.segments {
                lines.push(format!("{segment}"));
            }
            lines.push(format!("{}", fg.ge));
        }
        lines.push(format!("{}", self.iea));
        let all = lines.join("");
        write!(f, "{all}")
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204 {
    pub st: ST,
    pub b2: B2,
    pub b2a: B2A,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub l11: Vec<L11>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g62: Option<G62>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms3: Option<MS3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at5: Option<AT5>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pld: Option<PLD>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lh6: Vec<LH6>,
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

impl<'a> Parser<&'a str, _204, nom::error::Error<&'a str>> for _204 {
    fn parse(input: &'a str) -> IResult<&'a str, _204> {
        let mut output = _204::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = B2::parse(rest)?;
        output.b2 = obj;
        let (rest, obj) = B2A::parse(rest)?;
        output.b2a = obj;
        let (rest, obj) = many0(L11::parse).parse(rest)?;
        output.l11 = obj;
        let (rest, obj) = opt(G62::parse).parse(rest)?;
        output.g62 = obj;
        let (rest, obj) = opt(MS3::parse).parse(rest)?;
        output.ms3 = obj;
        let (rest, obj) = opt(AT5::parse).parse(rest)?;
        output.at5 = obj;
        let (rest, obj) = opt(PLD::parse).parse(rest)?;
        output.pld = obj;
        let (rest, obj) = many0(LH6::parse).parse(rest)?;
        output.lh6 = obj;
        let (rest, obj) = opt(NTE::parse).parse(rest)?;
        output.nte = obj;
        // loop 100
        let mut loop_100 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = opt(N1::parse).parse(loop_rest)?;
            let (rest, n2) = opt(N2::parse).parse(rest)?;
            let (rest, n3) = many0(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, l11) = opt(L11::parse).parse(rest)?;
            let (rest, g61) = many0(G61::parse).parse(rest)?;
            loop_rest = rest;
            loop_100.push(_204Loop100 {
                n1,
                n2,
                n3,
                n4,
                l11,
                g61,
            });
        }
        let rest = loop_rest;
        output.loop_100 = loop_100;
        // loop 200
        let mut loop_200 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N7::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n7) = opt(N7::parse).parse(loop_rest)?;
            let (rest, n7a) = opt(N7A::parse).parse(rest)?;
            let (rest, n7b) = opt(N7B::parse).parse(rest)?;
            let (rest, mea) = opt(MEA::parse).parse(rest)?;
            let (rest, m7) = opt(M7::parse).parse(rest)?;
            loop_rest = rest;
            loop_200.push(_204Loop200 {
                n7,
                n7a,
                n7b,
                mea,
                m7,
            });
        }
        let rest = loop_rest;
        output.loop_200 = loop_200;
        // loop 300
        let mut loop_300 = vec![];
        let mut loop_rest = rest;
        while peek(opt(S5::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, s5) = S5::parse(loop_rest)?;
            let (rest, l11) = many0(L11::parse).parse(rest)?;
            let (rest, g62) = many0(G62::parse).parse(rest)?;
            let (rest, at8) = opt(AT8::parse).parse(rest)?;
            let (rest, lad) = many0(LAD::parse).parse(rest)?;
            let (rest, at5) = many0(AT5::parse).parse(rest)?;
            let (rest, pld) = opt(PLD::parse).parse(rest)?;
            let (rest, nte) = many0(NTE::parse).parse(rest)?;
            loop_rest = rest;
            // loop 310
            let mut loop_310 = vec![];
            while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, n1) = opt(N1::parse).parse(loop_rest)?;
                let (rest, n2) = opt(N2::parse).parse(rest)?;
                let (rest, n3) = many0(N3::parse).parse(rest)?;
                let (rest, n4) = opt(N4::parse).parse(rest)?;
                let (rest, g61) = many0(G61::parse).parse(rest)?;
                loop_rest = rest;
                loop_310.push(_204Loop310 {
                    n1,
                    n2,
                    n3,
                    n4,
                    g61,
                });
            }
            // loop 320
            let mut loop_320 = vec![];
            while peek(opt(L5::parse)).parse(loop_rest)?.1.is_some()
                || peek(opt(LH1::parse)).parse(loop_rest)?.1.is_some()
            {
                let (rest, l5) = opt(L5::parse).parse(loop_rest)?;
                let (rest, at8) = opt(AT8::parse).parse(rest)?;
                loop_rest = rest;
                // loop 325
                let mut loop_325 = vec![];
                while peek(opt(G61::parse)).parse(loop_rest)?.1.is_some()
                    || peek(opt(LH1::parse)).parse(loop_rest)?.1.is_some()
                {
                    let (rest, g61) = opt(G61::parse).parse(loop_rest)?;
                    let (rest, l11) = many0(L11::parse).parse(rest)?;
                    let (rest, lh6) = opt(LH6::parse).parse(rest)?;
                    loop_rest = rest;
                    // loop 330
                    let mut loop_330 = vec![];
                    while peek(opt(LH1::parse)).parse(loop_rest)?.1.is_some() {
                        let (rest, lh1) = opt(LH1::parse).parse(loop_rest)?;
                        let (rest, lh2) = many0(LH2::parse).parse(rest)?;
                        let (rest, lh3) = many0(LH3::parse).parse(rest)?;
                        let (rest, lfh) = many0(LFH::parse).parse(rest)?;
                        let (rest, lep) = many0(LEP::parse).parse(rest)?;
                        let (rest, lh4) = opt(LH4::parse).parse(rest)?;
                        let (rest, lht) = many0(LHT::parse).parse(rest)?;
                        loop_rest = rest;
                        loop_330.push(_204Loop330 {
                            lh1,
                            lh2,
                            lh3,
                            lfh,
                            lep,
                            lh4,
                            lht,
                        });
                    }
                    loop_325.push(_204Loop325 {
                        g61,
                        l11,
                        lh6,
                        loop_330,
                    });
                }
                loop_320.push(_204Loop320 { l5, at8, loop_325 });
            }
            // loop 380
            let mut loop_380 = vec![];
            while peek(opt(N7::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, n7) = opt(N7::parse).parse(loop_rest)?;
                let (rest, n7a) = opt(N7A::parse).parse(rest)?;
                let (rest, n7b) = opt(N7B::parse).parse(rest)?;
                let (rest, mea) = opt(MEA::parse).parse(rest)?;
                let (rest, m7) = opt(M7::parse).parse(rest)?;
                loop_rest = rest;
                loop_380.push(_204Loop380 {
                    n7,
                    n7a,
                    n7b,
                    mea,
                    m7,
                });
            }
            loop_300.push(_204Loop300 {
                s5,
                l11,
                g62,
                at8,
                lad,
                at5,
                pld,
                nte,
                loop_310,
                loop_320,
                loop_350: vec![],
                loop_380,
            });
        }
        let rest = loop_rest;
        output.loop_300 = loop_300;
        let (rest, obj) = opt(L3::parse).parse(rest)?;
        output.l3 = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204Loop100 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Option<L11>,
    pub g61: Vec<G61>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204Loop200 {
    pub n7: Option<N7>,
    pub n7a: Option<N7A>,
    pub n7b: Option<N7B>,
    pub mea: Option<MEA>,
    pub m7: Option<M7>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204Loop310 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g61: Vec<G61>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204Loop320 {
    pub l5: Option<L5>,
    pub at8: Option<AT8>,
    pub loop_325: Vec<_204Loop325>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204Loop325 {
    pub g61: Option<G61>,
    pub l11: Vec<L11>,
    pub lh6: Option<LH6>,
    pub loop_330: Vec<_204Loop330>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204Loop330 {
    pub lh1: Option<LH1>,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Vec<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Vec<LHT>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204Loop350 {
    pub oid: Option<OID>,
    pub g62: Vec<G62>,
    pub lad: Vec<LAD>,
    pub loop_360: Vec<_204Loop360>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204Loop360 {
    pub l5: Option<L5>,
    pub at8: Option<AT8>,
    pub loop_365: Vec<_204Loop365>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204Loop365 {
    pub g61: Option<G61>,
    pub l11: Vec<L11>,
    pub lh6: Vec<LH6>,
    pub loop_370: Vec<_204Loop370>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _204Loop370 {
    pub lh1: Option<LH1>,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Vec<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Vec<LHT>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
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

impl<'a> Parser<&'a str, _214, nom::error::Error<&'a str>> for _214 {
    fn parse(input: &'a str) -> IResult<&'a str, _214> {
        let mut output = _214::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = B10::parse(rest)?;
        output.b10 = obj;
        let (rest, obj) = many0(L11::parse).parse(rest)?;
        output.l11 = obj;
        let (rest, obj) = many0(MAN::parse).parse(rest)?;
        output.man = obj;
        let (rest, obj) = many0(K1::parse).parse(rest)?;
        output.k1 = obj;
        // loop 100
        let mut loop_100 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = opt(N1::parse).parse(loop_rest)?;
            let (rest, n2) = opt(N2::parse).parse(rest)?;
            let (rest, n3) = many0(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, g61) = opt(G61::parse).parse(rest)?;
            let (rest, g62) = opt(G62::parse).parse(rest)?;
            let (rest, l11) = many0(L11::parse).parse(rest)?;
            loop_rest = rest;
            loop_100.push(_214Loop0100 {
                n1,
                n2,
                n3,
                n4,
                g61,
                g62,
                l11,
            });
        }
        let rest = loop_rest;
        output.loop_0100 = loop_100;
        let (rest, obj) = many0(MS3::parse).parse(rest)?;
        output.ms3 = obj;
        // loop 200
        let mut loop_200 = vec![];
        loop_rest = rest;
        while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, lx) = LX::parse(loop_rest)?;
            // loop 0205
            let mut loop_0205 = vec![];
            loop_rest = rest;
            while peek(opt(AT7::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, at7) = AT7::parse(loop_rest)?;
                let (rest, ms1) = opt(MS1::parse).parse(rest)?;
                let (rest, ms2) = opt(MS2::parse).parse(rest)?;
                loop_rest = rest;
                loop_0205.push(_214Loop0205 { at7, ms1, ms2 });
            }
            let rest = loop_rest;
            let (rest, l11) = many0(L11::parse).parse(rest)?;
            let (rest, man) = many0(MAN::parse).parse(rest)?;
            let (rest, q7) = many0(Q7::parse).parse(rest)?;
            let (rest, k1) = many0(K1::parse).parse(rest)?;
            let (rest, at5) = many0(AT5::parse).parse(rest)?;
            let (rest, at8) = many0(AT8::parse).parse(rest)?;
            loop_rest = rest;
            // loop 0210
            // loop 0230
            // loop 0250
            // loop 0260
            loop_200.push(_214Loop0200 {
                lx,
                loop_0205,
                l11,
                man,
                q7,
                k1,
                at5,
                at8,
                loop_0210: vec![],
                loop_0230: vec![],
                loop_0250: vec![],
                loop_0260: vec![],
            });
        }
        let rest = loop_rest;
        output.loop_0200 = loop_200;

        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0100 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g61: Option<G61>,
    pub g62: Option<G62>,
    pub l11: Vec<L11>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0200 {
    pub lx: LX,
    pub loop_0205: Vec<_214Loop0205>,
    pub l11: Vec<L11>,
    pub man: Vec<MAN>,
    pub q7: Vec<Q7>,
    pub k1: Vec<K1>,
    pub at5: Vec<AT5>,
    pub at8: Vec<AT8>,
    pub loop_0210: Vec<_214Loop0210>,
    pub loop_0230: Vec<_214Loop0230>,
    pub loop_0250: Vec<_214Loop0250>,
    pub loop_0260: Vec<_214Loop0260>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0205 {
    pub at7: AT7,
    pub ms1: Option<MS1>,
    pub ms2: Option<MS2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0210 {
    pub cd3: Option<CD3>,
    pub l11: Vec<L11>,
    pub loop_0215: Vec<_214Loop0210Loop0215>,
    pub nm1: Option<NM1>,
    pub q7: Vec<Q7>,
    pub at8: Option<AT8>,
    pub man: Vec<MAN>,
    pub loop_0220: Vec<_214Loop0210Loop0220>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0210Loop0215 {
    pub at7: Option<AT7>,
    pub ms1: Option<MS1>,
    pub ms2: Option<MS2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0210Loop0220 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0230 {
    pub prf: Option<PRF>,
    pub loop_0231: Vec<_214Loop0231>,
    pub loop_0233: Vec<_214Loop0233>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0231 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0233 {
    pub cd3: Option<CD3>,
    pub l11: Vec<L11>,
    pub loop_0240: Vec<_214Loop0240>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0240 {
    pub at7: Option<AT7>,
    pub ms1: Option<MS1>,
    pub ms2: Option<MS2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0250 {
    pub spo: Option<SPO>,
    pub sdq: Option<SDQ>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _214Loop0260 {
    pub efi: Option<EFI>,
    pub bin: BIN,
}

/// 301 Confirmation (Ocean)
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
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

impl<'a> Parser<&'a str, _301, nom::error::Error<&'a str>> for _301 {
    fn parse(input: &'a str) -> IResult<&'a str, _301> {
        let mut output = _301::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = B1::parse(rest)?;
        output.b1 = obj;
        let (rest, obj) = Y3::parse(rest)?;
        output.y3 = obj;
        // loop y4
        let mut loop_y4 = vec![];
        let mut loop_rest = rest;
        while peek(opt(Y4::parse)).parse(loop_rest)?.1.is_some()
            || peek(opt(W09::parse)).parse(loop_rest)?.1.is_some()
        {
            let (rest, y4) = opt(Y4::parse).parse(loop_rest)?;
            let (rest, w09) = opt(W09::parse).parse(rest)?;
            loop_rest = rest;
            loop_y4.push(_301LoopY4 { y4, w09 });
        }
        let rest = loop_rest;
        output.loop_y4 = loop_y4;
        let (rest, obj) = many0(N9::parse).parse(rest)?;
        output.n9 = obj;
        let (rest, obj) = many0(R2A::parse).parse(rest)?;
        output.r2a = obj;
        // loop n1
        let mut loop_n1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            println!("n1");
            let (rest, n1) = opt(N1::parse).parse(loop_rest)?;
            let (rest, n2) = opt(N2::parse).parse(rest)?;
            let (rest, n3) = opt(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, g61) = opt(G61::parse).parse(rest)?;
            loop_rest = rest;
            loop_n1.push(_301LoopN1 {
                n1,
                n2,
                n3,
                n4,
                g61,
            });
        }
        let rest = loop_rest;
        output.loop_n1 = loop_n1;
        // loop r4
        let mut loop_r4 = vec![];
        let mut loop_rest = rest;
        while peek(opt(R4::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, r4) = R4::parse(loop_rest)?;
            let (rest, dtm) = many0(DTM::parse).parse(rest)?;
            loop_rest = rest;
            loop_r4.push(_301LoopR4 { r4, dtm });
        }
        let rest = loop_rest;
        output.loop_r4 = loop_r4;
        let (rest, obj) = opt(W09::parse).parse(rest)?;
        output.w09 = obj;
        let (rest, obj) = opt(H3::parse).parse(rest)?;
        output.h3 = obj;
        let (rest, obj) = many0(EA::parse).parse(rest)?;
        output.ea = obj;
        // loop lx
        let mut loop_lx = vec![];
        let mut loop_rest = rest;
        while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some()
            || peek(opt(W09::parse)).parse(loop_rest)?.1.is_some()
        {
            let (rest, lx) = LX::parse(loop_rest)?;
            let (rest, n7) = opt(N7::parse).parse(rest)?;
            let (rest, w09) = opt(W09::parse).parse(rest)?;
            let (rest, k1) = many0(K1::parse).parse(rest)?;
            let (rest, l0) = opt(L0::parse).parse(rest)?;
            let (rest, l5) = opt(L5::parse).parse(rest)?;
            let (rest, l4) = opt(L4::parse).parse(rest)?;
            let (rest, l1) = opt(L1::parse).parse(rest)?;
            loop_rest = rest;
            // loop h1
            let mut loop_h1 = vec![];
            while peek(opt(H1::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, h1) = opt(H1::parse).parse(loop_rest)?;
                let (rest, h2) = many0(H2::parse).parse(rest)?;
                loop_rest = rest;
                loop_h1.push(_301LoopLxLoopH1 { h1, h2 });
            }
            loop_lx.push(_301LoopLx {
                lx,
                n7,
                w09,
                k1,
                l0,
                l5,
                l4,
                l1,
                loop_h1,
            });
        }
        let rest = loop_rest;
        output.loop_lx = loop_lx;
        let (rest, obj) = many0(V1::parse).parse(rest)?;
        output.v1 = obj;
        let (rest, obj) = many0(V9::parse).parse(rest)?;
        output.v9 = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _301LoopY4 {
    pub y4: Option<Y4>,
    pub w09: Option<W09>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _301LoopN1 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub g61: Option<G61>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _301LoopLxLoopH1 {
    pub h1: Option<H1>,
    pub h2: Vec<H2>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _301LoopR4 {
    pub r4: R4,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _309 {
    pub st: ST,
    pub m10: M10,
    pub loop_p4: Vec<_309LoopP4>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _309, nom::error::Error<&'a str>> for _309 {
    fn parse(input: &'a str) -> IResult<&'a str, _309> {
        let mut output = _309::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = M10::parse(rest)?;
        output.m10 = obj;
        // loop p4
        let mut loop_p4 = vec![];
        let mut loop_rest = rest;
        while peek(opt(P4::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, p4) = P4::parse(loop_rest)?;
            loop_rest = rest;
            // loop lx
            let mut loop_lx = vec![];
            while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, lx) = LX::parse(loop_rest)?;
                let (rest, m13) = opt(M13::parse).parse(rest)?;
                let (rest, m11) = opt(M11::parse).parse(rest)?;
                let (rest, n9) = many0(N9::parse).parse(rest)?;
                loop_rest = rest;
                // loop n1
                let mut loop_n1 = vec![];
                while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, n1) = opt(N1::parse).parse(loop_rest)?;
                    let (rest, n3) = opt(N3::parse).parse(rest)?;
                    let (rest, n4) = opt(N4::parse).parse(rest)?;
                    let (rest, dtm) = opt(DTM::parse).parse(rest)?;
                    let (rest, per) = opt(PER::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_n1.push(_309LoopN1 {
                        n1,
                        n3,
                        n4,
                        dtm,
                        per,
                    });
                }
                // loop m12
                let mut loop_m12 = vec![];
                while peek(opt(M12::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, m12) = opt(M12::parse).parse(loop_rest)?;
                    let (rest, r4) = many0(R4::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_m12.push(_309LoopM12 { m12, r4 });
                }
                // loop vid
                let mut loop_vid = vec![];
                while peek(opt(VID::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, vid) = opt(VID::parse).parse(loop_rest)?;
                    let (rest, m7) = many0(M7::parse).parse(rest)?;
                    loop_rest = rest;
                    // loop n10
                    let mut loop_n10 = vec![];
                    while peek(opt(N10::parse)).parse(loop_rest)?.1.is_some() {
                        let (rest, n10) = opt(N10::parse).parse(loop_rest)?;
                        let (rest, vc) = many0(VC::parse).parse(rest)?;
                        loop_rest = rest;
                        // loop h1
                        let mut loop_h1 = vec![];
                        while peek(opt(H1::parse)).parse(loop_rest)?.1.is_some() {
                            let (rest, h1) = opt(H1::parse).parse(loop_rest)?;
                            let (rest, h2) = many0(H2::parse).parse(rest)?;
                            loop_rest = rest;
                            loop_h1.push(_309LoopH1 { h1, h2 });
                        }
                        loop_n10.push(_309LoopN10 { n10, vc, loop_h1 });
                    }
                    loop_vid.push(_309LoopVID { vid, m7, loop_n10 });
                }
                loop_lx.push(_309LoopLX {
                    lx,
                    m13,
                    m11,
                    n9,
                    loop_n1,
                    loop_m12,
                    loop_vid,
                });
            }
            loop_p4.push(_309LoopP4 { p4, loop_lx });
        }
        output.loop_p4 = loop_p4;
        let rest = loop_rest;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _309LoopP4 {
    pub p4: P4,
    pub loop_lx: Vec<_309LoopLX>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _309LoopLX {
    pub lx: LX,
    pub m13: Option<M13>,
    pub m11: Option<M11>,
    pub n9: Vec<N9>,
    pub loop_n1: Vec<_309LoopN1>,
    pub loop_m12: Vec<_309LoopM12>,
    pub loop_vid: Vec<_309LoopVID>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _309LoopN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dtm: Option<DTM>,
    pub per: Option<PER>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _309LoopM12 {
    pub m12: Option<M12>,
    pub r4: Vec<R4>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _309LoopVID {
    pub vid: Option<VID>,
    pub m7: Vec<M7>,
    pub loop_n10: Vec<_309LoopN10>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _309LoopN10 {
    pub n10: Option<N10>,
    pub vc: Vec<VC>,
    pub loop_h1: Vec<_309LoopH1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
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

impl<'a> Parser<&'a str, _310, nom::error::Error<&'a str>> for _310 {
    fn parse(input: &'a str) -> IResult<&'a str, _310> {
        let (rest, st) = ST::parse(input)?;
        let (rest, b3) = B3::parse(rest)?;
        let (rest, b2a) = opt(B2A::parse).parse(rest)?;
        let (rest, y6) = many0(Y6::parse).parse(rest)?;
        let (rest, g3) = opt(G3::parse).parse(rest)?;
        let (rest, n9) = many0(N9::parse).parse(rest)?;
        let (rest, v1) = many0(V1::parse).parse(rest)?;
        let (rest, m0) = opt(M0::parse).parse(rest)?;
        let (rest, m1) = many0(M1::parse).parse(rest)?;
        let (rest, c2) = opt(C2::parse).parse(rest)?;
        let (rest, c3) = opt(C3::parse).parse(rest)?;
        let (rest, y2) = many0(Y2::parse).parse(rest)?;
        // n1 loop
        let mut loop_n1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = opt(N2::parse).parse(rest)?;
            let (rest, n3) = opt(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            loop_rest = rest;
            loop_n1.push(_310LoopN1 { n1, n2, n3, n4 });
        }
        let rest = loop_rest;
        let (rest, g61) = many0(G61::parse).parse(rest)?;
        // loop r4
        let mut loop_r4 = vec![];
        let mut loop_rest = rest;
        while peek(opt(R4::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, r4) = R4::parse(loop_rest)?;
            let (rest, dtm) = opt(DTM::parse).parse(rest)?;
            loop_rest = rest;
            loop_r4.push(_310LoopR4 { r4, dtm });
        }
        let rest = loop_rest;
        let (rest, r2a) = many0(R2A::parse).parse(rest)?;
        let (rest, r2) = many0(R2::parse).parse(rest)?;
        let (rest, k1) = many0(K1::parse).parse(rest)?;
        let (rest, h3) = many0(H3::parse).parse(rest)?;
        let (rest, l5) = opt(L5::parse).parse(rest)?;
        // loop c8
        let mut loop_c8 = vec![];
        let mut loop_rest = rest;
        while peek(opt(C8::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, c8) = opt(C8::parse).parse(loop_rest)?;
            let (rest, c8c) = many0(C8C::parse).parse(rest)?;
            loop_rest = rest;
            loop_c8.push(_310LoopC8 { c8, c8c });
        }
        let rest = loop_rest;
        // loop lx
        let mut loop_lx = vec![];
        let mut loop_rest = rest;
        while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, lx) = LX::parse(loop_rest)?;
            loop_rest = rest;
            // loop n7
            let mut loop_n7 = vec![];
            while peek(opt(N7::parse)).parse(loop_rest)?.1.is_some()
                || peek(opt(L1::parse)).parse(loop_rest)?.1.is_some()
            {
                let (rest, n7) = opt(N7::parse).parse(loop_rest)?;
                let (rest, qty) = opt(QTY::parse).parse(rest)?;
                let (rest, v4) = opt(V4::parse).parse(rest)?;
                let (rest, n12) = opt(N12::parse).parse(rest)?;
                let (rest, m7) = many0(M7::parse).parse(rest)?;
                let (rest, w09) = opt(W09::parse).parse(rest)?;
                // loop l1
                let mut loop_l1 = vec![];
                loop_rest = rest;
                while peek(opt(L1::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, l1) = opt(L1::parse).parse(loop_rest)?;
                    let (rest, c3) = opt(C3::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_l1.push(_310LoopL1 { l1, c3 });
                }
                let (rest, l7) = opt(L7::parse).parse(loop_rest)?;
                let (rest, x1) = opt(X1::parse).parse(rest)?;
                let (rest, x2) = opt(X2::parse).parse(rest)?;
                let (rest, n9) = many0(N9::parse).parse(rest)?;
                loop_rest = rest;
                loop_n7.push(_310LoopN7 {
                    n7,
                    qty,
                    v4,
                    n12,
                    m7,
                    w09,
                    loop_l1,
                    l7,
                    x1,
                    x2,
                    n9,
                    loop_h1: vec![],
                });
            }
            // loop l0
            let mut loop_l0 = vec![];
            while peek(opt(L0::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, l0) = opt(L0::parse).parse(loop_rest)?;
                let (rest, l5) = many0(L5::parse).parse(rest)?;
                loop_rest = rest;
                // loop l1
                let mut loop_l1 = vec![];
                while peek(opt(L1::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, l1) = opt(L1::parse).parse(loop_rest)?;
                    let (rest, c3) = opt(C3::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_l1.push(_310LoopL1 { l1, c3 });
                }
                // loop c8
                let mut loop_c8 = vec![];
                while peek(opt(C8::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, c8) = opt(C8::parse).parse(loop_rest)?;
                    let (rest, c8c) = many0(C8C::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_c8.push(_310LoopC8 { c8, c8c });
                }
                // loop h1
                let mut loop_h1 = vec![];
                while peek(opt(H1::parse)).parse(loop_rest)?.1.is_some() {
                    let (rest, h1) = opt(H1::parse).parse(loop_rest)?;
                    let (rest, h2) = many0(H2::parse).parse(rest)?;
                    loop_rest = rest;
                    loop_h1.push(_310LoopH1 { h1, h2 });
                }
                loop_l0.push(_310LoopL0 {
                    l0,
                    l5,
                    loop_l1: vec![],
                    l7: None,
                    x1: None,
                    x2: None,
                    loop_c8,
                    loop_h1,
                });
            }
            loop_lx.push(_310LoopLX {
                lx,
                loop_n7,
                loop_l0,
            });
        }
        let rest = loop_rest;
        let (rest, l3) = L3::parse(rest)?;
        let (rest, pwk) = many0(PWK::parse).parse(rest)?;
        // loop l1
        let mut loop_l1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(L1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, l1) = opt(L1::parse).parse(loop_rest)?;
            let (rest, c3) = opt(C3::parse).parse(rest)?;
            loop_rest = rest;
            loop_l1.push(_310LoopL1 { l1, c3 });
        }
        let rest = loop_rest;
        let (rest, v9) = many0(V9::parse).parse(rest)?;
        let (rest, c8) = many0(C8::parse).parse(rest)?;
        let (rest, k1_2) = many0(K1::parse).parse(rest)?;
        let (rest, l11) = opt(L11::parse).parse(rest)?;
        let (rest, se) = SE::parse(rest)?;
        let output = _310 {
            st,
            b3,
            b2a,
            y6,
            g3,
            n9,
            v1,
            m0,
            m1,
            c2,
            c3,
            y2,
            loop_n1,
            g61,
            loop_r4,
            r2a,
            r2,
            k1,
            h3,
            l5,
            loop_c8,
            loop_lx,
            l3,
            pwk,
            loop_l1: vec![],
            v9,
            c8,
            k1_2,
            l11,
            se,
        };
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _310LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _310LoopR4 {
    pub r4: R4,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtm: Option<DTM>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _310LoopC8 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c8: Option<C8>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub c8c: Vec<C8C>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _310LoopLX {
    pub lx: LX,
    pub loop_n7: Vec<_310LoopN7>,
    pub loop_l0: Vec<_310LoopL0>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _310LoopL0 {
    pub l0: Option<L0>,
    pub l5: Vec<L5>,
    pub loop_l1: Vec<_310LoopL1>,
    pub l7: Option<L7>,
    pub x1: Option<X1>,
    pub x2: Option<X2>,
    pub loop_c8: Vec<_310LoopC8>,
    pub loop_h1: Vec<_310LoopH1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _310LoopL1 {
    pub l1: Option<L1>,
    pub c3: Option<C3>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _310LoopH1 {
    pub h1: Option<H1>,
    pub h2: Vec<H2>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
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

impl<'a> Parser<&'a str, _315, nom::error::Error<&'a str>> for _315 {
    fn parse(input: &'a str) -> IResult<&'a str, _315> {
        let mut output = _315::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = B4::parse(rest)?;
        output.b4 = obj;
        let (rest, obj) = many0(N9::parse).parse(rest)?;
        output.n9 = obj;
        let (rest, obj) = opt(Q2::parse).parse(rest)?;
        output.q2 = obj;
        let (rest, obj) = many0(SG::parse).parse(rest)?;
        output.sg = obj;
        // loop r4
        let mut loop_r4 = vec![];
        let mut loop_rest = rest;
        while peek(opt(R4::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, r4) = R4::parse(loop_rest)?;
            let (rest, dtm) = many0(DTM::parse).parse(rest)?;
            loop_rest = rest;
            loop_r4.push(_315LoopR4 { r4, dtm });
        }
        let rest = loop_rest;
        output.loop_r4 = loop_r4;
        let (rest, obj) = opt(V9::parse).parse(rest)?;
        output.v9 = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _315LoopR4 {
    pub r4: R4,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtm: Vec<DTM>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _322 {
    pub st: ST,
    pub zc1: Option<ZC1>,
    pub q5: Q5,
    pub loop_n7: Vec<_322LoopN7>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _322, nom::error::Error<&'a str>> for _322 {
    fn parse(input: &'a str) -> IResult<&'a str, _322> {
        let mut output = _322::default();
        println!("before ST");
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        println!("after ST");
        let (rest, obj) = opt(ZC1::parse).parse(rest)?;
        output.zc1 = obj;
        println!("before Q5");
        let (rest, obj) = Q5::parse(rest)?;
        output.q5 = obj;
        println!("after Q5");
        // loop n7
        let mut loop_n7 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N7::parse)).parse(loop_rest)?.1.is_some() {
            println!("loop n7");
            let (rest, n7) = N7::parse(loop_rest)?;
            let (rest, v4) = opt(V4::parse).parse(rest)?;
            let (rest, dtm) = many0(DTM::parse).parse(rest)?;
            let (rest, m7) = many0(M7::parse).parse(rest)?;
            let (rest, w09) = opt(W09::parse).parse(rest)?;
            let (rest, w2) = opt(W2::parse).parse(rest)?;
            let (rest, na) = many0(NA::parse).parse(rest)?;
            let (rest, gr5) = many0(GR5::parse).parse(rest)?;
            let (rest, y7) = opt(Y7::parse).parse(rest)?;
            let (rest, v1) = opt(V1::parse).parse(rest)?;
            loop_rest = rest;
            // loop r4
            let mut loop_r4 = vec![];
            while peek(opt(R4::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, r4) = R4::parse(loop_rest)?;
                let (rest, dtm) = many0(DTM::parse).parse(rest)?;
                loop_rest = rest;
                loop_r4.push(_322LoopR4 { r4, dtm })
            }
            let rest = loop_rest;
            let (rest, h3) = many0(H3::parse).parse(rest)?;
            loop_rest = rest;
            // loop n1
            let mut loop_n1 = vec![];
            while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, n1) = N1::parse(loop_rest)?;
                let (rest, n3) = many0(N3::parse).parse(rest)?;
                let (rest, n4) = opt(N4::parse).parse(rest)?;
                loop_rest = rest;
                loop_n1.push(_322LoopN1 { n1, n3, n4 })
            }
            let rest = loop_rest;
            let (rest, k1) = many0(K1::parse).parse(rest)?;
            let (rest, n9) = many0(N9::parse).parse(rest)?;
            // loop l0
            let (rest, l3) = many0(L3::parse).parse(rest)?;
            loop_rest = rest;
            loop_n7.push(_322LoopN7 {
                n7,
                v4,
                dtm,
                m7,
                w09,
                w2,
                na,
                gr5,
                y7,
                v1,
                loop_r4: vec![],
                h3,
                loop_n1: vec![],
                k1,
                n9,
                loop_l0: vec![],
                l3,
            });
        }
        let rest = loop_rest;
        output.loop_n7 = loop_n7;
        println!("before SE");
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _322LoopN7 {
    pub n7: N7,
    pub v4: Option<V4>,
    pub dtm: Vec<DTM>,
    pub m7: Vec<M7>,
    pub w09: Option<W09>,
    pub w2: Option<W2>,
    pub na: Vec<NA>,
    pub gr5: Vec<GR5>,
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _322LoopR4 {
    r4: R4,
    #[serde(default)]
    dtm: Vec<DTM>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _322LoopN1 {
    n1: N1,
    n3: Vec<N3>,
    n4: Option<N4>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404 {
    pub st: ST,
    pub zc1: Option<ZC1>,
    pub bx: Option<BX>,
    pub bnx: Option<BNX>,
    pub m3: M3,
    pub n9: Vec<N9>,
    pub cm: Vec<CM>,
    pub m1: Option<M1>,
    pub dtm: Option<DTM>,
    pub loop_n7: Vec<_404LoopN7>,
    pub na: Option<NA>,
    pub f9: F9,
    pub d9: D9,
    pub loop_n1: Vec<_404LoopN1>,
    pub loop_s1: Vec<_404LoopS1>,
    pub r2: Vec<R2>,
    pub r9: Option<R9>,
    pub loop_e1: Vec<_404LoopE1>,
    pub h3: Vec<H3>,
    pub ps: Vec<PS>,
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

impl<'a> Parser<&'a str, _404, nom::error::Error<&'a str>> for _404 {
    fn parse(input: &'a str) -> IResult<&'a str, _404> {
        let mut output = _404::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = opt(ZC1::parse).parse(rest)?;
        output.zc1 = obj;
        let (rest, obj) = opt(BX::parse).parse(rest)?;
        output.bx = obj;
        let (rest, obj) = opt(BNX::parse).parse(rest)?;
        output.bnx = obj;
        let (rest, obj) = M3::parse(rest)?;
        output.m3 = obj;
        let (rest, obj) = many0(N9::parse).parse(rest)?;
        output.n9 = obj;
        let (rest, obj) = many0(CM::parse).parse(rest)?;
        output.cm = obj;
        let (rest, obj) = opt(M1::parse).parse(rest)?;
        output.m1 = obj;
        let (rest, obj) = opt(DTM::parse).parse(rest)?;
        output.dtm = obj;
        // loop n7
        let mut loop_n7 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N7::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n7) = N7::parse(loop_rest)?;
            let (rest, em) = opt(EM::parse).parse(rest)?;
            let (rest, m7) = opt(M7::parse).parse(rest)?;
            let (rest, n5) = opt(N5::parse).parse(rest)?;
            let (rest, ic) = opt(IC::parse).parse(rest)?;
            let (rest, im) = opt(IM::parse).parse(rest)?;
            let (rest, m12) = opt(M12::parse).parse(rest)?;
            let (rest, ga) = opt(GA::parse).parse(rest)?;
            loop_rest = rest;
            loop_n7.push(_404LoopN7 {
                n7,
                em,
                loop_vc: vec![],
                m7,
                n5,
                ic,
                im,
                m12,
                loop_e1: vec![],
                ga,
                loop_ref: vec![],
            });
        }
        let rest = loop_rest;
        output.loop_n7 = loop_n7;
        let (rest, obj) = opt(NA::parse).parse(rest)?;
        output.na = obj;
        let (rest, obj) = F9::parse(rest)?;
        output.f9 = obj;
        let (rest, obj) = D9::parse(rest)?;
        output.d9 = obj;
        // loop n1
        let mut loop_n1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = opt(N2::parse).parse(rest)?;
            let (rest, n3) = opt(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, r#ref) = opt(REF::parse).parse(rest)?;
            let (rest, per) = opt(PER::parse).parse(rest)?;
            let (rest, bl) = opt(BL::parse).parse(rest)?;
            loop_rest = rest;
            loop_n1.push(_404LoopN1 {
                n1,
                n2,
                n3,
                n4,
                r#ref,
                per,
                bl,
            });
        }
        let rest = loop_rest;
        output.loop_n1 = loop_n1;
        let (rest, obj) = many0(R2::parse).parse(rest)?;
        output.r2 = obj;
        let (rest, obj) = opt(R9::parse).parse(rest)?;
        output.r9 = obj;
        let (rest, obj) = many0(H3::parse).parse(rest)?;
        output.h3 = obj;
        let (rest, obj) = many0(PS::parse).parse(rest)?;
        output.ps = obj;
        // loop lx
        let mut loop_lx = vec![];
        let mut loop_rest = rest;
        while peek(opt(LX::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, lx) = LX::parse(loop_rest)?;
            let (rest, l5) = L5::parse(rest)?;
            let (rest, x1) = opt(X1::parse).parse(rest)?;
            loop_rest = rest;
            // loop l0
            let mut loop_l0 = vec![];
            while peek(opt(L0::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, l0) = opt(L0::parse).parse(loop_rest)?;
                let (rest, mea) = opt(MEA::parse).parse(rest)?;
                let (rest, l1) = opt(L1::parse).parse(rest)?;
                let (rest, pi) = opt(PI::parse).parse(rest)?;
                loop_rest = rest;
                loop_l0.push(_404LoopL0 { l0, mea, l1, pi });
            }
            loop_lx.push(_404LoopLX {
                lx,
                l5,
                loop_l0,
                x1,
            });
        }
        output.loop_lx = loop_lx;
        let rest = loop_rest;
        let (rest, obj) = opt(L3::parse).parse(rest)?;
        output.l3 = obj;
        let (rest, obj) = opt(LS::parse).parse(rest)?;
        output.ls = obj;
        // loop lh1
        let mut loop_lh1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(LH1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, lh1) = opt(LH1::parse).parse(loop_rest)?;
            let (rest, lh2) = many0(LH2::parse).parse(rest)?;
            let (rest, lh3) = many0(LH3::parse).parse(rest)?;
            let (rest, lfh) = many0(LFH::parse).parse(rest)?;
            let (rest, lep) = opt(LEP::parse).parse(rest)?;
            let (rest, lh4) = opt(LH4::parse).parse(rest)?;
            let (rest, lht) = opt(LHT::parse).parse(rest)?;
            let (rest, lhr) = opt(LHR::parse).parse(rest)?;
            let (rest, per) = opt(PER::parse).parse(rest)?;
            loop_rest = rest;
            loop_lh1.push(_404LoopLH1 {
                lh1,
                lh2,
                lh3,
                lfh,
                lep,
                lh4,
                lht,
                lhr,
                per,
            });
        }
        output.loop_lh1 = loop_lh1;
        let rest = loop_rest;
        let (rest, obj) = opt(LE::parse).parse(rest)?;
        output.le = obj;
        let (rest, obj) = opt(PER::parse).parse(rest)?;
        output.per = obj;
        let (rest, obj) = opt(LH2::parse).parse(rest)?;
        output.lh2 = obj;
        let (rest, obj) = opt(LHR::parse).parse(rest)?;
        output.lhr = obj;
        let (rest, obj) = opt(LH6::parse).parse(rest)?;
        output.lh6 = obj;
        let (rest, obj) = opt(XH::parse).parse(rest)?;
        output.xh = obj;
        let (rest, obj) = opt(X7::parse).parse(rest)?;
        output.x7 = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopN7Ref {
    pub _ref: Option<REF>,
    pub n10: Option<N10>,
    pub loop_n1: Vec<_404LoopN7RefN1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopN7RefN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopVC {
    pub vc: Option<VC>,
    pub loop_n1: Vec<_404LoopVcN1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopVcN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub h3: Option<H3>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub r#ref: Option<REF>,
    pub per: Option<PER>,
    pub bl: Option<BL>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopN7E1 {
    pub e1: E1,
    pub e4: Option<E4>,
    pub e5: Option<E5>,
    pub pi: Option<PI>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopE1 {
    pub e1: E1,
    pub e4: Option<E4>,
    pub e5: Option<E5>,
    pub pi: Option<PI>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopL0 {
    pub l0: Option<L0>,
    pub mea: Option<MEA>,
    pub l1: Option<L1>,
    pub pi: Option<PI>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopT1 {
    pub t1: Option<T1>,
    pub t2: Option<T2>,
    pub t3: Option<T3>,
    pub t6: Option<T6>,
    pub t8: Option<T8>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopLH1 {
    pub lh1: Option<LH1>,
    pub lh2: Vec<LH2>,
    pub lh3: Vec<LH3>,
    pub lfh: Vec<LFH>,
    pub lep: Option<LEP>,
    pub lh4: Option<LH4>,
    pub lht: Option<LHT>,
    pub lhr: Option<LHR>,
    pub per: Option<PER>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopLX {
    pub lx: LX,
    pub l5: L5,
    pub loop_l0: Vec<_404LoopL0>,
    pub x1: Option<X1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopRef {
    pub _ref: Option<REF>,
    pub n10: Option<N10>,
    pub loop_n1: Vec<_404LoopRefN1>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct _404LoopRefN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}

/// 810 - Invoice
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810 {
    pub st: ST,
    pub big: BIG,
    pub nte: Vec<NTE>,
    pub cur: Option<CUR>,
    pub r#ref: Vec<REF>,
    pub ynq: Vec<YNQ>,
    pub per: Vec<PER>,
    pub loop_n1: Vec<_810LoopN1>,
    pub itd: Vec<ITD>,
    pub dtm: Vec<DTM>,
    pub fob: Option<FOB>,
    pub pid: Vec<PID>,
    pub mea: Vec<MEA>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub l7: Option<L7>,
    pub bal: Vec<BAL>,
    pub inc: Option<INC>,
    pub pam: Vec<PAM>,
    pub loop_lm: Vec<_810LoopLM>,
    pub loop_n9: Vec<_810LoopN9>,
    pub loop_v1: Vec<_810LoopV1>,
    pub loop_fa1: Vec<_810LoopFA1>,
    pub loop_it1: Vec<_810LoopIT1>,
    pub tds: TDS,
    pub txi: Vec<TXI>,
    pub cad: Option<CAD>,
    pub amt: Vec<AMT>,
    pub loop_sac: Vec<_810LoopSAC>,
    pub loop_iss: Vec<_810LoopISS>,
    pub ctt: Option<CTT>,
    pub se: SE,
}

impl<'a> Parser<&'a str, _810, nom::error::Error<&'a str>> for _810 {
    fn parse(input: &'a str) -> IResult<&'a str, _810> {
        let mut output = _810::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = BIG::parse(rest)?;
        output.big = obj;
        let (rest, obj) = many0(NTE::parse).parse(rest)?;
        output.nte = obj;
        let (rest, obj) = opt(CUR::parse).parse(rest)?;
        output.cur = obj;
        let (rest, obj) = many0(REF::parse).parse(rest)?;
        output.r#ref = obj;
        let (rest, obj) = many0(YNQ::parse).parse(rest)?;
        output.ynq = obj;
        let (rest, obj) = many0(PER::parse).parse(rest)?;
        output.per = obj;
        // loop n1
        let mut loop_n1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(N1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n1) = N1::parse(loop_rest)?;
            let (rest, n2) = many0(N2::parse).parse(rest)?;
            let (rest, n3) = many0(N3::parse).parse(rest)?;
            let (rest, n4) = opt(N4::parse).parse(rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            let (rest, per) = many0(PER::parse).parse(rest)?;
            let (rest, dmg) = opt(DMG::parse).parse(rest)?;
            loop_rest = rest;
            loop_n1.push(_810LoopN1 {
                n1,
                n2,
                n3,
                n4,
                r#ref,
                per,
                dmg,
            });
        }
        let rest = loop_rest;
        output.loop_n1 = loop_n1;
        let (rest, obj) = many0(ITD::parse).parse(rest)?;
        output.itd = obj;
        let (rest, obj) = many0(DTM::parse).parse(rest)?;
        output.dtm = obj;
        let (rest, obj) = opt(FOB::parse).parse(rest)?;
        output.fob = obj;
        let (rest, obj) = many0(PID::parse).parse(rest)?;
        output.pid = obj;
        let (rest, obj) = many0(MEA::parse).parse(rest)?;
        output.mea = obj;
        let (rest, obj) = many0(PWK::parse).parse(rest)?;
        output.pwk = obj;
        let (rest, obj) = many0(PKG::parse).parse(rest)?;
        output.pkg = obj;
        let (rest, obj) = opt(L7::parse).parse(rest)?;
        output.l7 = obj;
        let (rest, obj) = many0(BAL::parse).parse(rest)?;
        output.bal = obj;
        let (rest, obj) = opt(INC::parse).parse(rest)?;
        output.inc = obj;
        let (rest, obj) = many0(PAM::parse).parse(rest)?;
        output.pam = obj;
        // loop lm
        let mut loop_lm = vec![];
        loop_rest = rest;
        while peek(opt(LM::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, lm) = LM::parse(loop_rest)?;
            let (rest, lq) = LQ::parse(rest)?;
            loop_rest = rest;
            loop_lm.push(_810LoopLM { lm, lq });
        }
        let rest = loop_rest;
        output.loop_lm = loop_lm;
        // loop n9
        let mut loop_n9 = vec![];
        loop_rest = rest;
        while peek(opt(N9::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, n9) = N9::parse(loop_rest)?;
            let (rest, msg) = many0(MSG::parse).parse(rest)?;
            loop_rest = rest;
            loop_n9.push(_810LoopN9 { n9, msg });
        }
        let rest = loop_rest;
        output.loop_n9 = loop_n9;
        // loop v1
        let mut loop_v1 = vec![];
        loop_rest = rest;
        while peek(opt(V1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, v1) = V1::parse(loop_rest)?;
            let (rest, r4) = many0(R4::parse).parse(rest)?;
            let (rest, dtm) = many0(DTM::parse).parse(rest)?;
            loop_rest = rest;
            loop_v1.push(_810LoopV1 { v1, r4, dtm });
        }
        let rest = loop_rest;
        output.loop_v1 = loop_v1;
        // loop fa1
        let mut loop_fa1 = vec![];
        loop_rest = rest;
        while peek(opt(FA1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, fa1) = FA1::parse(loop_rest)?;
            let (rest, fa2) = many0(FA2::parse).parse(rest)?;
            loop_rest = rest;
            loop_fa1.push(_810LoopFA1 { fa1, fa2 });
        }
        let rest = loop_rest;
        output.loop_fa1 = loop_fa1;
        // loop it1
        let mut loop_it1 = vec![];
        loop_rest = rest;
        while peek(opt(IT1::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, it1) = IT1::parse(loop_rest)?;
            let (rest, crc) = opt(CRC::parse).parse(rest)?;
            let (rest, qty) = many0(QTY::parse).parse(rest)?;
            let (rest, cur) = opt(CUR::parse).parse(rest)?;
            let (rest, it3) = many0(IT3::parse).parse(rest)?;
            let (rest, txi) = many0(TXI::parse).parse(rest)?;
            let (rest, ctp) = many0(CTP::parse).parse(rest)?;
            let (rest, pam) = many0(PAM::parse).parse(rest)?;
            let (rest, mea) = many0(MEA::parse).parse(rest)?;
            // loop_pid
            let mut loop_pid = vec![];
            loop_rest = rest;
            while peek(opt(PID::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, pid) = PID::parse(loop_rest)?;
                let (rest, mea) = many0(MEA::parse).parse(rest)?;
                loop_rest = rest;
                loop_pid.push(_810LoopPID { pid, mea });
            }
            let rest = loop_rest;
            let (rest, pwk) = many0(PWK::parse).parse(rest)?;
            let (rest, pkg) = many0(PKG::parse).parse(rest)?;
            let (rest, po4) = opt(PO4::parse).parse(rest)?;
            let (rest, itd) = many0(ITD::parse).parse(rest)?;
            let (rest, r#ref) = many0(REF::parse).parse(rest)?;
            let (rest, ynq) = many0(YNQ::parse).parse(rest)?;
            let (rest, per) = many0(PER::parse).parse(rest)?;
            let (rest, sdq) = many0(SDQ::parse).parse(rest)?;
            let (rest, dtm) = many0(DTM::parse).parse(rest)?;
            let (rest, cad) = many0(CAD::parse).parse(rest)?;
            let (rest, l7) = many0(L7::parse).parse(rest)?;
            let (mut rest, sr) = opt(SR::parse).parse(rest)?;
            // loop_sln
            let mut loop_sln = vec![];
            loop_rest = rest;
            while peek(opt(SLN::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, sln) = SLN::parse(loop_rest)?;
                let (rest, dtm) = opt(DTM::parse).parse(rest)?;
                let (rest, r#ref) = many0(REF::parse).parse(rest)?;
                let (rest, pid) = many0(PID::parse).parse(rest)?;
                let (rest, sac) = many0(SAC::parse).parse(rest)?;
                let (rest, tc2) = many0(TC2::parse).parse(rest)?;
                let (rest, txi) = many0(TXI::parse).parse(rest)?;
                loop_rest = rest;
                loop_sln.push(_810LoopSLN {
                    sln,
                    dtm,
                    r#ref,
                    pid,
                    sac,
                    tc2,
                    txi,
                });
            }
            std::mem::swap(&mut rest, &mut loop_rest);
            loop_rest = rest;
            loop_it1.push(_810LoopIT1 {
                it1,
                crc,
                qty,
                cur,
                it3,
                txi,
                ctp,
                pam,
                mea,
                loop_pid,
                pwk,
                pkg,
                po4,
                itd,
                r#ref,
                ynq,
                per,
                sdq,
                dtm,
                cad,
                l7,
                sr,
                loop_sac: vec![],
                loop_sln,
                loop_n1: vec![],
                loop_lm: vec![],
                loop_v1: vec![],
                loop_fa1: vec![],
            });
        }
        let rest = loop_rest;
        output.loop_it1 = loop_it1;
        let (rest, obj) = TDS::parse(rest)?;
        output.tds = obj;
        let (rest, obj) = many0(TXI::parse).parse(rest)?;
        output.txi = obj;
        let (rest, obj) = opt(CAD::parse).parse(rest)?;
        output.cad = obj;
        let (rest, obj) = many0(AMT::parse).parse(rest)?;
        output.amt = obj;
        // loop sac
        // loop iss
        let (rest, obj) = opt(CTT::parse).parse(rest)?;
        output.ctt = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810LoopN1 {
    pub n1: N1,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub r#ref: Vec<REF>,
    pub per: Vec<PER>,
    pub dmg: Option<DMG>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810LoopLM {
    pub lm: LM,
    pub lq: LQ,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810LoopN9 {
    pub n9: N9,
    pub msg: Vec<MSG>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810LoopV1 {
    pub v1: V1,
    pub r4: Vec<R4>,
    pub dtm: Vec<DTM>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810LoopFA1 {
    pub fa1: FA1,
    pub fa2: Vec<FA2>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810LoopIT1 {
    pub it1: IT1,
    pub crc: Option<CRC>,
    pub qty: Vec<QTY>,
    pub cur: Option<CUR>,
    pub it3: Vec<IT3>,
    pub txi: Vec<TXI>,
    pub ctp: Vec<CTP>,
    pub pam: Vec<PAM>,
    pub mea: Vec<MEA>,
    pub loop_pid: Vec<_810LoopPID>,
    pub pwk: Vec<PWK>,
    pub pkg: Vec<PKG>,
    pub po4: Option<PO4>,
    pub itd: Vec<ITD>,
    pub r#ref: Vec<REF>,
    pub ynq: Vec<YNQ>,
    pub per: Vec<PER>,
    pub sdq: Vec<SDQ>,
    pub dtm: Vec<DTM>,
    pub cad: Vec<CAD>,
    pub l7: Vec<L7>,
    pub sr: Option<SR>,
    pub loop_sac: Vec<_810LoopSAC>,
    pub loop_sln: Vec<_810LoopSLN>,
    pub loop_n1: Vec<_810LoopN1>,
    pub loop_lm: Vec<_810LoopLM>,
    pub loop_v1: Vec<_810LoopV1>,
    pub loop_fa1: Vec<_810LoopFA1>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810LoopPID {
    pub pid: PID,
    pub mea: Vec<MEA>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810LoopSAC {
    pub sac: SAC,
    pub txi: Vec<TXI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810LoopSLN {
    pub sln: SLN,
    pub dtm: Option<DTM>,
    pub r#ref: Vec<REF>,
    pub pid: Vec<PID>,
    pub sac: Vec<SAC>,
    pub tc2: Vec<TC2>,
    pub txi: Vec<TXI>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _810LoopISS {
    pub iss: ISS,
    pub pid: Option<PID>,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _997 {
    pub st: ST,
    pub ak1: AK1,
    pub loop_ak2: Vec<_997LoopAK2>,
    pub ak9: AK9,
    pub se: SE,
}

impl<'a> Parser<&'a str, _997, nom::error::Error<&'a str>> for _997 {
    fn parse(input: &'a str) -> IResult<&'a str, _997> {
        let mut output = _997::default();
        let (rest, obj) = ST::parse(input)?;
        output.st = obj;
        let (rest, obj) = AK1::parse(rest)?;
        output.ak1 = obj;
        // loop ak2
        let mut loop_ak2 = vec![];
        let mut loop_rest = rest;
        while peek(opt(AK2::parse)).parse(loop_rest)?.1.is_some() {
            let (rest, ak2) = AK2::parse(loop_rest)?;
            // loop ak3
            loop_rest = rest;
            let mut loop_ak3 = vec![];
            while peek(opt(AK3::parse)).parse(loop_rest)?.1.is_some() {
                let (rest, ak3) = AK3::parse(loop_rest)?;
                let (rest, ak4) = many0(AK4::parse).parse(rest)?;
                loop_rest = rest;
                loop_ak3.push(_997LoopAK3 { ak3, ak4 });
            }
            let rest = loop_rest;
            let (rest, ak5) = AK5::parse(rest)?;
            loop_rest = rest;
            loop_ak2.push(_997LoopAK2 { ak2, loop_ak3, ak5 });
        }
        output.loop_ak2 = loop_ak2;
        let rest = loop_rest;
        let (rest, obj) = AK9::parse(rest)?;
        output.ak9 = obj;
        let (rest, obj) = SE::parse(rest)?;
        output.se = obj;
        Ok((rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _997LoopAK2 {
    pub ak2: AK2,
    pub loop_ak3: Vec<_997LoopAK3>,
    pub ak5: AK5,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _997LoopAK3 {
    pub ak3: AK3,
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _998 {
    pub st: ST,
    pub zd: ZD,
    pub se: SE,
}

impl<'a> Parser<&'a str, _998, nom::error::Error<&'a str>> for _998 {
    fn parse(input: &'a str) -> IResult<&'a str, _998> {
        let mut output = _998::default();
        let (input, obj) = ST::parse(input)?;
        output.st = obj;
        let (input, obj) = ZD::parse(input)?;
        output.zd = obj;
        let (input, obj) = SE::parse(input)?;
        output.se = obj;
        Ok((input, output))
    }
}
