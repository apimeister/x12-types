use crate::v005010::*;

#[test]
fn test_st() {
    let str = "ST*834*193230001*005010X220A1~";
    let (rest, obj) = ST::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._03, Some("005010X220A1".to_string()));
}

#[test]
fn test_st_newline() {
    let str = "ST*834*193230001*005010X220A1~\n";
    let (rest, obj) = ST::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._03, Some("005010X220A1".to_string()));
}

#[test]
fn test_n1() {
    let s = "N1*P5*MEDICAID*FI*141797357~";
    let (rest, obj) = N1::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._02, Some("MEDICAID".to_string()));
}

#[test]
fn test_qty() {
    let s = "QTY*TO*1~";
    let (rest, obj) = QTY::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._02, Some("1".to_string()));
}

#[test]
fn test_isa() {
    let s = "ISA*00*          *00*          *ZZ*EMEDNYMCR      *ZZ*8-DIGIT PLAN ID*191125*1409*^*00501*193290002*0*T*:~";
    let (rest, obj) = ISA::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "00");
}

#[test]
fn test_ref() {
    let s = "REF*0F*XX99991X~";
    let (rest, obj) = REF::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "0F");
    assert_eq!(obj._02, Some("XX99991X".to_string()));
}

#[test]
fn test_dtp() {
    let s = "DTP*356*D8*20190101~";
    let (rest, obj) = DTP::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "356");
    assert_eq!(obj._02, "D8");
}

#[test]
fn test_dmg() {
    let s = "DMG*D8*20010101*M**C~";
    let (rest, obj) = DMG::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, Some("D8".to_string()));
    assert_eq!(obj._02, Some("20010101".to_string()));
}

#[test]
fn test_lx() {
    let s = "LX*1~";
    let (rest, obj) = LX::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "1");
}

#[test]
fn test_bgn() {
    let s = "BGN*00*1932900000000002XF1932900000000002*20191125*140915****2~";
    let (rest, obj) = BGN::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "00");
}

#[test]
fn test_ins() {
    let s = "INS*Y*18*024**A***TE~";
    let (rest, obj) = INS::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "Y");
}

#[test]
fn test_nm1() {
    let s = "NM1*IL*1*SUBSCRIBER 3 LAST NAME*SUBSCRIBER 3 FIRST NAME*MI***34*999999993~";
    let (rest, obj) = NM1::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "IL");
}

#[test]
fn test_le() {
    let s = "LE*2700~";
    let (rest, obj) = LE::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "2700");
}

#[test]
fn test_se() {
    let s = "SE*68*193230003~";
    let (rest, obj) = SE::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "68");
}

#[test]
fn test_act() {
    let obj1 = ACT {
        _01: "".to_string(),
        _02: Some("".to_string()),
        _03: Some("".to_string()),
        _04: Some("".to_string()),
        _05: Some("".to_string()),
        _06: Some("".to_string()),
        _07: Some("".to_string()),
        _08: Some("".to_string()),
        _09: Some("X".to_string()),
    };
    let s = format!("{obj1}");
    let (rest, obj) = ACT::parse(&s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "");
    assert_eq!(obj._09, Some("X".to_string()));
}

#[test]
fn test_ge() {
    let s = "GE*3*193230001~";
    let (rest, obj) = GE::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "3");
}

#[test]
fn test_iea() {
    let s = "IEA*1*193230001~";
    let (rest, obj) = IEA::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "1");
}

#[test]
fn test_spi() {
    let s = "SPI*01*AB*12345*Entity Title*Entity Purpose~";
    let (rest, obj) = SPI::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._01, "01");
    assert_eq!(obj._02, Some("AB".to_string()));
    assert_eq!(obj._03, Some("12345".to_string()));
    assert_eq!(obj._04, Some("Entity Title".to_string()));
    assert_eq!(obj._05, Some("Entity Purpose".to_string()));
}
