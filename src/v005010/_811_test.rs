use crate::util::Parser;
use crate::v005010::*;

// Consolidated service invoice: heading party (N1) and financial (FA1) loops, an HL loop
// with an LX service loop (SI/AMT + QTY loop), an HL-level NM1 party loop and an IT1 item
// loop (with AMT, QTY, ITA and NM1 sub-loops); summary TDS plus an ITA loop, a BAL loop
// and an N1 loop carrying BAL and ITA sub-loops.
const SAMPLE: &str = r#"ST*811*0001~
BIG*20200101*INV1~
REF*PO*PO123~
DTM*097*20200101~
N1*BT*BUYER*92*B1~
N3*1 BUYER ST~
N4*DALLAS*TX*75201~
FA1*F1~
FA2*TP*100~
HL*1**1~
LX*1~
SI*CA*WD~
AMT*1*250~
QTY*39*10~
NM1*1P*2*PROVIDER~
IT1*1*10*EA*5**VN*ABC~
PID*F****WIDGET~
AMT*1*50~
QTY*39*10~
ITA*A***6~
NM1*BY*2*BUYERNAME~
TDS*5000~
ITA*C***6~
DTM*097*20200101~
BAL*M*TB*1000~
N1*RE*REMIT~
BAL*M*TB*500~
ITA*A***6~
CTT*1~
SE*28*0001~"#;

#[test]
fn parse_811() {
    let (rest, obj) = _811::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "811");
    assert_eq!(obj.big._02, "INV1");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_fa1.len(), 1);
    assert_eq!(obj.loop_fa1[0].fa2.len(), 1);
    assert_eq!(obj.loop_hl.len(), 1);
    let hl = &obj.loop_hl[0];
    assert_eq!(hl.loop_lx.len(), 1);
    assert_eq!(hl.loop_lx[0].amt.len(), 1);
    assert_eq!(hl.loop_lx[0].loop_qty.len(), 1);
    assert_eq!(hl.loop_nm1.len(), 1);
    assert_eq!(hl.loop_it1.len(), 1);
    let it1 = &hl.loop_it1[0];
    assert_eq!(it1.pid.len(), 1);
    assert_eq!(it1.loop_amt.len(), 1);
    assert_eq!(it1.loop_qty.len(), 1);
    assert_eq!(it1.loop_ita.len(), 1);
    assert_eq!(it1.loop_nm1.len(), 1);
    // summary
    assert_eq!(obj.tds._01.to_string(), "5000");
    assert_eq!(obj.loop_ita.len(), 1);
    assert!(obj.loop_ita[0].dtm.is_some());
    assert_eq!(obj.loop_bal.len(), 1);
    assert_eq!(obj.loop_n1_sum.len(), 1);
    let n1s = &obj.loop_n1_sum[0];
    assert_eq!(n1s.loop_bal.len(), 1);
    assert_eq!(n1s.loop_ita.len(), 1);
    assert!(obj.ctt.is_some());
}

#[test]
fn roundtrip_811() {
    let (_, a) = _811::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _811::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_811() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*CI*S*R*20200101*1200*1*X*005010~
ST*811*0001~
BIG*20200101*INV1~
N1*BT*BUYER~
HL*1**1~
LX*1~
TDS*5000~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_811>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.big._02, "INV1");
    assert_eq!(t.loop_hl.len(), 1);
    assert_eq!(t.tds._01.to_string(), "5000");
}
