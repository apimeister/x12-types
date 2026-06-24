use crate::util::Parser;
use crate::v005010::*;

// Grocery products invoice: heading party (N1) and allowance (G72/G73) loops, then a G17
// item-detail loop carrying G69/G19/G20/N9 and its own G72 allowance loop, followed by an
// ENT entity loop whose REF sub-loop carries QTY/AMT/G72 and a nested G17 item sub-loop.
const SAMPLE: &str = r#"ST*880*0001~
G01*INV1*20200101~
N9*IV*INV1~
G62*04*20200101~
CAD*M***UPSN~
G23*01*2~
N1*BY*BUYER*92*B1~
N3*1 BUYER ST~
N4*DALLAS*TX*75201~
G72*01*06~
G73*Promo allowance~
G17*10*EA*5.00*012345678905~
G69*WIDGET~
G19*10*EA~
G20*12*5~
N9*PO*PO1~
G72*02*06~
G73*Line allowance~
ENT*1~
N3*1 STORE ST~
N9*ST*ST1~
REF*PO*PO1~
QTY*39*10~
AMT*1*250~
G72*03*06~
G17*5*EA~
G19*5*EA~
G31*15*100*L~
G33*250~
SE*29*0001~"#;

#[test]
fn parse_880() {
    let (rest, obj) = _880::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "880");
    assert_eq!(obj.g01._01, "INV1");
    assert_eq!(obj.cad.len(), 1);
    assert_eq!(obj.g23.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_g72.len(), 1);
    assert_eq!(obj.loop_g72[0].g73.len(), 1);
    assert_eq!(obj.loop_g17.len(), 1);
    let g17 = &obj.loop_g17[0];
    assert_eq!(g17.g69.len(), 1);
    assert_eq!(g17.g19.len(), 1);
    assert!(g17.g20.is_some());
    assert_eq!(g17.n9.len(), 1);
    assert_eq!(g17.loop_g72.len(), 1);
    assert_eq!(obj.loop_ent.len(), 1);
    let ent = &obj.loop_ent[0];
    assert!(ent.n3.is_some());
    assert_eq!(ent.n9.len(), 1);
    assert_eq!(ent.loop_ref.len(), 1);
    let r = &ent.loop_ref[0];
    assert!(r.qty.is_some());
    assert_eq!(r.amt.len(), 1);
    assert!(r.g72.is_some());
    assert_eq!(r.loop_g17.len(), 1);
    assert_eq!(r.loop_g17[0].g19.len(), 1);
    assert_eq!(obj.g31._01, "15");
    assert_eq!(obj.g33._01, "250");
}

#[test]
fn roundtrip_880() {
    let (_, a) = _880::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _880::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_880() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*GP*S*R*20200101*1200*1*X*005010~
ST*880*0001~
G01*INV1*20200101~
N1*BY*BUYER~
G17*10*EA~
G31*15*100*L~
G33*250~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_880>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.g01._01, "INV1");
    assert_eq!(t.loop_g17.len(), 1);
    assert_eq!(t.g31._01, "15");
}
