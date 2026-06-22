use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*846*0001~BIA*00*SI*REF1*20200101~N1*SU*SUPPLIER*92*S1~LIN*1*VN*ABC123~QTY*33*100*EA~LIN*2*VN*DEF456~QTY*33*50*EA~CTT*2~SE*9*0001~";

#[test]
fn parse_846() {
    let (rest, obj) = _846::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "846");
    assert_eq!(obj.bia._01, "00");
    assert_eq!(obj.bia._02, "SI");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01, "SU");
    // two item loops, both top-level, each with a QTY
    assert_eq!(obj.loop_lin.len(), 2);
    assert_eq!(obj.loop_lin[0].lin._02, "VN");
    assert_eq!(obj.loop_lin[0].qty.len(), 1);
    assert_eq!(obj.loop_lin[1].qty.len(), 1);
    assert_eq!(obj.ctt.as_ref().unwrap()._01, "2");
}

#[test]
fn roundtrip_846() {
    let (_, first) = _846::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _846::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_846() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*IB*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*846*0001~
BIA*00*SI*REF1*20200101~
N1*SU*SUPPLIER*92*S1~
LIN*1*VN*ABC123~
QTY*33*100*EA~
CTT*1~
SE*7*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_846>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bia._02, "SI");
}
