use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*870*0001~BSR*1*PO*REF1*20200101~N1*BY*BUYER~HL*1**O~PRF*PO1~HL*2*1*I~SE*7*0001~";

#[test]
fn parse_870() {
    let (rest, obj) = _870::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "870");
    assert_eq!(obj.bsr._01, "1");
    assert_eq!(obj.loop_hl.len(), 2);
    assert_eq!(obj.loop_hl[0].prf.len(), 1);
}

#[test]
fn roundtrip_870() {
    let (_, a) = _870::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _870::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_870() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*RS*S*R*20200101*1200*1*X*005010~\nST*870*0001~\nBSR*1*PO*REF1*20200101~\nHL*1**O~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_870>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bsr._01, "1");
}
