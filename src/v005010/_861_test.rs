use crate::util::Parser;
use crate::v005010::*;

// A receiving advice with a header party loop and one RCD detail loop that carries item
// detail (LIN/PID/REF), a subline (SLN) loop and a detail party (N1) loop.
const SAMPLE: &str = r#"ST*861*0001~
BRA*REF1*20200101*00*1~
DTM*050*20200101~
N1*ST*RECEIVER*92*RCV01~
N3*1 DOCK ST~
N4*DALLAS*TX*75201~
RCD*1*10*CA*9*CA~
SN1**10*CA~
LIN*1*UP*012345678905~
PID*F****WIDGET~
REF*PO*PO999~
SLN*1**A*5*EA~
PID*F****SUBITEM~
N1*SU*SUPPLIER*92*SUP01~
CTT*1~
SE*16*0001~"#;

#[test]
fn parse_861() {
    let (rest, obj) = _861::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "861");
    assert_eq!(obj.bra._01, "REF1");
    assert_eq!(obj.dtm.len(), 1);
    // one header party
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "ST");
    // one receiving-conditions loop with item detail + sub-loops
    assert_eq!(obj.loop_rcd.len(), 1);
    let rcd = &obj.loop_rcd[0];
    assert!(rcd.sn1.is_some());
    assert_eq!(rcd.lin.len(), 1);
    assert_eq!(rcd.pid.len(), 1);
    assert_eq!(rcd.r#ref.len(), 1);
    assert_eq!(rcd.loop_sln.len(), 1);
    assert_eq!(rcd.loop_sln[0].pid.len(), 1);
    assert_eq!(rcd.loop_n1.len(), 1);
    assert_eq!(rcd.loop_n1[0].n1._01.to_string(), "SU");
    assert_eq!(obj.ctt.as_ref().unwrap()._01, "1");
}

#[test]
fn roundtrip_861() {
    let (_, a) = _861::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _861::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_861() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*RC*S*R*20200101*1200*1*X*005010~
ST*861*0001~
BRA*REF1*20200101*00*1~
N1*ST*RECEIVER*92*RCV01~
RCD*1*10*CA~
LIN*1*UP*012345678905~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_861>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_rcd.len(), 1);
    assert_eq!(t.loop_rcd[0].lin.len(), 1);
}
