use crate::util::Parser;
use crate::v005010::*;

// A workers'-comp first-report-of-injury exercising the heading party loop and one HL
// level carrying: a party loop, an employment-status loop (with its income LX loop), a
// loan loop (with vehicle/amount/party sub-loops), an LS/LE-bracketed impairment LX loop
// (III -> LM and a party NM1 sub-loop), and a CFI loop (with AD1 and party sub-loops).
const SAMPLE: &str = r#"ST*148*0001~
BHT*0010*00*REF001*20200115*1200~
REF*TN*TRACK001~
NM1*PE*2*ACME EMPLOYER*****FI*123456789~
N3*100 FACTORY RD~
N4*DETROIT*MI*48201*US~
PER*IC*HR DEPT*TE*5551234~
HL*1**20~
CRI***A1*Y*1*1~
DTP*431*D8*20200110~
NM1*IL*1*DOE*JOHN****34*987654321~
N3*456 HOME ST~
N4*DETROIT*MI*48202*US~
ESI*Y*N*N*0800*40*01~
EMT**MGR~
DTP*431*D8*20200110~
LX*1~
AIN*WG*5000*RT~
WS*D*0800*1700~
DTP*193*D8*20200101~
LN*LOAN001*10000~
REF*ZZ*LNREF~
VEH*1*VIN12345678~
N4*DETROIT*MI*48201*US~
PID*F****COMPANY VEHICLE~
AMT*GW*5000~
DTP*193*D8*20200101~
NM1*LH*2*LIENHOLDER BANK~
DMG*D8*19700101*M~
N4*CHICAGO*IL*60601*US~
LS*LX~
LX*2~
III*ZZ*BACK INJURY~
IMP*10*25~
DTP*431*D8*20200110~
LM*AS~
LQ*AS*1234~
NM1*DD*1*SMITH*JANE~
N4*DETROIT*MI*48201*US~
LE*LX~
CFI*01*A1~
AMT*ZZ*15000~
DTP*431*D8*20200110~
AD1*A1*GW*C*1000~
DTP*193*D8*20200101~
NM1*PR*2*INSURANCE CARRIER~
N4*HARTFORD*CT*06101*US~
GRI*OSHA1*Y~
SE*49*0001~"#;

#[test]
fn parse_148() {
    let (rest, obj) = _148::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "148");
    assert_eq!(obj.bht._01.to_string(), "0010");
    // heading party loop
    assert_eq!(obj.loop_nm1.len(), 1);
    assert_eq!(obj.loop_nm1[0].nm1._01.to_string(), "PE");
    // one hierarchical level
    assert_eq!(obj.loop_hl.len(), 1);
    let hl = &obj.loop_hl[0];
    assert_eq!(hl.hl._01, "1");
    assert_eq!(hl.cri.len(), 1);
    assert_eq!(hl.cri[0]._06.to_string(), "1");
    // HL-level party (injured worker)
    assert_eq!(hl.loop_nm1.len(), 1);
    assert_eq!(hl.loop_nm1[0].nm1._01.to_string(), "IL");
    // employment-status loop with its income LX loop
    assert_eq!(hl.loop_esi.len(), 1);
    let esi = &hl.loop_esi[0];
    assert_eq!(esi.esi._01.to_string(), "Y");
    assert!(esi.emt.is_some());
    assert_eq!(esi.loop_lx.len(), 1);
    assert!(esi.loop_lx[0].ain.is_some());
    assert_eq!(esi.loop_lx[0].ws.len(), 1);
    // loan loop with vehicle / amount / party sub-loops
    assert_eq!(hl.loop_ln.len(), 1);
    let ln = &hl.loop_ln[0];
    assert_eq!(ln.ln._01, "LOAN001");
    assert_eq!(ln.loop_veh.len(), 1);
    assert_eq!(ln.loop_veh[0].pid.len(), 1);
    assert_eq!(ln.loop_amt.len(), 1);
    assert_eq!(ln.loop_nm1.len(), 1);
    assert_eq!(ln.loop_nm1[0].nm1._01.to_string(), "LH");
    // LS/LE-bracketed impairment LX loop
    assert!(hl.ls.is_some());
    assert!(hl.le.is_some());
    assert_eq!(hl.loop_lx.len(), 1);
    let lx = &hl.loop_lx[0];
    assert_eq!(lx.lx._01, "2");
    assert_eq!(lx.loop_iii.len(), 1);
    assert_eq!(lx.loop_iii[0].imp.len(), 1);
    assert_eq!(lx.loop_iii[0].loop_lm.len(), 1);
    assert_eq!(lx.loop_iii[0].loop_lm[0].lq.len(), 1);
    assert_eq!(lx.loop_nm1.len(), 1);
    assert_eq!(lx.loop_nm1[0].nm1._01.to_string(), "DD");
    // compensation financial information loop
    assert_eq!(hl.loop_cfi.len(), 1);
    let cfi = &hl.loop_cfi[0];
    assert_eq!(cfi.cfi._01.to_string(), "01");
    assert_eq!(cfi.loop_ad1.len(), 1);
    assert_eq!(cfi.loop_nm1.len(), 1);
    assert_eq!(cfi.loop_nm1[0].nm1._01.to_string(), "PR");
    // summary
    assert_eq!(obj.gri.len(), 1);
    assert_eq!(obj.gri[0]._01, "OSHA1");
}

#[test]
fn roundtrip_148() {
    let (_, first) = _148::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _148::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_148() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200101*1200*U*00501*000000001*0*P*>~
GS*AG*SENDER*RECEIVER*20200101*1200*1*X*005010~
ST*148*0001~
BHT*0010*00*REF001*20200115*1200~
NM1*PE*2*ACME EMPLOYER*****FI*123456789~
HL*1**20~
NM1*IL*1*DOE*JOHN****34*987654321~
N4*DETROIT*MI*48202*US~
SE*6*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_148>::parse(str).unwrap();
    assert!(rest.is_empty());
    let t = &obj.functional_group[0].segments[0];
    assert_eq!(t.bht._01.to_string(), "0010");
    assert_eq!(t.loop_nm1.len(), 1);
    assert_eq!(t.loop_hl.len(), 1);
    assert_eq!(t.loop_hl[0].loop_nm1.len(), 1);
    assert_eq!(t.loop_hl[0].loop_nm1[0].nm1._01.to_string(), "IL");
}
