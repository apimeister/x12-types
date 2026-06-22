use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*274*0001~BGN*11*REF1*20200101~N1*PR*PAYER~LX*1~NM1*1P*2*PROVIDER*****XX*1234567890~LX*2~NM1*1P*2*PROVIDER2*****XX*1234567891~SE*8*0001~";

#[test]
fn parse_274() {
    let (rest, obj) = _274::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "274");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_lx.len(), 2);
    assert_eq!(obj.loop_lx[0].loop_nm1.len(), 1);
    assert_eq!(obj.loop_lx[0].loop_nm1[0].nm1._01, "1P");
}

#[test]
fn roundtrip_274() {
    let (_, a) = _274::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _274::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_274() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*HU*S*R*20200101*1200*1*X*005010~\nST*274*0001~\nBGN*11*REF1*20200101~\nLX*1~\nNM1*1P*2*PROVIDER*****XX*1234567890~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_274>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].loop_lx.len(), 1);
}
