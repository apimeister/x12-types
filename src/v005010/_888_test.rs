use crate::util::Parser;
use crate::v005010::*;

// Item maintenance: heading BGN/N1/N9/NTE/G62 and an LM code loop, then a G53 maintenance
// loop with an LX line-item loop carrying pricing/description detail and its N1 party
// (0311), G55 consumer-unit (0312) and LM (0313) sub-loops.
const SAMPLE: &str = r#"ST*888*0001~
BGN*00*REF1*20200101~
N1*BY*BUYER*92*B1~
N3*1 BUYER ST~
N4*DALLAS*TX*75201~
N9*PO*PO123~
NTE*GEN*ITEM MAINT~
G62*04*20200101~
LM*VI~
LQ*0*A~
G53*A~
G62*04*20200101~
LX*1~
G39*012345678905~
G69*WIDGET~
QTY*39*100~
LIN*1*UP*012345678905~
PID*F****WIDGET~
G24*PROMO1~
G40*A*5.00~
N1*MF*MAKER~
PAL*1~
G55*UP*012345678905~
G69*CONSUMER UNIT~
PID*F****CONSUMER~
LM*VI~
LQ*0*B~
SE*27*0001~"#;

#[test]
fn parse_888() {
    let (rest, obj) = _888::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "888");
    assert!(obj.bgn.is_some());
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.n9.len(), 1);
    assert_eq!(
        obj.g62._01.as_ref().map(|e| e.to_string()).as_deref(),
        Some("04")
    );
    assert_eq!(obj.loop_lm.len(), 1);
    assert_eq!(obj.loop_g53.len(), 1);
    let g53 = &obj.loop_g53[0];
    assert_eq!(g53.g62.len(), 1);
    assert_eq!(g53.loop_lx.len(), 1);
    let lx = &g53.loop_lx[0];
    assert!(lx.g39.is_some());
    assert_eq!(lx.qty.len(), 1);
    assert!(lx.lin.is_some());
    assert_eq!(lx.pid.len(), 1);
    assert_eq!(lx.g24.len(), 1);
    assert_eq!(lx.g40.len(), 1);
    assert_eq!(lx.loop_n1.len(), 1);
    assert_eq!(lx.loop_n1[0].pal.len(), 1);
    assert_eq!(lx.loop_g55.len(), 1);
    assert_eq!(lx.loop_g55[0].pid.len(), 1);
    assert_eq!(lx.loop_lm.len(), 1);
    assert_eq!(lx.loop_lm[0].lq.len(), 1);
}

#[test]
fn roundtrip_888() {
    let (_, a) = _888::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _888::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_888() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*QG*S*R*20200101*1200*1*X*005010~
ST*888*0001~
BGN*00*REF1*20200101~
N1*BY*BUYER~
G62*04*20200101~
G53*A~
LX*1~
LIN*1*UP*012345678905~
SE*7*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_888>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(
        t.g62._01.as_ref().map(|e| e.to_string()).as_deref(),
        Some("04")
    );
    assert_eq!(t.loop_g53.len(), 1);
    assert_eq!(t.loop_g53[0].loop_lx.len(), 1);
}
