use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*816*0001~BGN*00*REF1*20200101~N1*41*ORG1~N1*40*ORG2~SE*5*0001~";

#[test]
fn parse_816() {
    let (rest, obj) = _816::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "816");
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n1._01, "41");
}

#[test]
fn roundtrip_816() {
    let (_, a) = _816::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _816::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_816() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*OR*S*R*20200101*1200*1*X*005010~\nST*816*0001~\nBGN*00*REF1*20200101~\nN1*41*ORG1~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_816>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].loop_n1.len(), 1);
}
