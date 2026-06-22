use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*204*18711~B2*A*B*C*D*E*SCAC~B2A*00~S5*1*LD~S5*2*UL~SE*5*18711~";

#[test]
fn parse_204() {
    let (rest, obj) = _204::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "204");
    assert_eq!(obj.b2a._01, "00");
    // two stop-off loops, both top-level
    assert_eq!(obj.loop_300.len(), 2);
    assert_eq!(obj.loop_300[0].s5._01, "1");
    assert_eq!(obj.loop_300[1].s5._01, "2");
}

#[test]
fn roundtrip_204() {
    let (_, first) = _204::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _204::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_204() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*SM*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*204*18711~
B2*A*B*C*D*E*SCAC~
B2A*00~
S5*1*LD~
SE*4*18711~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_204>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].b2a._01, "00");
}
