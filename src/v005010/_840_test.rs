use crate::util::Parser;
use crate::v005010::*;

// Request for quotation: heading N9/N1 loops, then a PO1 line-item loop carrying PID, SAC,
// QTY, SCH, subline (SLN) and party (N1) sub-loops.
const SAMPLE: &str = r#"ST*840*0001~
BQT*00*RFQ1*20200101~
CUR*BY*USD~
REF*RQ*RFQ1~
DTM*119*20200101~
N9*PO*PO1~
N1*BY*BUYER*92*B1~
N3*1 BUYER ST~
N4*DALLAS*TX*75201~
PO1*1*10*EA*5**VN*ABC~
PID*F****WIDGET~
REF*PO*PO1~
SAC*A*C000~
QTY*39*10~
SCH*5*EA***002*20200201~
SLN*1**A*5*EA~
MTX*ZZ*sub item~
N1*ST*STORE*92*ST1~
CTT*1~
SE*20*0001~"#;

#[test]
fn parse_840() {
    let (rest, obj) = _840::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "840");
    assert_eq!(obj.bqt._01, "00");
    assert_eq!(obj.loop_n9.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_po1.len(), 1);
    let po1 = &obj.loop_po1[0];
    assert_eq!(po1.loop_pid.len(), 1);
    assert_eq!(po1.loop_sac.len(), 1);
    assert_eq!(po1.loop_qty.len(), 1);
    assert_eq!(po1.loop_sch.len(), 1);
    assert_eq!(po1.loop_sln.len(), 1);
    assert_eq!(po1.loop_sln[0].mtx.len(), 1);
    assert_eq!(po1.loop_n1.len(), 1);
}

#[test]
fn roundtrip_840() {
    let (_, a) = _840::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _840::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_840() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*RQ*S*R*20200101*1200*1*X*005010~
ST*840*0001~
BQT*00*RFQ1*20200101~
N1*BY*BUYER~
PO1*1*10*EA*5**VN*ABC~
SE*4*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_840>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bqt._01, "00");
    assert_eq!(t.loop_po1.len(), 1);
}
