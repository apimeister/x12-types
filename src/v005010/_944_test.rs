use crate::util::Parser;
use crate::v005010::*;

// Warehouse stock-transfer receipt advice: header party loop plus carrier/probe/pallet
// detail, and a detail LX loop with a W07 item loop and a nested W13 exception loop.
const SAMPLE: &str = r#"ST*944*0001~
W17*20200101*RCT1*SHP1~
N1*ST*WAREHOUSE*9*WH01~
N3*1 DOCK ST~
N4*DALLAS*TX*75201~
N9*PO*PO123~
G62*17*20200101~
W08*M*SCAC*ROUTE~
W18*P1*38*FA~
G08*10*0~
TD1*PLT94*5~
LX*1~
MAN*GM*1234567890~
PAL*B~
W07*10*EA*UPC*012345678905~
G69*WIDGET~
W20*10*EA*100*G~
W13*2*EA*D~
N9*RC*DAMAGE~
SE*19*0001~"#;

#[test]
fn parse_944() {
    let (rest, obj) = _944::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "944");
    assert_eq!(obj.w17._02, "RCT1");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.n9.len(), 1);
    assert!(obj.w08.is_some());
    assert_eq!(obj.w18.len(), 1);
    assert_eq!(obj.g08.len(), 1);
    assert_eq!(obj.td1.len(), 1);
    assert_eq!(obj.loop_lx.len(), 1);
    let lx = &obj.loop_lx[0];
    assert!(lx.man.is_some());
    assert_eq!(lx.pal.len(), 1);
    assert_eq!(lx.loop_w07.len(), 1);
    let w07 = &lx.loop_w07[0];
    assert_eq!(w07.w07._01, "10");
    assert_eq!(w07.g69.len(), 1);
    assert_eq!(w07.w20.len(), 1);
    // nested exception loop
    assert_eq!(w07.loop_w13.len(), 1);
    assert_eq!(w07.loop_w13[0].n9.len(), 1);
}

#[test]
fn roundtrip_944() {
    let (_, a) = _944::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _944::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_944() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*RE*S*R*20200101*1200*1*X*005010~
ST*944*0001~
W17*20200101*RCT1*SHP1~
N1*ST*WAREHOUSE*9*WH01~
LX*1~
W07*10*EA~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_944>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.w17._02, "RCT1");
    assert_eq!(t.loop_lx[0].loop_w07[0].w07._01, "10");
}
