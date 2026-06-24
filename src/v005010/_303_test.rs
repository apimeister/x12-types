use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*303*0001~B1*SCAC*BOOK001*20200101*C~Y6*00*AUTHORITY*AUTH123~Y5*BOOK001~V9*CAN~SE*6*0001~";

#[test]
fn parse_303() {
    let (rest, obj) = _303::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "303");
    assert_eq!(obj.b1._02.as_deref(), Some("BOOK001"));
    assert_eq!(obj.y6.len(), 1);
    assert_eq!(obj.y5._01, "BOOK001");
    assert_eq!(obj.v9.len(), 1);
    assert_eq!(obj.v9[0]._01, "CAN");
}

#[test]
fn roundtrip_303() {
    let (_, first) = _303::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _303::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_303() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*RO*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*303*0001~
B1*SCAC*BOOK001*20200101*C~
Y6*00*AUTHORITY*AUTH123~
Y5*BOOK001~
V9*CAN~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_303>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].y5._01, "BOOK001");
}
