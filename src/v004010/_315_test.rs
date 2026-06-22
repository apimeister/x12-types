use crate::v004010::*;

#[test]
fn parse_315() {
    let x = Transmission {
        isa: ISA {
            _01: "00".to_string(),
            _02: "          ".to_string(),
            _03: "00".to_string(),
            _04: "          ".to_string(),
            _05: "ZZ".to_string(),
            _06: "SOURCE         ".to_string(),
            _07: "ZZ".to_string(),
            _08: "TARGET         ".to_string(),
            _09: "220524".to_string(),
            _10: "1120".to_string(),
            _11: "U".to_string(),
            _12: "00401".to_string(),
            _13: "000000001".to_string(),
            _14: "0".to_string(),
            _15: crate::v004010::segment::i::UsageIndicator::Production,
            _16: ">".to_string(),
            ..Default::default()
        },
        functional_group: vec![FunctionalGroup {
            gs: GS {
                _01: "QO".to_string(),
                _02: "SOURCE".to_string(),
                _03: "TARGET".to_string(),
                _04: "20220524".to_string(),
                _05: "1600".to_string(),
                _06: "1".to_string(),
                _07: "X".to_string(),
                _08: "004010".to_string(),
            },
            segments: vec![_315 {
                st: ST {
                    _01: "315".to_string(),
                    _02: "00001".to_string(),
                },
                b4: B4 {
                    _03: Some("VA".to_string()),
                    _04: Some("20220901".to_string()),
                    _05: Some("0807".to_string()),
                    _07: Some("GMCU".to_string()),
                    _08: Some("609413".to_string()),
                    _09: Some("E".to_string()),
                    _11: Some("LOCKBOURNE".to_string()),
                    _12: Some("CI".to_string()),
                    _13: Some("7".to_string()),
                    ..Default::default()
                },
                n9: vec![
                    N9 {
                        _01: "BM".to_string(),
                        _02: "21001ASK5V9U".to_string(),
                        ..Default::default()
                    },
                    N9 {
                        _01: "BN".to_string(),
                        _02: "1NAN910141".to_string(),
                        ..Default::default()
                    },
                    N9 {
                        _01: "EQ".to_string(),
                        _02: "GMCU6094137".to_string(),
                        ..Default::default()
                    },
                ],
                q2: Some(Q2 {
                    _01: "9330141".to_string(),
                    _09: Some("202N".to_string()),
                    _12: Some("L".to_string()),
                    _13: Some("MARIM".to_string()),
                    ..Default::default()
                }),
                sg: vec![],
                loop_r4: vec![
                    _315LoopR4 {
                        r4: R4 {
                            _01: "L".to_string(),
                            _02: Some("UN".to_string()),
                            _03: Some("USMEM".to_string()),
                            _04: Some("BNSF MEMPHIS RAMP".to_string()),
                            _05: Some("US".to_string()),
                            _08: Some("US".to_string()),
                            ..Default::default()
                        },
                        dtm: vec![],
                    },
                    _315LoopR4 {
                        r4: R4 {
                            _01: "E".to_string(),
                            _02: Some("UN".to_string()),
                            _03: Some("USDAL".to_string()),
                            _04: Some("BNSF ALLIANCE RAMP".to_string()),
                            _05: Some("US".to_string()),
                            _08: Some("US".to_string()),
                            ..Default::default()
                        },
                        dtm: vec![],
                    },
                ],
                v9: None,
                se: SE {
                    _01: "9".to_string(),
                    _02: "00001".to_string(),
                },
            }],
            ge: GE {
                _01: "1".to_string(),
                _02: "1".to_string(),
            },
        }],
        iea: IEA {
            _01: "1".to_string(),
            _02: "000000001".to_string(),
        },
    };
    let str = format!("{x}");
    println!("{str}");
    // let obj: Transmission<_315> = serde_x12::from_str(&new_input).unwrap();
    // println!("{:?}", obj);
    // assert_eq!(x, obj);
}

#[test]
fn test_315() {
    let x = Transmission {
        isa: ISA {
            _01: "00".to_string(),
            _02: "          ".to_string(),
            _03: "00".to_string(),
            _04: "          ".to_string(),
            _05: "ZZ".to_string(),
            _06: "SOURCE         ".to_string(),
            _07: "ZZ".to_string(),
            _08: "TARGET         ".to_string(),
            _09: "220524".to_string(),
            _10: "1120".to_string(),
            _11: "U".to_string(),
            _12: "00401".to_string(),
            _13: "000000001".to_string(),
            _14: "0".to_string(),
            _15: crate::v004010::segment::i::UsageIndicator::Production,
            _16: ">".to_string(),
        },
        functional_group: vec![FunctionalGroup {
            gs: GS {
                _01: "QO".to_string(),
                _02: "SOURCE".to_string(),
                _03: "TARGET".to_string(),
                _04: "20220524".to_string(),
                _05: "1600".to_string(),
                _06: "1".to_string(),
                _07: "X".to_string(),
                _08: "004010".to_string(),
            },
            segments: vec![_315 {
                st: ST {
                    _01: "315".to_string(),
                    _02: "00001".to_string(),
                },
                b4: B4 {
                    _01: None,
                    _02: None,
                    _03: Some("VA".to_string()),
                    _04: Some("20220901".to_string()),
                    _05: Some("0807".to_string()),
                    _06: Some("".to_string()),
                    _07: Some("GMCU".to_string()),
                    _08: Some("609413".to_string()),
                    _09: Some("E".to_string()),
                    _10: Some("".to_string()),
                    _11: Some("LOCKBOURNE".to_string()),
                    _12: Some("CI".to_string()),
                    _13: Some("7".to_string()),
                },
                n9: vec![
                    N9 {
                        _01: "BM".to_string(),
                        _02: "21001ASK5V9U".to_string(),
                        _03: None,
                        _04: None,
                        _05: None,
                        _06: None,
                        _07: None,
                    },
                    N9 {
                        _01: "BN".to_string(),
                        _02: "1NAN910141".to_string(),
                        _03: None,
                        _04: None,
                        _05: None,
                        _06: None,
                        _07: None,
                    },
                    N9 {
                        _01: "EQ".to_string(),
                        _02: "GMCU6094137".to_string(),
                        _03: None,
                        _04: None,
                        _05: None,
                        _06: None,
                        _07: None,
                    },
                ],
                q2: Some(Q2 {
                    _01: "9330141".to_string(),
                    _02: None,
                    _03: None,
                    _04: None,
                    _05: None,
                    _06: None,
                    _07: None,
                    _08: None,
                    _09: Some("202N".to_string()),
                    _10: None,
                    _11: None,
                    _12: Some("L".to_string()),
                    _13: Some("MARIM".to_string()),
                    _14: None,
                    _15: None,
                    _16: None,
                }),
                sg: vec![],
                loop_r4: vec![
                    _315LoopR4 {
                        r4: R4 {
                            _01: "L".to_string(),
                            _02: Some("UN".to_string()),
                            _03: Some("USMEM".to_string()),
                            _04: Some("BNSF MEMPHIS RAMP".to_string()),
                            _05: Some("US".to_string()),
                            _06: Some("".to_string()),
                            _07: Some("".to_string()),
                            _08: Some("US".to_string()),
                        },
                        dtm: vec![],
                    },
                    _315LoopR4 {
                        r4: R4 {
                            _01: "E".to_string(),
                            _02: Some("UN".to_string()),
                            _03: Some("USDAL".to_string()),
                            _04: Some("BNSF ALLIANCE RAMP".to_string()),
                            _05: Some("US".to_string()),
                            _06: Some("".to_string()),
                            _07: Some("".to_string()),
                            _08: Some("US".to_string()),
                        },
                        dtm: vec![],
                    },
                ],
                v9: None,
                se: SE {
                    _01: "9".to_string(),
                    _02: "00001".to_string(),
                },
            }],
            ge: GE {
                _01: "1".to_string(),
                _02: "1".to_string(),
            },
        }],
        iea: IEA {
            _01: "1".to_string(),
            _02: "000000001".to_string(),
        },
    };
    let serialized = format!("{x}");
    let original = r#"ISA*00*          *00*          *ZZ*SOURCE         *ZZ*TARGET         *220524*1120*U*00401*000000001*0*P*>~
GS*QO*SOURCE*TARGET*20220524*1600*1*X*004010~
ST*315*00001~
B4***VA*20220901*0807**GMCU*609413*E**LOCKBOURNE*CI*7~
N9*BM*21001ASK5V9U~
N9*BN*1NAN910141~
N9*EQ*GMCU6094137~
Q2*9330141********202N***L*MARIM~
R4*L*UN*USMEM*BNSF MEMPHIS RAMP*US***US~
R4*E*UN*USDAL*BNSF ALLIANCE RAMP*US***US~
SE*9*00001~
GE*1*1~
IEA*1*000000001~
"#;
    assert_eq!(serialized, original);
}

#[test]
fn test_315_defaults() {
    let x = Transmission {
        isa: ISA {
            _01: "00".to_string(),
            _02: "          ".to_string(),
            _03: "00".to_string(),
            _04: "          ".to_string(),
            _05: "ZZ".to_string(),
            _06: "SOURCE         ".to_string(),
            _07: "ZZ".to_string(),
            _08: "TARGET         ".to_string(),
            _09: "220524".to_string(),
            _10: "1120".to_string(),
            _11: "U".to_string(),
            _12: "00401".to_string(),
            _13: "000000001".to_string(),
            _14: "0".to_string(),
            _15: crate::v004010::segment::i::UsageIndicator::Production,
            _16: ">".to_string(),
            ..Default::default()
        },
        functional_group: vec![FunctionalGroup {
            gs: GS {
                _01: "QO".to_string(),
                _02: "SOURCE".to_string(),
                _03: "TARGET".to_string(),
                _04: "20220524".to_string(),
                _05: "1600".to_string(),
                _06: "1".to_string(),
                _07: "X".to_string(),
                _08: "004010".to_string(),
            },
            segments: vec![_315 {
                st: ST {
                    _01: "315".to_string(),
                    _02: "00001".to_string(),
                },
                b4: B4 {
                    _03: Some("VA".to_string()),
                    _04: Some("20220901".to_string()),
                    _05: Some("0807".to_string()),
                    _06: Some("".to_string()),
                    _07: Some("GMCU".to_string()),
                    _08: Some("609413".to_string()),
                    _09: Some("E".to_string()),
                    _11: Some("LOCKBOURNE".to_string()),
                    _12: Some("CI".to_string()),
                    _13: Some("7".to_string()),
                    ..Default::default()
                },
                n9: vec![
                    N9 {
                        _01: "BM".to_string(),
                        _02: "21001ASK5V9U".to_string(),
                        ..Default::default()
                    },
                    N9 {
                        _01: "BN".to_string(),
                        _02: "1NAN910141".to_string(),
                        ..Default::default()
                    },
                    N9 {
                        _01: "EQ".to_string(),
                        _02: "GMCU6094137".to_string(),
                        ..Default::default()
                    },
                ],
                q2: Some(Q2 {
                    _01: "9330141".to_string(),
                    _09: Some("202N".to_string()),
                    _12: Some("L".to_string()),
                    _13: Some("MARIM".to_string()),
                    ..Default::default()
                }),
                sg: vec![],
                loop_r4: vec![
                    _315LoopR4 {
                        r4: R4 {
                            _01: "L".to_string(),
                            _02: Some("UN".to_string()),
                            _03: Some("USMEM".to_string()),
                            _04: Some("BNSF MEMPHIS RAMP".to_string()),
                            _05: Some("US".to_string()),
                            _08: Some("US".to_string()),
                            ..Default::default()
                        },
                        dtm: vec![],
                    },
                    _315LoopR4 {
                        r4: R4 {
                            _01: "E".to_string(),
                            _02: Some("UN".to_string()),
                            _03: Some("USDAL".to_string()),
                            _04: Some("BNSF ALLIANCE RAMP".to_string()),
                            _05: Some("US".to_string()),
                            _08: Some("US".to_string()),
                            ..Default::default()
                        },
                        dtm: vec![],
                    },
                ],
                v9: None,
                se: SE {
                    _01: "9".to_string(),
                    _02: "00001".to_string(),
                },
            }],
            ge: GE {
                _01: "1".to_string(),
                _02: "1".to_string(),
            },
        }],
        iea: IEA {
            _01: "1".to_string(),
            _02: "000000001".to_string(),
        },
    };
    let serialized = format!("{x}");
    let original = r#"ISA*00*          *00*          *ZZ*SOURCE         *ZZ*TARGET         *220524*1120*U*00401*000000001*0*P*>~
GS*QO*SOURCE*TARGET*20220524*1600*1*X*004010~
ST*315*00001~
B4***VA*20220901*0807**GMCU*609413*E**LOCKBOURNE*CI*7~
N9*BM*21001ASK5V9U~
N9*BN*1NAN910141~
N9*EQ*GMCU6094137~
Q2*9330141********202N***L*MARIM~
R4*L*UN*USMEM*BNSF MEMPHIS RAMP*US***US~
R4*E*UN*USDAL*BNSF ALLIANCE RAMP*US***US~
SE*9*00001~
GE*1*1~
IEA*1*000000001~
"#;
    assert_eq!(serialized, original);
}

#[test]
fn test_2() {
    let str = r#"ISA*00*          *00*          *ZZ*XXXX           *ZZ*XXXX           *221121*1110*U*00401*012359227*0*P*>~GS*QO*XXXX*XXXX*20221121*1110*12359218*X*004010~ST*315*33366799~B4***VA*20221114*1450**TCKU*724880*L**MAPTM*UN*7~N9*BM*52BUE013415X~N9*BN*2BUE013415~N9*EQ*TCKU7248999~Q2*9633999********242N***L*CAP SAN MALEAS~R4*L*UN*ARBUE*TERMINAL 4*AR~DTM*140*20221025*0255*LT~R4*T*UN*MAPTM*APM TERMINALS TANGER MED*MA~DTM*140*20221114*1450*LT~R4*D*UN*DEHAM*EUROGATE CONTAINER TERMI*DE~DTM*139*20221130*2300*LT~SE*13*33366742~GE*1*12359218~IEA*1*012359227~"#;
    let (rest, obj) = Transmission::<_315>::parse(str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
}

#[test]
fn test_3() {
    let str = r#"ISA*00*          *00*          *ZZ*HAMSUD         *ZZ*GSCL           *221121*0751*U*00401*000720857*0*P*>~GS*QO*HAMSUD*GSCL*20221121*0751*720857*X*004010~ST*315*1883403~B4***UV*20221121*0124**MRSU*437163*L*45G1*MXZLO*UN*9~N9*4F*800992458*SAMSUNG SDS~N9*BM*SUDUN2SL008733AF~N9*BN*2SOL008733~N9*EQ*MRSU4371639~N9*SCA*SUDU~N9*SN*5512862~N9*WU*MAERSK STEPNICA~Q2*9352004********242S***L*MAERSK STEPNICA~R4*L*UN*KRPUS*PUSAN NEWPORT CONTAINER *KR~DTM*140*20221103*125500*LT~R4*D*UN*MXZLO*SSA MEXICO HOLDINGS*MX~DTM*140*20221120*230000*LT~R4*E*UN*MXQRO*QUERETARO*MX***QU~DTM*139*20221125*230000*LT~R4*5*UN*MXZLO*MANZANILLO~DTM*140*20221121*012400*LT~SE*19*1883403~GE*1*720857~IEA*1*000720857~"#;
    let (rest, obj) = Transmission::<_315>::parse(str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
}
