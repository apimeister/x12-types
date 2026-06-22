use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*210*0001~B3*A*INV123*C*PP*CC*20200101*0700*H*N*S*SCAC~N1*SH*SHIPPER~LX*1~L0*1*100~LX*2~L0*1*200~L3*300~SE*8*0001~";

#[test]
fn parse_210() {
    let (rest, obj) = _210::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "210");
    assert_eq!(obj.b3._02, "INV123");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01, "SH");
    // two LX loops, both top-level, each with an L0
    assert_eq!(obj.loop_lx.len(), 2);
    assert_eq!(obj.loop_lx[0].lx._01, "1");
    assert_eq!(obj.loop_lx[1].lx._01, "2");
    assert_eq!(obj.loop_lx[0].loop_l0.len(), 1);
    assert_eq!(obj.loop_lx[1].loop_l0.len(), 1);
}

#[test]
fn roundtrip_210() {
    let (_, first) = _210::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _210::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_210() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*IM*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*210*0001~
B3*A*INV123*C*PP*CC*20200101*0700*H*N*S*SCAC~
N1*SH*SHIPPER~
LX*1~
L0*1*100~
L3*300~
SE*7*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_210>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].b3._02, "INV123");
}
