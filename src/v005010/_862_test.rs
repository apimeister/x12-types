use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*862*0001~BSS*00*REL1*20200101*DL*20200101*20200201~N1*SU*SUPPLIER~LIN*1*BP*ABC~FST*100*C*D*20200201~FST*200*C*D*20200301~SE*7*0001~";

#[test]
fn parse_862() {
    let (rest, obj) = _862::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "862");
    assert_eq!(obj.bss._01, "00");
    assert_eq!(obj.loop_lin.len(), 1);
    assert_eq!(obj.loop_lin[0].loop_fst.len(), 2);
}

#[test]
fn roundtrip_862() {
    let (_, a) = _862::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _862::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_862() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*SS*S*R*20200101*1200*1*X*005010~\nST*862*0001~\nBSS*00*REL1*20200101*DL*20200101*20200201~\nLIN*1*BP*ABC~\nFST*100*C*D*20200201~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_862>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bss._01, "00");
}
