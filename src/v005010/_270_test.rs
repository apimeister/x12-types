use crate::util::Parser;
use crate::v005010::Transmission;
use crate::v005010::_270;

#[test]
fn parse_270_01() {
    //source: https://github.com/EdiFabric/X12.NET/blob/master/Files/HIPAA/GenericRequest.txt
    let str = r#"ISA*00*          *00*          *ZZ*1234567        *ZZ*11111          *170508*1141*^*00501*000000101*1*P*:~
GS*HC*XXXXXXX*XXXXX*20170617*1741*101*X*005010X279A1~
ST*270*1234*005010X279A1~
BHT*0022*13*10001234*20060501*1319~
HL*1**20*1~
NM1*PR*2*ABC COMPANY*****PI*842610001~
HL*2*1*21*1~
NM1*1P*2*BONE AND JOINT CLINIC*****SV*2000035~
HL*3*2*22*0~
TRN*1*93175-012547*9877281234~
NM1*IL*1*SMITH*ROBERT****MI*11122333301~
DMG*D8*19430519~
DTP*291*D8*20060501~
EQ*30~
SE*13*1234~
GE*1*101~
IEA*1*000000101~"#;
    let parsed = Transmission::<_270>::parse(str).unwrap();
    println!("{parsed:?}");
}

#[test]
fn parse_270_basic_structure() {
    let str = r#"ISA*00*          *00*          *ZZ*1234567        *ZZ*11111          *170508*1141*^*00501*000000101*1*P*:~
GS*HC*XXXXXXX*XXXXX*20170617*1741*101*X*005010X279A1~
ST*270*1234*005010X279A1~
BHT*0022*13*10001234*20060501*1319~
HL*1**20*1~
NM1*PR*2*ABC COMPANY*****PI*842610001~
SE*13*1234~
GE*1*101~
IEA*1*000000101~"#;
    let (_, parsed) = Transmission::<_270>::parse(str).unwrap();

    // Verify basic structure
    assert_eq!(parsed.functional_group.len(), 1);
    let fg = &parsed.functional_group[0];
    assert_eq!(fg.segments.len(), 1);
    let transaction = &fg.segments[0];

    // Verify ST segment
    assert_eq!(transaction.st._01, "270");
    assert_eq!(transaction.st._02, "1234");

    // Verify BHT segment
    assert_eq!(transaction.bht._01, "0022");
    assert_eq!(transaction.bht._02, "13");

    // Verify SE segment
    assert_eq!(transaction.se._01, "13");
    assert_eq!(transaction.se._02, "1234");

    // Verify loop structure
    assert_eq!(transaction.loop_2000.len(), 1);
    let loop_2000 = &transaction.loop_2000[0];
    assert_eq!(loop_2000.hl._01, "1");
    assert_eq!(loop_2000.hl._03, "20");

    assert_eq!(loop_2000.loop_2100.len(), 1);
    let loop_2100 = &loop_2000.loop_2100[0];
    assert_eq!(loop_2100.nm1._01, "PR");
    assert_eq!(loop_2100.nm1._02, "2");
    assert_eq!(loop_2100.nm1._03, Some("ABC COMPANY".to_string()));
}
