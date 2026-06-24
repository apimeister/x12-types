use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*990*0001~B1*SCAC*SHIP123*20200101*A~N9*BM*BOL1~SE*4*0001~";

#[test]
fn parse_990() {
    let (rest, obj) = _990::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "990");
    assert_eq!(obj.b1._02.as_deref(), Some("SHIP123"));
    assert_eq!(obj.b1._04.as_deref(), Some("A"));
    assert_eq!(obj.n9.len(), 1);
}

#[test]
fn roundtrip_990() {
    let (_, a) = _990::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _990::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_990() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*SR*S*R*20200101*1200*1*X*005010~\nST*990*0001~\nB1*SCAC*SHIP123*20200101*A~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_990>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        obj.functional_group[0].segments[0].b1._02.as_deref(),
        Some("SHIP123")
    );
}
