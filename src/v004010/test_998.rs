use crate::util::Parser;
use crate::v004010::{Transmission, _998};

#[test]
fn test_998_parse() {
    let s = r#"ISA*00*SWBOL     *00*          *ZZ*XXXXXXX        *ZZ*XXXXXX         *230523*1816*U*00401*000097614*0*P*>~
GS*SR*XXXXXXX*XXXXXX*20230523*1816*97614*X*004010~
ST*998*2577~
ZD*404*3PHLT00XXX*TRHU*653199*3PHLT00XXX**CA~
SE*3*2577~
GE*1*97614~
IEA*1*000097614~"#;
    let (rest, obj) = Transmission::<_998>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.iea._02, "000097614");
}
