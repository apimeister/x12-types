use crate::util::Parser;
use crate::v005010::*;

// Motor carrier shipment pickup notification: PUN beginning with TEM totals and an N1
// location loop (ship-from and ship-to).
const SAMPLE: &str = r#"ST*216*0001~
PUN*SCAC*20200101*1200*PU123**00~
G61*IC*JOHN*TE*5551234~
TEM*5**L*500~
PRF*PO123~
AT5*DDP~
K2*HANDLE WITH CARE~
N1*SF*SHIPPER*92*SF1~
N3*1 SHIPPER ST~
N4*DALLAS*TX*75201~
N1*ST*STORE*92*ST1~
N3*2 STORE AVE~
N4*AUSTIN*TX*78701~
SE*13*0001~"#;

#[test]
fn parse_216() {
    let (rest, obj) = _216::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "216");
    assert_eq!(obj.pun._01, "SCAC");
    assert!(obj.g61.is_some());
    assert!(obj.tem.is_some());
    assert_eq!(obj.prf.len(), 1);
    assert_eq!(obj.at5.len(), 1);
    assert!(obj.k2.is_some());
    assert_eq!(obj.loop_0100.len(), 2);
    assert_eq!(obj.loop_0100[0].n1._01.to_string(), "SF");
    assert_eq!(obj.loop_0100[0].n3.len(), 1);
    assert_eq!(obj.loop_0100[1].n1._01.to_string(), "ST");
}

#[test]
fn roundtrip_216() {
    let (_, a) = _216::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _216::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_216() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*PU*S*R*20200101*1200*1*X*005010~
ST*216*0001~
PUN*SCAC*20200101*1200*PU123**00~
N1*SF*SHIPPER~
N1*ST*STORE~
SE*4*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_216>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.pun._01, "SCAC");
    assert_eq!(t.loop_0100.len(), 2);
}
