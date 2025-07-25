use crate::util::Parser;
use crate::v005010::Transmission;
use crate::v005010::_277;

#[test]
fn parse_277_01() {
    //source: https://github.com/dwai1714/edi_parser/tree/main/spike/data_based_on_type/277
    let str = r#"ISA*00*          *00*          *ZZ*123456789012345*ZZ*123456789012346*080503*1705*>*00501*000010216*0*T*:~GS*HN*1234567890*1234567890*20080503*1705*20213*X*005010X214~ST*277*0003*005010X214~BHT*0085*08*277X2140003*20230221*1025*TH~HL*1**20*1~NM1*PR*2*YOUR INSURANCE COMPANY*****PI*YIC01~TRN*1*0091182~DTP*050*D8*20230220~DTP*009*D8*20230221~HL*2*1*21*1~NM1*41*1*JONES*HARRY*B***46*S00003~TRN*2*2002022045678~STC*A1:19:PR*20230221*WQ*365.5~QTY*90*3~QTY*AA*2~AMT*YU*200.5~AMT*YY*165~HL*3*2*19*1~NM1*85*1*JONES*HARRY*B**MD*FI*234567894~HL*4*3*PT~NM1*QC*1*PATIENT*FEMALE****MI*2222222222~TRN*2*PATIENT22222~STC*A2:20*20230221*WQ*100~REF*1K*220216359803X~DTP*472*D8*20230214~HL*5*3*PT~NM1*QC*1*PATIENT*MALE****MI*3333333333~TRN*2*PATIENT33333~STC*A3:21*20230221*U*65******A3:187~DTP*472*D8*20230229~HL*6*3*PT~NM1*QC*1*JONES*LARRY****MI*4444444444~TRN*2*JONES44444~STC*A7:21*20230221*U*100******A7:249~DTP*472*D8*20230211~HL*7*3*PT~NM1*QC*1*JOHNSON*MARY****MI*5555555555~TRN*2*JOHNSON55555~STC*A2:20*20230221*WQ*50.5~REF*1K*220216359806X~DTP*472*D8*20230210~HL*8*3*PT~NM1*QC*1*MILLER*HARRIETT****MI*6666666666~TRN*2*MILLS66666~STC*A2:20*20230221*WQ*50~REF*1K*220216359807X~DTP*472*D8*20230205~SE*46*0003~GE*1*20213~IEA*1*000010216~"#;
    let parsed = Transmission::<_277>::parse(str).unwrap();
    println!("{parsed:?}");
}

#[test]
fn parse_277_02() {
    //source: https://github.com/dwai1714/edi_parser/tree/main/spike/data_based_on_type/277
    let str = r#"ISA*00*          *00*          *ZZ*123456789012345*ZZ*123456789012346*080503*1705*>*00501*000010216*0*T*:~GS*HN*1234567890*1234567891*20080503*1705*20213*X*005010X364~ST*277*0003*005010X364~BHT*0085*08*0000221*20190221*1025~HL*1**20*1~NM1*ACV*2*ALL PAYER CLAIMS DATABASE*****46*APCD01~TRN*1*ABC12345~DTP*050*D8*20190220~DTP*009*D8*20190221~HL*2*1*21*1~NM1*40*2*YOUR INSURANCE COMPANY*****46*S00003~TRN*2*206438976580901~STC*DR02:20*20190221*WQ*365.5~QTY*90*3~QTY*AA*2~AMT*YU*200.5~AMT*YY*165~HL*3*2*19*1~NM1*85*1*JONES*HARRY*B**MD*XX*1546326897~HL*4*3*PT~NM1*QC*1*PATIENT*FEMALE****MI*2222222222~TRN*2*PATIENT22222~STC*DR02:20:PR*20190221*WQ*100~REF*F8*IC847502~REF*1K*220216359803X~DTP*472*D8*20190214~HL*5*3*PT~NM1*QC*1*PATIENT*MALE****MI*3333333333~TRN*2*PATIENT33333~STC*DR06:21*20190221*U*65******DR06:255~REF*F8*IC429783~REF*1K*220216359954X~DTP*472*D8*20190121~HL*6*3*PT~NM1*QC*1*JONES*LARRY****MI*4444444444~TRN*2*JONES44444~STC*DR03:26:77*20190221*U*100~REF*F8*IC429805~REF*1K*220216359964X~DTP*472*D8*20190211~HL*7*2*19*1~NM1*85*1*SMITH*JOHN*C**MD*XX*1546326780~TRN*1*0~REF*LU*AB142~QTY*QA*2~AMT*YU*100.5~HL*8*7*PT~NM1*QC*1*JOHNSON*MARY****MI*5555555555~TRN*2*JOHNSON55555~STC*DR08:20:PR*20190221*EZ*50.5~REF*F8*IC429888~REF*1K*220216359806X~DTP*472*D8*20190210~SVC*HC:G9938*50.5*****1~STC*DR08:475**EZ~REF*6R*1~DTP*472*D8*20190210~HL*9*7*PT~NM1*QC*1*MILLS*HARRIETT****MI*6666666666~TRN*2*MILLS66666~STC*DR02:20:PR*20190221*WQ*50~REF*F8*IC429956~REF*1K*220216359807X~DTP*472*D8*20190205~SE*63*0003~GE*1*20213~IEA*1*000010216~"#;
    let parsed = Transmission::<_277>::parse(str).unwrap();
    println!("{parsed:?}");
}

#[test]
fn parse_277_03() {
    //source: https://github.com/dwai1714/edi_parser/tree/main/spike/data_based_on_type/277
    let str = r#"ISA*00*          *00*          *ZZ*123456789012345*ZZ*123456789012346*080503*1705*>*00501*000010216*0*T*:~GS*HN*1234567890*1234567891*20080503*1705*20213*X*005010X364~ST*277*0002*005010X364~BHT*0085*08*123456789*20190201*0405~HL*1**20*1~NM1*ACV*2*STATE ENCOUNTER SYSTEM*****46*STATE01~TRN*1*20201312005S00002XYZABC~DTP*050*D8*20190131~DTP*009*D8*20190201~HL*2*1*21*0~NM1*40*2*ABC PAYER*****46*S00002~TRN*2*2020131052389~STC*DR03:24:41*20190201*U*800~QTY*AA*3~AMT*YY*800~SE*14*0002~GE*1*20213~IEA*1*000010216~"#;
    let parsed = Transmission::<_277>::parse(str).unwrap();
    println!("{parsed:?}");
}

#[test]
fn parse_277_04() {
    //source: https://github.com/Project-Herophilus/Project-Herophilus-Assets/blob/main/Testing/TestData/samples-edi/277CA%20Sample%20(Accepted).txt
    let str = r#"ISA*00*          *00*          *ZZ*EMEDNYBAT      *ZZ*ETIN           *110311*1008*^*00501*000014420*0*P*:~GS*HN*EMEDNYBAT*XDZ*20110311*100831*14420*X*005010X214~ST*277*000014420*005010X214~BHT*0085*08*000000674*20110311*100831*TH~HL*1**20*1~NM1*PR*2*NYSDOH*****FI*141797357~TRN*1*000014420~DTP*050*D8*20110311~DTP*009*D8*20110311~HL*2*1*21*1~NM1*41*2*A NY MD GROUP*****46*ETIN~TRN*2*000000674~STC*A1:20*20110311*WQ*125.0~QTY*90*1~AMT*YU*125.0~HL*3*2*19*1~NM1*85*1*OCTOR*IMA*D***XX*1234567890~TRN*1*1107000000000135FF~HL*4*3*PT~NM1*QC*1*TPATIENT*THEBE*S***MI*LL99999X~TRN*2*5781~STC*A2:20*20110311*WQ*125~REF*1K*1107000000014420~DTP*472*D8*10010101~SE*23*000014420~GE*1*14420~IEA*1*000014420~"#;
    let parsed = Transmission::<_277>::parse(str).unwrap();
    println!("{parsed:?}");
}

#[test]
fn parse_277_05() {
    //source: https://github.com/Project-Herophilus/Project-Herophilus-Assets/blob/main/Testing/TestData/samples-edi/277CA%20Sample%20(Rejected).txt
    let str = r#"ISA*00*          *00*          *ZZ*EMEDNYBAT      *ZZ*ETIN           *110311*0512*^*00501*000001120*0*T*|~GS*HN*EMEDNYBAT*ABCD*20110311*051225*1120*X*005010X214~ST*277*000001120*005010X214~BHT*0085*08*3920394930203*20110311*051225*TH~HL*1**20*1~NM1*PR*2*NYSDOH*****FI*141797357~TRN*1*000001120~DTP*050*D8*20110311~DTP*009*D8*20110311~HL*2*1*21*1~NM1*41*1*TEST*TEST****46*ETIN~TRN*2*11200001~STC*A1|20*20110311*WQ*199.14~QTY*90*1~AMT*YU*199.14~HL*3*2*19*1~NM1*85*2*PROVLNAME*****XX*123456789~TRN*1*1107000000000001FF~HL*4*3*PT~NM1*QC*1*CLILNAME*CLIFNAME*CLIMI***MI*XX99999X~TRN*2*3920394930203~STC*A7|562|85*20110311*WQ*199.14~REF*1K*1107000000001120~REF*BLT*731~DTP*472*D8*20090311~SE*24*000001120~GE*1*1120~IEA*1*000001120~"#;
    let parsed = Transmission::<_277>::parse(str).unwrap();
    println!("{parsed:?}");
}

#[test]
fn parse_277_06() {
    //source: https://github.com/Project-Herophilus/Project-Herophilus-Assets/blob/main/Testing/TestData/samples-edi/277Sample(276Resp-ClaimInfo).txt
    let str = r#"ISA*00*          *00*          *ZZ*EMEDNYBAT      *ZZ*ETIN           *110101*0100*^*00501*000000001*0*T*:~GS*HN*EMEDNYBAT*ETIN*20110101*010000*1*X*005010X212~ST*277*000000001*005010X212~BHT*0010*08*701620002 162000002*20110101*010000*DG~HL*1**20*1~NM1*PR*2*NYSDOH*****PI*141797357~HL*2*1*21*1~NM1*41*2*SUBMITTER*****46*ETIN~HL*3*2*19*1~NM1*1P*2*BUSY PROVIDER*****XX*1234567891~HL*4*3*22*0~NM1*IL*1*Patient Last Name*Patient First Name****MI*XX99999X~TRN*2*10060216012300233HSP1156IC002-~STC*F1:3::RX*20101214**524.49*389.73~REF*1K*1026000000099999~REF*EJ*PT12345~REF*D9*20041513010001~REF*XZ*0898779~DTP*472*RD8*20100729-20100729~SVC*N4:10337080301*524.49*389.73****100.00~STC*F1:3*20101214~DTP*472*RD8*20100729-20100729~SE*21*000000001~GE*1*1~IEA*1*000000001~"#;
    let parsed = Transmission::<_277>::parse(str).unwrap();
    println!("{parsed:?}");
}

#[test]
fn parse_277_07() {
    //source: https://github.com/EdiFabric/X12.NET/blob/master/Files/HIPAA/ResponseTransmission.txt
    let str = r#"ISA*00*          *00*          *ZZ*1234567        *ZZ*11111          *170508*1141*^*00501*000000101*1*P*:~
GS*HC*XXXXXXX*XXXXX*20170617*1741*101*X*005010X212~
ST*277*0001*005010X212~
BHT*0010*08*277X212*20050916*0810*DG~
HL*1**20*1~
NM1*PR*2*ABC INSURANCE*****PI*12345~
HL*2*1*21*1~
NM1*41*2*XYZ SERVICE*****46*X67E~
HL*3*2*19*1~
NM1*1P*2*HOME HOSPITAL*****XX*1666666661~
HL*4*3*22*0~
NM1*IL*1*SMITH*FRED****MI*123456789A~
TRN*2*ABCXYZ1~
STC*P3:317*20050913**8513.88~
REF*1K*05347006051~
REF*BLT*111~
REF*EJ*SM123456~
DTP*472*RD8*20050831-20050906~
HL*5*3*22*0~
NM1*IL*1*JONES*MARY****MI*234567890A~
TRN*2*ABCXYZ2~
STC*F0:3*20050915**7599*7599~
REF*1K*0529675341~
REF*BLT*111~
REF*EJ*JO234567~
DTP*472*RD8*20050731-20050809~
HL*6*2*19*1~
NM1*1P*2*HOME HOSPITAL PHYSICIANS*****XX*1666666666~
HL*7*6*22*1~
NM1*IL*1*MANN*JOHN****MI*345678901~
HL*8*7*23~
NM1*QC*1*MANN*JOSEPH~
TRN*2*ABCXYC3~
STC*F2:88:QC*20050612**150*0~
REF*1K*051681010827~
REF*EJ*MA345678~
SVC*HC:99203*150*0****1~
STC*F2:88:QC*20050612~
DTP*472*D8*20050501~
SE*38*0001~
GE*1*101~
IEA*1*000000101~"#;
    let parsed = Transmission::<_277>::parse(str).unwrap();
    println!("{parsed:?}");
}
