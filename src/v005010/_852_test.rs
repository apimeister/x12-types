use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*852*0001~XQ*F*20200101~N1*ST*STORE~LIN*1*UP*012345678905~ZA*QS*100~LIN*2*UP*012345678912~ZA*QS*50~SE*8*0001~";

#[test]
fn parse_852() {
    let (rest, obj) = _852::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "852");
    assert_eq!(obj.xq._01, "F");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_lin.len(), 2);
    assert_eq!(obj.loop_lin[0].za.len(), 1);
}

#[test]
fn roundtrip_852() {
    let (_, a) = _852::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _852::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_852() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*PD*S*R*20200101*1200*1*X*005010~\nST*852*0001~\nXQ*F*20200101~\nLIN*1*UP*012345678905~\nZA*QS*100~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_852>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].xq._01, "F");
}
