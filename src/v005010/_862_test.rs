use crate::util::Parser;
use crate::v005010::*;

// A shipping schedule with one item carrying two forecast (FST) loops -- the first with a
// just-in-time (JIT) sub-loop -- a shipped/received (SHP) loop, and carrier detail.
const SAMPLE: &str = r#"ST*862*0001~
BSS*00*REL1*20200101*DL*20200101*20200201~
N1*SU*SUPPLIER*92*SUP01~
LIN*1*BP*ABC~
UIT*EA~
PRS*A~
REF*PO*PO123~
FST*100*C*D*20200201~
JIT*100*0800~
FST*200*C*D*20200301~
SHP*01*100*011*20200115~
TD5**2*UPSN~
CTT*1~
SE*14*0001~"#;

#[test]
fn parse_862() {
    let (rest, obj) = _862::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "862");
    assert_eq!(obj.bss._01, "00");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_lin.len(), 1);
    let item = &obj.loop_lin[0];
    assert_eq!(item.uit._01, "EA");
    assert!(item.prs.is_some());
    assert_eq!(item.r#ref.len(), 1);
    // two forecast loops; the first carries a JIT sub-loop
    assert_eq!(item.loop_fst.len(), 2);
    assert_eq!(item.loop_fst[0].fst._01, "100");
    assert_eq!(item.loop_fst[0].loop_jit.len(), 1);
    assert_eq!(item.loop_fst[1].loop_jit.len(), 0);
    // shipped/received loop and carrier detail
    assert_eq!(item.loop_shp.len(), 1);
    assert_eq!(item.td5.len(), 1);
}

#[test]
fn roundtrip_862() {
    let (_, a) = _862::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _862::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_862() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*PS*S*R*20200101*1200*1*X*005010~
ST*862*0001~
BSS*00*REL1*20200101*DL*20200101*20200201~
N1*SU*SUPPLIER*92*SUP01~
LIN*1*BP*ABC~
UIT*EA~
FST*100*C*D*20200201~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_862>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_lin.len(), 1);
    assert_eq!(t.loop_lin[0].loop_fst.len(), 1);
}
