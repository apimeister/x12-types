use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*315*0001~B4*1~R4*L*UN*USNYC~R4*D*UN*USLAX~SE*5*0001~";

#[test]
fn parse_315() {
    let (rest, obj) = _315::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "315");
    // two port/terminal loops, both top-level
    assert_eq!(obj.loop_r4.len(), 2);
    assert_eq!(obj.loop_r4[0].r4._01, "L");
    assert_eq!(obj.loop_r4[1].r4._01, "D");
}

#[test]
fn roundtrip_315() {
    let (_, first) = _315::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _315::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_315() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*QO*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*315*0001~
B4*1~
R4*L*UN*USNYC~
SE*4*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_315>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].loop_r4.len(), 1);
}
