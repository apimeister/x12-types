use crate::util::Parser;
use crate::v005010::*;

// Request for routing instructions: heading party loop, then the detail LX plus a
// routing-stop loop carrying location/reference/unitized detail and an OID order loop.
const SAMPLE: &str = r#"ST*753*0001~
BGN*00*RR123*20200101~
PER*IC*SHIPPER CONTACT*TE*5551234~
N1*SH*SHIPPER*92*SH01~
N3*1 SHIP ST~
N4*DALLAS*TX*75201~
LX*1~
N1*ST*STORE*92*ST01~
N4*HOUSTON*TX*77001~
L11*PO*PO999~
G62*10*20200101~
USI*5*CTN~
OID*ORD1**5*CA~
CMC*1234*70~
SE*15*0001~"#;

#[test]
fn parse_753() {
    let (rest, obj) = _753::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "753");
    assert_eq!(obj.bgn._01.to_string(), "00");
    assert_eq!(obj.per.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "SH");
    assert_eq!(obj.lx._01, "1");
    assert_eq!(obj.loop_detail.len(), 1);
    let d = &obj.loop_detail[0];
    assert_eq!(d.n1._01.to_string(), "ST");
    assert_eq!(d.l11.len(), 1);
    assert!(d.usi.is_some());
    assert_eq!(d.loop_oid.len(), 1);
    assert!(d.loop_oid[0].cmc.is_some());
}

#[test]
fn roundtrip_753() {
    let (_, a) = _753::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _753::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_753() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*RU*S*R*20200101*1200*1*X*005010~
ST*753*0001~
BGN*00*RR123*20200101~
N1*SH*SHIPPER~
LX*1~
N1*ST*STORE~
OID*ORD1~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_753>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.lx._01, "1");
    assert_eq!(t.loop_detail.len(), 1);
}
