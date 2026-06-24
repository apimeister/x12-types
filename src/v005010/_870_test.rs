use crate::util::Parser;
use crate::v005010::*;

// Order status report: heading REF and party loops, then an HL level with an order-status
// (ISR) loop, a party loop, and a PO1 line-item loop carrying its own ISR loop (with party
// and carrier detail) and an LX loop with a code-source sub-loop.
const SAMPLE: &str = r#"ST*870*0001~
BSR*1*PO*REF1*20200101~
DTM*004*20200101~
REF*CO*CONTRACT1~
DTM*009*20200101~
N1*BY*BUYER*92*BY01~
N3*1 BUYER ST~
N4*DALLAS*TX*75201~
HL*1**O~
PRF*PO999~
ISR*IC*20200101~
PID*F****ORDER STATUS~
N1*ST*STORE*92*ST01~
PO1*1*100*EA*5.00**VP*ITEM001~
PID*F****WIDGET~
ISR*SH*20200102~
QTY*39*50~
N1*SH*SHIPPER~
TD5**2*UPSN~
LX*1~
REF*ZZ*LXREF~
LM*VI~
LQ*AS*CODE1~
CTT*1~
SE*25*0001~"#;

#[test]
fn parse_870() {
    let (rest, obj) = _870::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "870");
    assert_eq!(obj.bsr._01, "1");
    assert_eq!(obj.dtm.len(), 1);
    assert_eq!(obj.loop_ref.len(), 1);
    assert_eq!(obj.loop_ref[0].dtm.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_hl.len(), 1);
    let hl = &obj.loop_hl[0];
    assert!(hl.prf.is_some());
    assert_eq!(hl.loop_isr.len(), 1);
    assert_eq!(hl.loop_isr[0].pid.len(), 1);
    assert_eq!(hl.loop_n1.len(), 1);
    assert_eq!(hl.loop_po1.len(), 1);
    let po1 = &hl.loop_po1[0];
    assert_eq!(po1.pid.len(), 1);
    assert_eq!(po1.loop_isr.len(), 1);
    assert_eq!(po1.loop_isr[0].qty.len(), 1);
    assert!(po1.loop_isr[0].n1.is_some());
    assert_eq!(po1.loop_lx.len(), 1);
    assert_eq!(po1.loop_lx[0].loop_lm.len(), 1);
    assert!(obj.ctt.is_some());
}

#[test]
fn roundtrip_870() {
    let (_, a) = _870::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _870::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_870() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*RS*S*R*20200101*1200*1*X*005010~
ST*870*0001~
BSR*1*PO*REF1*20200101~
N1*BY*BUYER~
HL*1**O~
PRF*PO1~
SE*4*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_870>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bsr._01, "1");
    assert_eq!(t.loop_hl.len(), 1);
    assert!(t.loop_hl[0].prf.is_some());
}
