use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*214*0001~B10*REF1*SHIP1*SCAC~LX*1~LX*2~SE*4*0001~";

#[test]
fn parse_214() {
    let (rest, obj) = _214::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "214");
    assert_eq!(obj.b10._03.to_string(), "SCAC");
    // two LX loops, both top-level
    assert_eq!(obj.loop_0200.len(), 2);
    assert_eq!(obj.loop_0200[0].lx._01, "1");
    assert_eq!(obj.loop_0200[1].lx._01, "2");
}

#[test]
fn roundtrip_214() {
    let (_, first) = _214::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _214::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_214() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*QM*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*214*0001~
B10*REF1*SHIP1*SCAC~
LX*1~
SE*3*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_214>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        obj.functional_group[0].segments[0].b10._03.to_string(),
        "SCAC"
    );
}
