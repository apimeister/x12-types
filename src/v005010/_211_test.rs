use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*211*0001~BOL*SCAC*PP*BOL123456*20200101~B2A*00~N1*SH*SHIPPER NAME*93*SHIP01~N3*123 MAIN ST~N4*ANYTOWN*CA*90001*US~N1*CN*CONSIGNEE NAME*93*CONS01~AT1*1~AT4*GENERAL MERCHANDISE~AT2*10*CTN*G*L*5000~MAN*GM*1234567890~L4*48*40*36*I~LX*1~MAN*GM*9999999999~AT1*2~AT4*HAZARDOUS GOODS~G61*IC*SAFETY OFFICER~LH6*HAZMAT CERT~LH1*UN*1203~LH2*3~LH3*GASOLINE~SE*22*0001~";

#[test]
fn parse_211() {
    let (rest, obj) = _211::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "211");
    assert_eq!(obj.bol._01, "SCAC");
    assert_eq!(obj.bol._03, "BOL123456");
    assert_eq!(obj.b2a._01.to_string(), "00");
    // two header parties (ship-from, consignee)
    assert_eq!(obj.loop_100.len(), 2);
    assert_eq!(obj.loop_100[0].n1.as_ref().unwrap()._01.to_string(), "SH");
    assert_eq!(obj.loop_100[1].n1.as_ref().unwrap()._01.to_string(), "CN");
    // two bill-of-lading line items
    assert_eq!(obj.loop_200.len(), 2);
    assert_eq!(obj.loop_200[0].at1._01.to_string(), "1");
    assert_eq!(obj.loop_200[1].at1._01.to_string(), "2");
    // line item 1 carries the AT2 detail loop and an LX marks loop
    assert_eq!(obj.loop_200[0].loop_210.len(), 1);
    assert_eq!(obj.loop_200[0].loop_210[0].at2._05.to_string(), "5000");
    assert_eq!(
        obj.loop_200[0].loop_210[0]
            .l4
            .as_ref()
            .unwrap()
            ._01
            .to_string(),
        "48"
    );
    assert_eq!(obj.loop_200[0].loop_220.len(), 1);
    assert_eq!(obj.loop_200[0].loop_220[0].lx._01, "1");
    // line item 2 carries the hazardous-material loop
    assert_eq!(obj.loop_200[1].loop_230.len(), 1);
    assert_eq!(obj.loop_200[1].loop_230[0].loop_231.len(), 1);
    assert_eq!(
        obj.loop_200[1].loop_230[0].loop_231[0].lh1._02.to_string(),
        "1203"
    );
}

#[test]
fn roundtrip_211() {
    let (_, first) = _211::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _211::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_211() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*BL*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*211*0001~
BOL*SCAC*PP*BOL123456*20200101~
B2A*00~
MS3*SCAC*O*ROUTE~
MS2*SCAC*TRL12345*TL~
L11*REF001*BM~
G62*10*20200101~
AT5*HM~
K1*HANDLE WITH CARE~
N1*SH*SHIPPER NAME*93*SHIP01~
N3*123 MAIN ST~
N4*ANYTOWN*CA*90001*US~
G61*IC*JOHN SHIPPER*TE*5551112222~
N1*CN*CONSIGNEE NAME*93*CONS01~
N3*456 OAK AVE~
N4*OTHERTOWN*NY*10001*US~
AT1*1~
L11*PO12345*PO~
AT3*10000*FR*1.5~
AT4*GENERAL MERCHANDISE~
AT2*10*CTN*G*L*5000~
MAN*GM*1234567890~
OID*ID001*PO9988*5*CA*5000~
L4*48*40*36*I~
LX*1~
MAN*GM*9999999999~
AT1*2~
AT4*HAZARDOUS GOODS~
G61*IC*SAFETY OFFICER~
LH6*HAZMAT CERT~
LH1*UN*1203~
LH2*3~
LH3*GASOLINE~
SE*34*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_211>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bol._03, "BOL123456");
    assert_eq!(t.loop_100.len(), 2);
    assert_eq!(t.loop_200.len(), 2);
    // header contact captured inside the first party loop
    assert_eq!(t.loop_100[0].g61.len(), 1);
    assert_eq!(t.loop_100[0].g61[0]._02, "JOHN SHIPPER");
}
