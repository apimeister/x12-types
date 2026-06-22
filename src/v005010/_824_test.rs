use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*824*021390001*005010X186A1~BGN*11*FFA.ABCDEF.123456*20020709*0932**123456789**WQ~N1*41*ABC INSURANCE*46*111111111~PER*IC*JOHN JOHNSON*TE*8005551212*EX*1439~N1*40*SMITHCO*46*A1234~OTI*TA*TN*NA***20020709*0902*2*0001*834*005010X220A1~SE*7*021390001~";

#[test]
fn parse_824() {
    let (rest, obj) = _824::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "824");
    // two party loops, first carries the PER
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n1._01, "41");
    assert_eq!(obj.loop_n1[0].per.len(), 1);
    assert_eq!(obj.loop_n1[1].n1._01, "40");
    // one OTI loop
    assert_eq!(obj.loop_oti.len(), 1);
    assert_eq!(obj.loop_oti[0].oti._01, "TA");
    assert_eq!(obj.loop_oti[0].oti._03, "NA");
}

#[test]
fn roundtrip_824() {
    let (_, first) = _824::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _824::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_824() {
    let str = r#"ISA*00*          *00*          *08*9254110060     *ZZ*123456789      *041216*0805*U*00501*000095071*0*P*>~
GS*AG*5137624388*123456789*20041216*0805*95071*X*005010~
ST*824*021390001*005010X186A1~
BGN*11*FFA.ABCDEF.123456*20020709*0932**123456789**WQ~
N1*41*ABC INSURANCE*46*111111111~
PER*IC*JOHN JOHNSON*TE*8005551212*EX*1439~
N1*40*SMITHCO*46*A1234~
OTI*TA*TN*NA***20020709*0902*2*0001*834*005010X220A1~
SE*7*021390001~
GE*1*95071~
IEA*1*000095071~"#;
    let (rest, obj) = Transmission::<_824>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        obj.functional_group[0].segments[0].loop_oti[0].oti._01,
        "TA"
    );
}
