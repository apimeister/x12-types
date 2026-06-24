use crate::util::Parser;
use crate::v005010::*;

// PO change request: heading SAC/N9/N1 loops, then two POC line-change loops -- the first
// carrying PID, SAC, QTY, SCH, party (N1), subline (SLN) and AMT sub-loops.
const SAMPLE: &str = r#"ST*860*0001~
BCH*01*NE*PO123*0*1*20200101~
CUR*BY*USD~
REF*CO*CONTRACT1~
SAC*A*B000~
CUR*BY*USD~
DTM*004*20200101~
N9*PO*PO123~
N1*BY*BUYER*92*B1~
N3*1 BUYER ST~
N4*DALLAS*TX*75201~
N1*SE*SELLER*92*S1~
POC*1*CA*5*10*EA*2.50**VN*ABC~
PID*F****WIDGET~
SAC*A*C000~
QTY*39*10~
SCH*5*EA***002*20200201~
N1*ST*STORE*92*ST1~
SLN*1**A*5*EA~
PID*F****SUBITEM~
AMT*1*250~
POC*2*PR*3*5*EA*1.00**VN*DEF~
CTT*2~
SE*24*0001~"#;

#[test]
fn parse_860() {
    let (rest, obj) = _860::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "860");
    assert_eq!(obj.bch._03, "PO123");
    assert_eq!(obj.loop_sac.len(), 1);
    assert_eq!(obj.loop_n9.len(), 1);
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "BY");
    assert_eq!(obj.loop_n1[0].n3.len(), 1);
    assert_eq!(obj.loop_poc.len(), 2);
    let poc = &obj.loop_poc[0];
    assert_eq!(poc.poc._02, "CA");
    assert_eq!(poc.loop_pid.len(), 1);
    assert_eq!(poc.loop_sac.len(), 1);
    assert_eq!(poc.loop_qty.len(), 1);
    assert_eq!(poc.loop_sch.len(), 1);
    assert_eq!(poc.loop_n1.len(), 1);
    assert_eq!(poc.loop_sln.len(), 1);
    assert_eq!(poc.loop_sln[0].pid.len(), 1);
    assert_eq!(poc.loop_amt.len(), 1);
    assert_eq!(obj.loop_poc[1].poc._02, "PR");
    assert_eq!(obj.ctt.as_ref().unwrap()._01, "2");
}

#[test]
fn roundtrip_860() {
    let (_, first) = _860::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _860::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_860() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*PC*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*860*0001~
BCH*01*NE*PO123*0*1*20200101~
N1*BY*BUYER*92*B1~
POC*1*CA*5*10*EA*2.50**VN*ABC~
CTT*1~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_860>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bch._03, "PO123");
    assert_eq!(t.loop_poc.len(), 1);
    assert_eq!(t.loop_poc[0].poc._02, "CA");
}
