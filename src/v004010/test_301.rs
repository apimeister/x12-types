use crate::v004010::*;

#[test]
fn test_301() {
    let obj = _301 {
        st: ST {
            _01: "301".to_string(),
            _02: "33233".to_string(),
        },
        b1: B1 {
            _01: Some("SNDR".to_string()),
            _02: "ERXX412223".to_string(),
            _03: Some("20221121".to_string()),
            _04: "A".to_string(),
        },
        y3: Y3 {
            _01: "ERXX412223".to_string(),
            _02: Some("SNDR".to_string()),
            _03: Some("20230104".to_string()),
            _07: Some("20221229".to_string()),
            _10: Some("HP".to_string()),
            ..Default::default()
        },
        loop_y4: vec![_301LoopY4 {
            y4: Some(Y4 {
                _01: Some("ERXX412223".to_string()),
                _03: Some("20221216".to_string()),
                _05: Some("1".to_string()),
                _06: Some("45G1".to_string()),
                ..Default::default()
            }),
            w09: None,
        }],
        n9: vec![N9 {
            _01: "BN".to_string(),
            _02: "ERXX412223".to_string(),
            ..Default::default()
        }],
        loop_n1: vec![_301LoopN1 {
            n1: Some(N1 {
                _01: "SH".to_string(),
                _02: Some("ABC GMBH".to_string()),
                _03: Some("25".to_string()),
                _04: Some("312343123".to_string()),
                ..Default::default()
            }),
            n3: Some(N3 {
                _01: "TEST STR. 56".to_string(),
                _02: Some("XA 124324 4".to_string()),
            }),
            n4: Some(N4 {
                _01: Some("MUNICH".to_string()),
                _02: Some("BY".to_string()),
                _03: Some("80348".to_string()),
                _04: Some("DE".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }],
        loop_r4: vec![
            _301LoopR4 {
                r4: R4 {
                    _01: "R".to_string(),
                    _02: Some("UN".to_string()),
                    _03: Some("DEWKD".to_string()),
                    _04: Some("WACKERSDORF".to_string()),
                    _05: Some("DE".to_string()),
                    _08: Some("BY".to_string()),
                    ..Default::default()
                },
                dtm: vec![],
            },
            _301LoopR4 {
                r4: R4 {
                    _01: "L".to_string(),
                    _02: Some("UN".to_string()),
                    _03: Some("DEHAM".to_string()),
                    _04: Some("HAMBURG".to_string()),
                    _05: Some("DE".to_string()),
                    _08: Some("HH".to_string()),
                    ..Default::default()
                },
                dtm: vec![DTM {
                    _01: "649".to_string(),
                    _02: Some("20230102".to_string()),
                    ..Default::default()
                }],
            },
            _301LoopR4 {
                r4: R4 {
                    _01: "D".to_string(),
                    _02: Some("K".to_string()),
                    _03: Some("35180".to_string()),
                    _04: Some("ITAPOA".to_string()),
                    _05: Some("BR".to_string()),
                    _08: Some("SC".to_string()),
                    ..Default::default()
                },
                dtm: vec![],
            },
        ],
        loop_lx: vec![_301LoopLx {
            lx: LX {
                _01: "2".to_string(),
            },
            l0: Some(L0 {
                _01: Some("1".to_string()),
                _04: Some("14000".to_string()),
                _05: Some("G".to_string()),
                _08: Some("1".to_string()),
                _09: Some("CNT".to_string()),
                _11: Some("K".to_string()),
                _12: Some("HP".to_string()),
                ..Default::default()
            }),
            l5: Some(L5 {
                _01: Some("1".to_string()),
                _02: Some("VEHICLES:PARTS".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }],
        v1: vec![V1 {
            _01: "3465322".to_string(),
            _02: Some("CAP SAN ANTONIO".to_string()),
            _03: Some("DK".to_string()),
            _04: Some("456S".to_string()),
            _08: Some("L".to_string()),
            ..Default::default()
        }],
        se: SE {
            _01: "17".to_string(),
            _02: "33233".to_string(),
        },
        ..Default::default()
    };
    let obj = format!("{obj}");
    println!("{obj}");
}

#[test]
fn test_parse_301_nom() {
    let str = r#"ST*301*33233~
B1*SNDR*ERXX412223*20221121*A~
Y3*ERXX412223*SNDR*20230104*20230105***20221229***HP~
Y4*ERXX412223**20221216**1*45G1~
N9*BN*ERXX412223~
N1*SH*ABC GMBH*25*312343123~
N3*TEST STR. 56*XA 124324 4~
N4*MUNICH*BY*80348*DE~
R4*R*UN*DEWKD*WACKERSDORF*DE***BY~
R4*L*UN*DEHAM*HAMBURG*DE***HH~
DTM*649*20230102~
R4*D*K*35180*ITAPOA*BR***SC~
LX*2~
L0*1***14000*G***1*CNT**K*HP~
L5*1*VEHICLES:PARTS~
V1*3465322*CAP SAN ANTONIO*DK*456S****L~
SE*17*33233~"#;
    let obj = _301::parse(str).unwrap();
    println!("{obj:?}");
    assert!(obj.0.is_empty());
    let obj = obj.1;
    assert_eq!(obj.st._01, "301");
    assert_eq!(obj.st._02, "33233");
    assert_eq!(obj.se._01, "17");
    assert_eq!(obj.se._02, "33233");
}

#[test]
fn test_2() {
    let str = r#"ISA*00*          *00*          *ZZ*XXXXXX999      *ZZ*XXXX           *230616*1315*U*00401*000999999*0*P*>~GS*RO*XXXXXX999*XXXX*20230616*1315*999999*X*004010~ST*301*999999~B1*301*3PHL999999*20230616*N~Y3*3PHL999999*XXXX*20230711*20230807*****CH~Y4*3PHL999999****1*4510~N9*SFC*100950400~R4*R*UN*USLVI*LIVONIA MI US*US***MI~R4*L*UN*USLGB*LONG BEACH CA US*US*ITS LONG BEACH**CA~R4*D*UN*AUMEL*MELBOURNE VIC AU*AU***VI~LX*1~K1*CONTAINER(S) 1: MODE OF TRANSP*ORT: TRUCK.~L0*1***6031*G***1*CNT**K~V1*9450571*SYNERGY BUSAN*MH*324S****L~SE*13*999999~GE*1*999999~IEA*1*000999999~"#;
    let (rest, obj) = Transmission::<_301>::parse(str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn test_3() {
    let str = r#"ISA*00*          *00*          *ZZ*XXXX           *ZZ*XXXXXXXXX      *230607*1940*U*00401*000107033*0*P*>~GS*RO*XXXX*XXXXXXXXX*20230607*1940*107033*X*004010~ST*301*105115~B1*XXXX*3JAX019999*20230607*U~Y3*3JAX011892*XXXX*20230615*20230625***20230613~Y4*3JAX099999**20230614**1*42G1~N9*BN*3JAX099999~N9*SI*170310636~N1*SH*DUPONT FILAMENTS-AMERICAS, LLC*25*100952999~N3*CENTRE RD 974~N4*WILMINGTON*DE*19805*US~G61*EM*NN***DUPONT FILAMENTS-AME~R4*R*UN*USNYC*NEW YORK*US~R4*L*UN*USNYC*NEW YORK*US*APM TERMINALS ELIZABETH~R4*D*UN*COCTG*CARTAGENA*CO*SOCIEDAD PORTUARIA DE CARTAGEN~R4*N*UN*COCTG*CARTAGENA*CO~LX*1~L0*1***21844*G***1*CNT**K*PP~L5*1*CHEMICALS:(NOS)~H1*1325*4.1*I*FLAMMABLE SOLID, ORGANIC, N.O.*****II~H2*S.*METHYL METHACRYLATE~H1*1325*4.1*I*FLAMMABLE SOLID, ORGANIC, N.O.*****II~H2*S.*METHYL METHACRYLATE~H1*1325*4.1*I*FLAMMABLE SOLID, ORGANIC, N.O.*****II~H2*S.*METHYL METHACRYLATE~H1*1325*4.1*I*FLAMMABLE SOLID, ORGANIC, N.O.*****II~H2*S.*METHYL METHACRYLATE~H1*1325*4.1*I*FLAMMABLE SOLID, ORGANIC, N.O.*****II~H2*S.*METHYL METHACRYLATE~H1*1325*4.1*I*FLAMMABLE SOLID, ORGANIC, N.O.*****II~H2*S.*METHYL METHACRYLATE~V1*9193240*MAERSK KENTUCKY*SG*324S****L~SE*31*105115~GE*1*107033~IEA*1*000107033~"#;
    let (rest, obj) = Transmission::<_301>::parse(str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn test_4() {
    let str = r#"ISA*00*          *00*          *ZZ*XXXXXXXXX      *ZZ*XXXXXXXX       *230603*1728*U*00401*000099999*0*P*>~GS*RO*XXXXXXXXX*XXXXXXXX*20230603*1728*99999*X*004010~ST*301*99999~B1*XXXX*3BOG009999*20230603*U~Y3*3BOG009999*XXXX*20230607*20230620***20230606~Y4*3BOG009999**20230601**1*45R1~N9*BN*3BOG009999~N1*SH*NOVACAMPO S.A. S.C.I*25*100355156~N3*KM 18~N4*CHIQUINQUIRA*BO**CO~R4*R*UN*COCTG*CARTAGENA*CO~R4*L*UN*COCTG*CARTAGENA*CO*TERMINAL DE CONTENEDORES DE CO~R4*D*UN*NLRTM*ROTTERDAM*NL*ECT DELTA TERMINALS~R4*N*UN*NLRTM*ROTTERDAM*NL~W09*CZ*8.0*CE*8.0*CE****0~H3*MR*GENSET YES~LX*1~L0*1***13852*G***1*CNT**K*PP~L5*1*FRUIT:PHYSALIS/CAPE GOOSEBERRIES(FRESH)~V1*9729104*CMA CGM JACQUES JOSEPH*MT*323N****L~SE*19*99999~GE*1*99999~IEA*1*000099999~"#;
    let (rest, obj) = Transmission::<_301>::parse(str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn test_5() {
    let str = r#"ISA*00*          *00*          *ZZ*XXX            *02*XXXX           *230606*1656*U*00401*000999999*0*P*>~GS*RO*XXXXXX*XXXX*20230606*1656*999999*X*004010~ST*301*116513~B1*XXXX*3PHL099999*20230606*U~Y3*3PHL099999*XXXX*20230629****20230627***HH~Y4*3PHL099999**20230626**1*45R1~W09*CZ*-23.3*CE*-23.3*CE****0~N9*SI*276246~N9*BN*3PHL099999~N1*SH*ATEC LOGISTICS*25*100941176~N3*W STATE ROAD 434 2170*STE 400~N4*LONGWOOD*FL*32779*US~R4*R*D*1808*ORLANDO*US***FL~R4*L*UN*USPEF*PORT EVERGLADES*US***FL~DTM*649*20230626~R4*D*K*33779*SAN ANTONIO*CL***VS~R4*E*K*33776*SANTIAGO*CL~EA*GEN~LX*1~L0*1***19504.47*G***1*CNT**K*HH~L5*1*CONSUMER PRODUCTS:NOS(FROZEN)~V1*9786774*POLAR ECUADOR*SG*325S****L~SE*21*116513~GE*1*999999~IEA*1*000999999~"#;
    let (rest, obj) = Transmission::<_301>::parse(str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
}
