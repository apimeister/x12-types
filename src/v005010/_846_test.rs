use crate::util::Parser;
use crate::v005010::*;

// Inventory advice with two items; the first item carries a QTY loop with a schedule
// (SCH) sub-loop, the second carries a subline (SLN) loop.
const SAMPLE: &str = r#"ST*846*0001~
BIA*00*SI*REF1*20200101~
DTM*007*20200101~
N1*SU*SUPPLIER*92*S1~
N3*123 SUPPLY ST~
N4*DALLAS*TX*75201*US~
LIN*1*VN*ABC123~
PID*F****WIDGET~
MEA*PD*N*100~
REF*PO*PO999~
QTY*33*100*EA~
SCH*50*EA***002*20200201~
QTY*20*10*EA~
LIN*2*VN*DEF456~
SLN*1**A*5*EA~
QTY*33*50*EA~
CTT*2~
SE*18*0001~"#;

#[test]
fn parse_846() {
    let (rest, obj) = _846::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "846");
    assert_eq!(obj.bia._01.to_string(), "00");
    assert_eq!(obj.bia._02, "SI");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "SU");
    assert_eq!(obj.loop_n1[0].n3.len(), 1);
    assert_eq!(obj.loop_lin.len(), 2);
    // item 1: header detail + a QTY loop with a SCH sub-loop
    let item1 = &obj.loop_lin[0];
    assert_eq!(item1.lin._02.to_string(), "VN");
    assert_eq!(item1.pid.len(), 1);
    assert_eq!(item1.r#ref.len(), 1);
    assert_eq!(item1.loop_qty.len(), 2);
    assert_eq!(item1.loop_qty[0].qty._01.to_string(), "33");
    assert_eq!(item1.loop_qty[0].loop_sch.len(), 1);
    // item 2: a subline loop and a quantity loop
    let item2 = &obj.loop_lin[1];
    assert_eq!(item2.loop_sln.len(), 1);
    assert_eq!(item2.loop_qty.len(), 1);
    assert_eq!(obj.ctt.as_ref().unwrap()._01, "2");
}

#[test]
fn roundtrip_846() {
    let (_, first) = _846::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _846::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_846() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*IB*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*846*0001~
BIA*00*SI*REF1*20200101~
N1*SU*SUPPLIER*92*S1~
LIN*1*VN*ABC123~
QTY*33*100*EA~
CTT*1~
SE*7*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_846>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bia._02, "SI");
    assert_eq!(t.loop_lin.len(), 1);
    assert_eq!(t.loop_lin[0].loop_qty.len(), 1);
}
