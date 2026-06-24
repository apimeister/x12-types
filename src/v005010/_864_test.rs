use crate::util::Parser;
use crate::v005010::*;

// Text message: heading date and party loop, then a MIT message loop carrying its own
// party loop and the message text lines.
const SAMPLE: &str = r#"ST*864*0001~
BMG*00*Subject Line~
DTM*097*20200101~
N1*FR*SENDER*92*SND01~
N4*DALLAS*TX*75201~
REF*ZZ*REF1~
MIT*1*Message Subject~
N1*TO*RECIPIENT*92*RCP01~
MSG*Line one of the message~
MSG*Line two of the message~
SE*11*0001~"#;

#[test]
fn parse_864() {
    let (rest, obj) = _864::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "864");
    assert_eq!(obj.bmg._01, "00");
    assert_eq!(obj.dtm.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "FR");
    assert_eq!(obj.loop_n1[0].r#ref.len(), 1);
    assert_eq!(obj.loop_mit.len(), 1);
    let mit = &obj.loop_mit[0];
    assert_eq!(mit.mit._01, "1");
    assert_eq!(mit.loop_n1.len(), 1);
    assert_eq!(mit.loop_n1[0].n1._01.to_string(), "TO");
    assert_eq!(mit.msg.len(), 2);
}

#[test]
fn roundtrip_864() {
    let (_, a) = _864::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _864::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_864() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*TX*S*R*20200101*1200*1*X*005010~
ST*864*0001~
BMG*00*Message~
MIT*1*Subject~
MSG*Line one~
MSG*Line two~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_864>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_mit.len(), 1);
    assert_eq!(t.loop_mit[0].msg.len(), 2);
}
