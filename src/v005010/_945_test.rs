use crate::util::Parser;
use crate::v005010::*;

const SAMPLE: &str = "ST*945*0001~W06*F*01766655*20131029*1035**9116*4896833*001001~N1*CN*FOOD DISTRIBUTING INC*91*10333648~N3*4820 BRADLEY DR~N4*JEFFERSON*LA*70121-3204~N1*SF*FORT WORTH*9*708066~N1*DE*J.R. Simplot*9*0377912820000~N9*SN*(Seal Number)~N9*ZZ*(Temperature Recording Device Number)~W27*H*TRUK*TRUCKING*CC***(Equipment ID)~LX*1~W12*CC*72*72**CA**007117901645*UK*10071179016458*40550*799*G*L~N9*LI*1000~N9*PC*989JAN281301~N9*LV*00100752782101847618*36~W03*240*15360*LB~SE*17*0001~";

#[test]
fn parse_945() {
    let (rest, obj) = _945::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");

    assert_eq!(obj.st._01, "945");
    assert_eq!(obj.st._02, "0001");
    assert_eq!(obj.w06._01, "F");
    assert_eq!(obj.w06._02, "01766655");
    assert_eq!(obj.w06._03, Some("20131029".to_string()));

    assert_eq!(obj.loop_n1.len(), 3);
    assert_eq!(obj.loop_n1[0].n1._01, "CN");
    assert_eq!(
        obj.loop_n1[0].n1._02,
        Some("FOOD DISTRIBUTING INC".to_string())
    );

    assert_eq!(obj.n9.len(), 2);
    assert_eq!(obj.n9[0]._01, Some("SN".to_string()));
    assert!(obj.w27.is_some());

    assert_eq!(obj.loop_lx.len(), 1);
    assert_eq!(obj.loop_lx[0].lx._01, "1");
    assert_eq!(obj.loop_lx[0].loop_w12[0].w12._01, "CC");
    assert_eq!(obj.loop_lx[0].loop_w12[0].n9.len(), 3);

    assert_eq!(obj.w03.as_ref().unwrap()._01, "240");
    assert_eq!(obj.se._01, "17");
}

#[test]
fn parse_945_minimal() {
    let input = "ST*945*0001~W06*F*12345~LX*1~W12*CC*10*10~SE*4*0001~";
    let (rest, obj) = _945::parse(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "945");
    assert_eq!(obj.w06._01, "F");
    assert_eq!(obj.loop_lx.len(), 1);
    assert_eq!(obj.loop_lx[0].loop_w12[0].w12._01, "CC");
    assert_eq!(obj.se._01, "4");
}

/// Parse -> render -> re-parse must round-trip to an identical structure.
#[test]
fn roundtrip_945() {
    let (_, first) = _945::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _945::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn render_945_minimal() {
    let obj = _945 {
        st: ST {
            _01: "945".to_string(),
            _02: "0001".to_string(),
            ..Default::default()
        },
        w06: W06 {
            _01: "F".to_string(),
            _02: "12345".to_string(),
            ..Default::default()
        },
        loop_lx: vec![_945LoopLX {
            lx: LX {
                _01: "1".to_string(),
            },
            loop_w12: vec![_945LoopW12 {
                w12: W12 {
                    _01: "CC".to_string(),
                    _02: "10".to_string(),
                    _03: Some("10".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        }],
        se: SE {
            _01: "4".to_string(),
            _02: "0001".to_string(),
        },
        ..Default::default()
    };
    let serialized = format!("{obj}");
    let expected = "ST*945*0001~\nW06*F*12345~\nLX*1~\nW12*CC*10*10~\nSE*4*0001~\n";
    assert_eq!(serialized, expected);
}

#[test]
fn full_transmission_945() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *200901*1409*U*00501*043000123*0*P*>~
GS*SW*SENDER*RECEIVER*20200901*1409*000000123*X*005010~
ST*945*000006123~
W06*F*IF102*20190604*BOL12345**PO9865~
N1*ST*CUSTOMER XYZ~
N3*123 CUSTOMER WAY~
N4*ANYTOWN*KS*66216-4563~
N1*SF***CHI~
G62*10*20190602~
NTE*WHI*Extra shrink applied~
W27*M*NPLT*NORTHPOINT TRANSPOR*PP~
W10*1~
LX*10~
W12*CC*3000*3000*0*EA**VN*123456789*LOT12314~
G69*WIDGETS 100 PER BOX~
N9*LV*LPN6569~
W03*3000*15000*LB~
SE*12*000006123~
GE*1*000000123~
IEA*1*043000123~"#;
    let (rest, obj) = Transmission::<_945>::parse(str).unwrap();
    assert!(rest.is_empty());
    let tx = &obj.functional_group[0].segments[0];
    assert_eq!(tx.st._01, "945");
    assert_eq!(tx.w06._02, "IF102");
    assert_eq!(tx.loop_lx[0].loop_w12[0].w12._01, "CC");
}

/// Regression: each LX detail loop is a separate top-level line item.
#[test]
fn parse_945_multi_lx() {
    let input = "ST*945*0001~W06*F*12345~LX*1~W12*CC*10*10~N9*LI*1000~LX*2~W12*CC*20*20~N9*LI*2000~W03*30~SE*9*0001~";
    let (rest, obj) = _945::parse(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.loop_lx.len(), 2);
    assert_eq!(obj.loop_lx[0].lx._01, "1");
    assert_eq!(obj.loop_lx[1].lx._01, "2");
    assert_eq!(obj.loop_lx[0].loop_w12[0].w12._02, "10");
    assert_eq!(obj.loop_lx[1].loop_w12[0].w12._02, "20");
}
