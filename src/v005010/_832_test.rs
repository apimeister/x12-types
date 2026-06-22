use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*832*0001~BCT*AB~N1*VN*VENDOR~LIN*1*UP*012~PID*F****WIDGET~LIN*2*UP*013~SE*7*0001~";

#[test]
fn parse_832() {
    let (rest, obj) = _832::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "832");
    assert_eq!(obj.bct._01, "AB");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_lin.len(), 2);
}

#[test]
fn roundtrip_832() {
    let (_, a) = _832::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _832::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_832() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*SC*S*R*20200101*1200*1*X*005010~\nST*832*0001~\nBCT*AB~\nLIN*1*UP*012~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_832>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bct._01, "AB");
}
