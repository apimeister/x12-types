use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*812*0001~BCD*20200101*ADJ1*D~N1*RE*REMIT~IT1*1*10*EA*5~IT1*2*5*EA*3~SE*7*0001~";

#[test]
fn parse_812() {
    let (rest, obj) = _812::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "812");
    assert_eq!(obj.bcd._01, "20200101");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_it1.len(), 2);
}

#[test]
fn roundtrip_812() {
    let (_, a) = _812::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _812::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_812() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*CD*S*R*20200101*1200*1*X*005010~\nST*812*0001~\nBCD*20200101*ADJ1*D~\nIT1*1*10*EA*5~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_812>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bcd._01, "20200101");
}
