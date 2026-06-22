use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 856 - Ship Notice/Manifest
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Ship Notice/Manifest Transaction Set (856) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to notify a trading partner that a shipment has been or will be sent. The transaction set enables the sender to describe the contents and configuration of a shipment in various levels of detail and provides an organized flexibility that allows both the sender and receiver to carry out automated business processes.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1
/// 0020 | BSN | Beginning Segment for Ship Notice | M | 1
/// 0030 | DTM | Date/Time Reference | O | 10
/// LOOP ID - HL | 200000
/// HL -> 0040 | HL | Hierarchical Level | M | 1
/// HL -> 0050 | TD1 | Carrier Details (Routing Sequence/Transit Time) | O | 20
/// HL -> 0060 | TD5 | Carrier Details (Routing Sequence/Transit Time) | O | 12
/// HL -> 0070 | TD3 | Carrier Details (Equipment) | O | 12
/// HL -> 0080 | TD4 | Carrier Details (Special Handling, or Hazardous Materials, or Both) | O | 5
/// HL -> 0090 | REF | Reference Identification | O | 200
/// HL -> 0100 | DTM | Date/Time Reference | O | 10
/// HL -> 0110 | FOB | F.O.B. Related Instructions | O | 1
/// HL -> 0120 | PKG | Marking, Packaging, Loading | O | 25
/// HL -> 0130 | L5 | Description, Marks and Numbers | O | 999
/// HL -> 0140 | H1 | Hardship Exemption | O | 1
/// HL -> 0150 | H2 | Additional Reference Information | O | 6
/// HL -> 0160 | H3 | Special Handling Instructions | O | 6
/// HL -> 0170 | L0 | Line Item - Quantity and Weight | O | 1
/// HL -> 0180 | L1 | Rate and Charges | O | 1
/// HL -> 0190 | L4 | Description, Marks and Numbers | O | 1
/// HL -> 0200 | L7 | Tariff Reference | O | 1
/// HL -> 0210 | L9 | Charge Detail | O | 1
/// HL -> 0220 | LH1 | Hazardous Identification Information | O | 1
/// HL -> 0230 | LH2 | Hazardous Classification Information | O | 4
/// HL -> 0240 | LH3 | Hazardous Material Shipping Name | O | 10
/// HL -> 0250 | LFH | Free Form Hazardous Material Information | O | 20
/// HL -> 0260 | LEP | Hazardous Material Packing Group | O | 3
/// HL -> 0270 | LH4 | Hazardous Material Shipment Information | O | 1
/// HL -> 0280 | LHT | Hazardous Material Identifying Reference Numbers | O | 3
/// HL -> 0290 | LHR | Hazardous Material Reportable Quantity | O | 1
/// HL -> 0300 | PER | Administrative Communications Contact | O | 3
/// HL -> 0310 | N1 | Name | O | 200
/// HL -> 0320 | N2 | Additional Name Information | O | 2
/// HL -> 0330 | N3 | Address Information | O | 2
/// HL -> 0340 | N4 | Geographic Location | O | 1
/// HL -> 0350 | REF | Reference Identification | O | 12
/// HL -> 0360 | PER | Administrative Communications Contact | O | 3
/// HL -> 0370 | FOB | F.O.B. Related Instructions | O | 1
/// HL -> 0380 | CTT | Transaction Totals | O | 1
/// 0390 | SE | Transaction Set Trailer | M | 1
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856 {
    pub st: ST,
    pub bsn: BSN,
    pub dtm: Vec<DTM>,
    #[x12(loop_trigger = "HL")]
    pub loop_hl: Vec<_856LoopHL>,
    pub se: SE,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _856LoopHL {
    pub hl: HL,
    pub td1: Vec<TD1>,
    pub td5: Vec<TD5>,
    pub td3: Vec<TD3>,
    pub prf: Vec<PRF>,
    pub ref_segments: Vec<REF>,
    pub dtm: Vec<DTM>,
    pub n1: Vec<N1>,
    pub n2: Vec<N2>,
    pub n3: Vec<N3>,
    pub n4: Vec<N4>,
    pub per: Vec<PER>,
    pub man: Vec<MAN>,
    pub mea: Vec<MEA>,
    pub cld: Vec<CLD>,
    pub lin: Vec<LIN>,
    pub sn1: Vec<SN1>,
    pub ctt: Option<CTT>,
}
