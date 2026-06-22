use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*754*0001~G62*10*20200101~N1*CA*CARRIER~LX*1~R4*L*UN*USNYC~SE*6*0001~";

#[test]
fn parse_754() {
    let (rest, obj) = _754::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "754");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_lx.len(), 1);
    assert_eq!(obj.loop_lx[0].r4.len(), 1);
}

#[test]
fn roundtrip_754() {
    let (_, a) = _754::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _754::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_754() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*RI*S*R*20200101*1200*1*X*005010~\nST*754*0001~\nG62*10*20200101~\nLX*1~\nR4*L*UN*USNYC~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_754>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].loop_lx.len(), 1);
}
