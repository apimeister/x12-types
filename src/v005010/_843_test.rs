use crate::util::Parser;
use crate::v005010::*;

// Response to RFQ: heading N9/N1 loops, then a PO1 quote-line loop carrying PID, SAC, QTY,
// cost-analysis (CST), subline (SLN with its own QTY), pricing-data (PD), party (N1) and
// AMT sub-loops.
const SAMPLE: &str = r#"ST*843*0001~
BQR*00*RFQ1*20200101~
CUR*BY*USD~
REF*RQ*RFQ1~
N9*PO*PO1~
N1*SE*SELLER*92*S1~
N4*DALLAS*TX*75201~
PO1*1*10*EA*5**VN*ABC~
PID*F****WIDGET~
REF*PO*PO1~
SAC*A*C000~
QTY*39*10~
CST*ABC*100~
PID*F****COST DETAIL~
SLN*1**A*5*EA~
QTY*39*5~
PD*DA*20200101*EA*10*PLAN~
PDD*1*5~
N1*ST*STORE*92*ST1~
AMT*1*250~
CTT*1~
SE*22*0001~"#;

#[test]
fn parse_843() {
    let (rest, obj) = _843::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "843");
    assert_eq!(obj.bqr._01, "00");
    assert_eq!(obj.loop_n9.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_po1.len(), 1);
    let po1 = &obj.loop_po1[0];
    assert_eq!(po1.loop_pid.len(), 1);
    assert_eq!(po1.loop_sac.len(), 1);
    assert_eq!(po1.loop_qty.len(), 1);
    assert_eq!(po1.loop_cst.len(), 1);
    assert_eq!(po1.loop_cst[0].pid.len(), 1);
    assert_eq!(po1.loop_sln.len(), 1);
    assert_eq!(po1.loop_sln[0].loop_qty.len(), 1);
    assert_eq!(po1.loop_pd.len(), 1);
    assert_eq!(po1.loop_pd[0].pdd.len(), 1);
    assert_eq!(po1.loop_n1.len(), 1);
    assert_eq!(po1.loop_amt.len(), 1);
}

#[test]
fn roundtrip_843() {
    let (_, a) = _843::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _843::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_843() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*RR*S*R*20200101*1200*1*X*005010~
ST*843*0001~
BQR*00*RFQ1*20200101~
N1*SE*SELLER~
PO1*1*10*EA*5**VN*ABC~
SE*4*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_843>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bqr._01, "00");
    assert_eq!(t.loop_po1.len(), 1);
}
