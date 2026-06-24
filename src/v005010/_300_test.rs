use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*300*0001~B1*SCAC*BOOK001*20200101*N~G61*IC*BOOKING AGENT*TE*5551234~Y6*00*AUTHORITY*AUTH123~Y7*1*1*A*1234*20200101~Y1*20200115*20200110*SCAC*O~Y2*2*40HC*SCAC*FCL~W09*A*40*F~N9*BN*BOOK001~R2A*USNYC*1~N1*SH*SHIPPER NAME*25*SHIP01~N3*123 MAIN ST~N4*NEW YORK*NY*10001*US~R4*L*UN*USNYC*NEW YORK*US~DTM*649*20200115~R4*D*UN*NLRTM*ROTTERDAM*NL~DTM*649*20200201~W09*A*40*F~H3*HOT~EA*BR~LX*1~N7*CONT*ABCD1234567~DTM*002*20200110~L0*1~L5*1*GENERAL CARGO~H1*UN1203*3*3~H2*FLAMMABLE~LH1*1*UN*1203~LH3*GASOLINE~V1*IMO9999999*EVER GIVEN*PA*VOY01~V9*ARR~K1*BOOKING REMARK~SE*33*0001~";

#[test]
fn parse_300() {
    let (rest, obj) = _300::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "300");
    assert_eq!(obj.b1._02.as_deref(), Some("BOOK001"));
    assert_eq!(obj.y1._01.as_deref(), Some("20200115"));
    assert!(obj.y7.is_some());
    // container-details loop with its equipment/temperature
    assert_eq!(obj.loop_y2.len(), 1);
    assert_eq!(obj.loop_y2[0].y2._01, "2");
    assert!(obj.loop_y2[0].w09.is_some());
    // single party loop
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "SH");
    // two port/terminal loops, each with a date
    assert_eq!(obj.loop_r4.len(), 2);
    assert_eq!(obj.loop_r4[0].dtm.len(), 1);
    assert!(obj.w09.is_some());
    assert_eq!(obj.ea.len(), 1);
    // detail line with a date and both hazmat loops
    assert_eq!(obj.loop_lx.len(), 1);
    assert_eq!(obj.loop_lx[0].lx._01, "1");
    assert!(obj.loop_lx[0].dtm.is_some());
    assert_eq!(obj.loop_lx[0].loop_h1.len(), 1);
    assert_eq!(obj.loop_lx[0].loop_lh1.len(), 1);
    assert_eq!(obj.loop_lx[0].loop_lh1[0].lh1._02, "UN");
    assert_eq!(obj.v1.len(), 1);
    assert_eq!(obj.v9.len(), 1);
    assert_eq!(obj.k1.len(), 1);
}

#[test]
fn roundtrip_300() {
    let (_, first) = _300::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _300::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_300() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*RO*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*300*0001~
B1*SCAC*BOOK001*20200101*N~
Y1*20200115*20200110*SCAC*O~
Y2*2*40HC*SCAC*FCL~
N9*BN*BOOK001~
N1*SH*SHIPPER NAME*25*SHIP01~
N3*123 MAIN ST~
N4*NEW YORK*NY*10001*US~
R4*L*UN*USNYC*NEW YORK*US~
DTM*649*20200115~
LX*1~
N7*CONT*ABCD1234567~
L5*1*GENERAL CARGO~
SE*15*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_300>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.y1._01.as_deref(), Some("20200115"));
    assert_eq!(t.loop_y2.len(), 1);
    assert_eq!(t.loop_lx.len(), 1);
}
