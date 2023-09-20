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
            _01: Some("3465322".to_string()),
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
    let obj = serde_x12::to_string(&obj).unwrap();
    println!("{}", obj);
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
    // let str = str.replace("\n", "");
    let obj = parse_301(&str).unwrap();
    println!("{:?}", obj);
    assert!(obj.0.is_empty());
    let obj = obj.1;
    assert_eq!(obj.st._01, "301");
    assert_eq!(obj.st._02, "33233");
    assert_eq!(obj.se._01, "17");
    assert_eq!(obj.se._02, "33233");
}
