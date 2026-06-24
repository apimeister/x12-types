use crate::util::Parser;
use crate::v005010::*;

// A planning schedule with one item carrying a subline (SLN) loop, an item party (N1)
// loop, a code-source (LM) loop, two forecast (FST) loops -- the first with a quantity and
// an LM sub-loop -- a ship/delivery-pattern (SDP) loop with a forecast, and an SHP loop.
const SAMPLE: &str = r#"ST*830*0001~
BFR*00*REL1**DL*A*20200101*20200101~
REF*DK*RELEASE1~
N1*SU*SUPPLIER*92*SUP01~
N3*1 VENDOR RD~
N4*DETROIT*MI*48201~
LIN*1*BP*ABC123~
UIT*EA~
PRS*A~
QTY*2*1000~
ATH*KB*20200101*5000~
SLN*1**A*10*EA~
PID*F****COMPONENT~
N1*ST*SHIP TO*92*ST01~
N4*CHICAGO*IL*60601~
LM*VI~
LQ*AS*CODE1~
FST*100*C*D*20200201~
QTY*1*100~
LM*VI~
LQ*AS*FSTCODE~
FST*200*C*W*20200301~
SDP*X*Y~
FST*50*D*D*20200401~
SHP*01*500*011*20200115~
CTT*1~
SE*27*0001~"#;

#[test]
fn parse_830() {
    let (rest, obj) = _830::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "830");
    assert_eq!(obj.bfr._01, "00");
    // one heading party
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "SU");
    // one detail line item with the full set of sub-loops
    assert_eq!(obj.loop_lin.len(), 1);
    let item = &obj.loop_lin[0];
    assert_eq!(item.qty.len(), 1);
    assert_eq!(item.ath.len(), 1);
    assert_eq!(item.loop_sln.len(), 1);
    assert_eq!(item.loop_n1.len(), 1);
    assert_eq!(item.loop_n1[0].n1._01.to_string(), "ST");
    assert_eq!(item.loop_lm.len(), 1);
    assert_eq!(item.loop_fst.len(), 2);
    assert_eq!(item.loop_fst[0].qty.len(), 1);
    assert_eq!(item.loop_fst[0].loop_lm.len(), 1);
    assert_eq!(item.loop_sdp.len(), 1);
    assert_eq!(item.loop_sdp[0].fst.len(), 1);
    assert_eq!(item.loop_shp.len(), 1);
}

#[test]
fn roundtrip_830() {
    let (_, a) = _830::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _830::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_830() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*PS*S*R*20200101*1200*1*X*005010~
ST*830*0001~
BFR*00*REL1**DL*A*20200101*20200101~
N1*SU*SUPPLIER*92*SUP01~
LIN*1*BP*ABC~
FST*100*C*D*20200201~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_830>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_lin.len(), 1);
    assert_eq!(t.loop_lin[0].loop_fst.len(), 1);
}
