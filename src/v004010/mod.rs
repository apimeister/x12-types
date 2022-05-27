//! v004010 repesents all entitties von the 004010 specification.

use serde::{Deserialize, Serialize};
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
    _315(_315),
    _404(_404),
    _997(_997),
    _998(_998),
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
/// 0060 | R4 | Port or Terminal | M | 1
/// 0070 | DTM | Date/Time Reference | O | 15
/// 0080 | V9 | Event Detail | O | 10
/// 0090 | SE | Transaction Set Trailer | M | 1
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct _315 {
    pub st: ST,
    pub b4: B4,
    pub n9: Vec<N9>,
    pub q2: Option<Q2>,
    pub sg: Vec<SG>,
    pub loop_r4: Vec<(R4, Option<DTM>)>,
    pub v9: Option<V9>,
    pub se: SE,
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
    pub loop_n7: Vec<(
        N7,
        Option<EM>,
        // loop_vc
        Vec<(
            Option<VC>,
            // loop_n1
            Vec<(Option<N1>, Option<N3>, Option<N4>, Option<H3>)>,
        )>,
        Option<M7>,
        Option<N5>,
        Option<IC>,
        Option<IM>,
        Option<M12>,
        // loop e1
        Vec<(E1, Option<E4>, Option<E5>, Option<PI>)>,
        Option<GA>,
        // loop ref
        Vec<(
            Option<REF>,
            Option<N10>,
            //loop n1
            Vec<(Option<N1>, Option<N3>, Option<N4>)>,
        )>,
    )>,
    pub na: Option<NA>,
    pub f9: F9,
    pub d9: D9,
    //loop n1
    pub loop_n1: Vec<(
        N1,
        Option<N2>,
        Option<N3>,
        Option<N4>,
        Option<REF>,
        Option<PER>,
        Option<BL>,
    )>,
    // loop s1
    pub loop_s1: Vec<(
        Option<S1>,
        Option<S2>,
        Option<S9>,
        Option<N1>,
        Option<N2>,
        Option<N3>,
        Option<N4>,
        Option<PER>,
    )>,
    pub r2: Option<R2>,
    pub r9: Option<R9>,
    // loop e1
    pub loop_e1: Vec<(E1, Option<E4>, Option<E5>, Option<PI>)>,
    pub h3: Option<H3>,
    pub ps: Option<PS>,
    // loop lx
    pub loop_lx: Vec<(
        LX,
        L5,
        // loop l0
        Vec<(Option<L0>, Option<MEA>, Option<L1>, Option<PI>)>,
        Option<X1>,
    )>,
    // loop t1
    pub loop_t1: Vec<(Option<T1>, Option<T2>, Option<T3>, Option<T6>, Option<T8>)>,
    pub l3: Option<L3>,
    pub ls: Option<LS>,
    // loop lh1
    pub loop_lh1: Vec<(
        Option<LH1>,
        Option<LH2>,
        Option<LH3>,
        Option<LFH>,
        Option<LEP>,
        Option<LH4>,
        Option<LHT>,
        Option<LHR>,
        Option<PER>,
    )>,
    pub le: Option<LE>,
    pub per: Option<PER>,
    pub lh2: Option<LH2>,
    pub lhr: Option<LHR>,
    pub lh6: Option<LH6>,
    pub xh: Option<XH>,
    pub x7: Option<X7>,
    pub se: SE,
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
    pub loop_ak2: Vec<(AK2, Vec<(Option<AK3>, Vec<AK4>)>)>,
    pub ak9: AK9,
    pub se: SE,
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
