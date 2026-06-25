use crate::util::Parser;
use crate::v005010::*;

// Exercises the heading N1 (with nested PER loop) and LM loops, the detail PTD loop
// with its N1 and SII loops, the SII -> QTY -> (LM, LX -> (LM, FA1)) nest, and the
// summary CTT loop.
const SAMPLE: &str = r#"ST*867*0001~
BPT*00*RPT123*20200115*PS~
CUR*SE*USD~
DTM*090*20200101~
REF*BM*REF001~
PER*IC*JOHN DOE*TE*5551234~
MEA*PD*G*1000~
PSA*92*PARTNER01*0.5~
N1*SU*SUPPLIER NAME*92*SUP01~
N3*123 SUPPLY ST~
N4*CHICAGO*IL*60601*US~
PER*IC*JANE SMITH*TE*5559999~
REF*ZZ*CONTACTREF~
N1*BY*BUYER NAME*92*BUY01~
N4*DALLAS*TX*75201*US~
LM*VI~
LQ*A*CODE1~
LCD*1*BY*A*20200101~
PTD*PT~
DTM*514*20200110~
REF*IA*ACCT001~
PRF*PO9988~
PER*IC*CONTACT*TE*5551111~
MAN*GM*1234567890~
LCD*1*SU*A*20200101~
LQ*A*PTDLQ~
MEA*WT*G*5000~
N1*ST*SHIP TO NAME*92*STO01~
N4*MIAMI*FL*33101*US~
REF*ZZ*PTDN1REF~
PER*IC*STCONTACT*TE*5552222~
SII*VP*ITEM001*100*EA*5.00~
N9*PO*PO9988~
QTY*39*100~
LIN*1*VP*ITEM001~
PO4*1*12*EA~
UIT*EA*5.00~
AMT*TT*500~
ITA*A*DT*ADV*06*ALLOW01*0.05~
PID*F****WIDGET ASSEMBLY~
MEA*WT*G*5000~
DD*MAT~
LDT*AG*10*DA~
LM*VI~
LQ*A*QTYLQ~
LX*1~
REF*ZZ*LXREF~
DTM*514*20200110~
N1*WH*WAREHOUSE~
LM*VI~
LQ*A*LXLQ~
FA1*BY~
FA2*01*100~
CTT*1~
AMT*TT*500~
SE*56*0001~"#;

#[test]
fn parse_867() {
    let (rest, obj) = _867::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "867");
    assert_eq!(obj.bpt._01, "00");
    // heading parties; the first carries a nested PER loop
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "SU");
    assert_eq!(obj.loop_n1[0].loop_per.len(), 1);
    assert_eq!(obj.loop_n1[0].loop_per[0].r#ref.len(), 1);
    assert_eq!(obj.loop_n1[1].n1._01.to_string(), "BY");
    // heading code-source loop
    assert_eq!(obj.loop_lm.len(), 1);
    assert_eq!(obj.loop_lm[0].lcd.len(), 1);
    // detail
    assert_eq!(obj.loop_ptd.len(), 1);
    let ptd = &obj.loop_ptd[0];
    assert_eq!(ptd.ptd._01.to_string(), "PT");
    assert!(ptd.prf.is_some());
    assert_eq!(ptd.loop_n1.len(), 1);
    assert_eq!(ptd.loop_n1[0].n1._01.to_string(), "ST");
    assert_eq!(ptd.loop_sii.len(), 1);
    let sii = &ptd.loop_sii[0];
    assert_eq!(sii.sii._02, "ITEM001");
    assert!(sii.n9.is_some());
    assert_eq!(sii.loop_qty.len(), 1);
    let qty = &sii.loop_qty[0];
    assert_eq!(qty.qty._01.to_string(), "39");
    assert_eq!(qty.amt.len(), 1);
    assert_eq!(qty.ita.len(), 1);
    assert!(qty.ldt.is_some());
    // QTY-level code-source loop and the LX loop with its own LM + FA1 loops
    assert_eq!(qty.loop_lm.len(), 1);
    assert_eq!(qty.loop_lx.len(), 1);
    let lx = &qty.loop_lx[0];
    assert_eq!(lx.lx._01, "1");
    assert!(lx.n1.is_some());
    assert_eq!(lx.loop_lm.len(), 1);
    assert_eq!(lx.loop_fa1.len(), 1);
    assert_eq!(lx.loop_fa1[0].fa2.len(), 1);
    // summary
    assert_eq!(obj.loop_ctt.len(), 1);
    assert_eq!(obj.loop_ctt[0].ctt._01, "1");
    assert_eq!(obj.loop_ctt[0].amt.len(), 1);
}

#[test]
fn roundtrip_867() {
    let (_, first) = _867::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _867::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_867() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*PT*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*867*0001~
BPT*00*RPT123*20200115*PS~
N1*SU*SUPPLIER NAME*92*SUP01~
PTD*PT~
SII*VP*ITEM001*100*EA*5.00~
QTY*39*100~
CTT*1~
SE*8*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_867>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bpt._01, "00");
    assert_eq!(t.loop_n1.len(), 1);
    assert_eq!(t.loop_ptd.len(), 1);
    assert_eq!(t.loop_ptd[0].loop_sii.len(), 1);
    assert_eq!(t.loop_ptd[0].loop_sii[0].loop_qty.len(), 1);
    assert_eq!(t.loop_ctt.len(), 1);
}
