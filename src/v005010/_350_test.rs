use crate::util::Parser;
use crate::v005010::*;

// Import (P4) flow: a port loop with two nested customs-release (X4) loops.
const SAMPLE: &str = r#"ST*350*0001~
M10*SCAC*O*US*1234*EVER GIVEN*VOY001~
P4*USNYC*20200115~
V9*ARR~
VID*CN*ABCD*1234567~
K1*PORT REMARK~
X4*BOL123*1*01*ENTRY001*20200115*1200*AR*BOL123*SCAC~
K1*RELEASE REMARK~
N9*BM*BOL123~
N7*CONT*ABCD1234567~
X4*BOL456*1*01*ENTRY002*20200116*1200*HD*BOL456*SCAC~
K1*HOLD REMARK~
SE*13*0001~"#;

// Export (BA1) flow: an export-shipment loop with a nested X4 loop.
const SAMPLE_BA1: &str = "ST*350*0001~BA1*1*A*O*US*REF001*10001*US*NY*AUTHORITY*SCAC~X4*BOL789*1*01*ENTRY003*20200117*1200*AR*BOL789*SCAC~K1*EXPORT REMARK~SE*5*0001~";

#[test]
fn parse_350_p4() {
    let (rest, obj) = _350::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "350");
    assert_eq!(obj.m10.as_ref().unwrap()._01, "SCAC");
    assert_eq!(obj.loop_p4.len(), 1);
    let p4 = &obj.loop_p4[0];
    assert_eq!(p4.p4._01, "USNYC");
    assert_eq!(p4.v9.len(), 1);
    assert_eq!(p4.vid.len(), 1);
    assert_eq!(p4.k1.len(), 1);
    // two customs-release loops; the first carries N9 + N7
    assert_eq!(p4.loop_x4.len(), 2);
    assert_eq!(p4.loop_x4[0].x4._07, "AR");
    assert_eq!(p4.loop_x4[0].n9.len(), 1);
    assert_eq!(p4.loop_x4[0].n7.len(), 1);
    assert_eq!(p4.loop_x4[1].x4._07, "HD");
    assert!(obj.loop_ba1.is_empty());
}

#[test]
fn parse_350_ba1() {
    let (rest, obj) = _350::parse(SAMPLE_BA1).unwrap();
    assert_eq!(rest, "");
    assert!(obj.loop_p4.is_empty());
    assert_eq!(obj.loop_ba1.len(), 1);
    let ba1 = &obj.loop_ba1[0];
    assert_eq!(ba1.ba1._05, "REF001");
    assert_eq!(ba1.ba1._10, "SCAC");
    assert_eq!(ba1.loop_x4.len(), 1);
    assert_eq!(ba1.loop_x4[0].x4._01.as_deref(), Some("BOL789"));
    assert_eq!(ba1.loop_x4[0].k1.len(), 1);
}

#[test]
fn roundtrip_350() {
    for s in [SAMPLE, SAMPLE_BA1] {
        let (_, first) = _350::parse(s).unwrap();
        let rendered = format!("{first}");
        let (rest, second) = _350::parse(&rendered).unwrap();
        assert_eq!(rest, "");
        assert_eq!(first, second);
    }
}

#[test]
fn full_transmission_350() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*AR*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*350*0001~
M10*SCAC*O*US*1234*EVER GIVEN*VOY001~
P4*USNYC*20200115~
X4*BOL123*1*01*ENTRY001*20200115*1200*AR*BOL123*SCAC~
N9*BM*BOL123~
SE*5*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_350>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_p4.len(), 1);
    assert_eq!(t.loop_p4[0].loop_x4.len(), 1);
    assert_eq!(t.loop_p4[0].loop_x4[0].x4._09, "SCAC");
}
