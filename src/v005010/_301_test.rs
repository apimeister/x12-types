use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*301*0001~B1*SCAC*BOOK001*20200101*A~G61*IC*BOOKING AGENT*TE*5551234~Y6*00*AUTHORITY*AUTH123~Y3*BOOK001*SCAC*20200110*20200115*USNYC*PIER 5*20200112~Y4*BOOK001*REL001*20200112*USNYC*2*40HC*SCAC~W09*A*40*F~N9*BN*BOOK001~R2A*USNYC*1~N1*SH*SHIPPER NAME*25*SHIP01~N3*123 MAIN ST~N4*NEW YORK*NY*10001*US~N1*CN*CONSIGNEE NAME*25*CONS01~N4*ROTTERDAM**3000*NL~R4*L*UN*USNYC*NEW YORK*US~DTM*649*20200115~R4*D*UN*NLRTM*ROTTERDAM*NL~DTM*649*20200201~W09*A*40*F~H3*HOT~EA*BR~LX*1~N7*CONT*ABCD1234567~W09*A*40*F~L0*1~L5*1*GENERAL CARGO~H1*UN1203*3*3~H2*FLAMMABLE~LH1*1*UN*1203~LH3*GASOLINE~V1*IMO9999999*EVER GIVEN*PA*VOY01~V9*ARR~SE*33*0001~";

#[test]
fn parse_301() {
    let (rest, obj) = _301::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "301");
    assert_eq!(obj.b1._02.as_deref(), Some("BOOK001"));
    assert_eq!(obj.y3._01, "BOOK001");
    // container-release loop
    assert_eq!(obj.loop_y4.len(), 1);
    assert_eq!(obj.loop_y4[0].y4._01.as_deref(), Some("BOOK001"));
    assert!(obj.loop_y4[0].w09.is_some());
    // two parties
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "SH");
    assert_eq!(obj.loop_n1[1].n1._01.to_string(), "CN");
    // two port/terminal loops, each with a date
    assert_eq!(obj.loop_r4.len(), 2);
    assert_eq!(obj.loop_r4[0].r4._01, "L");
    assert_eq!(obj.loop_r4[0].dtm.len(), 1);
    // top-level equipment/temperature after the R4 loop
    assert!(obj.w09.is_some());
    // single detail line with both hazmat loops
    assert_eq!(obj.loop_lx.len(), 1);
    assert_eq!(obj.loop_lx[0].lx._01, "1");
    assert_eq!(obj.loop_lx[0].loop_h1.len(), 1);
    assert_eq!(obj.loop_lx[0].loop_h1[0].h2.len(), 1);
    assert_eq!(obj.loop_lx[0].loop_lh1.len(), 1);
    assert_eq!(obj.loop_lx[0].loop_lh1[0].lh1._02, "UN");
    assert_eq!(obj.v1.len(), 1);
    assert_eq!(obj.v9.len(), 1);
}

#[test]
fn roundtrip_301() {
    let (_, first) = _301::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _301::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_301() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*RO*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*301*0001~
B1*SCAC*BOOK001*20200101*A~
Y6*00*AUTHORITY*AUTH123~
Y3*BOOK001*SCAC*20200110*20200115~
Y4*BOOK001*REL001*20200112*USNYC*2*40HC*SCAC~
N9*BN*BOOK001~
N1*SH*SHIPPER NAME*25*SHIP01~
N3*123 MAIN ST~
N4*NEW YORK*NY*10001*US~
R4*L*UN*USNYC*NEW YORK*US~
DTM*649*20200115~
LX*1~
N7*CONT*ABCD1234567~
L5*1*GENERAL CARGO~
V1*IMO9999999*EVER GIVEN*PA*VOY01~
V9*ARR~
SE*18*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_301>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.y3._01, "BOOK001");
    assert_eq!(t.loop_n1.len(), 1);
    assert_eq!(t.loop_lx.len(), 1);
}
