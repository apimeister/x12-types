use crate::util::Parser;
use crate::v005010::*;

// Product activity for two items; the first carries a ZA reporting loop with a
// performance-requirements (G95) sub-loop.
const SAMPLE: &str = r#"ST*852*0001~
XQ*F*20200101~
N9*PO*PO123~
N1*ST*STORE*9*STORE01~
N3*1 MAIN ST~
N4*DALLAS*TX*75201~
LIN*1*UP*012345678905~
QTY*38*500~
ZA*QS*100*EA~
QTY*17*50~
G95*PA*PR~
DTM*008*20200101~
LIN*2*UP*012345678912~
ZA*QS*50~
CTT*2~
SE*16*0001~"#;

#[test]
fn parse_852() {
    let (rest, obj) = _852::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "852");
    assert_eq!(obj.xq._01, "F");
    assert_eq!(obj.n9.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "ST");
    assert_eq!(obj.loop_lin.len(), 2);
    let item1 = &obj.loop_lin[0];
    assert_eq!(item1.qty.len(), 1);
    assert_eq!(item1.loop_za.len(), 1);
    assert_eq!(item1.loop_za[0].za._01.as_deref(), Some("QS"));
    assert_eq!(item1.loop_za[0].qty.len(), 1);
    assert_eq!(item1.loop_za[0].loop_g95.len(), 1);
    assert_eq!(item1.loop_za[0].loop_g95[0].dtm.len(), 1);
    assert_eq!(obj.ctt.as_ref().unwrap()._01, "2");
}

#[test]
fn roundtrip_852() {
    let (_, a) = _852::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _852::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_852() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*PD*S*R*20200101*1200*1*X*005010~
ST*852*0001~
XQ*F*20200101~
N1*ST*STORE*9*STORE01~
LIN*1*UP*012345678905~
ZA*QS*100~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_852>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_lin.len(), 1);
    assert_eq!(t.loop_lin[0].loop_za.len(), 1);
}
