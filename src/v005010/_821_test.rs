use crate::util::Parser;
use crate::v005010::*;

// Financial information reporting: heading B2A/DTM/TRN + forwarder N1, then an ENT entity
// loop with a party N1 loop and an ACT account loop carrying LM, RTE, BLN, TSU and FIR
// sub-loops (the FIR loop with an NM1 party loop).
const SAMPLE: &str = r#"ST*821*0001~
B2A*00~
DTM*186*20200101~
TRN*1*TRACE1~
N1*FR*FORWARDER~
ENT*1~
N1*40*ACCOUNT HOLDER~
ACT*ACCT1~
CUR*ZZ*USD~
LM*VI~
LQ*0*A~
RTE*CR*5.5~
BLN*ZZ*OPEN*10000~
AVA*10000*0~
TSU*ZZ*CR*5000~
FIR*ZZ*DEBIT*250~
DTM*186*20200101~
NM1*PE*2*PAYER~
SE*18*0001~"#;

#[test]
fn parse_821() {
    let (rest, obj) = _821::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "821");
    assert_eq!(obj.dtm.len(), 1);
    assert_eq!(obj.trn.len(), 1);
    assert_eq!(obj.n1.len(), 1);
    assert_eq!(obj.loop_ent.len(), 1);
    let ent = &obj.loop_ent[0];
    assert_eq!(ent.loop_n1.len(), 1);
    assert_eq!(ent.loop_act.len(), 1);
    let act = &ent.loop_act[0];
    assert!(act.cur.is_some());
    assert_eq!(act.loop_lm.len(), 1);
    assert_eq!(act.loop_lm[0].loop_lq.len(), 1);
    assert_eq!(act.loop_rte.len(), 1);
    assert_eq!(act.loop_bln.len(), 1);
    assert_eq!(act.loop_bln[0].ava.len(), 1);
    assert_eq!(act.loop_tsu.len(), 1);
    assert_eq!(act.loop_fir.len(), 1);
    let fir = &act.loop_fir[0];
    assert_eq!(fir.dtm.len(), 1);
    assert_eq!(fir.loop_nm1.len(), 1);
}

#[test]
fn roundtrip_821() {
    let (_, a) = _821::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _821::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_821() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*FR*S*R*20200101*1200*1*X*005010~
ST*821*0001~
B2A*00~
DTM*186*20200101~
TRN*1*TRACE1~
ENT*1~
ACT*ACCT1~
BLN*ZZ*OPEN*10000~
SE*7*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_821>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_ent.len(), 1);
    assert_eq!(t.loop_ent[0].loop_act.len(), 1);
    assert_eq!(t.loop_ent[0].loop_act[0].loop_bln.len(), 1);
}
