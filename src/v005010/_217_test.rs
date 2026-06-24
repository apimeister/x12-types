use crate::util::Parser;
use crate::v005010::*;

// Motor carrier loading and route guide: heading BLR + N1 terminal loop; an LS-bracketed
// service-matrix section (0200 N1 loop with an LS-bracketed 0210 LX loop), then a 0300 LX
// service-points loop.
const SAMPLE: &str = r#"ST*217*0001~
BLR*SCAC*ZZ~
N1*TM*TERMINAL A*92*T1~
N3*1 TERMINAL ST~
N4*DALLAS*TX*75201~
L11*REF1*ZZ~
LS*0200~
N1*SM*SERVICE MATRIX~
GY*A~
N4*DALLAS*TX*75201~
LS*0210~
LX*1~
N1*ZZ*CARRIER~
GY*A~
SV*DA~
RST*FOO~
LE*0210~
LE*0200~
LX*2~
N1*ZZ*POINT~
GY*A~
SV*DA~
SE*22*0001~"#;

#[test]
fn parse_217() {
    let (rest, obj) = _217::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "217");
    assert_eq!(obj.blr._01, "SCAC");
    assert_eq!(obj.loop_0100.len(), 1);
    assert_eq!(obj.loop_0100[0].n3.len(), 1);
    assert!(obj.loop_0100[0].l11.is_some());
    assert_eq!(obj.loop_0200_section.len(), 1);
    let sec = &obj.loop_0200_section[0];
    assert!(sec.le.is_some());
    assert_eq!(sec.loop_0200.len(), 1);
    let l0200 = &sec.loop_0200[0];
    assert_eq!(l0200.gy.len(), 1);
    assert_eq!(l0200.n4.len(), 1);
    assert_eq!(l0200.loop_0210_section.len(), 1);
    let sub = &l0200.loop_0210_section[0];
    assert_eq!(sub.loop_0210.len(), 1);
    assert_eq!(sub.loop_0210[0].n1.len(), 1);
    assert_eq!(sub.loop_0210[0].rst.len(), 1);
    assert!(sub.le.is_some());
    assert_eq!(obj.loop_0300.len(), 1);
    assert_eq!(obj.loop_0300[0].n1.len(), 1);
    assert!(obj.loop_0300[0].sv.is_some());
}

#[test]
fn roundtrip_217() {
    let (_, a) = _217::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _217::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_217() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*MA*S*R*20200101*1200*1*X*005010~
ST*217*0001~
BLR*SCAC*ZZ~
N1*TM*TERMINAL A~
LX*1~
N1*ZZ*POINT~
SV*DA~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_217>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.blr._01, "SCAC");
    assert_eq!(t.loop_0100.len(), 1);
    assert_eq!(t.loop_0300.len(), 1);
}
