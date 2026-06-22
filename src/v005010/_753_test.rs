use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*753*0001~G62*10*20200101~N1*SH*SHIPPER~LX*1~N7*A*123~LX*2~N7*B*456~SE*8*0001~";

#[test]
fn parse_753() {
    let (rest, obj) = _753::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "753");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_lx.len(), 2);
}

#[test]
fn roundtrip_753() {
    let (_, a) = _753::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _753::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_753() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*GR*S*R*20200101*1200*1*X*005010~\nST*753*0001~\nG62*10*20200101~\nLX*1~\nN7*A*123~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_753>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].loop_lx.len(), 1);
}
