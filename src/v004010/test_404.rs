use crate::v004010::*;

#[test]
fn test_1() {
    let str = r#"ISA*00*          *00*          *ZZ*XXXXXX004      *02*XX             *230504*1347*U*00401*000015113*0*P*>~
GS*SR*XXXXXX004*XX*20230504*1347*15113*X*004010~
ST*404*15113~
BX*04*R*PP*3PHLTXXXXX*XX*K*B*N~
BNX*N**S~
M3*R*20230504*084558~
N9*PO*3PHLTXXXXX~
N9*BN*3PHLTXXXXX**20230504*084558~
N9*BM*XXXXN3KSZ000871X**20230504*084558~
N9*OB*9313XXXXXXXXN3KSZ000*112*20230504*084558~
CM*314N*L*SHANGHAI SH XX**3PHLTXXXXX****ANNA XXXXXX~
CM*314N*1*PRINCE RUPERT BC CA**3PHLTXXXXX*NSPQ*XXXX**ANNA XXXXXX~
N7*MRSU*360296*10469*N******S*XX****4000***5****4500~
M7*ML-XX0697792~
M12*A6****209373***XC*9313XXXXXXXXN3KSZ000~
F9**PRINCE RUPERT TERM*BC~
D9**CHIINTTER*IL~
N1*SF*XXXXX INTL ( HONG KONG) LTD~
N3*999 NATHAN RD RM 9999 WU SANG HSE K~
N4*HONG KONG*ZZ**HK~
PER*DC****TE*86 76022622332~
N1*PF*XXXXXXX XXX NORTH AMERICA, INC~
N3*180 PARK AVE~
N4*FLORHAM PARK*NJ*07932*US~
PER*DC**FX*215-999-9999*TE*1 9735199999~
N1*SH*XXXXXXX XXX NORTH AMERICA,~
N3*2000 MARKET ST F 9 STE 900~
N4*PHILADELPHIA*PA*19103*US~
PER*DC**FX*215 999 9999*TE*001 2159299999~
N1*N1*XXXXXXX XXX NORTH AMERICA,~
N3*2000 MARKET ST F 9 STE 900~
N4*PHILADELPHIA*PA*19103*US~
PER*DC**FX*215 999 9999*TE*001 2159299999~
N1*XX*XXXXXXX XXX NORTH AMERICA,~
N3*2000 MARKET ST F 9 STE 900~
N4*PHILADELPHIA*PA*19103*US~
PER*DC**FX*215 999 9999*TE*001 2159299999~
N1*UC*XX XXXX XXX LLC~
N3*9999 S ROSELLE RD~
N4*PALATINE*IL*60067*US~
PER*DC****TE*1 2248399999~
R2*XX*S***85*X~
H3*IP~
H3*IB~
LX*1~
L5*1*XXX XXXXX 112 CTNS/112 PCS HTS: 7321.11.6000*4611110*T~
L0*1***10469*N***112*CTN**K~
SE*46*15113~
GE*1*15113~
IEA*1*000015113~
"#;
    let (rest, obj) = Transmission::<_404>::parse(str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
    let s = serde_x12::to_string(&obj).unwrap();
    assert_eq!(s, str);
}

#[test]
fn test_2() {
    let str = r#"ISA*00*          *00*          *ZZ*XXXXXX004      *02*XX             *230601*1651*U*00401*000015623*0*P*>~
GS*SR*XXXXXX004*XX*20230601*1651*15623*X*004010~
ST*404*15623~
BX*04*R*PP*3PHLTXXXXX*XX*K*B*N~
BNX*N**S~
M3*R*20230601*115110~
N9*PO*3PHLTXXXXX~
N9*BN*3MIS999999**20230601*115110~
N9*BM*3MIS999999**20230601*115110~
N9*HS*4801.00.0140~
N9*ED*Sum2036~
N9*FRC*E962~
CM*324S*L*NEW YORK NY US**3PHLTXXXXX****MAERSK KENTUCKY~
CM*324S*1*CARTAGENA CO**3PHLTXXXXX*XXXU*NSPQ**MAERSK KENTUCKY~
N7*MRSU*444423*18274*N******S*XX****4000***1****4500~
M7*20001540~
M12*62**1001*30107*$20 PER***S4*58-179737300~
F9**MONTREAL TASCH YARD*QC~
D9**ELIZABETH MARINE TE*NJ~
N1*SF*RESOLUTE FP US INC~
N3*5020 HIGHWAY 11 S~
N4*CALHOUN*TN*37309*US~
PER*DC****TE*1 5072040934~
N1*PF*XXXXXXX XXX NORTH AMERICA, INC~
N3*180 PARK AVE~
N4*FLORHAM PARK*NJ*07932*US~
PER*DC**FX*999-999-9999*TE*1 9735145338~
N1*SH*XXXXXXX XXX NORTH AMERICA,~
N3*2000 MARKET ST F 9 STE 900~
N4*PHILADELPHIA*PA*19103*US~
PER*DC**FX*999 999 9999*TE*001 2159299999~
N1*N1*XXXXXXX XXX NORTH AMERICA,~
N3*2000 MARKET ST F 9 STE 900~
N4*PHILADELPHIA*PA*19103*US~
PER*DC**FX*999 999 9999*TE*001 2159299999~
N1*XX*XXXXXXX XXX NORTH AMERICA,~
N3*2000 MARKET ST F 9 STE 900~
N4*PHILADELPHIA*PA*19103*US~
PER*DC**FX*999 999 9999*TE*001 2159299999~
N1*XU*XXXXXXX XXX NORTH AMERICA,~
N3*2000 MARKET ST F 9 STE 900~
N4*PHILADELPHIA*PA*19103*US~
PER*DC**FX*999 999 9999*TE*001 2159299999~
R2*XX*S*HUNT**85*X~
R2*CSXT*1***85*X~
H3*XP~
LX*1~
L5*1*XXXXXXXXX  SHADE*4611110*T~
L0*1***18274*N***2*PKG**K~
PI*CT*XX 999999*TP**NS*XX 999999~
SE*49*15623~
GE*1*15623~
IEA*1*000015623~
"#;
    let (rest, obj) = Transmission::<_404>::parse(str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
    let s = serde_x12::to_string(&obj).unwrap();
    assert_eq!(s, str);
}
