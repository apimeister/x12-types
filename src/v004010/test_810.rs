use crate::v004010::*;

#[test]
fn test_810_1() {
    // source https://channelpartners.adobe.com/content/dam/cpp_assets/assets/public/edi/EDI_810_ANSI_X12_122007_001.pdf
    let str = r#"ISA*00* *00* *16*102096559TEST *14*PARTNERTEST*071214*1406*U*00040*810000263*1*T*>~
GS*IN*102096559TEST*PARTNER*20071214*1406*810000263*X*004010~
ST*810*166061414~
BIG*20070129*0013833070*20070129*V8748745***DI*00~
NTE*GEN*If 0% VAT is charged and your VAT ID number is displayed above, this~
NTE*GEN*is either an exempt or a reverse charge transaction.~
NTE*GEN*This supply is exempt from VAT according to the section 64 of the Czec~
NTE*GEN*h VAT Act.~
CUR*SE*USD~
REF*61*noval*Adobe VAT Number~
REF*CO*121212~
REF*VN*0070861270~
N1*II*ADOBE SYSTEMS INCORPORATED*91*0000012137~
N2*ADOBE SYSTEMS INCORPORATED~
N3*345 PARK AVENUE~
N4*SAN JOSE*CA*95110*US~
N1*BY*SOFTWARE.*91*0000012137~
N2*SOFTWARE~
N3*111 MAIN DR~
N4*PLANO*TX*75075*US~
N1*BT*SOFTWARE.*91*0000012137~
N2*SOFTWARE~
N3*111 MAIN DR~
N4*PLANO*TX*75075*US~
N1*PR*SOFTWARE*91*0000012137~
N2*SOFTWARE~
N3*111 MAIN DR~
N4*PLANO*TX*75075*US~
N1*ST*SHIP TO PARTNER*91*0050480425~
N2*SHIP TO~
N3*122 MAIN AVE~
N4*SANTA CLARA*CA*95050*US~
PER*CN*JOHN DOE~
N1*SE*ADOBE SYSTEMS INCORPORATED*91*ADUS~
N2*ADOBE SYSTEMS INCORPORATED~
N3*345 Park Avenue~
N4*SAN JOSE*CA*95110*US~
N1*EN**91*0020064630~
N1*DU**91*0010013997~
ITD*14*3*****35*****Up to 03/05/2007 without deduction~
DTM*011*20070129~
IT1*000010*1*EA*160*CP*VP*65008841AB02A00*BP*7167946*UP*883919019161~
PID*F****ACRO,8.0,WIN,AOO,UE,1PK,N/A~
TDS*16000~
TXI*ST*0*0******noval*Customer VAT Number~
CTT*1~
SE*44*166061414~
GE*1*810000263~
IEA*1*810000263~
"#;
    let (rest, obj) = Transmission::<_810>::parse(str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
    let s = format!("{obj}");
    assert_eq!(s, str);
}

#[test]
fn test_810_2() {
    // source https://www.edi2xml.com/blog/what-is-an-ansi-asc-x12-edi-810-invoice/
    let str = r#"ISA*00*          *00*          *12*5141231234     *12*5034564567     *181122*1220*U*00501*000000001*0*T*>~
GS*IN*5141231234*504564567*20181122*122047*1*X*005010~
ST*810*0001~
BIG*20181122*I-0042537~
N1*RI**92*10055500~
N1*ST**92*00262~
ITD*05*1*****100~
DTM*011*20190120~
IT1**2*EA*5.45**UP*888077648572~
SDQ*EA*92*00682*1*70674*1~
IT1**2*EA*5.45**UP*888077650123~
SDQ*EA*92*00111*1*11356*1~
IT1**3*EA*4.85**UP*888077648954~
SDQ*EA*92*00682*1*11356*1*70674*1~
IT1**3*EA*5.15**UP*888077649105~
SDQ*EA*92*00011*2*00111*1~
IT1**1*EA*5.9**UP*888077648867~
SDQ*EA*92*00682*1~
TDS*5770~
CAD*A***FDEG~
CTT*5~
SE*20*0001~
GE*1*1~
IEA*1*000000001~
"#;
    let (rest, obj) = Transmission::<_810>::parse(str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
    let s = format!("{obj}");
    assert_eq!(s, str);
}

#[test]
fn test_810_3() {
    // source https://www.stedi.com/app/guides/view/maersk-line/invoice/01H3M9T93PWMVDA81W76Y6EQJ2/inspector?sample=01H3M9T9PAXB881E71YDNRX81F&value=EISQ4gVATADFUEY4FYIFUDK8C0M8GYARMAeSggosJAD8AoAYTQCUIwB2N4ABRoAI6zAKIAxCNxIR8ANgAs86dPYBOGMnrCxEqXIVLV6waIgAJE12WXL-OgDkEEADIB1CAFkAhgFMATgGcAaz4AQQB6DD4hPwAHABsPADsPABMvBL5kGAgALWyIJDwNYzMIQgBpZHwEfGVpZEMANQdlKGUADmkO92ChZgwyvhASZ2CKBFkEcjcetEpHCAahDVlSyjBgEQANABUbEG2HZtLR%2BQQAOnx8cSWBBm3uSkepZXYoem4QQggxCGCV7bwCBUEGEol6fAYJAw2ww9AYwS%2BiwosngVmU7lsm02c1QCDhCIWQmRqKsGKxOOg9AwjlsEBgDnpvwg6KEwXohG2bnyXVg8EmUGkVJpdIZ5FGLLZdA5XIQPLgiHg7Ho20IGGer2VmxAEG22KyWQoS0Y222%2BRoQA&inspectorView=rich
    let s = r#"ST*810*0001~
BIG*20221025*US22-0003DGO2*****DI~
CUR*G7*GBP~
REF*PO*36444667905~
REF*PO*36444667905~
REF*HH*GB99999~
N1*LW*Maersk A/S Esplanaden 50*ZZ*1000~
REF*HH*DK53139655~
V1*9298686*MAERSK IOWA**1412*MAEU***L*VE~
R4*D***GBFXT~
IT1*1*9*DA*441.33*PE~
CTP********3972~
PID*F**A4*T00179*REEFER COSTS~
CAD*VE**4229999*MNXX***L5*1~
CAD*VE**4229999*MNXX***L5*2~
SLN*01*01*A*9*EA~
DTM*186*20221226~
SLN*01*02*A*9*EA~
DTM*186*20221227~
TDS*3972~
TXI*TX*0*0***E~
CTT*1~
SE*23*0001~
"#;
    let (rest, obj) = _810::parse(s).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
    let s2 = format!("{obj}");
    assert_eq!(s, s2);
}