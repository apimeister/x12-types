use crate::util::Parser;
use crate::v005010::*;

// Order status inquiry: HL-based. Heading BSI/NTE, then an HL loop carrying PRF/DTM/LIN/QTY
// and its REF, N1 (party) and LM (code) sub-loops.
const SAMPLE: &str = r#"ST*869*0001~
BSI*INQ1*20200101*BA~
NTE*GEN*STATUS PLEASE~
HL*1**1~
PRF*PO123~
DTM*004*20200101~
LIN*1*UP*012345678905~
QTY*39*10~
REF*PO*PO123~
DTM*004*20200101~
N1*BY*BUYER*92*B1~
N3*1 BUYER ST~
N4*DALLAS*TX*75201~
PER*IC*JOHN~
LM*VI~
LQ*0*A~
CTT*1~
SE*16*0001~"#;

#[test]
fn parse_869() {
    let (rest, obj) = _869::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "869");
    assert_eq!(obj.bsi._01, "INQ1");
    assert_eq!(obj.nte.len(), 1);
    assert_eq!(obj.loop_hl.len(), 1);
    let hl = &obj.loop_hl[0];
    assert!(hl.prf.is_some());
    assert_eq!(hl.dtm.len(), 1);
    assert_eq!(hl.lin.len(), 1);
    assert!(hl.qty.is_some());
    assert_eq!(hl.loop_ref.len(), 1);
    assert_eq!(hl.loop_ref[0].dtm.len(), 1);
    assert_eq!(hl.loop_n1.len(), 1);
    assert_eq!(hl.loop_n1[0].n3.len(), 1);
    assert_eq!(hl.loop_n1[0].per.len(), 1);
    assert_eq!(hl.loop_lm.len(), 1);
    assert_eq!(hl.loop_lm[0].lq.len(), 1);
    assert!(obj.ctt.is_some());
}

#[test]
fn roundtrip_869() {
    let (_, a) = _869::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _869::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_869() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*RS*S*R*20200101*1200*1*X*005010~
ST*869*0001~
BSI*INQ1*20200101*BA~
HL*1**1~
PRF*PO123~
SE*4*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_869>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bsi._01, "INQ1");
    assert_eq!(t.loop_hl.len(), 1);
}
