use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str =
    "ST*843*0001~BQR*00*RFQ1*20200101~N1*SE*SELLER~PO1*1*10*EA*5**VN*ABC~SE*5*0001~";

#[test]
fn parse_843() {
    let (rest, obj) = _843::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "843");
    assert_eq!(obj.bqr._01, "00");
    assert_eq!(obj.loop_po1.len(), 1);
}

#[test]
fn roundtrip_843() {
    let (_, a) = _843::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _843::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_843() {
    let s = "ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~\nGS*RR*S*R*20200101*1200*1*X*005010~\nST*843*0001~\nBQR*00*RFQ1*20200101~\nPO1*1*10*EA*5**VN*ABC~\nSE*3*0001~\nGE*1*1~\nIEA*1*000000001~";
    let (rest, obj) = Transmission::<_843>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].bqr._01, "00");
}
