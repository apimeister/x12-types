use crate::util::Parser;
use crate::v005010::*;

// Promotion announcement: heading G42 + N1 party loop and a G94 promotion-conditions loop,
// then an LX detail loop carrying G46/G51, a nested G94->G95 conditions loop and a G45
// promotional-product loop.
const SAMPLE: &str = r#"ST*889*0001~
G42*A*PROMO1~
N9*PR*PROMO1~
G62*04*20200101~
NTE*GEN*SPRING PROMO~
G43*ZZ*NE~
N1*BY*BUYER*92*B1~
N3*1 BUYER ST~
N4*DALLAS*TX*75201~
G62*04*20200101~
G94*01*OPT1~
G95*01*5~
LX*1~
G46*A*02~
G51*1*EA*10*EA~
G94*01*OPT2~
G95*01*5~
G62*04*20200101~
G45*012345678905~
G69*PROMO ITEM~
G62*04*20200101~
QTY*39*100~
SE*22*0001~"#;

#[test]
fn parse_889() {
    let (rest, obj) = _889::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "889");
    assert_eq!(obj.g42._02, "PROMO1");
    assert_eq!(obj.g62.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].g62.len(), 1);
    assert_eq!(obj.loop_g94.len(), 1);
    assert_eq!(obj.loop_g94[0].g95.len(), 1);
    assert_eq!(obj.loop_lx.len(), 1);
    let lx = &obj.loop_lx[0];
    assert_eq!(lx.g46.len(), 1);
    assert!(lx.g51.is_some());
    assert_eq!(lx.loop_g94.len(), 1);
    assert_eq!(lx.loop_g94[0].loop_g95.len(), 1);
    assert_eq!(lx.loop_g94[0].loop_g95[0].g62.len(), 1);
    assert_eq!(lx.loop_g45.len(), 1);
    assert_eq!(lx.loop_g45[0].g69.len(), 1);
    assert_eq!(lx.loop_g45[0].g62.len(), 1);
    assert_eq!(lx.loop_g45[0].qty.len(), 1);
}

#[test]
fn roundtrip_889() {
    let (_, a) = _889::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _889::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_889() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*PN*S*R*20200101*1200*1*X*005010~
ST*889*0001~
G42*A*PROMO1~
G62*04*20200101~
N1*BY*BUYER~
LX*1~
G45*012345678905~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_889>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.g42._02, "PROMO1");
    assert_eq!(t.loop_lx.len(), 1);
    assert_eq!(t.loop_lx[0].loop_g45.len(), 1);
}
