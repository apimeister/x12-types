use crate::util::Parser;
use crate::v005010::*;

// Customs events advisory details: M10/P4 heading, then an M15 loop carrying K1 remarks.
const SAMPLE: &str = r#"ST*353*0001~
M10*ZZ*O*SCAC*VESSEL*1*MANIFEST1~
P4*PORT1*20200101~
M15*A*EVENT1*20200101*PORT1*SCAC*1200~
K1*EVENT NOTE~
M15*D*EVENT2*20200102*PORT2*SCAC*1300~
K1*DEPARTURE~
SE*7*0001~"#;

#[test]
fn parse_353() {
    let (rest, obj) = _353::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "353");
    assert!(obj.m10.is_some());
    assert!(obj.p4.is_some());
    assert_eq!(obj.loop_m15.len(), 2);
    assert_eq!(obj.loop_m15[0].m15._02, "EVENT1");
    assert_eq!(obj.loop_m15[0].k1.len(), 1);
    assert_eq!(obj.loop_m15[1].m15._01, "D");
}

#[test]
fn roundtrip_353() {
    let (_, a) = _353::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _353::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_353() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*AM*S*R*20200101*1200*1*X*005010~
ST*353*0001~
M15*A*EVENT1*20200101*PORT1*SCAC*1200~
SE*3*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_353>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_m15.len(), 1);
}
