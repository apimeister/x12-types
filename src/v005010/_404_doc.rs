use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 404 - Rail Carrier Shipment Information
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Rail Carrier Shipment Information Transaction Set (404) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to transmit rail-carrier-specific bill of lading information to a railroad. It is the initial tender of a shipment between the consignor or its agent and the rail carrier.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
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
    #[x12(loop_trigger = "N7")]
    pub loop_n7: Vec<_404LoopN7>,
    pub na: Option<NA>,
    pub f9: F9,
    pub d9: D9,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_404LoopN1>,
    #[x12(loop_trigger = "S1")]
    pub loop_s1: Vec<_404LoopS1>,
    pub r2: Vec<R2>,
    pub r9: Option<R9>,
    #[x12(loop_trigger = "E1")]
    pub loop_e1: Vec<_404LoopE1>,
    pub h3: Vec<H3>,
    pub ps: Vec<PS>,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_404LoopLX>,
    #[x12(loop_trigger = "T1")]
    pub loop_t1: Vec<_404LoopT1>,
    pub l3: Option<L3>,
    pub ls: Option<LS>,
    #[x12(loop_trigger = "LH1")]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopN7 {
    pub n7: N7,
    pub em: Option<EM>,
    #[x12(loop_trigger = "VC")]
    pub loop_vc: Vec<_404LoopVC>,
    pub m7: Option<M7>,
    pub n5: Option<N5>,
    pub ic: Option<IC>,
    pub im: Option<IM>,
    pub m12: Option<M12>,
    #[x12(loop_trigger = "E1")]
    pub loop_e1: Vec<_404LoopN7E1>,
    pub ga: Option<GA>,
    #[x12(loop_trigger = "REF")]
    pub loop_ref: Vec<_404LoopN7Ref>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopN7Ref {
    pub r#ref: Option<REF>,
    pub n10: Option<N10>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_404LoopN7RefN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopN7RefN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopVC {
    pub vc: Option<VC>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_404LoopVcN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopVcN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub h3: Option<H3>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub r#ref: Option<REF>,
    pub per: Option<PER>,
    pub bl: Option<BL>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopN7E1 {
    pub e1: E1,
    pub e4: Option<E4>,
    pub e5: Option<E5>,
    pub pi: Option<PI>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopE1 {
    pub e1: E1,
    pub e4: Option<E4>,
    pub e5: Option<E5>,
    pub pi: Option<PI>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopL0 {
    pub l0: Option<L0>,
    pub mea: Option<MEA>,
    pub l1: Option<L1>,
    #[x12(loop_trigger = "PI")]
    pub loop_pi: Vec<_404LoopL0PI>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopL0PI {
    pub pi: Option<PI>,
    pub cd: Vec<CD>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopT1 {
    pub t1: Option<T1>,
    pub t2: Option<T2>,
    pub t3: Option<T3>,
    pub t6: Option<T6>,
    pub t8: Option<T8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
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
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_404LoopLh1N1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopLh1N1 {
    pub n1: Option<N1>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _404LoopLX {
    pub lx: LX,
    pub l5: L5,
    #[x12(loop_trigger = "L0")]
    pub loop_l0: Vec<_404LoopL0>,
    pub x1: Option<X1>,
}
