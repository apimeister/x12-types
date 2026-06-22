use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*861*0001~BRA*REF1*20200101*00*1~N1*ST*RECEIVER~RCD*1*10*CA~RCD*2*20*CA~SE*6*0001~";

#[test]
fn parse_861() {
    let (rest, obj) = _861::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "861");
    assert_eq!(obj.bra._01, "REF1");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_rcd.len(), 2);
}

#[test]
fn roundtrip_861() {
    let (_, a) = _861::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _861::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_861() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*RC*S*R*20200101*1200*1*X*005010~\nST*861*0001~\nBRA*REF1*20200101*00*1~\nN1*ST*RECEIVER~\nRCD*1*10*CA~\nSE*5*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_861>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bra._01, "REF1");
}
