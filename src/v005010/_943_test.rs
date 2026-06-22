use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*943*0001~W06*N*0001~N1*ST*RECEIVER~LX*1~W07*10*EA~LX*2~W07*20*EA~SE*8*0001~";

#[test]
fn parse_943() {
    let (rest, obj) = _943::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "943");
    assert_eq!(obj.w06._01, "N");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_lx.len(), 2);
    assert_eq!(obj.loop_lx[0].loop_w07.len(), 1);
    assert_eq!(obj.loop_lx[0].loop_w07[0].w07._01, "10");
}

#[test]
fn roundtrip_943() {
    let (_, a) = _943::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _943::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_943() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*AR*S*R*20200101*1200*1*X*005010~\nST*943*0001~\nW06*N*0001~\nLX*1~\nW07*10*EA~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_943>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].w06._01, "N");
}
