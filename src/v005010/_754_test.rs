use crate::util::Parser;
use crate::v005010::*;

// Routing instructions: heading party loop, then a detail LX loop with carrier/manifest/
// order/stop detail, a quantity loop (with trailer dimensions) and a destination party loop.
const SAMPLE: &str = r#"ST*754*0001~
BGN*00*RI123*20200101~
PER*IC*CARRIER CONTACT*TE*5551234~
N1*CA*CARRIER*92*CA01~
N4*DALLAS*TX*75201~
LX*1~
L11*RT*ROUTE1~
BLR*SCAC*20200105~
SMD*AB*PP~
OID*ORD1**5*CA~
G62*10*20200105~
MSI*Y*1~
QTY*39*100~
AT9*48*40*36~
N1*ST*STORE*92*ST01~
N4*HOUSTON*TX*77001~
SE*17*0001~"#;

#[test]
fn parse_754() {
    let (rest, obj) = _754::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "754");
    assert_eq!(obj.bgn._01.to_string(), "00");
    assert_eq!(obj.per.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "CA");
    assert_eq!(obj.loop_lx.len(), 1);
    let lx = &obj.loop_lx[0];
    assert_eq!(lx.l11.len(), 1);
    assert!(lx.blr.is_some());
    assert_eq!(lx.smd.len(), 1);
    assert!(lx.msi.is_some());
    assert_eq!(lx.loop_qty.len(), 1);
    assert!(lx.loop_qty[0].at9.is_some());
    assert_eq!(lx.loop_n1.len(), 1);
    assert_eq!(lx.loop_n1[0].n1._01.to_string(), "ST");
}

#[test]
fn roundtrip_754() {
    let (_, a) = _754::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _754::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_754() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*RU*S*R*20200101*1200*1*X*005010~
ST*754*0001~
BGN*00*RI123*20200101~
N1*CA*CARRIER~
LX*1~
BLR*SCAC~
N1*ST*STORE~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_754>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_lx.len(), 1);
    assert!(t.loop_lx[0].blr.is_some());
}
