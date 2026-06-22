use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*940*0001~W05*N*0001~LX*1~LX*2~SE*4*0001~";

#[test]
fn parse_940() {
    let (rest, obj) = _940::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj._010._01, "940");
    assert_eq!(obj._020._01, "N");
    // two LX detail loops, both top-level
    assert_eq!(obj._0300_loop.len(), 2);
    assert_eq!(obj._0300_loop[0]._005._01, "1");
    assert_eq!(obj._0300_loop[1]._005._01, "2");
}

#[test]
fn roundtrip_940() {
    let (_, first) = _940::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _940::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_940() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*OW*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*940*0001~
W05*N*0001~
LX*1~
SE*3*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_940>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0]._020._01, "N");
}
