use crate::util::Parser;
use crate::v005010::*;

// Shipment and billing notice: HL-based. A shipment HL (BS1: G05/TD5 + N1 party), an order
// HL (BS2: TDS/PRF + SAC and N1 sub-loops) and an item HL (BS5: IT1 + PID, SLN and SAC
// sub-loops).
const SAMPLE: &str = r#"ST*857*0001~
BHT*0001*00*REF1*20200101~
HL*1**S~
G05*10*CA*500*LB~
TD5*B*2*XYZ*M~
N1*ST*STORE*92*ST1~
N3*1 STORE ST~
N4*DALLAS*TX*75201~
HL*2*1*O~
TDS*10000~
PRF*PO123~
SAC*A*C000~
TXI*ST*50~
N1*BY*BUYER~
HL*3*2*I~
IT1*1*10*EA*5**VN*ABC~
PID*F****WIDGET~
MEA*PD*N*5~
SLN*1**A*5*EA~
PID*F****SUBITEM~
SAC*A*D000~
SE*21*0001~"#;

#[test]
fn parse_857() {
    let (rest, obj) = _857::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "857");
    assert_eq!(obj.bht._01.to_string(), "0001");
    assert_eq!(obj.loop_hl.len(), 3);
    // shipment level
    let s = &obj.loop_hl[0];
    assert_eq!(s.hl._03.to_string(), "S");
    assert_eq!(s.loop_bs1.len(), 1);
    assert_eq!(s.loop_bs1[0].td5.len(), 1);
    assert_eq!(s.loop_bs1[0].loop_n1.len(), 1);
    assert_eq!(s.loop_bs1[0].loop_n1[0].n3.len(), 1);
    // order level
    let o = &obj.loop_hl[1];
    assert_eq!(o.loop_bs2.len(), 1);
    let bs2 = &o.loop_bs2[0];
    assert_eq!(bs2.loop_sac.len(), 1);
    assert_eq!(bs2.loop_sac[0].txi.len(), 1);
    assert_eq!(bs2.loop_n1.len(), 1);
    // item level
    let i = &obj.loop_hl[2];
    assert_eq!(i.loop_bs5.len(), 1);
    let bs5 = &i.loop_bs5[0];
    assert_eq!(bs5.loop_pid.len(), 1);
    assert_eq!(bs5.loop_pid[0].mea.len(), 1);
    assert_eq!(bs5.loop_sln.len(), 1);
    assert_eq!(bs5.loop_sln[0].pid.len(), 1);
    assert_eq!(bs5.loop_sac.len(), 1);
}

#[test]
fn roundtrip_857() {
    let (_, a) = _857::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _857::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_857() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*CB*S*R*20200101*1200*1*X*005010~
ST*857*0001~
BHT*0001*00*REF1*20200101~
HL*1**S~
G05*10*CA~
HL*2*1*I~
IT1*1*10*EA*5**VN*ABC~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_857>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bht._01.to_string(), "0001");
    assert_eq!(t.loop_hl.len(), 2);
    assert_eq!(t.loop_hl[1].loop_bs5.len(), 1);
}
