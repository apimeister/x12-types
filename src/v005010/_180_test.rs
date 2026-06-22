use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*180*0001~BGN*00*RMA1*20200101~N1*ST*RETURNER~LIN*1*UP*012~LIN*2*UP*013~SE*6*0001~";

#[test]
fn parse_180() {
    let (rest, obj) = _180::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "180");
    assert_eq!(obj.loop_lin.len(), 2);
}

#[test]
fn roundtrip_180() {
    let (_, a) = _180::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _180::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_180() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*RA*S*R*20200101*1200*1*X*005010~\nST*180*0001~\nBGN*00*RMA1*20200101~\nLIN*1*UP*012~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_180>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].loop_lin.len(), 1);
}
