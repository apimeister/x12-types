use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*840*0001~BQT*00*RFQ1*20200101~N1*BY*BUYER~PO1*1*10*EA*5**VN*ABC~PO1*2*5*EA*3**VN*DEF~SE*6*0001~";

#[test]
fn parse_840() {
    let (rest, obj) = _840::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "840");
    assert_eq!(obj.bqt._01, "00");
    assert_eq!(obj.loop_po1.len(), 2);
}

#[test]
fn roundtrip_840() {
    let (_, a) = _840::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _840::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_840() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*RQ*S*R*20200101*1200*1*X*005010~\nST*840*0001~\nBQT*00*RFQ1*20200101~\nPO1*1*10*EA*5**VN*ABC~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_840>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bqt._01, "00");
}
