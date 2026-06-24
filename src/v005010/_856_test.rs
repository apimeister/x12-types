use crate::util::Parser;
use crate::v005010::*;

// Shipment -> order -> item hierarchy. The shipment level carries two parties (ST, SF)
// in the N1 sub-loop -- the case the previous flat model could not round-trip.
const SAMPLE: &str = r#"ST*856*0001~
BSN*00*SHIP001*20200101*1200~
DTM*011*20200101~
HL*1**S~
TD1*CTN25*10~
TD5**2*UPSN*M~
REF*BM*BOL123~
DTM*011*20200101~
N1*ST*SHIP TO NAME*92*STORE01~
N3*123 MAIN ST~
N4*DALLAS*TX*75201*US~
N1*SF*SHIP FROM NAME*92*WH01~
N4*CHICAGO*IL*60601*US~
HL*2*1*O~
PRF*PO9988~
REF*IV*INV001~
HL*3*2*I~
LIN**VP*ITEM001*UP*012345678905~
SN1**100*EA~
PID*F****WIDGET~
CTT*3~
SE*22*0001~"#;

#[test]
fn parse_856() {
    let (rest, obj) = _856::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "856");
    assert_eq!(obj.bsn._01, "00");
    assert!(obj.ctt.is_some());
    // shipment / order / item levels
    assert_eq!(obj.loop_hl.len(), 3);
    let ship = &obj.loop_hl[0];
    assert_eq!(ship.hl._03.to_string(), "S");
    assert_eq!(ship.td1.len(), 1);
    assert_eq!(ship.td5.len(), 1);
    // two parties on the shipment level -- the multi-party case
    assert_eq!(ship.loop_n1.len(), 2);
    assert_eq!(ship.loop_n1[0].n1._01.to_string(), "ST");
    assert_eq!(ship.loop_n1[0].n3.len(), 1);
    assert!(ship.loop_n1[0].n4.is_some());
    assert_eq!(ship.loop_n1[1].n1._01.to_string(), "SF");
    // order level
    let order = &obj.loop_hl[1];
    assert_eq!(order.hl._03.to_string(), "O");
    assert!(order.prf.is_some());
    assert_eq!(order.r#ref.len(), 1);
    // item level
    let item = &obj.loop_hl[2];
    assert_eq!(item.hl._03.to_string(), "I");
    assert!(item.lin.is_some());
    assert!(item.sn1.is_some());
    assert_eq!(item.pid.len(), 1);
}

#[test]
fn roundtrip_856() {
    let (_, first) = _856::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _856::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_856() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*SH*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*856*0001~
BSN*00*0001*20200101*1200~
HL*1**S~
TD1*CTN25*2~
REF*BM*BOL123~
N1*ST*STORE*92*STORE01~
N4*DALLAS*TX*75201*US~
SE*7*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_856>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bsn._01, "00");
    assert_eq!(t.loop_hl.len(), 1);
    assert_eq!(t.loop_hl[0].loop_n1.len(), 1);
    assert_eq!(t.loop_hl[0].loop_n1[0].n1._01.to_string(), "ST");
}
