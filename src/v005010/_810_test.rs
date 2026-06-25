use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*810*0001~BIG*20200101*INV123~N1*ST*BUYER~N1*BT*BILLER~IT1*1*10*EA*5**VN*ABC~IT1*2*20*EA*3**VN*DEF~TDS*130~SE*9*0001~";

#[test]
fn parse_810() {
    let (rest, obj) = _810::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "810");
    assert_eq!(obj.big._01.to_string(), "20200101");
    // two name loops and two line-item loops, all top-level
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "ST");
    assert_eq!(obj.loop_n1[1].n1._01.to_string(), "BT");
    assert_eq!(obj.loop_it1.len(), 2);
    assert_eq!(obj.loop_it1[0].it1._01, Some("1".to_string()));
    assert_eq!(obj.loop_it1[1].it1._01, Some("2".to_string()));
    assert_eq!(obj.tds._01.to_string(), "130");
}

#[test]
fn roundtrip_810() {
    let (_, first) = _810::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _810::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_810() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*IN*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*810*0001~
BIG*20200101*INV123~
N1*ST*BUYER~
IT1*1*10*EA*5**VN*ABC~
TDS*130~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_810>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].big._02, "INV123");
}
