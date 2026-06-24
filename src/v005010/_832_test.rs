use crate::util::Parser;
use crate::v005010::*;

// Price/sales catalog: heading party (N1), code (LM), note (N9) and bracket (G93) loops,
// then a LIN item loop carrying a CTP price-tier loop (with a G40 cost loop and an
// LS-bracketed N1 party), a LIN party loop, a G39 manufacturer-item loop, a PKL pack loop,
// an LFG hazardous-material loop (with a CRC loop), an LM/LQ/PID code loop, an SLN subline
// loop (with LM and N1 sub-loops) and an N9 loop.
const SAMPLE: &str = r#"ST*832*0001~
BCT*PC*CAT1~
CTP*WS*CAT*5.00~
REF*ZZ*CATALOG~
DTM*009*20200101~
N1*SU*SUPPLIER*92*S1~
N3*1 SUPPLIER ST~
N4*DALLAS*TX*75201~
LM*VI~
LQ*0*A~
N9*L1*NOTE~
DTM*009*20200101~
G93*001*10*EA~
SAC*A*C000~
LIN*1*UP*012345678905~
PID*F****WIDGET~
MEA*PD*N*5~
PKG*F*01***DESC~
CTP*WS*RES*9.99~
G36*PL1*ISS1*20200101~
G40*A*5.00~
LS*0010~
N1*MF*MAKER~
LE*0010~
N1*ST*STORE*92*ST1~
N3*1 STORE ST~
G39*012345678905~
PKL*UP*012345678905*EA*12~
LFG*FLAMMABLE LIQUID*3*UN1234*FLAMMABLE~
CRC*ZZ*Y*ABC~
QTY*39*10~
LM*VI~
LQ*0*B~
PID*F****CODE DESC~
SLN*1**A*5*EA~
PID*F****SUBITEM~
QTY*39*5~
LM*VI~
LQ*0*C~
N1*BY*BUYER~
N9*L1*ITEMNOTE~
CTT*1~
SE*44*0001~"#;

#[test]
fn parse_832() {
    let (rest, obj) = _832::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "832");
    assert_eq!(obj.bct._01, "PC");
    assert_eq!(obj.ctp.len(), 1);
    assert_eq!(obj.loop_n1.len(), 1);
    assert_eq!(obj.loop_lm.len(), 1);
    assert_eq!(obj.loop_n9.len(), 1);
    assert_eq!(obj.loop_g93.len(), 1);
    assert!(obj.loop_g93[0].sac.is_some());
    assert_eq!(obj.loop_lin.len(), 1);
    let lin = &obj.loop_lin[0];
    assert_eq!(lin.pid.len(), 1);
    assert_eq!(lin.mea.len(), 1);
    assert_eq!(lin.pkg.len(), 1);
    // CTP price-tier loop with its G40 cost loop and LS-bracketed party.
    assert_eq!(lin.loop_ctp.len(), 1);
    let ctp = &lin.loop_ctp[0];
    assert!(ctp.g36.is_some());
    assert_eq!(ctp.loop_g40.len(), 1);
    assert_eq!(ctp.loop_ls.len(), 1);
    assert_eq!(ctp.loop_ls[0].loop_n1.len(), 1);
    assert!(ctp.loop_ls[0].le.is_some());
    // LIN-level party loop (after the CTP loop).
    assert_eq!(lin.loop_n1.len(), 1);
    assert_eq!(lin.loop_n1[0].n3.len(), 1);
    // G39 manufacturer-item, PKL pack and LFG hazmat loops.
    assert_eq!(lin.loop_g39.len(), 1);
    assert_eq!(lin.loop_pkl.len(), 1);
    assert_eq!(lin.loop_lfg.len(), 1);
    assert_eq!(lin.loop_lfg[0].loop_crc.len(), 1);
    assert!(lin.loop_lfg[0].loop_crc[0].qty.is_some());
    // LIN-level LM code loop with its LQ/PID nest.
    assert_eq!(lin.loop_lm.len(), 1);
    assert_eq!(lin.loop_lm[0].loop_lq.len(), 1);
    assert_eq!(lin.loop_lm[0].loop_lq[0].loop_pid.len(), 1);
    // SLN subline loop with LM and N1 sub-loops.
    assert_eq!(lin.loop_sln.len(), 1);
    let sln = &lin.loop_sln[0];
    assert_eq!(sln.pid.len(), 1);
    assert_eq!(sln.qty.len(), 1);
    assert_eq!(sln.loop_lm.len(), 1);
    assert_eq!(sln.loop_n1.len(), 1);
    // LIN-level note loop.
    assert_eq!(lin.loop_n9.len(), 1);
    assert!(obj.ctt.is_some());
}

#[test]
fn roundtrip_832() {
    let (_, a) = _832::parse(SAMPLE).unwrap();
    let r = format!("{a}");
    let (rest, b) = _832::parse(&r).unwrap();
    assert_eq!(rest, "");
    assert_eq!(a, b);
}

#[test]
fn full_transmission_832() {
    let s = r#"ISA*00*          *00*          *ZZ*S              *ZZ*R              *200101*1200*U*00501*000000001*0*P*>~
GS*SC*S*R*20200101*1200*1*X*005010~
ST*832*0001~
BCT*PC*CAT1~
LIN*1*UP*012~
PID*F****WIDGET~
SE*4*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_832>::parse(s).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bct._01, "PC");
    assert_eq!(t.loop_lin.len(), 1);
    assert_eq!(t.loop_lin[0].pid.len(), 1);
}
