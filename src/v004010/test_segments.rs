use crate::v004010::*;

#[test]
fn test_st() {
    let obj = ST {
        _01: "301".to_string(),
        _02: "33233".to_string(),
    };
    let str = format!("{obj}");
    println!("{str}");
    println!("{obj}");
    let result = ST::parse(&str).unwrap();
    // not data left to process
    assert!(result.0.is_empty());
    // check actual object
    let obj = result.1;
    assert_eq!(obj._01, "301");
    assert_eq!(obj._02, "33233");
}

#[test]
fn st_display() {
    let obj = ST {
        _01: "301".to_string(),
        _02: "33233".to_string(),
    };
    let str = format!("{obj}");
    assert_eq!(str, "ST*301*33233~\n");
}

#[test]
fn t1_display() {
    let obj = T1 {
        _01: "301".to_string(),
        _02: Some("33233".to_string()),
        ..Default::default()
    };
    let str = format!("{obj}");
    assert_eq!(str, "T1*301*33233~\n");
}

#[test]
fn test_b1() {
    let obj = B1 {
        _01: Some("SNDR".to_string()),
        _02: "ERXX412223".to_string(),
        _03: Some("20221121".to_string()),
        _04: "A".to_string(),
    };
    let str = format!("{obj}");
    let result = B1::parse(&str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._04, "A");
}

#[test]
fn test_y3_parse_line() {
    let str = "Y3*ERXX412223*SNDR*20230104****20221229***HP~";
    let result = Y3::parse(str).unwrap();
    // not data left to process
    assert!(result.0.is_empty());
    // check actual object
    let obj = result.1;
    assert_eq!(obj._01, "ERXX412223");
    println!("{obj:?}");
    assert_eq!(obj._10, Some("HP".to_string()));
}

#[test]
fn test_y3_parse_newline() {
    let str = "Y3*ERXX412223*SNDR*20230104****20221229***HP~\n";
    let result = Y3::parse(str).unwrap();
    // not data left to process
    assert!(result.0.is_empty());
    // check actual object
    let obj = result.1;
    assert_eq!(obj._01, "ERXX412223");
    println!("{obj:?}");
    assert_eq!(obj._10, Some("HP".to_string()));
}

#[test]
fn test_y3() {
    let obj = Y3 {
        _01: "ERXX412223".to_string(),
        _02: Some("SNDR".to_string()),
        _03: Some("20230104".to_string()),
        _07: Some("20221229".to_string()),
        _10: Some("HP".to_string()),
        ..Default::default()
    };
    let str = format!("{obj}");
    let result = Y3::parse(&str).unwrap();
    let obj2 = result.1;
    assert!(result.0.is_empty());
    assert_eq!(obj2._10, Some("HP".to_string()));
    println!("obj before: {obj:?}");
    println!("obj str: {str}");
    println!("obj  after: {obj2:?}");
}

#[test]
fn test_y4() {
    let obj = Y4 {
        _01: Some("ERXX412223".to_string()),
        _03: Some("20221216".to_string()),
        _05: Some("1".to_string()),
        _06: Some("45G1".to_string()),
        ..Default::default()
    };
    let y4_str = format!("{obj}");
    let result = Y4::parse(&y4_str).unwrap();
    let obj2 = result.1;
    assert!(result.0.is_empty());
    assert_eq!(obj2._06, Some("45G1".to_string()));
}

#[test]
fn test_n9() {
    let obj = N9 {
        _01: "BN".to_string(),
        _02: "ERXX412223".to_string(),
        ..Default::default()
    };
    let n9_str = format!("{obj}");
    let result = N9::parse(&n9_str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, "BN");
    assert_eq!(result.1._02, "ERXX412223");
}

#[test]
fn test_n1() {
    let obj = N1 {
        _01: "SH".to_string(),
        _02: Some("ABC GMBH".to_string()),
        _03: Some("25".to_string()),
        _04: Some("312343123".to_string()),
        ..Default::default()
    };
    let n1_str = format!("{obj}");
    let result = N1::parse(&n1_str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, "SH");
    assert_eq!(result.1._04, Some("312343123".to_string()));
}

#[test]
fn test_n1_2() {
    let str = "N1*SF*VANWARD INTL ( HONG KONG) LTD~\n";
    let (str, obj) = N1::parse(str).unwrap();
    assert!(str.is_empty());
    assert_eq!(obj._01, "SF");
    assert_eq!(obj._02, Some("VANWARD INTL ( HONG KONG) LTD".to_string()));
    assert_eq!(obj._03, None);
}

#[test]
fn test_n3() {
    let obj = N3 {
        _01: "TEST STR. 56".to_string(),
        _02: Some("XA 124324 4".to_string()),
    };
    let n3_str = format!("{obj}");
    let result = N3::parse(&n3_str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, "TEST STR. 56");
    assert_eq!(result.1._02, Some("XA 124324 4".to_string()));
}

#[test]
fn test_n4() {
    let obj = N4 {
        _01: Some("MUNICH".to_string()),
        _02: Some("BY".to_string()),
        _03: Some("80348".to_string()),
        _04: Some("DE".to_string()),
        ..Default::default()
    };
    let n4_str = format!("{obj}");
    let result = N4::parse(&n4_str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, Some("MUNICH".to_string()));
    assert_eq!(result.1._02, Some("BY".to_string()));
}

#[test]
fn test_r4() {
    let obj = R4 {
        _01: "R".to_string(),
        _02: Some("UN".to_string()),
        _03: Some("DEWKD".to_string()),
        _04: Some("WACKERSDORF".to_string()),
        _05: Some("DE".to_string()),
        _08: Some("BY".to_string()),
        ..Default::default()
    };
    let str = format!("{obj}");
    let result = R4::parse(&str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, "R");
    assert_eq!(result.1._02, Some("UN".to_string()));
}

#[test]
fn test_dtm() {
    let obj = DTM {
        _01: "649".to_string(),
        _02: Some("20230102".to_string()),
        ..Default::default()
    };
    let str = format!("{obj}");
    let result = DTM::parse(&str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, "649");
    assert_eq!(result.1._02, Some("20230102".to_string()));
}

#[test]
fn test_lx() {
    let obj = LX {
        _01: "2".to_string(),
    };
    let str = format!("{obj}");
    let result = LX::parse(&str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, "2");
}

#[test]
fn test_l0() {
    let obj = L0 {
        _01: Some("1".to_string()),
        _04: Some("14000".to_string()),
        _05: Some("G".to_string()),
        _08: Some("1".to_string()),
        _09: Some("CNT".to_string()),
        _11: Some("K".to_string()),
        _12: Some("HP".to_string()),
        ..Default::default()
    };
    let str = format!("{obj}");
    let result = L0::parse(&str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, Some("1".to_string()));
    assert_eq!(result.1._04, Some("14000".to_string()));
}

#[test]
fn test_l5() {
    let obj = L5 {
        _01: Some("1".to_string()),
        _02: Some("VEHICLES:PARTS".to_string()),
        ..Default::default()
    };
    let str = format!("{obj}");
    let result = L5::parse(&str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, Some("1".to_string()));
    assert_eq!(result.1._02, Some("VEHICLES:PARTS".to_string()));
}

#[test]
fn test_v1() {
    let obj = V1 {
        _01: "3465322".to_string(),
        _02: Some("CAP SAN ANTONIO".to_string()),
        _03: Some("DK".to_string()),
        _04: Some("456S".to_string()),
        _08: Some("L".to_string()),
        ..Default::default()
    };
    let str = format!("{obj}");
    let result = V1::parse(&str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, "3465322".to_string());
    assert_eq!(result.1._02, Some("CAP SAN ANTONIO".to_string()));
}

#[test]
fn test_se() {
    let obj = SE {
        _01: "17".to_string(),
        _02: "33233".to_string(),
    };
    let str = format!("{obj}");
    let result = SE::parse(&str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, "17");
    assert_eq!(result.1._02, "33233");
}

#[test]
fn test_m12() {
    let obj = M12 {
        _01: "01".to_string(),
        _02: Some("123456789".to_string()),
        _03: Some("LOC1".to_string()),
        _04: Some("LOC2".to_string()),
        _05: Some("1000".to_string()),
        _06: Some("INBOND123".to_string()),
        _07: Some("CARRIER".to_string()),
        _08: Some("REF".to_string()),
        _09: Some("REF123".to_string()),
        _10: Some("T".to_string()),
        _11: Some("VESSEL NAME".to_string()),
    };
    let str = format!("{obj}");
    let result = M12::parse(&str).unwrap();
    assert!(result.0.is_empty());
    assert_eq!(result.1._01, "01".to_string());
    assert_eq!(result.1._02, Some("123456789".to_string()));
    assert_eq!(result.1._03, Some("LOC1".to_string()));
}

// #[test]
// fn test_parse_301_nom() {
//     let str = r#"ST*301*33233~
// B1*SNDR*ERXX412223*20221121*A~
// Y3*ERXX412223*SNDR*20230104*20230105***20221229***HP~
// Y4*ERXX412223**20221216**1*45G1~
// N9*BN*ERXX412223~
// N1*SH*ABC GMBH*25*312343123~
// N3*TEST STR. 56*XA 124324 4~
// N4*MUNICH*BY*80348*DE~
// R4*R*UN*DEWKD*WACKERSDORF*DE***BY~
// R4*L*UN*DEHAM*HAMBURG*DE***HH~
// DTM*649*20230102~
// R4*D*K*35180*ITAPOA*BR***SC~
// LX*2~
// L0*1***14000*G***1*CNT**K*HP~
// L5*1*VEHICLES:PARTS~
// V1*3465322*CAP SAN ANTONIO*DK*456S****L~
// SE*17*33233~"#;
//     // let str = str.replace("\n", "");
//     let obj = parse_301(&str);
//     println!("{:?}", obj);
// }
