use crate::util::Parser;
use crate::v005010::*;

// PO change acknowledgment: heading N9/N1 loops, then a POC line-change loop carrying PID,
// SAC, an ACK acknowledgment loop, QTY, SCH, party (N1), subline (SLN with ACK) and a
// pricing-data (PD) loop.
const SAMPLE: &str = r#"ST*865*0001~
BCA*00*AC*PO1*0*1*20200101~
CUR*BY*USD~
REF*CO*CONTRACT1~
DTM*004*20200101~
N9*PO*PO1~
N1*SE*SELLER*92*S1~
N4*DALLAS*TX*75201~
POC*1*CA*5*10*EA*2.50**VN*ABC~
PID*F****WIDGET~
SAC*A*C000~
ACK*IA*5*EA~
DTM*068*20200115~
QTY*39*10~
SCH*5*EA***002*20200201~
N1*ST*STORE*92*ST1~
SLN*1**A*5*EA~
ACK*IA*5*EA~
PD*DA*20200101*EA*10*PRICEPLAN~
PDD*1*5~
CTT*1~
SE*22*0001~"#;

#[test]
fn parse_865() {
    let (rest, obj) = _865::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "865");
    assert_eq!(obj.bca._01, "00");
    assert_eq!(obj.loop_n9.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_poc.len(), 1);
    let poc = &obj.loop_poc[0];
    assert_eq!(poc.poc._02, "CA");
    assert_eq!(poc.loop_pid.len(), 1);
    assert_eq!(poc.loop_sac.len(), 1);
    assert_eq!(poc.loop_ack.len(), 1);
    assert_eq!(poc.loop_ack[0].dtm.len(), 1);
    assert_eq!(poc.loop_qty.len(), 1);
    assert_eq!(poc.loop_sch.len(), 1);
    assert_eq!(poc.loop_n1.len(), 1);
    assert_eq!(poc.loop_sln.len(), 1);
    assert_eq!(poc.loop_sln[0].ack.len(), 1);
    assert_eq!(poc.loop_pd.len(), 1);
    assert_eq!(poc.loop_pd[0].pdd.len(), 1);
}

#[test]
fn roundtrip_865() {
    let (_, a) = _865::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _865::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_865() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*CA*S*R*20200101*1200*1*X*005010~
ST*865*0001~
BCA*00*AC*PO1*0*1*20200101~
N1*SE*SELLER~
POC*1*CA*5*10*EA*2.50**VN*ABC~
ACK*IA*5*EA~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_865>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bca._01, "00");
    assert_eq!(t.loop_poc.len(), 1);
    assert_eq!(t.loop_poc[0].loop_ack.len(), 1);
}
