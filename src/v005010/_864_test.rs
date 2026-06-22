use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*864*0001~BMG*00*Message~N9*1*GROUP~MIT*1*Subject~MSG*Line one~MSG*Line two~SE*7*0001~";

#[test]
fn parse_864() {
    let (rest, obj) = _864::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "864");
    assert_eq!(obj.bmg._01, "00");
    assert_eq!(obj.loop_mit.len(), 1);
    assert_eq!(obj.loop_mit[0].msg.len(), 2);
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
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*TX*S*R*20200101*1200*1*X*005010~\nST*864*0001~\nBMG*00*Message~\nMIT*1*Subject~\nMSG*Line one~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_864>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bmg._01, "00");
}
