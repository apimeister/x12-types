use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*860*0001~BCH*01*NE*PO123*0*1*20200101~N1*BY*BUYER*92*B1~N1*SE*SELLER*92*S1~POC*1*CA*5*10*EA*2.50**VN*ABC~POC*2*PR*3*5*EA*1.00**VN*DEF~CTT*2~SE*8*0001~";

#[test]
fn parse_860() {
    let (rest, obj) = _860::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "860");
    assert_eq!(obj.bch._03, "PO123");
    // two name loops and two line-change loops, all top-level
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n1._01, "BY");
    assert_eq!(obj.loop_poc.len(), 2);
    assert_eq!(obj.loop_poc[0].poc._01, Some("1".to_string()));
    assert_eq!(obj.loop_poc[0].poc._02, "CA");
    assert_eq!(obj.loop_poc[1].poc._02, "PR");
    assert_eq!(obj.ctt.as_ref().unwrap()._01, "2");
}

#[test]
fn roundtrip_860() {
    let (_, first) = _860::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _860::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_860() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*PC*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*860*0001~
BCH*01*NE*PO123*0*1*20200101~
N1*BY*BUYER*92*B1~
POC*1*CA*5*10*EA*2.50**VN*ABC~
CTT*1~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_860>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bch._03, "PO123");
}
