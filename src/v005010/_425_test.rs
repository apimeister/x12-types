use crate::util::Parser;
use crate::v005010::*;

// Rail waybill request: ST then a repeating ZT loop, each with optional F9 origin and D9
// destination stations.
const SAMPLE: &str = r#"ST*425*0001~
ZT*1*MRKU*550775~
F9**CINCINNATI*OH~
D9**LOS ANGELES*CA~
ZT*1*ABCD*1234~
SE*6*0001~"#;

#[test]
fn parse_425() {
    let (rest, obj) = _425::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "425");
    assert_eq!(obj.loop_zt.len(), 2);
    assert_eq!(obj.loop_zt[0].zt._02, "MRKU");
    assert!(obj.loop_zt[0].f9.is_some());
    assert!(obj.loop_zt[0].d9.is_some());
    assert_eq!(obj.loop_zt[1].zt._02, "ABCD");
    assert!(obj.loop_zt[1].f9.is_none());
}

#[test]
fn roundtrip_425() {
    let (_, a) = _425::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _425::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_425() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*ER*S*R*20200101*1200*1*X*005010~
ST*425*0001~
ZT*1*MRKU*550775~
F9**CINCINNATI*OH~
D9**LOS ANGELES*CA~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_425>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_zt.len(), 1);
    assert_eq!(t.loop_zt[0].zt._02, "MRKU");
}
