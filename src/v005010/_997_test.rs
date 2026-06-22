use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*997*1234~AK1*HC*17456~AK2*837*0001~AK5*A~AK2*837*0002~AK3*CLM*5~AK4*1*1722*1~AK5*R~AK9*P*2*2*1~SE*9*1234~";

#[test]
fn parse_997() {
    let (rest, obj) = _997::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "997");
    assert_eq!(obj.ak1._01, Some("HC".to_string()));
    // two transaction-set responses (AK2 loops), the 2nd with an AK3 sub-loop
    assert_eq!(obj.loop_ak2.len(), 2);
    assert_eq!(obj.loop_ak2[0].ak5._01, "A");
    assert_eq!(obj.loop_ak2[1].loop_ak3.len(), 1);
    assert_eq!(obj.ak9._01, Some("P".to_string()));
}

#[test]
fn roundtrip_997() {
    let (_, first) = _997::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _997::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_997() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*FA*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*997*1234~
AK1*HC*17456~
AK2*837*0001~
AK5*A~
AK9*A*1*1*1~
SE*6*1234~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_997>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        obj.functional_group[0].segments[0].ak1._01,
        Some("HC".to_string())
    );
}
