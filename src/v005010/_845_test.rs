use crate::util::Parser;
use crate::v005010::*;

// Price authorization acknowledgment: heading BPA + N1 party loop, then a CON contract loop
// with an N1 party loop and a PAD product-adjustment loop (item pricing + LIN, N1 and CTP
// sub-loops, the CTP loop carrying its own party loop).
const SAMPLE: &str = r#"ST*845*0001~
BPA*00*20200101~
REF*CO*CONTRACT1~
DTM*009*20200101~
N1*BY*BUYER*92*B1~
N3*1 BUYER ST~
N4*DALLAS*TX*75201~
CON*CT*CONTRACT1*AC~
REF*PO*PO123~
DTM*009*20200101~
N1*SE*SELLER~
PAD*1*BV~
PID*F****WIDGET~
QTY*39*100~
AMT*1*500~
LIN*1*UP*012345678905~
SLN*1**A*5*EA~
N1*ST*STORE~
CTP*WS*RES*5.00~
N1*MF*MAKER~
CTT*1~
SE*21*0001~"#;

#[test]
fn parse_845() {
    let (rest, obj) = _845::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "845");
    assert_eq!(obj.bpa._01, "00");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n3.len(), 1);
    assert_eq!(obj.loop_con.len(), 1);
    let con = &obj.loop_con[0];
    assert_eq!(con.con._03.to_string(), "AC");
    assert_eq!(con.r#ref.len(), 1);
    assert_eq!(con.loop_n1.len(), 1);
    assert_eq!(con.loop_pad.len(), 1);
    let pad = &con.loop_pad[0];
    assert_eq!(pad.pid.len(), 1);
    assert_eq!(pad.qty.len(), 1);
    assert_eq!(pad.amt.len(), 1);
    assert_eq!(pad.loop_lin.len(), 1);
    assert_eq!(pad.loop_lin[0].sln.len(), 1);
    assert_eq!(pad.loop_n1.len(), 1);
    assert_eq!(pad.loop_ctp.len(), 1);
    assert_eq!(pad.loop_ctp[0].loop_n1.len(), 1);
    assert_eq!(obj.ctt._01, "1");
}

#[test]
fn roundtrip_845() {
    let (_, a) = _845::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _845::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_845() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*PA*S*R*20200101*1200*1*X*005010~
ST*845*0001~
BPA*00*20200101~
N1*BY*BUYER~
CON*CT*CONTRACT1*AC~
PAD*1*BV~
CTT*1~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_845>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bpa._01, "00");
    assert_eq!(t.loop_con.len(), 1);
    assert_eq!(t.loop_con[0].loop_pad.len(), 1);
}
