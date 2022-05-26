use serde::{Serialize,Deserialize};
mod segment;
pub use segment::*;

#[cfg(test)]
mod test;

#[derive(Serialize,Deserialize)]
pub struct Transmission {
    pub isa: ISA,
    pub functional_group: Vec<FunctionalGroup>,
    pub iea: IEA,
}

#[derive(Serialize,Deserialize)]
pub struct FunctionalGroup {
    pub gs: GS,
    pub segments: Vec<Segments>,
    pub ge: GE,
}

#[derive(Serialize,Deserialize)]
pub enum Segments{
    _315(_315),
}

/// 315 - Status Details (Ocean)
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Status Details (Ocean) Transaction Set (315) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to provide all the information necessary to report status or event details for selected shipments or containers. It is intended to accommodate the details for one status or event associated with many shipments or containers, as well as more than one status or event for one shipment or container.
/// POS	ID	NAME	REQ	MAX	REPEAT
/// 0010	ST	Transaction Set Header	M	1	 
/// 0020	B4	Beginning Segment for Inquiry or Reply	M	1	 
/// 0030	N9	Reference Identification	O	30	 
/// 0040	Q2	Status Details (Ocean)	O	1	 
/// 0050	SG	Shipment Status	O	15	 
/// LOOP ID - R4	20
/// 0060	R4	Port or Terminal	M	1	 
/// 0070	DTM	Date/Time Reference	O	15	 
/// 0080	V9	Event Detail	O	10	 
/// 0090	SE	Transaction Set Trailer	M	1
#[derive(Serialize,Deserialize)]
pub struct _315 {
    st: ST,
    b4: B4,
    n9: Vec<N9>,
    q2: Option<Q2>,
    sg: Vec<SG>,
    loop_r4: Vec<(R4, Option<DTM>)>,
    v9: Option<V9>,
    se: SE,
}