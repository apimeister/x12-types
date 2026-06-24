use crate::util::Parser;
use crate::v005010::*;

// Healthcare provider information: HL-based. Heading BHT/DTM/PER, then a 2000 HL loop with
// a 2100 NM1 loop carrying demographic detail (DMG/DEG/DTP/QTY) and its NX1 (2110),
// LQ (2120) and HPL (2130) sub-loops.
const SAMPLE: &str = r#"ST*274*0001~
BHT*0010*08*REF1*20200101~
DTM*007*20200101~
PER*IC*JOHN~
HL*1**20~
TRN*1*TRACE1~
NM1*1P*1*SMITH*JOHN****XX*1234567890~
PER*IC*OFFICE*TE*5551234~
DMG*D8*19700101*M~
DEG*BA~
DTP*348*D8*20200101~
QTY*QA*5~
NX1*BL~
N3*1 MAIN ST~
N4*DALLAS*TX*75201~
LQ*AS*ZZ~
N1*1P*PROVIDER~
HPL*0B*LIC123*A*TX~
DTP*007*D8*20200101~
SE*19*0001~"#;

#[test]
fn parse_274() {
    let (rest, obj) = _274::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "274");
    assert_eq!(obj.bht._01.to_string(), "0010");
    assert!(obj.dtm.is_some());
    assert!(obj.per.is_some());
    assert_eq!(obj.loop_2000.len(), 1);
    let hl = &obj.loop_2000[0];
    assert!(hl.trn.is_some());
    assert_eq!(hl.loop_2100.len(), 1);
    let nm1 = &hl.loop_2100[0];
    assert_eq!(nm1.nm1._01.to_string(), "1P");
    assert_eq!(nm1.per.len(), 1);
    assert!(nm1.dmg.is_some());
    assert_eq!(nm1.deg.len(), 1);
    assert_eq!(nm1.dtp.len(), 1);
    assert_eq!(nm1.qty.len(), 1);
    assert_eq!(nm1.loop_2110.len(), 1);
    assert_eq!(nm1.loop_2110[0].n3.len(), 1);
    assert_eq!(nm1.loop_2120.len(), 1);
    assert_eq!(nm1.loop_2120[0].n1.len(), 1);
    assert_eq!(nm1.loop_2130.len(), 1);
    assert_eq!(nm1.loop_2130[0].dtp.len(), 1);
}

#[test]
fn roundtrip_274() {
    let (_, a) = _274::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _274::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_274() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*HU*S*R*20200101*1200*1*X*005010~
ST*274*0001~
BHT*0010*08*REF1*20200101~
HL*1**20~
NM1*1P*1*SMITH*JOHN****XX*1234567890~
SE*4*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_274>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bht._01.to_string(), "0010");
    assert_eq!(t.loop_2000.len(), 1);
    assert_eq!(t.loop_2000[0].loop_2100.len(), 1);
}
