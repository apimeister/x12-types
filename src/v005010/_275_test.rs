use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*275*0001~BGN*11*REF1*20200101~NM1*PR*2*PAYER*****PI*123~NM1*QC*1*SMITH*JOHN~SE*5*0001~";

#[test]
fn parse_275() {
    let (rest, obj) = _275::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "275");
    assert_eq!(obj.loop_nm1.len(), 2);
    assert_eq!(obj.loop_nm1[0].nm1._01, "PR");
    assert_eq!(obj.loop_nm1[1].nm1._01, "QC");
}

#[test]
fn roundtrip_275() {
    let (_, a) = _275::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _275::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_275() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*PI*S*R*20200101*1200*1*X*005010~\nST*275*0001~\nBGN*11*REF1*20200101~\nNM1*PR*2*PAYER*****PI*123~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_275>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].loop_nm1.len(), 1);
}
