use super::*;

#[test]
fn parse_834() {
    //source: https://github.com/EdiFabric/X12.NET/blob/master/Files/HIPAA/BenefitEnrollment.txt
    let str = r#"ISA*00*          *00*          *ZZ*386028429      *30*382328142      *050221*0602*U*00501*000012345*0*P*:~
GS*BE*386028429*382328142*20050221*0602*000012345*X*005010X220A1~
ST*834*12345*005010X220A1~
BGN*00*12456*19980520*1200****2~
N1*P5**FI*999888777~
N1*IN**FI*654456654~
INS*Y*18*021*20*A***FT~
REF*0F*123456789~
REF*1L*123456001~
DTP*356*D8*19960523~
NM1*IL*1*DOE*JOHN*P***34*123456789~
PER*IP**HP*7172343334*WP*7172341240~
N3*100 MARKET ST*APT 3G~
N4*CAMP HILL*PA*17011**CY*CUMBERLAND~
DMG*D8*19400816*M~
HD*021**HLT~
DTP*348*D8*19960601~
COB*P*890111*5~
HD*021**DEN~
DTP*348*D8*19960601~
HD*021**VIS~
DTP*348*D8*19960601~
SE*21*12345~
GE*1*000012345~
IEA*1*000012345~"#;
    let (rest, obj) = Transmission::<_834>::parse(str).unwrap();
    println!("{rest}");
    println!("{obj:?}");
}

#[test]
fn render_834() {
    //source: https://github.com/EdiFabric/X12.NET/blob/master/Files/HIPAA/BenefitEnrollment.txt
    let str = r#"ISA*00*          *00*          *ZZ*386028429      *30*382328142      *050221*0602*U*00501*000012345*0*P*:~
GS*BE*386028429*382328142*20050221*0602*000012345*X*005010X220A1~
ST*834*12345*005010X220A1~
BGN*00*12456*19980520*1200****2~
N1*P5**FI*999888777~
N1*IN**FI*654456654~
INS*Y*18*021*20*A***FT~
REF*0F*123456789~
REF*1L*123456001~
DTP*356*D8*19960523~
NM1*IL*1*DOE*JOHN*P***34*123456789~
PER*IP**HP*7172343334*WP*7172341240~
N3*100 MARKET ST*APT 3G~
N4*CAMP HILL*PA*17011**CY*CUMBERLAND~
DMG*D8*19400816*M~
HD*021**HLT~
DTP*348*D8*19960601~
COB*P*890111*5~
HD*021**DEN~
DTP*348*D8*19960601~
HD*021**VIS~
DTP*348*D8*19960601~
SE*21*12345~
GE*1*000012345~
IEA*1*000012345~
"#;
    let obj: Transmission<_834> = Transmission::<_834> {
        isa: ISA {
            _01: "00".to_string(),
            _02: "          ".to_string(),
            _03: "00".to_string(),
            _04: "          ".to_string(),
            _05: "ZZ".to_string(),
            _06: "386028429      ".to_string(),
            _07: "30".to_string(),
            _08: "382328142      ".to_string(),
            _09: "050221".to_string(),
            _10: "0602".to_string(),
            _11: "U".to_string(),
            _12: "00501".to_string(),
            _13: "000012345".to_string(),
            _14: "0".to_string(),
            _15: "P".to_string(),
            _16: ":".to_string(),
        },
        functional_group: vec![FunctionalGroup {
            gs: GS {
                _01: "BE".to_string(),
                _02: "386028429".to_string(),
                _03: "382328142".to_string(),
                _04: "20050221".to_string(),
                _05: "0602".to_string(),
                _06: "000012345".to_string(),
                _07: "X".to_string(),
                _08: "005010X220A1".to_string(),
                ..Default::default()
            },
            segments: vec![_834 {
                st: ST {
                    _01: "834".to_string(),
                    _02: "12345".to_string(),
                    _03: Some("005010X220A1".to_string()),
                },
                bgn: BGN {
                    _01: "00".to_string(),
                    _02: "12456".to_string(),
                    _03: "19980520".to_string(),
                    _04: Some("1200".to_string()),
                    _08: Some("2".to_string()),
                    ..Default::default()
                },
                loop_1000: vec![
                    _834Loop1000 {
                        n1: N1 {
                            _01: "P5".to_string(),
                            _03: Some("FI".to_string()),
                            _04: Some("999888777".to_string()),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    _834Loop1000 {
                        n1: N1 {
                            _01: "IN".to_string(),
                            _03: Some("FI".to_string()),
                            _04: Some("654456654".to_string()),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                loop_2000: vec![_834Loop2000 {
                    ins: Some(INS {
                        _01: "Y".to_string(),
                        _02: "18".to_string(),
                        _03: Some("021".to_string()),
                        _04: Some("20".to_string()),
                        _05: Some("A".to_string()),
                        _08: Some("FT".to_string()),
                        ..Default::default()
                    }),
                    r#ref: vec![
                        REF {
                            _01: "0F".to_string(),
                            _02: Some("123456789".to_string()),
                            ..Default::default()
                        },
                        REF {
                            _01: "1L".to_string(),
                            _02: Some("123456001".to_string()),
                            ..Default::default()
                        },
                    ],
                    dtp: vec![DTP {
                        _01: "356".to_string(),
                        _02: "D8".to_string(),
                        _03: "19960523".to_string(),
                    }],
                    loop_2100: vec![_834Loop2100 {
                        nm1: Some(NM1 {
                            _01: "IL".to_string(),
                            _02: "1".to_string(),
                            _03: Some("DOE".to_string()),
                            _04: Some("JOHN".to_string()),
                            _05: Some("P".to_string()),
                            _08: Some("34".to_string()),
                            _09: Some("123456789".to_string()),
                            ..Default::default()
                        }),
                        per: Some(PER {
                            _01: "IP".to_string(),
                            _03: Some("HP".to_string()),
                            _04: Some("7172343334".to_string()),
                            _05: Some("WP".to_string()),
                            _06: Some("7172341240".to_string()),
                            ..Default::default()
                        }),
                        n3: Some(N3 {
                            _01: "100 MARKET ST".to_string(),
                            _02: Some("APT 3G".to_string()),
                        }),
                        n4: Some(N4 {
                            _01: Some("CAMP HILL".to_string()),
                            _02: Some("PA".to_string()),
                            _03: Some("17011".to_string()),
                            _05: Some("CY".to_string()),
                            _06: Some("CUMBERLAND".to_string()),
                            ..Default::default()
                        }),
                        dmg: Some(DMG {
                            _01: Some("D8".to_string()),
                            _02: Some("19400816".to_string()),
                            _03: Some("M".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    loop_2300: vec![
                        _834Loop2300 {
                            hd: Some(HD {
                                _01: "021".to_string(),
                                _03: Some("HLT".to_string()),
                                ..Default::default()
                            }),
                            dtp: vec![DTP {
                                _01: "348".to_string(),
                                _02: "D8".to_string(),
                                _03: "19960601".to_string(),
                            }],
                            loop_2320: vec![_834Loop2320 {
                                cob: Some(COB {
                                    _01: Some("P".to_string()),
                                    _02: Some("890111".to_string()),
                                    _03: Some("5".to_string()),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }],
                            ..Default::default()
                        },
                        _834Loop2300 {
                            hd: Some(HD {
                                _01: "021".to_string(),
                                _03: Some("DEN".to_string()),
                                ..Default::default()
                            }),
                            dtp: vec![DTP {
                                _01: "348".to_string(),
                                _02: "D8".to_string(),
                                _03: "19960601".to_string(),
                            }],
                            ..Default::default()
                        },
                        _834Loop2300 {
                            hd: Some(HD {
                                _01: "021".to_string(),
                                _03: Some("VIS".to_string()),
                                ..Default::default()
                            }),
                            dtp: vec![DTP {
                                _01: "348".to_string(),
                                _02: "D8".to_string(),
                                _03: "19960601".to_string(),
                            }],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                se: SE {
                    _01: "21".to_string(),
                    _02: "12345".to_string(),
                },
                ..Default::default()
            }],
            ge: GE {
                _01: "1".to_string(),
                _02: "000012345".to_string(),
            },
        }],
        iea: IEA {
            _01: "1".to_string(),
            _02: "000012345".to_string(),
        },
    };
    let obj_str = format!("{obj}");
    assert_eq!(str, obj_str);
}

#[test]
fn parse_834_2() {
    // source: https://www.emedny.org/hipaa/5010/transactions/834_sample_files/MCE834Sample_2.txt
    let str = r#"ISA*00*          *00*          *ZZ*EMEDNYMCR      *ZZ*8-DIGIT PLAN ID*191125*1409*^*00501*193290002*0*T*:~
GS*BE*EMEDNYMCR*ETIN*20191125*140914*193290002*X*005010X220A1~
ST*834*193290001*005010X220A1~
BGN*00*1932900000000002XF1932900000000001*20191125*140914****2~
QTY*TO*1~
N1*P5*MEDICAID*FI*141797357~
N1*IN**94*8-DIGIT PLAN ID~
INS*Y*18*024**A***TE~
REF*0F*XX99991X~
REF*17*XX99991X~
REF*3H*9919999999~
REF*ABB*XX99991X~
DTP*356*D8*20190101~
DTP*357*D8*20190101~
NM1*IL*1*SUBSCRIBER 1 LAST NAME*SUBSCRIBER 1 FIRST NAME*MI***34*999999991~
PER*IP**TE*9999991111~
N3*121 AYE ST~
N4*ANYTOWN*NY*12901~
DMG*D8*20010101*M**C~
HD*024**HLT**IND~
DTP*348*D8*20190101~
DTP*349*D8*20190101~
LX*1~
NM1*Y2*2******SV*8-DIGIT PLAN ID*72~
LS*2700~
LX*1~
N1*75*FAM IND~
REF*17*F~
LX*2~
N1*75*COE CODE~
REF*17*30~
DTP*007*D8*20191120~
LX*3~
N1*75*MONEY CODE~
REF*17*00~
DTP*007*D8*20191120~
LX*4~
N1*75*FISCAL COUNTY~
REF*17*36019~
DTP*007*D8*20191120~
LX*5~
N1*75*AID CAT CODE~
REF*17*31~
DTP*007*D8*20191120~
LX*6~
N1*75*BEN PKG CODE~
REF*17*14~
DTP*007*D8*20190101~
LX*7~
N1*75*ORIGIN CODE~
REF*17*U~
DTP*007*D8*20190101~
LX*8~
N1*75*COPAY EXEMPT IND~
REF*ZZ*Y~
LX*9~
N1*75*MED RATE CODE~
REF*17*2201~
LX*10~
N1*75*ADDL MAINT REASON~
REF*17*CANCEL~
LX*11~
N1*75*NAMI~
REF*9V*0~
DTP*007*D8*20190101~
LX*12~
N1*75*EXCESS~
REF*9V*0~
DTP*007*D8*20190101~
LE*2700~
SE*69*193290001~
ST*834*193290002*005010X220A1~
BGN*00*1932900000000002XF1932900000000002*20191125*140915****2~
QTY*TO*1~
N1*P5*MEDICAID*FI*141797357~
N1*IN**94*8-DIGIT PLAN ID~
INS*Y*18*024**A***TE~
REF*0F*XX99992X~
REF*17*XX99992X~
REF*3H*9929999999~
REF*ABB*XX99992X~
DTP*356*D8*20190101~
DTP*357*D8*20191130~
NM1*IL*1*SUBSCRIBER 2 LAST NAME*SUBSCRIBER 2 FIRST NAME*MI***34*999999992~
PER*IP**TE*9999992222~
N3*122 BEE ST*CARE OF NAME FOR SUB 2~
N4*BNYTOWN*NY*14001**CY*36029~
DMG*D8*20020202*F**C~
HD*024**HLT**IND~
DTP*348*D8*20190101~
DTP*349*D8*20191130~
LX*1~
NM1*Y2*2******SV*8-DIGIT PLAN ID*72~
LS*2700~
LX*1~
N1*75*FAM IND~
REF*17*I~
LX*2~
N1*75*COE CODE~
REF*17*30~
DTP*007*RD8*20190101-20191130~
LX*3~
N1*75*MONEY CODE~
REF*17*00~
DTP*007*RD8*20190101-20191130~
LX*4~
N1*75*FISCAL COUNTY~
REF*17*36029~
DTP*007*RD8*20190101-20191130~
LX*5~
N1*75*AID CAT CODE~
REF*17*90~
DTP*007*RD8*20190101-20191130~
LX*6~
N1*75*BEN PKG CODE~
REF*17*14~
DTP*007*RD8*20190101-20191130~
LX*7~
N1*75*ORIGIN CODE~
REF*17*U~
DTP*007*RD8*20190101-20191130~
LX*8~
N1*75*COPAY EXEMPT IND~
REF*ZZ*Y~
LX*9~
N1*75*MED RATE CODE~
REF*17*2205~
LX*10~
N1*75*ADDL MAINT REASON~
REF*17*TERM~
LX*11~
N1*75*NAMI~
REF*9V*0.00~
DTP*007*D8*20190101~
LX*12~
N1*75*EXCESS~
REF*9V*0.00~
DTP*007*D8*20190101~
LX*13~
N1*75*WMS ENROLL/DISENROLL REASON CODE~
REF*ZZ*93~
DTP*007*D8*20191201~
LE*2700~
SE*73*193290002~
ST*834*193290003*005010X220A1~
BGN*00*1932900000000002XF1932900000000003*20191125*140916****2~
QTY*TO*1~
N1*P5*MEDICAID*FI*141797357~
N1*IN**94*8-DIGIT PLAN ID~
INS*Y*18*001**A***AC~
REF*0F*XX99993X~
REF*1L*QHPGRPID99993X~
REF*17*XX99993X~
REF*23*QHPMEMID99993X~
REF*3H*9939999999~
REF*ABB*XX99993X~
REF*ZZ*QHPSUBID99993X~
DTP*303*D8*20191125~
DTP*356*D8*20190101~
NM1*IL*1*SUBSCRIBER 3 LAST NAME*SUBSCRIBER 3 FIRST NAME*MI***34*999999993~
PER*IP**TE*9999993333~
N3*123 SEA ST~
N4*CNYTOWN*NY*12414~
DMG*D8*20030303*F**H~
NM1*70*1*SUBSCRIBER 3 LAST NAME*SUBSCRIBER 3 FIRST NAME*MI***34*999999993~
DMG*D8*20080303*F**8~
HD*001**HLT**IND~
DTP*348*D8*20190101~
REF*X9*QHPPLCYID99993X~
LX*1~
NM1*Y2*2******SV*8-DIGIT PLAN ID*72~
COB*U*1903900000000001*1~
DTP*344*D8*20190901~
NM1*IN*2*COMMERCIAL INSURANCE NAME*****NI*95222~
N3*2850 WEST XXXXX BOULEV~
N4*DETROIT*MI*48202~
LS*2700~
LX*1~
N1*75*FAM IND~
REF*17*F~
LX*2~
N1*75*COE CODE~
REF*17*30~
DTP*007*D8*20190101~
LX*3~
N1*75*MONEY CODE~
REF*17*00~
DTP*007*D8*20190101~
LX*4~
N1*75*FISCAL COUNTY~
REF*17*36039~
DTP*007*D8*20190101~
LX*5~
N1*75*AID CAT CODE~
REF*17*91~
DTP*007*D8*20190101~
LX*6~
N1*75*BEN PKG CODE~
REF*17*14~
DTP*007*D8*20190101~
LX*7~
N1*75*ORIGIN CODE~
REF*17*U~
DTP*007*D8*20190101~
LX*8~
N1*75*COPAY EXEMPT IND~
REF*ZZ*N~
LX*9~
N1*75*MED RATE CODE~
REF*17*2205~
LX*10~
N1*75*TPL CVRG~
REF*ZZ*1903900000000001-030405060708111314171819202122~
LX*11~
N1*75*NAMI~
REF*9V*0~
DTP*007*D8*20190201~
LX*12~
N1*75*EXCESS~
REF*9V*0~
DTP*007*D8*20190201~
LE*2700~
SE*79*193290003~
ST*834*193290004*005010X220A1~
BGN*00*1932900000000002XF1932900000000004*20191125*140917****2~
QTY*TO*1~
N1*P5*MEDICAID*FI*141797357~
N1*IN**94*8-DIGIT PLAN ID~
INS*Y*18*024**A***TE~
REF*0F*XX99996X~
REF*17*XX99996X~
REF*3H*9969999999~
REF*ABB*XX99996X~
DTP*356*D8*20180101~
DTP*357*D8*20191130~
NM1*IL*1*SUBSCRIBER 6A LAST NAME*SUBSCRIBER 6A FIRST NAME*MI***34*999999996~
PER*IP**TE*9999996666~
N3*126 FOX ST~
N4*FNYTOWN*NY*14456~
DMG*D8*20060606*F**B~
LUI*LE*ENG**6~
LUI*LE*ENG**7~
LUI*LE*ENG**5~
HD*024**HLT**IND~
DTP*348*D8*20180101~
DTP*349*D8*20191130~
LX*1~
NM1*Y2*2******SV*8-DIGIT PLAN ID*72~
LS*2700~
LX*1~
N1*75*FAM IND~
REF*17*I~
LX*2~
N1*75*COE CODE~
REF*17*30~
DTP*007*RD8*20180101-20191130~
LX*3~
N1*75*MONEY CODE~
REF*17*00~
DTP*007*RD8*20180101-20191130~
LX*4~
N1*75*FISCAL COUNTY~
REF*17*36069~
DTP*007*RD8*20180101-20191130~
LX*5~
N1*75*AID CAT CODE~
REF*17*90~
DTP*007*RD8*20180101-20191130~
LX*6~
N1*75*BEN PKG CODE~
REF*17*70~
DTP*007*RD8*20180101-20181231~
LX*7~
N1*75*BEN PKG CODE~
REF*17*77~
DTP*007*RD8*20190101-20191130~
LX*8~
N1*75*ORIGIN CODE~
REF*17*U~
DTP*007*RD8*20180101-20191130~
LX*9~
N1*75*RRE CODES~
REF*17*02~
LX*10~
N1*75*COPAY EXEMPT IND~
REF*ZZ*Y~
LX*11~
N1*75*MED RATE CODE~
REF*17*2205~
LX*12~
N1*75*ADDL MAINT REASON~
REF*17*TERM~
LX*13~
N1*75*NAMI/EXCESS MSG~
REF*17*CONTACT DISTRICT FOR INFORMATION ON SPENDDOWN/NAMI~
DTP*007*D8*20190101~
LX*14~
N1*75*DISENROLL RSN~
REF*ZZ*LPS~
LE*2700~
SE*78*193290004~
ST*834*193290005*005010X220A1~
BGN*00*1932900000000002XF1932900000000005*20191125*140918****2~
QTY*TO*1~
N1*P5*MEDICAID*FI*141797357~
N1*IN**94*8-DIGIT PLAN ID~
INS*Y*18*024*03*A***TE***D8*20190630~
REF*0F*XX99994X~
REF*17*XX99994X~
REF*3H*9949999999~
REF*ABB*XX99994X~
DTP*356*D8*20190101~
DTP*357*D8*20190630~
NM1*IL*1*SUBSCRIBER 4 LAST NAME*SUBSCRIBER 4 FIRST NAME*MI***34*999999994~
PER*IP**TE*9999994444~
N3*124 DEE ST*CARE OF NAME FOR SUB 4~
N4*DNYTOWN*NY*13367~
DMG*D8*20040404*F**B~
HD*024**HLT**IND~
DTP*348*D8*20190101~
DTP*349*D8*20190630~
LX*1~
NM1*Y2*2******SV*8-DIGIT PLAN ID*72~
LS*2700~
LX*1~
N1*75*FAM IND~
REF*17*I~
LX*2~
N1*75*COE CODE~
REF*17*30~
DTP*007*RD8*20190101-20190630~
LX*3~
N1*75*MONEY CODE~
REF*17*00~
DTP*007*RD8*20190101-20190630~
LX*4~
N1*75*FISCAL COUNTY~
REF*17*36049~
DTP*007*RD8*20190101-20190630~
LX*5~
N1*75*AID CAT CODE~
REF*17*90~
DTP*007*RD8*20190101-20190630~
LX*6~
N1*75*BEN PKG CODE~
REF*17*70~
DTP*007*RD8*20190101-20190131~
LX*7~
N1*75*BEN PKG CODE~
REF*17*14~
DTP*007*RD8*20190201-20190630~
LX*8~
N1*75*ORIGIN CODE~
REF*17*U~
DTP*007*RD8*20190101-20190630~
LX*9~
N1*75*COPAY EXEMPT IND~
REF*ZZ*Y~
LX*10~
N1*75*MED RATE CODE~
REF*17*2205~
LX*11~
N1*75*ADDL MAINT REASON~
REF*17*TERM~
LX*12~
N1*75*WMS ENROLL/DISENROLL REASON CODE~
REF*ZZ*85~
DTP*007*D8*20190701~
LE*2700~
SE*69*193290005~
ST*834*193290006*005010X220A1~
BGN*00*1932900000000002XF1932900000000006*20191125*140920****2~
QTY*TO*1~
N1*P5*MEDICAID*FI*141797357~
N1*IN**94*8-DIGIT PLAN ID~
INS*Y*18*024**A***TE~
REF*0F*XX99995X~
REF*17*XX99995X~
REF*3H*9959999999~
REF*ABB*XX99995X~
DTP*356*D8*20200201~
DTP*357*D8*20200201~
NM1*IL*1*SUBSCRIBER 5 LAST NAME*SUBSCRIBER 5 FIRST NAME*MI***34*999999995~
PER*IP**TE*9999995555~
N3*125 EAT ST~
N4*ENYTOWN*NY*11758~
DMG*D8*20050505*M**A~
HD*024**HLT**IND~
DTP*348*D8*20200201~
DTP*349*D8*20200201~
LX*1~
NM1*Y2*2******SV*8-DIGIT PLAN ID*72~
LS*2700~
LX*1~
N1*75*FAM IND~
REF*17*F~
LX*2~
N1*75*COE CODE~
REF*17*30~
DTP*007*D8*20200201~
LX*3~
N1*75*MONEY CODE~
REF*17*00~
DTP*007*D8*20200201~
LX*4~
N1*75*FISCAL COUNTY~
REF*17*36059~
DTP*007*D8*20200201~
LX*5~
N1*75*AID CAT CODE~
REF*17*31~
DTP*007*D8*20200201~
LX*6~
N1*75*BEN PKG CODE~
REF*17*66~
DTP*007*D8*20200201~
LX*7~
N1*75*ORIGIN CODE~
REF*17*U~
DTP*007*D8*20200201~
LX*8~
N1*75*COPAY EXEMPT IND~
REF*ZZ*Y~
LX*9~
N1*75*MED RATE CODE~
REF*17*2201~
LX*10~
N1*75*ADDL MAINT REASON~
REF*17*CANCEL~
LX*11~
N1*75*NAMI~
REF*9V*0~
DTP*007*D8*20200201~
LX*12~
N1*75*EXCESS~
REF*9V*240.75~
DTP*007*D8*20200201~
LE*2700~
SE*69*193290006~
ST*834*193290007*005010X220A1~
BGN*00*1932900000000002XF1932900000000007*20191125*140919****2~
QTY*TO*1~
N1*P5*MEDICAID*FI*141797357~
N1*IN**94*8-DIGIT PLAN ID~
INS*Y*18*021*28*A***AC~
REF*0F*XX99995X~
REF*17*XX99995X~
REF*3H*9959999999~
REF*ABB*XX99995X~
DTP*356*D8*20200101~
NM1*IL*1*SUBSCRIBER 5 LAST NAME*SUBSCRIBER 5 FIRST NAME*MI***34*999999995~
PER*IP**TE*9999995555~
N3*125 EAT ST~
N4*ENYTOWN*NY*11758**CY*36059~
DMG*D8*20050505*M**A~
LUI*LE*CHI**6~
LUI*LE*CHI**7~
NM1*31*1~
N3*PATIENT'S MAILING ADDRESS*#505~
N4*ENYTOWN*NY*11758~
HD*021**HLT**IND~
DTP*348*D8*20200101~
LX*1~
NM1*Y2*2******SV*8-DIGIT PLAN ID*72~
COB*P*7AC0Y25NP85*1~
DTP*344*D8*20200101~
DTP*345*D8*20201231~
NM1*IN*2*MEDICARE-A~
COB*P*7AC0Y25NP55*1~
DTP*344*D8*20200101~
DTP*345*D8*20201231~
NM1*IN*2*MEDICARE-B~
COB*S*9999999999999995*1~
DTP*344*D8*20200101~
DTP*345*D8*20201231~
NM1*IN*2*COMMERCIAL INSURANCE NAME*****NI*94222~
N3*2850 WEST XXXXX BOULEV~
N4*DETROIT*MI*48202~
COB*U*9999999999999955*1~
DTP*344*D8*20200101~
DTP*345*D8*20201231~
LS*2700~
LX*1~
N1*75*FAM IND~
REF*17*I~
LX*2~
N1*75*COE CODE~
REF*17*30~
DTP*007*D8*20200101~
LX*3~
N1*75*MONEY CODE~
REF*17*00~
DTP*007*D8*20200101~
LX*4~
N1*75*FISCAL COUNTY~
REF*17*36059~
DTP*007*D8*20200101~
LX*5~
N1*75*AID CAT CODE~
REF*17*90~
DTP*007*D8*20200101~
LX*6~
N1*75*BEN PKG CODE~
REF*17*14~
DTP*007*D8*20200101~
LX*7~
N1*75*ORIGIN CODE~
REF*17*U~
DTP*007*D8*20200101~
LX*8~
N1*75*RRE CODES~
REF*17*05080910H1H3H9359296~
LX*9~
N1*75*COPAY EXEMPT IND~
REF*ZZ*Y~
LX*10~
N1*75*MED RATE CODE~
REF*17*2205~
LX*11~
N1*75*WMS ENROLL/DISENROLL REASON CODE~
REF*ZZ*02~
DTP*007*D8*20200101~
LX*12~
N1*75*NAMI~
REF*9V*0~
DTP*007*D8*20200101~
LX*13~
N1*75*EXCESS~
REF*9V*240.75~
DTP*007*D8*20200101~
LX*14~
N1*75*TPL CVRG~
REF*ZZ*9999999999999995-03040506070811131417~
LX*15~
N1*75*TPL CVRG~
REF*ZZ*9999999999999955-0304050607082122~
LE*2700~
SE*99*193290007~
GE*7*193290002~
IEA*1*193290002~"#;
    let (rest, obj) = Transmission::<_834>::parse(str).unwrap();
    println!("{rest}");
    println!("{obj:?}");
}

#[test]
fn parse_834_3() {
    // source:https://www.emedny.org/hipaa/5010/transactions/834_sample_files/MCE834Sample_verification.txt
    let str = r#"ISA*00*          *00*          *ZZ*EMEDNYVER      *ZZ*8-DIGIT PLAN ID*191119*2020*^*00501*193230001*0*T*:~
GS*BE*EMEDNYVER*ETIN*20191119*202000*193230001*X*005010X220A1~
ST*834*193230001*005010X220A1~
BGN*00*1932300000000011XF1932300000000001*20191119*202004****4~
QTY*TO*1~
N1*P5*MEDICAID*FI*141797357~
N1*IN**94*8-DIGIT PLAN ID~
INS*Y*18*030**A~
REF*0F*XX99996X~
REF*17*XX99996X~
REF*3H*9969999999~
REF*ABB*XX99996X~
DTP*356*D8*20180101~
NM1*IL*1*SUBSCRIBER 6 LAST NAME*SUBSCRIBER 6 FIRST NAME*MI***34*999999996~
PER*IP**TE*9999996666~
N3*126 FOX ST~
N4*FNYTOWN*NY*14456~
DMG*D8*20000606*F**B~
LUI*LE*ENG**6~
LUI*LE*ENG**7~
LUI*LE*ENG**5~
HD*030**HLT**IND~
DTP*348*D8*20180101~
LX*1~
NM1*Y2*2******SV*8-DIGIT PLAN ID*72~
COB*U*1923000000000001*1~
DTP*344*D8*20190901~
NM1*IN*2*COMMERCIAL INSURANCE NAME*****NI*95222~
N3*2850 WEST XXXXX BOULEV~
N4*DETROIT*MI*48202~
LS*2700~
LX*1~
N1*75*FAM IND~
REF*17*F~
LX*2~
N1*75*COE CODE~
REF*17*30~
DTP*007*D8*20190101~
LX*3~
N1*75*MONEY CODE~
REF*17*00~
DTP*007*D8*20190101~
LX*4~
N1*75*FISCAL COUNTY~
REF*17*36069~
DTP*007*D8*20190101~
LX*5~
N1*75*AID CAT CODE~
REF*17*91~
DTP*007*D8*20190101~
LX*6~
N1*75*BEN PKG CODE~
REF*17*70~
DTP*007*RD8*20180101-20181231~
LX*7~
N1*75*BEN PKG CODE~
REF*17*77~
DTP*007*D8*20190101~
LX*8~
N1*75*ORIGIN CODE~
REF*17*U~
DTP*007*D8*20180101~
LX*9~
N1*75*RECERT DATE~
REF*ZZ*20191031~
LX*10~
N1*75*RRE CODES~
REF*17*02~
LX*11~
N1*75*COPAY EXEMPT IND~
REF*ZZ*N~
LX*12~
N1*75*MED RATE CODE~
REF*17*2205~
LX*13~
N1*75*TPL CVRG~
REF*ZZ*1923000000000001-030405060708111314171819202122~
LX*14~
N1*75*NAMI/EXCESS MSG~
REF*17*CONTACT DISTRICT FOR INFORMATION ON SPENDDOWN/NAMI~
DTP*007*D8*20190101~
LX*15~
N1*75*WMS ENROLL/DISENROLL REASON CODE~
REF*ZZ*02~
DTP*007*D8*20180101~
LE*2700~
SE*85*193230001~
ST*834*193230002*005010X220A1~
BGN*00*1932300000000011XF1932300000000002*20191119*202005****4~
QTY*TO*1~
N1*P5*MEDICAID*FI*141797357~
N1*IN**94*8-DIGIT PLAN ID~
INS*Y*18*030**A~
REF*0F*XX99994X~
REF*17*XX99994X~
REF*3H*9949999999~
REF*ABB*XX99994X~
DTP*356*D8*20190101~
NM1*IL*1*SUBSCRIBER 4NA LAST NAME*SUBSCRIBER 4NA FIRST NAME*MI***34*999999994~
PER*IP**TE*9999994444~
N3*124 DEE ST*CARE OF NAME FOR SUB 4~
N4*DNYTOWN*NY*13367~
DMG*D8*20040404*F**8~
NM1*QD*1*CASE NAME~
HD*030**HLT**IND~
DTP*348*D8*20190101~
LX*1~
NM1*Y2*2******SV*8-DIGIT PLAN ID*72~
COB*P*4AC0Y25NP81*1~
DTP*344*D8*20190101~
DTP*345*D8*20191231~
NM1*36*2*MEDICARE-A~
COB*P*4AC0Y25NP82*1~
DTP*344*D8*20190601~
DTP*345*D8*20191231~
NM1*36*2*MEDICARE-B~
LS*2700~
LX*1~
N1*75*FAM IND~
REF*17*F~
LX*2~
N1*75*COE CODE~
REF*17*30~
DTP*007*D8*20190101~
LX*3~
N1*75*MONEY CODE~
REF*17*00~
DTP*007*D8*20190101~
LX*4~
N1*75*FISCAL COUNTY~
REF*17*36049~
DTP*007*D8*20190101~
LX*5~
N1*75*AID CAT CODE~
REF*17*91~
DTP*007*D8*20190101~
LX*6~
N1*75*BEN PKG CODE~
REF*17*70~
DTP*007*RD8*20190101-20190131~
LX*7~
N1*75*BEN PKG CODE~
REF*17*14~
DTP*007*D8*20190201~
LX*8~
N1*75*ORIGIN CODE~
REF*17*U~
DTP*007*D8*20190101~
LX*9~
N1*75*COPAY EXEMPT IND~
REF*ZZ*N~
LX*10~
N1*75*MED RATE CODE~
REF*17*2205~
LX*11~
N1*75*NAMI~
REF*9V*0.00~
DTP*007*D8*20190101~
LX*12~
N1*75*EXCESS~
REF*9V*0.00~
DTP*007*D8*20190101~
LX*13~
N1*75*WMS ENROLL/DISENROLL REASON CODE~
REF*ZZ*06~
DTP*007*D8*20190101~
LX*14~
N1*75*DISABILITY ACCOMMODATION INDICATOR~
REF*ZZ*V1~
LX*15~
N1*75*LOW INCOME SUBSIDY CVRG~
REF*ZZ*2~
DTP*007*D8*20190901~
LX*16~
N1*75*MEDICARE PART-D~
REF*17*H2775~
DTP*007*RD8*20190101-20191231~
LE*2700~
SE*92*193230002~
ST*834*193230003*005010X220A1~
BGN*00*1932300000000011XF1932300000000003*20191119*202007****4~
QTY*TO*1~
N1*P5*MEDICAID*FI*141797357~
N1*IN**94*8-DIGIT PLAN ID~
INS*Y*18*030**A~
REF*0F*XX99997X~
REF*17*XX99997X~
REF*3H*9979999999~
REF*ABB*XX99997X~
DTP*356*D8*20180201~
NM1*IL*1*SUBSCRIBER 7 LAST NAME*SUBSCRIBER 7 FIRST NAME*MI***34*999999997~
PER*IP**TE*9999997777~
N3*127 GREEN ST*APT 7G~
N4*GNYTOWN*NY*10512**CY*36079~
DMG*D8*20070707*F**8~
HD*030**HLT**IND~
DTP*348*D8*20180201~
LX*1~
NM1*Y2*2******SV*8-DIGIT PLAN ID*72~
LS*2700~
LX*1~
N1*75*FAM IND~
REF*17*I~
LX*2~
N1*75*COE CODE~
REF*17*30~
DTP*007*D8*20180201~
LX*3~
N1*75*MONEY CODE~
REF*17*00~
DTP*007*D8*20180201~
LX*4~
N1*75*FISCAL COUNTY~
REF*17*66~
DTP*007*D8*20180201~
LX*5~
N1*75*AID CAT CODE~
REF*17*90~
DTP*007*D8*20180201~
LX*6~
N1*75*BEN PKG CODE~
REF*17*66~
DTP*007*D8*20180201~
LX*7~
N1*75*ORIGIN CODE~
REF*17*D~
DTP*007*D8*20180201~
LX*8~
N1*75*COPAY EXEMPT IND~
REF*ZZ*Y~
LX*9~
N1*75*MED RATE CODE~
REF*17*2205~
LX*10~
N1*75*NAMI~
REF*9V*0.00~
DTP*007*D8*20190101~
LX*11~
N1*75*EXCESS~
REF*9V*0.00~
DTP*007*D8*20190101~
LX*12~
N1*75*WMS ENROLL/DISENROLL REASON CODE~
REF*ZZ*05~
DTP*007*D8*20180101~
LE*2700~
SE*68*193230003~
GE*3*193230001~
IEA*1*193230001~"#;
    let (rest, obj) = Transmission::<_834>::parse(str).unwrap();
    println!("{rest}");
    println!("{obj:?}");
}

#[test]
fn parse_834_4() {
    // source: https://www.vbaplans.com/media/xh1dj3ft/834-companion-guide0720.pdf
    let s = r#"ISA*00* *00* *30*123456789 *ZZ*VBA *190424*1253*^*00501*000000001*0*T*:~
GS*BE*123456789*VBA*20190402*1253*1*X*005010X220A1~
ST*834*0001*005010X220A1~
BGN*00*018140498*20190402*125319****4~
REF*38*1234~
DTP*007*D8*20191011~
N1*P5*VBA GROUP NAME*FI*123456789~
N1*IN*VBA*FI*25-1149206~
INS*Y*18*030**A***FT~
REF*0F*123456789~
DTP*336*D8*20190101~
NM1*IL*1*Doe*John*M***34*123456789~
PER*IP**HP*4128814900~
N3*400 Lydia Street~
N4*Carnegie*PA*15106~
DMG*D8*19790910*M~
HD*030**VIS*1234*EMP~
DTP*348*D8*20190101~
REF*1L*0000~
INS*Y*18*030**A***FT~
REF*0F*987654321~
DTP*336*D8*20190101~
NM1*IL*1*Flinstone*Fred*M***34*987654321~
PER*IP**HP*4128814901~
N3*300 Lydia Street~
N4*Carnegie*PA*15106~
DMG*D8*19801010*M~
HD*030**VIS*1234*ECH~
DTP*348*D8*20190101~
REF*1L*0000~
INS*N*19*030**A~
REF*0F*987654321~
DTP*336*D8*20190101~
NM1*IL*1*Flinstone*Jane*M***34*923456781~
PER*IP**HP*4128814901~
N3*300 Lydia Street~
N4*Carnegie*PA*15106~
DMG*D8*20000101*F~
HD*030**VIS*1234~
DTP*348*D8*20190101~
REF*1L*0000~
SE*40*0001~
GE*1*1~
IEA*1*000000001~"#;
    let (rest, obj) = Transmission::<_834>::parse(s).unwrap();
    println!("{rest}");
    println!("{obj:?}");
}
