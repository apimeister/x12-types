use crate::util::Parser;
use crate::v005010::*;

// Exercises every loop in the transaction set at least once:
// heading M1/N1/R4/C8 loops, the detail LX loop with its nested N7 (L1/H1/LH1)
// and L0 (PO4/PAL/CTP/L1/SAC/L9/H1/LH1/N1) loops, and the summary L3 loop.
const SAMPLE: &str = r#"ST*304*0001~
B2*A*B*C*D*E*SCAC~
B2A*00~
Y6*00*AUTHORITY*AUTH123~
G1*BC~
N9*BN*BOOK001~
V1*IMO9999999*EVER GIVEN*PA*VOY01~
M1*CA*1~
CUR*BY*USD~
DTM*649*20200115~
N1*SH*SHIPPER NAME*25*SHIP01~
N3*123 MAIN ST~
N4*NEW YORK*NY*10001*US~
N1*CN*CONSIGNEE NAME*25*CONS01~
N4*ROTTERDAM**3000*NL~
R4*L*UN*USNYC*NEW YORK*US~
DTM*649*20200115~
R4*D*UN*NLRTM*ROTTERDAM*NL~
DTM*649*20200201~
R2A*USNYC*1~
K1*HEADING REMARK~
L11*REF001*BM~
H3*HOT~
L5*1*GENERAL CARGO~
C8*1*CERT~
SUP*ABC~
LX*1~
Y2*2*A*PP*40HC~
N7*CONT*ABCD1234567~
W09*A*40*F~
L1*1~
CUR*BY*USD~
H1*UN1203*3*3~
H2*FLAMMABLE~
LH1*1*UN*1203~
LH3*GASOLINE~
L11*LINEREF*BM~
K1*LINE REMARK~
PO4*1*10*EA~
MEA*PD*G*5000~
MAN*GM*1234567890~
L0*1~
MEA*WT*G*5000~
PO4*1*10*EA~
MAN*GM*9999~
QTY*39*10~
PAL*B~
QTY*39*5~
CTP*DE*ABC*100~
CUR*BY*USD~
L5*1*LINE ITEM CARGO~
L1*1~
CUR*BY*USD~
SAC*A*B000~
CUR*BY*USD~
L9*ABC*100~
CUR*BY*USD~
H1*UN1203*3*3~
H2*CORROSIVE~
LH1*2*UN*1789~
LH3*HYDROCHLORIC ACID~
N1*MF*MANUFACTURER*25*MFR01~
N4*HAMBURG**2000*DE~
L3*5000*G~
CUR*BY*USD~
MEA*WT*G*5000~
L1*1~
CUR*BY*USD~
TDS*100000~
CUR*BY*USD~
SAC*A*B000~
CUR*BY*USD~
L9*ABC*100~
CUR*BY*USD~
ISS*10*CT~
V9*ARR~
K1*SUMMARY REMARK~
L11*SUMREF*BM~
SE*79*0001~"#;

#[test]
fn parse_304() {
    let (rest, obj) = _304::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "304");
    assert_eq!(obj.b2._06.to_string(), "SCAC");
    // heading loops
    assert_eq!(obj.loop_m1.len(), 1);
    assert!(obj.loop_m1[0].cur.is_some());
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "SH");
    assert_eq!(obj.loop_r4.len(), 2);
    assert_eq!(obj.loop_c8.len(), 1);
    assert_eq!(obj.loop_c8[0].sup.len(), 1);
    // detail
    assert_eq!(obj.loop_lx.len(), 1);
    let lx = &obj.loop_lx[0];
    assert_eq!(lx.lx._01, "1");
    assert_eq!(lx.y2.len(), 1);
    // equipment (N7) loop with its rate/hazmat sub-loops
    assert_eq!(lx.loop_n7.len(), 1);
    let n7 = &lx.loop_n7[0];
    assert_eq!(n7.loop_l1.len(), 1);
    assert!(n7.loop_l1[0].cur.is_some());
    assert_eq!(n7.loop_h1.len(), 1);
    assert_eq!(n7.loop_h1[0].h2.len(), 1);
    assert_eq!(n7.loop_lh1.len(), 1);
    assert_eq!(n7.loop_lh1[0].lh1._02.to_string(), "UN");
    // equipment-level PO4 loop
    assert_eq!(lx.loop_po4.len(), 1);
    assert_eq!(lx.loop_po4[0].man.len(), 1);
    // line-item (L0) loop with the full set of nested loops
    assert_eq!(lx.loop_l0.len(), 1);
    let l0 = &lx.loop_l0[0];
    assert_eq!(l0.loop_po4.len(), 1);
    assert_eq!(l0.loop_pal.len(), 1);
    assert!(l0.loop_pal[0].qty.is_some());
    assert_eq!(l0.loop_ctp.len(), 1);
    assert_eq!(l0.loop_l1.len(), 1);
    assert_eq!(l0.loop_sac.len(), 1);
    assert_eq!(l0.loop_l9.len(), 1);
    assert_eq!(l0.loop_h1.len(), 1);
    assert_eq!(l0.loop_lh1.len(), 1);
    assert_eq!(l0.loop_lh1[0].lh1._02.to_string(), "UN");
    assert_eq!(l0.loop_n1.len(), 1);
    assert_eq!(l0.loop_n1[0].n1._01.to_string(), "MF");
    // summary L3 loop with its nested loops
    assert_eq!(obj.loop_l3.len(), 1);
    let l3 = &obj.loop_l3[0];
    assert_eq!(l3.loop_l1.len(), 1);
    assert_eq!(l3.loop_tds.len(), 1);
    assert_eq!(l3.loop_tds[0].tds._01.to_string(), "100000");
    assert_eq!(l3.loop_sac.len(), 1);
    assert_eq!(l3.loop_l9.len(), 1);
    assert_eq!(l3.v9.len(), 1);
}

#[test]
fn roundtrip_304() {
    let (_, first) = _304::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _304::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_304() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*SO*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*304*0001~
B2*A*B*C*D*E*SCAC~
N9*BN*BOOK001~
N1*SH*SHIPPER NAME*25*SHIP01~
N3*123 MAIN ST~
N4*NEW YORK*NY*10001*US~
R4*L*UN*USNYC*NEW YORK*US~
DTM*649*20200115~
LX*1~
N7*CONT*ABCD1234567~
L0*1~
L5*1*GENERAL CARGO~
L3*5000*G~
SE*15*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_304>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.b2._06.to_string(), "SCAC");
    assert_eq!(t.loop_n1.len(), 1);
    assert_eq!(t.loop_lx.len(), 1);
    assert_eq!(t.loop_lx[0].loop_n7.len(), 1);
    assert_eq!(t.loop_lx[0].loop_l0.len(), 1);
    assert_eq!(t.loop_l3.len(), 1);
}
