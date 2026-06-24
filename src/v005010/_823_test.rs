use crate::util::Parser;
use crate::v005010::*;

// Lockbox: heading bank N1 loop + TRN/DTM, then a DEP deposit loop (AMT/QTY) with a BAT
// batch loop (AVA) whose BPR payment loop carries an RMR remittance loop with a payer N1,
// an IT1 item loop and its REF and SAC reference sub-loops.
const SAMPLE: &str = r#"ST*823*0001~
N1*PR*BANK*92*B1~
N3*1 BANK ST~
N4*NYC*NY*10001~
TRN*1*TRACE1~
DTM*009*20200101~
DEP*LOCKBOX1*20200101***01*123456789~
AMT*1*10000~
QTY*QA*5~
REF*ZZ*DEP1~
BAT*20200101**BATCH1~
AVA*10000*0~
BPR*C*5000*C*ACH~
TRN*1*PAYMENT1~
REF*ZZ*PMT1~
RMR*IV*INV100**5000~
N1*PE*PAYER~
REF*ZZ*REM1~
IT1*1*10*EA*5**VN*ABC~
REF*PO*PO123~
SAC*A*C000~
TXI*ST*50~
SE*23*0001~"#;

#[test]
fn parse_823() {
    let (rest, obj) = _823::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "823");
    assert_eq!(obj.loop_n1.len(), 1);
    assert!(obj.trn.is_some());
    assert_eq!(obj.dtm.len(), 1);
    assert_eq!(obj.loop_dep.len(), 1);
    let dep = &obj.loop_dep[0];
    assert_eq!(dep.amt._02, "10000");
    assert_eq!(dep.qty.len(), 1);
    assert_eq!(dep.r#ref.len(), 1);
    assert_eq!(dep.loop_bat.len(), 1);
    let bat = &dep.loop_bat[0];
    assert_eq!(bat.ava.len(), 1);
    assert_eq!(bat.loop_bpr.len(), 1);
    let bpr = &bat.loop_bpr[0];
    assert!(bpr.trn.is_some());
    assert_eq!(bpr.r#ref.len(), 1);
    assert_eq!(bpr.loop_rmr.len(), 1);
    let rmr = &bpr.loop_rmr[0];
    assert!(rmr.n1.is_some());
    assert_eq!(rmr.r#ref.len(), 1);
    assert_eq!(rmr.loop_it1.len(), 1);
    let it1 = &rmr.loop_it1[0];
    assert_eq!(it1.loop_ref.len(), 1);
    assert_eq!(it1.loop_sac.len(), 1);
    assert_eq!(it1.loop_sac[0].txi.len(), 1);
}

#[test]
fn roundtrip_823() {
    let (_, a) = _823::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _823::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_823() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*LB*S*R*20200101*1200*1*X*005010~
ST*823*0001~
N1*PR*BANK~
DEP*LOCKBOX1*20200101***01*123456789~
AMT*1*10000~
QTY*QA*5~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_823>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_n1.len(), 1);
    assert_eq!(t.loop_dep.len(), 1);
    assert_eq!(t.loop_dep[0].amt._02, "10000");
}
