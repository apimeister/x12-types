use crate::util::Parser;
use crate::v005010::*;

// Rail carrier waybill interchange: BX/N9 beginning, an N7 equipment loop (M7), N8 waybill
// reference, F9/D9 origin/destination, an N1 party loop, R2 routing and an LX line loop
// (L5 + L0 with PI).
const SAMPLE: &str = r#"ST*417*0001~
BX*00*R*11*REF1*CSXT*K*B~
N9*BM*WAYBILL1~
N7*MRKU*550775*2000*N*3810*****S*CN*ABCD***4000*A**3~
M7*SEAL1~
N8*123456*20200101~
F9**CINCINNATI*OH~
D9**LOS ANGELES*CA~
N1*SH*SHIPPER~
N3*123 MAIN ST~
N4*PHILADELPHIA*PA*19103*US~
R2*CSXT*R*CHGO**85*X~
LX*1~
L5*1*MACHINERY PARTS*4611110*T~
L0*1***2000*N***1*CNT~
PI*PI*REF1*TP**CSXT*AGRT~
SE*16*0001~"#;

#[test]
fn parse_417() {
    let (rest, obj) = _417::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "417");
    assert!(obj.bx.is_some());
    assert_eq!(obj.n9.len(), 1);
    assert_eq!(obj.loop_n7.len(), 1);
    assert_eq!(obj.loop_n7[0].m7.len(), 1);
    assert_eq!(obj.n8.len(), 1);
    assert_eq!(obj.n8[0]._01.to_string(), "123456");
    assert_eq!(obj.loop_n1.len(), 1);
    assert!(obj.loop_n1[0].n4.is_some());
    assert_eq!(obj.r2.len(), 1);
    assert_eq!(obj.loop_lx.len(), 1);
    let lx = &obj.loop_lx[0];
    assert_eq!(lx.l5.len(), 1);
    assert_eq!(lx.loop_l0.len(), 1);
    assert_eq!(lx.loop_l0[0].pi.len(), 1);
}

#[test]
fn roundtrip_417() {
    let (_, a) = _417::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _417::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_417() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*RU*S*R*20200101*1200*1*X*005010~
ST*417*0001~
BX*00*R*11*REF1*CSXT*K*B~
N7*MRKU*550775*2000*N*3810*****S*CN*ABCD***4000*A**3~
N8*123456*20200101~
F9**CINCINNATI*OH~
D9**LOS ANGELES*CA~
LX*1~
L5*1*MACHINERY PARTS*4611110*T~
SE*8*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_417>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.st._01, "417");
    assert_eq!(t.n8.len(), 1);
    assert_eq!(t.loop_lx.len(), 1);
}
