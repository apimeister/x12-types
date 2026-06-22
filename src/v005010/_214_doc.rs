use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214 {
    pub st: ST,
    pub b10: B10,
    pub l11: Vec<L11>,
    pub man: Vec<MAN>,
    pub k1: Vec<K1>,
    #[x12(loop_trigger = "N1")]
    pub loop_0100: Vec<_214Loop0100>,
    pub ms3: Vec<MS3>,
    #[x12(loop_trigger = "LX")]
    pub loop_0200: Vec<_214Loop0200>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0100 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub g61: Option<G61>,
    pub g62: Option<G62>,
    pub l11: Vec<L11>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0200 {
    pub lx: LX,
    #[x12(loop_trigger = "AT7")]
    pub loop_0205: Vec<_214Loop0205>,
    pub l11: Vec<L11>,
    pub man: Vec<MAN>,
    pub q7: Vec<Q7>,
    pub k1: Vec<K1>,
    pub at5: Vec<AT5>,
    pub at8: Vec<AT8>,
    #[x12(loop_trigger = "CD3")]
    pub loop_0210: Vec<_214Loop0210>,
    #[x12(loop_trigger = "PRF")]
    pub loop_0230: Vec<_214Loop0230>,
    #[x12(loop_trigger = "SPO")]
    pub loop_0250: Vec<_214Loop0250>,
    #[x12(loop_trigger = "EFI|BIN")]
    pub loop_0260: Vec<_214Loop0260>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0205 {
    pub at7: AT7,
    pub ms1: Option<MS1>,
    pub ms2: Option<MS2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0210 {
    pub cd3: Option<CD3>,
    pub l11: Vec<L11>,
    #[x12(loop_trigger = "AT7")]
    pub loop_0215: Vec<_214Loop0210Loop0215>,
    pub nm1: Option<NM1>,
    pub q7: Vec<Q7>,
    pub at8: Option<AT8>,
    pub man: Vec<MAN>,
    #[x12(loop_trigger = "N1")]
    pub loop_0220: Vec<_214Loop0210Loop0220>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0210Loop0215 {
    pub at7: Option<AT7>,
    pub ms1: Option<MS1>,
    pub ms2: Option<MS2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0210Loop0220 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0230 {
    pub prf: Option<PRF>,
    #[x12(loop_trigger = "N1")]
    pub loop_0231: Vec<_214Loop0231>,
    #[x12(loop_trigger = "CD3")]
    pub loop_0233: Vec<_214Loop0233>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0231 {
    pub n1: Option<N1>,
    pub n2: Option<N2>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub l11: Vec<L11>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0233 {
    pub cd3: Option<CD3>,
    pub l11: Vec<L11>,
    #[x12(loop_trigger = "AT7")]
    pub loop_0240: Vec<_214Loop0240>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0240 {
    pub at7: Option<AT7>,
    pub ms1: Option<MS1>,
    pub ms2: Option<MS2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0250 {
    pub spo: Option<SPO>,
    pub sdq: Option<SDQ>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _214Loop0260 {
    pub efi: Option<EFI>,
    pub bin: BIN,
}
