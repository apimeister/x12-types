use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*944*0001~W17*20200101*RCT1*SHP1~N1*ST*WAREHOUSE~LX*1~W07*10*EA~LX*2~W07*20*EA~SE*8*0001~";

#[test]
fn parse_944() {
    let (rest, obj) = _944::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "944");
    assert_eq!(obj.w17._02, "RCT1");
    assert_eq!(obj.loop_lx.len(), 2);
    assert_eq!(obj.loop_lx[0].loop_w07[0].w07._01, "10");
}

#[test]
fn roundtrip_944() {
    let (_, a) = _944::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _944::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_944() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*RE*S*R*20200101*1200*1*X*005010~\nST*944*0001~\nW17*20200101*RCT1*SHP1~\nLX*1~\nW07*10*EA~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_944>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].w17._02, "RCT1");
}
