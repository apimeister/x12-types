use crate::util::Parser;
use crate::v005010::*;

// A claim-attachment 275: heading NM1 loop (with IN1 and a nested NX1 loop) plus the
// detail LX loop carrying the attachment payload (DTP -> EFI -> BIN binary data).
const SAMPLE: &str = r#"ST*275*0001~
BGN*11*REF1*20200101~
DTM*368*20200101~
TRN*1*TRACE001~
NM1*PR*2*PAYER NAME*****PI*12345~
IN1*1*1A~
PER*IC*CONTACT*TE*5551234~
REF*EI*987654321~
NX1*BO~
N3*100 PAYER ST~
N4*CHICAGO*IL*60601~
NM1*QC*1*DOE*JOHN****MI*MEMBER01~
LX*1~
TRN*2*ATTACH001~
STC*A1~
NM1*41*2*SUBMITTER~
REF*EI*111223333~
DTP*368*D8*20200101~
CAT*AH*XX~
PID*F****MEDICAL RECORD~
EFI*05~
BIN*16*SGVsbG8gV29ybGQ=~
SE*23*0001~"#;

#[test]
fn parse_275() {
    let (rest, obj) = _275::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "275");
    assert!(obj.bgn.is_some());
    assert_eq!(obj.dtm.len(), 1);
    assert_eq!(obj.trn.len(), 1);
    // heading parties; the first carries IN1 and a nested NX1 loop
    assert_eq!(obj.loop_nm1.len(), 2);
    assert_eq!(obj.loop_nm1[0].nm1._01.to_string(), "PR");
    assert!(obj.loop_nm1[0].in1.is_some());
    assert_eq!(obj.loop_nm1[0].loop_nx1.len(), 1);
    assert_eq!(obj.loop_nm1[0].loop_nx1[0].nx1._01, "BO");
    assert!(obj.loop_nm1[0].loop_nx1[0].n4.is_some());
    assert_eq!(obj.loop_nm1[1].nm1._01.to_string(), "QC");
    // detail attachment loop with the binary payload
    assert_eq!(obj.loop_lx.len(), 1);
    let lx = &obj.loop_lx[0];
    assert_eq!(lx.lx._01, "1");
    assert!(lx.stc.is_some());
    assert!(lx.nm1.is_some());
    assert_eq!(lx.loop_dtp.len(), 1);
    let dtp = &lx.loop_dtp[0];
    assert!(dtp.cat.is_some());
    assert!(dtp.pid.is_some());
    assert_eq!(dtp.loop_efi.len(), 1);
    assert_eq!(dtp.loop_efi[0].bin._01, "16");
    assert_eq!(dtp.loop_efi[0].bin._02, "SGVsbG8gV29ybGQ=");
}

#[test]
fn roundtrip_275() {
    let (_, a) = _275::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _275::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_275() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*PI*S*R*20200101*1200*1*X*005010~
ST*275*0001~
BGN*11*REF1*20200101~
NM1*PR*2*PAYER*****PI*123~
LX*1~
DTP*368*D8*20200101~
EFI*05~
BIN*5*HELLO~
SE*7*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_275>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.loop_nm1.len(), 1);
    assert_eq!(t.loop_lx.len(), 1);
    assert_eq!(t.loop_lx[0].loop_dtp[0].loop_efi[0].bin._02, "HELLO");
}
