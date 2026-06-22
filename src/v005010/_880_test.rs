use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*880*0001~G01*20200101*INV1~N1*ST*STORE~LIN*1*UP*012~LIN*2*UP*013~SE*6*0001~";

#[test]
fn parse_880() {
    let (rest, obj) = _880::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "880");
    assert_eq!(obj.g01._01, "20200101");
    assert_eq!(obj.loop_lin.len(), 2);
}

#[test]
fn roundtrip_880() {
    let (_, a) = _880::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _880::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_880() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*GP*S*R*20200101*1200*1*X*005010~\nST*880*0001~\nG01*20200101*INV1~\nLIN*1*UP*012~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_880>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].g01._01, "20200101");
}
