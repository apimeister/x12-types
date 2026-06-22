use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*875*0001~G50*00*PO1*20200101~N1*BY*BUYER~LIN*1*UP*012~LIN*2*UP*013~SE*6*0001~";

#[test]
fn parse_875() {
    let (rest, obj) = _875::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "875");
    assert_eq!(obj.g50._01, "00");
    assert_eq!(obj.loop_lin.len(), 2);
}

#[test]
fn roundtrip_875() {
    let (_, a) = _875::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _875::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_875() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*OG*S*R*20200101*1200*1*X*005010~\nST*875*0001~\nG50*00*PO1*20200101~\nLIN*1*UP*012~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_875>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].g50._01, "00");
}
