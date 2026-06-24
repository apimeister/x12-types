use crate::util::Parser;
use crate::v005010::*;

// Warehouse stock-transfer shipment advice: header party loop plus carrier (W27), then
// two W04 item-detail loops (the first with description/reference/packing) and a W03 total.
const SAMPLE: &str = r#"ST*943*0001~
W06*N*SHIP001*20200101~
N1*ST*RECEIVER*92*RCV01~
N3*1 DOCK ST~
N4*DALLAS*TX*75201~
N9*WR*REF1~
W27*M*SCAC*ROUTE~
W04*10*EA*012345678905~
G69*WIDGET~
N9*LT*LOT1~
W20*10*EA~
W04*20*EA*012345678912~
W03*30~
SE*14*0001~"#;

#[test]
fn parse_943() {
    let (rest, obj) = _943::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "943");
    assert_eq!(obj.w06._01, "N");
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.n9.len(), 1);
    assert_eq!(obj.w27._01, "M");
    // two item loops; the first carries description/reference/packing
    assert_eq!(obj.loop_w04.len(), 2);
    assert_eq!(obj.loop_w04[0].w04._01, "10");
    assert_eq!(obj.loop_w04[0].g69.len(), 1);
    assert_eq!(obj.loop_w04[0].n9.len(), 1);
    assert_eq!(obj.loop_w04[0].w20.len(), 1);
    assert_eq!(obj.w03._01, "30");
}

#[test]
fn roundtrip_943() {
    let (_, a) = _943::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _943::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_943() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*AR*S*R*20200101*1200*1*X*005010~
ST*943*0001~
W06*N*SHIP001*20200101~
N1*ST*RECEIVER~
W27*M*SCAC~
W04*10*EA~
W03*10~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_943>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.w06._01, "N");
    assert_eq!(t.loop_w04.len(), 1);
    assert_eq!(t.w03._01, "10");
}
