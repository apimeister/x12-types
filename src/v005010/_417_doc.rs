use super::segment::*;
use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplayX12, ParseX12};

/// 417 - Rail Carrier Waybill Interchange
///
/// This X12 Transaction Set contains the format and establishes the data contents of the Rail Carrier Waybill Interchange Transaction Set (417) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set is used by rail carriers to interchange waybill information between carriers participating in the movement of a rail shipment.
///
/// Heading: ST, ZC1, BX, BNX, N9, CM, DTM, then the N7 equipment loop.
/// Then N8 waybill references, N8A, V9, F9/D9, an N1 party loop, an S1 stop-off loop, R2/R9
/// routing, an E1 empty-car loop, H3/PS, an LX line loop (L5 + L0), a T1 transit loop, and
/// an LH1 hazardous-material loop. Summary: SE.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417 {
    pub st: ST,
    pub zc1: Option<ZC1>,
    pub bx: Option<BX>,
    pub bnx: Option<BNX>,
    pub n9: Vec<N9>,
    pub cm: Vec<CM>,
    pub dtm: Option<DTM>,
    #[x12(loop_trigger = "N7")]
    pub loop_n7: Vec<_417LoopN7>,
    pub n8: Vec<N8>,
    pub n8a: Vec<N8A>,
    pub v9: Option<V9>,
    pub f9: F9,
    pub d9: D9,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_417LoopN1>,
    #[x12(loop_trigger = "S1")]
    pub loop_s1: Vec<_417LoopS1>,
    pub r2: Vec<R2>,
    pub r9: Option<R9>,
    #[x12(loop_trigger = "E1")]
    pub loop_e1: Vec<_417LoopE1>,
    pub h3: Vec<H3>,
    pub ps: Vec<PS>,
    #[x12(loop_trigger = "LX")]
    pub loop_lx: Vec<_417LoopLX>,
    #[x12(loop_trigger = "T1")]
    pub loop_t1: Vec<_417LoopT1>,
    pub ls: Option<LS>,
    #[x12(loop_trigger = "LH1")]
    pub loop_lh1: Vec<_417LoopLH1>,
    pub le: Option<LE>,
    pub per: Option<PER>,
    pub lh2: Option<LH2>,
    pub lhr: Option<LHR>,
    pub xh: Option<XH>,
    pub x7: Option<X7>,
    pub se: SE,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopN7 {
    pub n7: N7,
    pub em: Option<EM>,
    #[x12(loop_trigger = "VC")]
    pub loop_vc: Vec<_417LoopVC>,
    pub ic: Option<IC>,
    pub im: Option<IM>,
    pub m12: Vec<M12>,
    pub g4: Option<G4>,
    pub m7: Vec<M7>,
    pub n5: Option<N5>,
    pub h5: Option<H5>,
    #[x12(loop_trigger = "E1")]
    pub loop_e1: Vec<_417LoopN7E1>,
    pub ga: Option<GA>,
    #[x12(loop_trigger = "REF")]
    pub loop_ref: Vec<_417LoopN7Ref>,
    pub ima: Vec<IMA>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopVC {
    pub vc: Option<VC>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_417LoopVcN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopVcN1 {
    pub n1: Option<N1>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub h3: Option<H3>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopN7E1 {
    pub e1: E1,
    pub e4: Option<E4>,
    pub e5: Option<E5>,
    pub pi: Option<PI>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopN7Ref {
    pub r#ref: Option<REF>,
    pub n10: Option<N10>,
    #[x12(loop_trigger = "N1")]
    pub loop_n1: Vec<_417LoopN7RefN1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopN7RefN1 {
    pub n1: Option<N1>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopN1 {
    pub n1: N1,
    pub n2: Option<N2>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub r#ref: Option<REF>,
    pub per: Option<PER>,
    pub bl: Option<BL>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopS1 {
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
pub struct _417LoopE1 {
    pub e1: E1,
    pub e4: Option<E4>,
    pub e5: Option<E5>,
    pub pi: Option<PI>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopLX {
    pub lx: LX,
    pub l5: Vec<L5>,
    #[x12(loop_trigger = "L0")]
    pub loop_l0: Vec<_417LoopL0>,
    pub x1: Option<X1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopL0 {
    pub l0: Option<L0>,
    pub mea: Option<MEA>,
    pub pi: Vec<PI>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopT1 {
    pub t1: Option<T1>,
    pub t2: Option<T2>,
    pub t3: Option<T3>,
    pub t6: Option<T6>,
    pub t8: Option<T8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopLH1 {
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
    pub loop_n1: Vec<_417LoopLh1N1>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12, ParseX12)]
pub struct _417LoopLh1N1 {
    pub n1: Option<N1>,
    pub n3: Vec<N3>,
    pub n4: Option<N4>,
    pub per: Vec<PER>,
}
