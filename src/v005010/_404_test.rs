use crate::util::Parser;
use crate::v005010::*;

// Rail carrier shipment information: BX/BNX/M3 beginning, N9/CM, an N7 equipment loop, F9/D9
// origin/destination, N1 party loops, R2 routing, an LX line loop (L5 + L0 with a PI loop).
// Sample adapted from a real 005030 404 (the structure is shared with 005010).
const SAMPLE: &str = r#"ST*404*96814~
BX*04*R*11*3PHLT99999*CSXT*K*B~
BNX*N**S~
M3*R*20230519*0804~
N9*PO*3PHLT99999~
N9*BM*3PHL999999~
N9*CMH*CARRIER~
CM*320S*L*LONG BEACH CA US*20230605*****CONSIGNEE~
N7*MRKU*550775*2000*N*3810*****S*CN*ABCD***4000*A**3~
F9**CINCINNATI*OH~
D9**LOS ANGELES*CA~
N1*SF*SHIPPER~
N3*845 A AVE E~
N4*COLUMBUS**47274*US~
N1*CN*CONSIGNEE~
N3*2000 MARKET ST~
N4*PHILADELPHIA*PA*19103*US~
R2*CSXT*R*CHGO**85*X~
R2*BNSF*1***85*X~
H3*XP~
LX*1~
L5*1*MACHINERY PARTS*4611110*T~
L0*1***2000*N***1*CNT~
PI*PI*AGRT05012*TP**CSXT*AGRT~
PI*CT*80*TP**BNSF*MA~
SE*43*96814~"#;

#[test]
fn parse_404() {
    let (rest, obj) = _404::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "404");
    assert!(obj.bx.is_some());
    assert!(obj.bnx.is_some());
    assert_eq!(obj.n9.len(), 3);
    assert_eq!(obj.cm.len(), 1);
    assert_eq!(obj.loop_n7.len(), 1);
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n3.is_some(), true);
    assert_eq!(obj.r2.len(), 2);
    assert_eq!(obj.h3.len(), 1);
    assert_eq!(obj.loop_lx.len(), 1);
    let lx = &obj.loop_lx[0];
    assert_eq!(lx.loop_l0.len(), 1);
    assert_eq!(lx.loop_l0[0].loop_pi.len(), 2);
}

#[test]
fn roundtrip_404() {
    let (_, a) = _404::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _404::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_404() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*SR*S*R*20200101*1200*1*X*005010~
ST*404*0001~
BX*04*R*11*REF1*CSXT*K*B~
M3*R*20200101*0804~
N9*BM*BOL123~
N7*MRKU*550775*2000*N*3810*****S*CN*ABCD***4000*A**3~
F9**CINCINNATI*OH~
D9**LOS ANGELES*CA~
LX*1~
L5*1*MACHINERY PARTS*4611110*T~
SE*9*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_404>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.st._01, "404");
    assert_eq!(t.loop_lx.len(), 1);
}
