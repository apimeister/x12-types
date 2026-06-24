use crate::util::Parser;
use crate::v005010::*;

// Account analysis: heading BGN/DTM/CUR + forwarder N1 and default RTE loops, then an ENT
// entity loop with a party N1 loop and an ACT account loop carrying ADJ, an account RTE
// loop, an LX balance loop (BLN) and a SER service-charge loop (CTP).
const SAMPLE: &str = r#"ST*822*0001~
BGN*00*REF1*20200101~
DTM*186*20200131~
CUR*ZZ*USD~
N1*FR*BANK~
RTE*CR*5.0~
ENT*1~
N1*40*CORPORATION~
ACT*ACCT1~
CUR*ZZ*USD~
ADJ*BD*100**20200101*20200131~
RTE*CR*4.5~
LX*1~
BLN*ZZ*AVG*50000~
DTM*186*20200131~
SER*SV*SVC1*25~
CTP*WS*RES*0.10~
SE*18*0001~"#;

#[test]
fn parse_822() {
    let (rest, obj) = _822::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "822");
    assert_eq!(obj.bgn._01.to_string(), "00");
    assert_eq!(obj.dtm.len(), 1);
    assert!(obj.cur.is_some());
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_rte.len(), 1);
    assert_eq!(obj.loop_ent.len(), 1);
    let ent = &obj.loop_ent[0];
    assert_eq!(ent.loop_n1.len(), 1);
    assert_eq!(ent.loop_act.len(), 1);
    let act = &ent.loop_act[0];
    assert!(act.cur.is_some());
    assert_eq!(act.adj.len(), 1);
    assert_eq!(act.loop_rte.len(), 1);
    assert_eq!(act.loop_lx.len(), 1);
    assert_eq!(act.loop_lx[0].bln.len(), 1);
    assert_eq!(act.loop_lx[0].dtm.len(), 1);
    assert_eq!(act.loop_ser.len(), 1);
    assert_eq!(act.loop_ser[0].ctp.len(), 1);
}

#[test]
fn roundtrip_822() {
    let (_, a) = _822::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _822::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_822() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*AA*S*R*20200101*1200*1*X*005010~
ST*822*0001~
BGN*00*REF1*20200101~
DTM*186*20200131~
ENT*1~
ACT*ACCT1~
LX*1~
BLN*ZZ*AVG*50000~
SE*7*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_822>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bgn._01.to_string(), "00");
    assert_eq!(t.loop_ent.len(), 1);
    assert_eq!(t.loop_ent[0].loop_act[0].loop_lx.len(), 1);
}
