use crate::util::Parser;
use crate::v005010::*;

// Customs manifest: heading M10 + NM1 broker loop, then a P4 port loop with an LX line loop
// (N9) carrying an N1 party loop, an M12 in-bond loop (R4) and a VID conveyance loop (M7)
// whose N10 line-item loop carries VC/MAN and an H1 hazardous-material loop.
const SAMPLE: &str = r#"ST*309*0001~
M10*ZZ*O*SCAC*VESSEL*1*MANIFEST1~
NM1*CB*2*BROKER~
REF*ZZ*REF1~
N3*1 BROKER ST~
N4*NYC*NY*10001~
P4*PORT1*20200101~
LX*1~
N9*BM*BOL123~
N1*SH*SHIPPER~
N3*1 SHIPPER ST~
M12*01*ENTRY1~
R4*1*UN*PORT1*PORTNAME~
VID*CN*CONT1*4500~
M7*SEAL1~
N10*10*WIDGETS*MARKS~
VC*VIN999~
MAN*GM*0000~
H1*1234*F~
H2*FLAMMABLE~
SE*20*0001~"#;

#[test]
fn parse_309() {
    let (rest, obj) = _309::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "309");
    assert_eq!(obj.loop_nm1.len(), 1);
    assert_eq!(obj.loop_nm1[0].r#ref.len(), 1);
    assert_eq!(obj.loop_p4.len(), 1);
    let p4 = &obj.loop_p4[0];
    assert_eq!(p4.loop_lx.len(), 1);
    let lx = &p4.loop_lx[0];
    assert_eq!(lx.n9.len(), 1);
    assert_eq!(lx.loop_n1.len(), 1);
    assert_eq!(lx.loop_m12.len(), 1);
    assert_eq!(lx.loop_m12[0].r4.len(), 1);
    assert_eq!(lx.loop_vid.len(), 1);
    let vid = &lx.loop_vid[0];
    assert_eq!(vid.m7.len(), 1);
    assert_eq!(vid.loop_n10.len(), 1);
    let n10 = &vid.loop_n10[0];
    assert_eq!(n10.vc.len(), 1);
    assert_eq!(n10.man.len(), 1);
    assert_eq!(n10.loop_h1.len(), 1);
    assert_eq!(n10.loop_h1[0].h2.len(), 1);
}

#[test]
fn roundtrip_309() {
    let (_, a) = _309::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _309::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_309() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*AM*S*R*20200101*1200*1*X*005010~
ST*309*0001~
M10*ZZ*O*SCAC*VESSEL*1*MANIFEST1~
P4*PORT1*20200101~
LX*1~
N9*BM*BOL123~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_309>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_p4.len(), 1);
    assert_eq!(t.loop_p4[0].loop_lx.len(), 1);
}
