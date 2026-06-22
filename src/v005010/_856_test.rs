use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*856*0001~BSN*00*0001*20200101*1200~HL*1**S~TD1*CTN25*2~REF*BM*BOL123~HL*2*1*O~PRF*PO456~SE*7*0001~";

#[test]
fn parse_856() {
    let (rest, obj) = _856::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "856");
    assert_eq!(obj.bsn._01, "00");
    // two hierarchical levels, both top-level
    assert_eq!(obj.loop_hl.len(), 2);
    assert_eq!(obj.loop_hl[0].hl._01, "1");
    assert_eq!(obj.loop_hl[1].hl._01, "2");
    assert_eq!(obj.loop_hl[0].td1.len(), 1);
    assert_eq!(obj.loop_hl[1].prf.len(), 1);
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
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_856>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bsn._01, "00");
}
