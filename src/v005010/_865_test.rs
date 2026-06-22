use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*865*0001~BCA*00*AC*PO1*0*1*20200101~N1*SE*SELLER~POC*1*CA*5*10*EA*2.50**VN*ABC~POC*2*PR*3*5*EA*1**VN*DEF~SE*7*0001~";

#[test]
fn parse_865() {
    let (rest, obj) = _865::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "865");
    assert_eq!(obj.bca._01, "00");
    assert_eq!(obj.loop_poc.len(), 2);
    assert_eq!(obj.loop_poc[0].poc._02, "CA");
}

#[test]
fn roundtrip_865() {
    let (_, a) = _865::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _865::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_865() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*CA*S*R*20200101*1200*1*X*005010~\nST*865*0001~\nBCA*00*AC*PO1*0*1*20200101~\nPOC*1*CA*5*10*EA*2.50**VN*ABC~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_865>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bca._01, "00");
}
