use crate::util::Parser;
use crate::v005010::*;

// Grocery products purchase order: heading party and allowance (G72/G73) loops, then a
// G68 line-item loop carrying description (G69/G70), an allowance loop, a party (N1) loop
// and a subline (SLN) loop.
const SAMPLE: &str = r#"ST*875*0001~
G50*00*PO1*20200101~
N9*PO*PO1~
G62*04*20200101~
G23*01*2~
N1*BY*BUYER*92*B1~
N3*1 BUYER ST~
N4*DALLAS*TX*75201~
G72*01*06~
G73*Promo allowance~
G68*10*EA*5.00*012345678905~
G69*WIDGET~
G70*12~
N9*PO*PO1~
G72*02*06~
G73*Line allowance~
N1*ST*STORE*92*ST1~
QTY*39*10~
SLN*1**A*5*EA~
G72*03*06~
G76*10*EA~
SE*22*0001~"#;

#[test]
fn parse_875() {
    let (rest, obj) = _875::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "875");
    assert_eq!(obj.g50._01, "00");
    assert_eq!(obj.g23.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_g72.len(), 1);
    assert_eq!(obj.loop_g72[0].g73.len(), 1);
    assert_eq!(obj.loop_g68.len(), 1);
    let g68 = &obj.loop_g68[0];
    assert_eq!(g68.g69.len(), 1);
    assert_eq!(g68.g70.len(), 1);
    assert_eq!(g68.loop_g72.len(), 1);
    assert_eq!(g68.loop_n1.len(), 1);
    assert!(g68.loop_n1[0].qty.is_some());
    assert_eq!(g68.loop_sln.len(), 1);
    assert_eq!(g68.loop_sln[0].g72.len(), 1);
    assert_eq!(obj.g76._01, "10");
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
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*OG*S*R*20200101*1200*1*X*005010~
ST*875*0001~
G50*00*PO1*20200101~
N1*BY*BUYER~
G68*10*EA~
G76*10*EA~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_875>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.g50._01, "00");
    assert_eq!(t.loop_g68.len(), 1);
    assert_eq!(t.g76._01, "10");
}
