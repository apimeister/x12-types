use crate::util::Parser;
use crate::v005010::*;

// Motor carrier delivery trailer manifest: heading ATA/B2A, an N1 delivery-location loop, an
// AT7 status loop with an MS2 equipment sub-loop, then an LX detail loop with an OID order
// sub-loop and an N1 shipper sub-loop.
const SAMPLE: &str = r#"ST*212*0001~
ATA*SCAC*MANIFEST1*20200101~
B2A*00~
L11*REF1*ZZ~
N1*ST*STORE*92*ST1~
N3*1 STORE ST~
N4*DALLAS*TX*75201~
AT7*AA*NS~
G62*86*20200101~
MS2*SCAC*TRAILER1*TL~
M7*SEAL1~
LX*1~
L11*BOL1*BM~
AT7*X6*NS~
MAN*GM*00000000000000000~
OID*ORDER1~
SDQ*EA*92*ST1*10~
N1*SH*SHIPPER~
N3*1 SHIPPER ST~
SE*20*0001~"#;

#[test]
fn parse_212() {
    let (rest, obj) = _212::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "212");
    assert_eq!(obj.ata._02, "MANIFEST1");
    assert_eq!(obj.l11.len(), 1);
    assert_eq!(obj.loop_0100.len(), 1);
    assert_eq!(obj.loop_0100[0].n3.len(), 1);
    assert_eq!(obj.loop_0150.len(), 1);
    assert_eq!(obj.loop_0150[0].g62.len(), 1);
    assert_eq!(obj.loop_0150[0].loop_0160.len(), 1);
    assert!(obj.loop_0150[0].loop_0160[0].m7.is_some());
    assert_eq!(obj.loop_0200.len(), 1);
    let lx = &obj.loop_0200[0];
    assert_eq!(lx.l11.len(), 1);
    assert!(lx.at7.is_some());
    assert_eq!(lx.man.len(), 1);
    assert_eq!(lx.loop_0210.len(), 1);
    assert_eq!(lx.loop_0210[0].sdq.len(), 1);
    assert_eq!(lx.loop_0220.len(), 1);
    assert_eq!(lx.loop_0220[0].n3.len(), 1);
}

#[test]
fn roundtrip_212() {
    let (_, a) = _212::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _212::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_212() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*QM*S*R*20200101*1200*1*X*005010~
ST*212*0001~
ATA*SCAC*MANIFEST1*20200101~
B2A*00~
LX*1~
MAN*GM*00000000000000000~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_212>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.ata._02, "MANIFEST1");
    assert_eq!(t.loop_0200.len(), 1);
    assert_eq!(t.loop_0200[0].man.len(), 1);
}
