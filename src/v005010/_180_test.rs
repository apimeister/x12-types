use crate::util::Parser;
use crate::v005010::*;

// Return authorization: heading reason/party/code-source, then a BLI return-item loop
// carrying a QTY loop (with party and an LX sub-loop) and a financial-accounting loop.
const SAMPLE: &str = r#"ST*180*0001~
BGN*00*RMA1*20200101~
RDR*01~
N9*RA*AUTH123~
N1*ST*RETURNER*92*RET01~
N3*1 RETURN ST~
N4*DALLAS*TX*75201~
LM*VI~
LQ*AS*CODE1~
BLI*UP*012345678905*5*EA~
N9*PO*PO999~
PID*F****WIDGET~
RDR*02~
QTY*38*5~
AMT*1*50~
N1*MF*MANUFACTURER~
LX*1~
N9*SN*SERIAL1~
FA1*BY~
FA2*01*100~
SE*21*0001~"#;

#[test]
fn parse_180() {
    let (rest, obj) = _180::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "180");
    assert!(obj.rdr.is_some());
    assert_eq!(obj.n9.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_lm.len(), 1);
    assert_eq!(obj.loop_bli.len(), 1);
    let bli = &obj.loop_bli[0];
    assert_eq!(bli.bli._02, "012345678905");
    assert_eq!(bli.n9.len(), 1);
    assert_eq!(bli.pid.len(), 1);
    assert!(bli.rdr.is_some());
    assert_eq!(bli.loop_qty.len(), 1);
    let qty = &bli.loop_qty[0];
    assert_eq!(qty.amt.len(), 1);
    assert!(qty.n1.is_some());
    assert_eq!(qty.loop_lx.len(), 1);
    assert_eq!(qty.loop_lx[0].n9.len(), 1);
    assert_eq!(bli.loop_fa1.len(), 1);
    assert_eq!(bli.loop_fa1[0].fa2.len(), 1);
}

#[test]
fn roundtrip_180() {
    let (_, a) = _180::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _180::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_180() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*RA*S*R*20200101*1200*1*X*005010~
ST*180*0001~
BGN*00*RMA1*20200101~
N1*ST*RETURNER~
BLI*UP*012345678905~
SE*4*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_180>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_bli.len(), 1);
    assert_eq!(t.loop_bli[0].bli._01, "UP");
}
