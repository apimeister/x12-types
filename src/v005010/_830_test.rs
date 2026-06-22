use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*830*0001~BFR*00*REL1**DL*A*20200101*20200101~N1*SU*SUPPLIER~LIN*1*BP*ABC~FST*100*C*D*20200201~FST*200*C*D*20200301~SE*8*0001~";

#[test]
fn parse_830() {
    let (rest, obj) = _830::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "830");
    assert_eq!(obj.bfr._01, "00");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_lin.len(), 1);
    assert_eq!(obj.loop_lin[0].loop_fst.len(), 2);
    assert_eq!(obj.loop_lin[0].loop_fst[0].fst._01, "100");
}

#[test]
fn roundtrip_830() {
    let (_, a) = _830::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _830::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_830() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*PS*S*R*20200101*1200*1*X*005010~\nST*830*0001~\nBFR*00*REL1**DL*A*20200101*20200101~\nLIN*1*BP*ABC~\nFST*100*C*D*20200201~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_830>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bfr._01, "00");
}
