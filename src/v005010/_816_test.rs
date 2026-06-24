use crate::util::Parser;
use crate::v005010::*;

// Heading party loop plus a detail HL hierarchy: a parent org with two subordinate
// levels, each carrying a detail N1 sub-loop with quantity and action/status detail.
const SAMPLE: &str = r#"ST*816*0001~
BHT*0001*00*REF1*20200101*1200~
REF*ZZ*RELATIONSHIP~
N1*41*PARENT ORG*92*PARENT01~
N3*100 HQ BLVD~
N4*NEW YORK*NY*10001*US~
HL*1**1~
N1*40*SUBSIDIARY ORG*92*SUB01~
N4*BOSTON*MA*02101*US~
QTY*82*5~
ASI*A*001~
HL*2*1*2~
N1*40*DIVISION ORG*92*DIV01~
N4*CHICAGO*IL*60601*US~
SE*15*0001~"#;

#[test]
fn parse_816() {
    let (rest, obj) = _816::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "816");
    assert_eq!(obj.bht._01.to_string(), "0001");
    assert_eq!(obj.r#ref.len(), 1);
    // one heading party
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "41");
    assert_eq!(obj.loop_n1[0].n3.len(), 1);
    // two hierarchical levels in the detail
    assert_eq!(obj.loop_hl.len(), 2);
    assert_eq!(obj.loop_hl[0].hl._01, "1");
    assert_eq!(obj.loop_hl[0].loop_n1.len(), 1);
    let party = &obj.loop_hl[0].loop_n1[0];
    assert_eq!(party.n1._01.to_string(), "40");
    assert_eq!(party.qty.len(), 1);
    assert!(party.asi.is_some());
    assert_eq!(obj.loop_hl[1].hl._02.as_deref(), Some("1"));
}

#[test]
fn roundtrip_816() {
    let (_, a) = _816::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _816::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_816() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*OR*S*R*20200101*1200*1*X*005010~
ST*816*0001~
BHT*0001*00*REF1*20200101*1200~
N1*41*PARENT ORG*92*PARENT01~
HL*1**1~
N1*40*SUB ORG*92*SUB01~
N4*BOSTON*MA*02101*US~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_816>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_n1.len(), 1);
    assert_eq!(t.loop_hl.len(), 1);
    assert_eq!(t.loop_hl[0].loop_n1[0].n1._01.to_string(), "40");
}
