use crate::util::Parser;
use crate::v005010::*;

// Motor-carrier invoice with heading party/equipment/order loops, a stop-off (S5) loop
// carrying its own order and party-with-equipment loops, and an LX detail loop whose
// party loop carries a carton (CD3) loop with special services and charge detail.
const SAMPLE: &str = r#"ST*210*0001~
B3*A*INV123*C*PP*CC*20200101*0700*H*N*S*SCAC~
C3*USD~
L11*BM*BOL123~
G62*10*20200101~
N1*SH*SHIPPER*92*SH01~
N3*1 SHIP ST~
N4*DALLAS*TX*75201~
L11*PO*PO999~
N7*CONT*ABCD1234~
M7*SEAL001~
OID*ORD1**5*CA~
S5*1*CU~
L11*ZZ*STOP1~
OID*ORD2**3*CA~
N1*ST*STORE*92*ST01~
N4*HOUSTON*TX*77001~
N7*TRL*TRL999~
LX*1~
L11*ZZ*LINE1~
L5*1*FREIGHT~
L0*1*100*G*5000*L~
L1*1*100*FR~
OID*ORD3**2*CA~
N1*CN*CONSIGNEE*92*CN01~
N4*AUSTIN*TX*78701~
CD3*5*A*CTN~
H6*PKG~
L9*ABC*50~
POD*20200105~
L3*300*G*1000*MT~
SE*32*0001~"#;

#[test]
fn parse_210() {
    let (rest, obj) = _210::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "210");
    assert_eq!(obj.b3._02, "INV123");
    assert_eq!(obj.l11.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "SH");
    assert_eq!(obj.loop_n1[0].l11.len(), 1);
    assert_eq!(obj.loop_n7.len(), 1);
    assert_eq!(obj.loop_oid.len(), 1);
    // stop-off loop with its order and party-with-equipment sub-loops
    assert_eq!(obj.loop_s5.len(), 1);
    let s5 = &obj.loop_s5[0];
    assert_eq!(s5.loop_oid.len(), 1);
    assert_eq!(s5.loop_n1.len(), 1);
    assert_eq!(s5.loop_n1[0].loop_n7.len(), 1);
    // detail LX loop with order loop and party loop carrying a carton (CD3) loop
    assert_eq!(obj.loop_lx.len(), 1);
    let lx = &obj.loop_lx[0];
    assert_eq!(lx.l0.len(), 1);
    assert_eq!(lx.l1.len(), 1);
    assert_eq!(lx.loop_oid.len(), 1);
    assert_eq!(lx.loop_n1.len(), 1);
    let n1 = &lx.loop_n1[0];
    assert_eq!(n1.loop_cd3.len(), 1);
    assert_eq!(n1.loop_cd3[0].h6.len(), 1);
    assert_eq!(n1.loop_cd3[0].l9.len(), 1);
    assert!(n1.loop_cd3[0].pod.is_some());
    assert!(obj.l3.is_some());
}

#[test]
fn roundtrip_210() {
    let (_, first) = _210::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _210::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_210() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*IM*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*210*0001~
B3*A*INV123*C*PP*CC*20200101*0700*H*N*S*SCAC~
N1*SH*SHIPPER~
LX*1~
L0*1*100~
L3*300~
SE*7*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_210>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.b3._02, "INV123");
    assert_eq!(t.loop_lx.len(), 1);
    assert_eq!(t.loop_lx[0].l0.len(), 1);
}
