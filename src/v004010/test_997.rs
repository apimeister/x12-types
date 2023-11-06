use nom::multi::many0;

use crate::util::Parser;
use crate::v004010::_997;

#[test]
fn test_997_1() {
    // source https://community.boomi.com/s/article/x12997acknowledgementdetailedguide
    let s = r#"ST*997*0001~
AK1*PO*2~
AK2*850*103465910~
AK3*REF*3**8~
AK4*1**7*BM~
AK3*REF*4**8~
AK4*1**7*DP~
AK5*R*4*5~
AK9*R*1*1*0~
SE*10*0001~"#;
    let (rest, obj) = _997::parse(s).unwrap();
    println!("{rest}");
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn test_997_2() {
    // source https://community.boomi.com/s/article/x12997acknowledgementdetailedguide
    let s = r#"ST*997*0001~
AK1*PO*2~
AK2*850*103465910~
AK5*A~
AK2*850*103465911~
AK5*R*4~
AK9*R*2*2*0*5~
SE*8*0001~"#;
    let (rest, obj) = _997::parse(s).unwrap();
    println!("{rest}");
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn test_997_3() {
    // source https://community.boomi.com/s/article/x12997acknowledgementdetailedguide
    let s = r#"ST*997*0001~
AK1*PO*2~
AK2*850*103465910~
AK5*A~
AK2*850*103465911~
AK5*R*4~
AK9*R*2*2*0*5~
SE*8*0001~
ST*997*0002~
AK1*SH*3~
AK9*A*0*0*0~
SE*4*0002~"#;
    let (rest, obj) = many0(_997::parse)(s).unwrap();
    println!("{rest}");
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn test_997_4() {
    // source https://help.sap.com/docs/business-network-for-trading-partners/edi-configuration/x12-997
    let s = r#"ST*997*0001~
AK1*PO*000341217~
AK2*850*0001~
AK5*A~
AK9*A*1*1*1~
SE*6*0001~"#;
    let (rest, obj) = _997::parse(s).unwrap();
    println!("{rest}");
    println!("{obj:?}");
    assert!(rest.is_empty());
}
