use crate::util::Parser;
use crate::v005010::*;

// Production sequence: heading BSS/UIT and an N1 party loop, then a DTM loop (QTY) with a
// LIN loop (QTY/PID/OQS), an SLN subline loop (party) and a PID description loop (QTY/MEA).
const SAMPLE: &str = r#"ST*866*0001~
BSS*00*PS1*20200101*PS*20200101*20200131~
UIT*EA~
N1*ST*STORE*92*ST1~
N3*1 STORE ST~
N4*DALLAS*TX*75201~
DTM*002*20200115~
QTY*39*100~
LIN*1*BP*PART1~
QTY*39*100~
PID*F****WIDGET~
OQS*1*100~
SLN*1**A*50*EA~
N1*MF*MAKER~
PID*F****SUBASSEMBLY~
QTY*39*50~
MEA*PD*N*5~
CTT*1~
SE*18*0001~"#;

#[test]
fn parse_866() {
    let (rest, obj) = _866::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "866");
    assert_eq!(obj.bss._01, "00");
    assert!(obj.uit.is_some());
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n3.is_some(), true);
    assert_eq!(obj.loop_dtm.len(), 1);
    let dtm = &obj.loop_dtm[0];
    assert!(dtm.qty.is_some());
    assert_eq!(dtm.loop_lin.len(), 1);
    let lin = &dtm.loop_lin[0];
    assert!(lin.qty.is_some());
    assert!(lin.pid.is_some());
    assert!(lin.oqs.is_some());
    assert_eq!(lin.loop_sln.len(), 1);
    let sln = &lin.loop_sln[0];
    assert!(sln.n1.is_some());
    assert_eq!(sln.loop_pid.len(), 1);
    assert!(sln.loop_pid[0].qty.is_some());
    assert_eq!(sln.loop_pid[0].mea.len(), 1);
    assert_eq!(obj.ctt._01, "1");
}

#[test]
fn roundtrip_866() {
    let (_, a) = _866::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _866::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_866() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*NL*S*R*20200101*1200*1*X*005010~
ST*866*0001~
BSS*00*PS1*20200101*PS*20200101*20200131~
DTM*002*20200115~
LIN*1*BP*PART1~
CTT*1~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_866>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bss._01, "00");
    assert_eq!(t.loop_dtm.len(), 1);
    assert_eq!(t.loop_dtm[0].loop_lin.len(), 1);
}
