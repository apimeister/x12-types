use crate::util::Parser;
use crate::v005010::*;

// Credit/debit adjustment: heading currency/shipment/party (with amount) and FA1 loops,
// then a CDD adjustment-detail loop with a SAC loop and an N11 store loop (with a party
// sub-loop carrying amounts).
const SAMPLE: &str = r#"ST*812*0001~
BCD*20200101*ADJ1*D~
CUR*BY*USD~
N9*IV*INV999~
DTM*003*20200101~
SHD*100*98*EA~
N1*RE*REMIT TO*92*RE01~
N3*1 REMIT ST~
N4*DALLAS*TX*75201~
AMT*TT*5000~
FA1*BY~
FA2*01*100~
CDD*A1*C*ADJ001*500~
LIN*1*UP*012345678905~
N9*PO*PO999~
SAC*A*B000~
DTM*003*20200101~
N11*STORE01*LOC1~
AMT*ZZ*250~
N1*BY*BUYER~
AMT*ZZ*250~
SE*22*0001~"#;

#[test]
fn parse_812() {
    let (rest, obj) = _812::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "812");
    assert_eq!(obj.bcd._01, "20200101");
    assert!(obj.cur.is_some());
    assert_eq!(obj.shd.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].amt.len(), 1);
    assert_eq!(obj.loop_fa1.len(), 1);
    // adjustment-detail loop with its sub-loops
    assert_eq!(obj.loop_cdd.len(), 1);
    let cdd = &obj.loop_cdd[0];
    assert_eq!(cdd.cdd._01, "A1");
    assert!(cdd.lin.is_some());
    assert_eq!(cdd.n9.len(), 1);
    assert_eq!(cdd.loop_sac.len(), 1);
    assert_eq!(cdd.loop_sac[0].dtm.len(), 1);
    assert_eq!(cdd.loop_n11.len(), 1);
    assert_eq!(cdd.loop_n11[0].amt.len(), 1);
    assert_eq!(cdd.loop_n11[0].loop_n1.len(), 1);
    assert_eq!(cdd.loop_n11[0].loop_n1[0].amt.len(), 1);
}

#[test]
fn roundtrip_812() {
    let (_, a) = _812::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _812::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_812() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*CD*S*R*20200101*1200*1*X*005010~
ST*812*0001~
BCD*20200101*ADJ1*D~
N1*RE*REMIT~
CDD*A1*C~
LIN*1*UP*012345678905~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_812>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bcd._01, "20200101");
    assert_eq!(t.loop_cdd.len(), 1);
    assert_eq!(t.loop_cdd[0].cdd._01, "A1");
}
