use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*310*0001~B3*A*INV*C*PP*CC*20200101*0700*H*N*S*SCAC~N1*SH*SHIP~N1*CN*CONS~L3*100~SE*6*0001~";

#[test]
fn parse_310() {
    let (rest, obj) = _310::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "310");
    assert_eq!(obj.b3._02, "INV");
    // two name loops, both top-level
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "SH");
    assert_eq!(obj.loop_n1[1].n1._01.to_string(), "CN");
}

#[test]
fn roundtrip_310() {
    let (_, first) = _310::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _310::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_310() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*IO*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*310*0001~
B3*A*INV*C*PP*CC*20200101*0700*H*N*S*SCAC~
N1*SH*SHIP~
L3*100~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_310>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].b3._02, "INV");
}
