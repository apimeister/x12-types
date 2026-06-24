use crate::util::Parser;
use crate::v005010::*;

// Transportation appointment schedule: B13/B2A heading with G62/L11, an N1 party loop (with
// an OID order loop) and an S5 stop-off loop (with party/contact segments and an OID loop).
const SAMPLE: &str = r#"ST*163*0001~
B13*APPT1*SCAC~
B2A*00~
G62*10*20200101~
L11*REF1*ZZ~
N1*SH*SHIPPER*92*S1~
N3*1 SHIPPER ST~
N4*DALLAS*TX*75201~
G61*IC*JOHN~
OID*ORDER1~
SDQ*EA*92*ST1*10~
S5*1*CL~
N1*ST*STORE~
N3*1 STORE ST~
N4*AUSTIN*TX*78701~
G62*10*20200102~
OID*ORDER2~
SDQ*EA*92*ST2*5~
SE*18*0001~"#;

#[test]
fn parse_163() {
    let (rest, obj) = _163::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "163");
    assert_eq!(obj.b13._01, "APPT1");
    assert_eq!(obj.g62.len(), 1);
    assert_eq!(obj.l11.len(), 1);
    assert_eq!(obj.loop_0100.len(), 1);
    let p = &obj.loop_0100[0];
    assert_eq!(p.n3.len(), 1);
    assert_eq!(p.g61.len(), 1);
    assert_eq!(p.loop_0150.len(), 1);
    assert_eq!(p.loop_0150[0].sdq.len(), 1);
    assert_eq!(obj.loop_0300.len(), 1);
    let s = &obj.loop_0300[0];
    assert!(s.n1.is_some());
    assert_eq!(s.g62.len(), 1);
    assert_eq!(s.loop_0350.len(), 1);
    assert_eq!(s.loop_0350[0].oid._01.as_deref(), Some("ORDER2"));
}

#[test]
fn roundtrip_163() {
    let (_, a) = _163::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _163::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_163() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*SO*S*R*20200101*1200*1*X*005010~
ST*163*0001~
B13*APPT1*SCAC~
B2A*00~
N1*SH*SHIPPER~
OID*ORDER1~
S5*1*CL~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_163>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.b13._01, "APPT1");
    assert_eq!(t.loop_0100.len(), 1);
    assert_eq!(t.loop_0300.len(), 1);
}
