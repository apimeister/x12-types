use crate::util::Parser;
use crate::v004010::*;

#[test]
fn parse_322_1() {
    // source https://www.hamburgsud-line.com/liner/media/hamburg_sud_liner_shipping/ecommerce/edi_api/message_implementation_guides/X_4010_322_HSDG_v1.0.0.pdf
    let s = r#"ISA*00* *00* *ZZ*HAMSUD*ZZ*PARTNERID*160526*2245*U*00401*053849086*0*P*>~
GS*SO*PARTNERID*HAMSUD*20160526*224500*1000*X*004010~
ST*322*0001~
Q5*A*20160526*214520*LT**SAINT PAUL*MN*US~
N7*HASU*431617*43459*G*******CN*BNSF*****L*0****451G*BNSF~
DTM*371*20160528*091200*LT~
DTM*017*20160529*080000*LT~
DTM*830*20160528*160000*LT~
M7*SN1234567*SN1234568~
W2*HASU*431617**CC*L********0~
R4*5*UN*USSTP*SAINT PAUL*US***MN~
R4*6*UN*USLGB*LONG BEACH*US***CA~
R4*7*UN*USCHI*BNSF CHICAGO RAMP (CICERO)*US***IL~
N1*RR*BNSF Railway*ZZ*BNSF~
N9*BN*6PHLSA1234~
N9*BM*A5GEMEN1976X~
SE*15*0001~
GE*1*1000~
IEA*1*053849086~"#;
    let (rest, obj) = Transmission::<_322>::parse(s).unwrap();
    println!("{rest}");
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn parse_322_2() {
    // source https://www.hamburgsud-line.com/liner/media/hamburg_sud_liner_shipping/ecommerce/edi_api/message_implementation_guides/X_4010_322_HSDG_v1.0.0.pdf
    let s = r#"ISA*00* *00* *ZZ*HAMSUD*ZZ*PARTNERID*160526*2245*U*00401*053849086*0*P*>~
GS*SO*PARTNERID*HAMSUD*20160526*224500*1000*X*004010~
ST*322*0001~
Q5*OA*20160610*104520*LT**BNSF CHICAGO (LOGISTICS PARK)*IL*US~
N7*HASU*431617*43459*G*******CN*BNSF*****L*0****451G*BNSF~
M7*SN1234567*SN1234568~
W2*HASU*431617**CC*L********0~
R4*5*UN*USCHI*BNSF CHICAGO (LOGISTICS PARK)*US***IL~
R4*6*UN*USLGB*LONG BEACH*US***CA~
R4*7*UN*USCHI*BNSF CHICAGO RAMP (CICERO)*US***IL~
N1*RR*BNSF Railway*ZZ*BNSF~
N9*BN*6PHLSA1234~
N9*BM*A5GEMEN1976X~
SE*12*0001~
GE*1*1000~
IEA*1*053849086~"#;
    let (rest, obj) = Transmission::<_322>::parse(s).unwrap();
    println!("{rest}");
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn parse_322_3() {
    // source https://www.hamburgsud-line.com/liner/media/hamburg_sud_liner_shipping/ecommerce/edi_api/message_implementation_guides/X_4010_322_HSDG_v1.0.0.pdf
    let s = r#"ISA*00* *00* *ZZ*HAMSUD*ZZ*PARTNERID*160526*2245*U*00401*053849086*0*P*>~
GS*SO*PARTNERID*HAMSUD*20160526*224500*1000*X*004010~
ST*322*0001~
Q5*NF*20160625*120000*LT**BNSF CHICAGO (LOGISTICS PARK)*IL*US~
N7*HASU*431617*43459*G*******CN*BNSF*****L*0****451G*BNSF~
M7*SN1234567*SN1234568~
W2*HASU*431617**CC*L********0~
R4*5*UN*USCHI*BNSF CHICAGO (LOGISTICS PARK)*US***IL~
R4*6*UN*USLGB*LONG BEACH*US***CA~
R4*7*UN*USCHI*BNSF CHICAGO RAMP (CICERO)*US***IL~
N1*RR*BNSF Railway*ZZ*BNSF~
N9*BN*6PHLSA1234~
N9*BM*A5GEMEN1976X~
SE*12*0001~
GE*1*1000~
IEA*1*053849086~"#;
    let (rest, obj) = Transmission::<_322>::parse(s).unwrap();
    println!("{rest}");
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn parse_322_4() {
    // source https://www.hamburgsud-line.com/liner/media/hamburg_sud_liner_shipping/ecommerce/edi_api/message_implementation_guides/X_4010_322_HSDG_v1.0.0.pdf
    let s = r#"ISA*00* *00* *ZZ*HAMSUD*ZZ*PARTNERID*160526*2245*U*00401*053849086*0*P*>~
GS*SO*PARTNERID*HAMSUD*20160526*224500*1000*X*004010~
ST*322*0001~
Q5*AL*20160524*073000*LT**LONG BEACH*CA*US~
N7*HASU*431617*43459*G*******CN*BNSF*****L*0****451G*BNSF~
DTM*371*20160528*160000*LT~
DTM*830*20160528*160000*LT~
M7*SN1234567*SN1234568~
W2*HASU*431617**CC*L********0~
R4*5*UN*USLGB*LONG BEACH*US***CA~
R4*6*UN*USLGB*LONG BEACH*US***CA~
R4*7*UN*USCHI*BNSF CHICAGO RAMP (CICERO)*US***IL~
N1*RR*BNSF Railway*ZZ*BNSF~
N9*BN*6PHLSA1234~
N9*BM*A5GEMEN1976X~
SE*12*0001~
GE*1*1000~
IEA*1*053849086~"#;
    let (rest, obj) = Transmission::<_322>::parse(s).unwrap();
    println!("{rest}");
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn parse_322_5() {
    // source https://www.1edisource.com/resources/edi-transactions-sets/edi-322/
    let s = r#"ISA*01*0000000000*01*0000000000*ZZ*ABCDEFGHIJKLMNO*ZZ*123456789012345*101127*1719*U*00400*000003438*0*P*>~
GS*NL*1234567890*999999999*20120126*1211*1*T*004010~
ST*322*0001~
Q5*GI*20090925*0112*ET~
N7*ABCZ*544037**********CC*ABCU~
R4*5*SL*218317*PITCAIRN****PA~
N1*MC*JOE’S TRUCKING COMPANY*2*JTCI~
N1*DR*RAILROAD INC**~
N1*D1*JOE TRUCKER**~
N9*5J*123456*PA~
N9*X3*02*LIGHTS~
N9*X3*03*WHEEL~
SE*10*0001~
GE*1*1~
IEA*1*0000000012~"#;
    let (rest, obj) = Transmission::<_322>::parse(s).unwrap();
    println!("{rest}");
    println!("{obj:?}");
    assert!(rest.is_empty());
}
