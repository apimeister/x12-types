use serde_x12::to_string;
use validator::Validate;
use crate::v004010::*;

#[test]
fn valid_isa() {
    let isa = ISA {
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
        _13: "000002176".to_string(),
        _14: "0".to_string(),
        _15: "P".to_string(),
        _16: ">".to_string(),
    };
    isa.validate().unwrap();
}
#[test]
#[should_panic]
fn invalid_isa() {
    let isa = ISA {
        _01: "00".to_string(),
        _02: "         ".to_string(),
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
        _13: "000002176".to_string(),
        _14: "0".to_string(),
        _15: "P".to_string(),
        _16: ">".to_string(),
    };
    isa.validate().unwrap();
}
#[test]
fn test_isa() {
    let s = "ISA*00*          *00*          *ZZ*SOURCE         *ZZ*TARGET         *220524*1120*U*00401*000002176*0*P*>~";
    let isa = ISA {
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
        _13: "000002176".to_string(),
        _14: "0".to_string(),
        _15: "P".to_string(),
        _16: ">".to_string(),
    };
    let str = to_string(&isa).unwrap();
    assert_eq!(s, str.trim());
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
            _15: "P".to_string(),
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
            segments: vec![Segments::_315(_315 {
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
                loop_r4: vec![_315LoopR4{
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
                    dtm: None,
                },_315LoopR4{
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
                    dtm: None,
                }],
                v9: None,
                se: SE {
                    _01: "9".to_string(),
                    _02: "00001".to_string(),
                },
            })],
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
    let serialized = serde_x12::to_string(&x).unwrap();
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
            _15: "P".to_string(),
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
            segments: vec![Segments::_315(_315 {
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
                loop_r4: vec![_315LoopR4{
                    r4: R4 {
                        _01: "L".to_string(),
                        _02: Some("UN".to_string()),
                        _03: Some("USMEM".to_string()),
                        _04: Some("BNSF MEMPHIS RAMP".to_string()),
                        _05: Some("US".to_string()),
                        _08: Some("US".to_string()),
                        ..Default::default()
                    },
                    dtm: None,
                },_315LoopR4{
                    r4: R4 {
                        _01: "E".to_string(),
                        _02: Some("UN".to_string()),
                        _03: Some("USDAL".to_string()),
                        _04: Some("BNSF ALLIANCE RAMP".to_string()),
                        _05: Some("US".to_string()),
                        _08: Some("US".to_string()),
                        ..Default::default()
                    },
                    dtm: None,
                }],
                v9: None,
                se: SE {
                    _01: "9".to_string(),
                    _02: "00001".to_string(),
                },
            })],
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
    let serialized = serde_x12::to_string(&x).unwrap();
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