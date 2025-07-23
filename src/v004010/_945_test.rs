use crate::v004010::*;

#[test]
fn render_945() {
    let obj = _945 {
        st: ST {
            _01: "945".to_string(),
            _02: "0001".to_string(),
        },
        w06: W06 {
            _01: "F".to_string(),
            _02: "01766655".to_string(),
            _03: Some("20131029".to_string()),
            _04: Some("1035".to_string()),
            _05: None,
            _06: Some("9116".to_string()),
            _07: Some("4896833".to_string()),
            _08: Some("001001".to_string()),
            _09: None,
        },
        loop_n1: vec![
            _945LoopN1 {
                n1: N1 {
                    _01: "CN".to_string(),
                    _02: Some("FOOD DISTRIBUTING INC".to_string()),
                    _03: Some("91".to_string()),
                    _04: Some("10333648".to_string()),
                    ..Default::default()
                },
                n2: vec![],
                n3: vec![N3 {
                    _01: "4820 BRADLEY DR".to_string(),
                    ..Default::default()
                }],
                n4: Some(N4 {
                    _01: Some("JEFFERSON".to_string()),
                    _02: Some("LA".to_string()),
                    _03: Some("70121-3204".to_string()),
                    ..Default::default()
                }),
                per: vec![],
            },
            _945LoopN1 {
                n1: N1 {
                    _01: "SF".to_string(),
                    _02: Some("FORT WORTH".to_string()),
                    _03: Some("9".to_string()),
                    _04: Some("708066".to_string()),
                    ..Default::default()
                },
                n2: vec![],
                n3: vec![],
                n4: None,
                per: vec![],
            },
            _945LoopN1 {
                n1: N1 {
                    _01: "DE".to_string(),
                    _02: Some("J.R. Simplot".to_string()),
                    _03: Some("9".to_string()),
                    _04: Some("0377912820000".to_string()),
                    ..Default::default()
                },
                n2: vec![],
                n3: vec![],
                n4: None,
                per: vec![],
            },
        ],
        n9: vec![
            N9 {
                _01: "SN".to_string(),
                _02: "(Seal Number)".to_string(),
                ..Default::default()
            },
            N9 {
                _01: "ZZ".to_string(),
                _02: "(Temperature Recording Device Number)".to_string(),
                ..Default::default()
            },
        ],
        g62: vec![],
        w27: Some(W27 {
            _01: "H".to_string(),
            _02: "TRUK".to_string(),
            _03: Some("TRUCKING".to_string()),
            _04: Some("CC".to_string()),
            _05: None,
            _06: None,
            _07: Some("(Equipment ID)".to_string()),
            ..Default::default()
        }),
        w10: vec![],
        loop_lx: vec![
            _945LoopLX {
                lx: LX {
                    _01: "1".to_string(),
                },
                loop_w12: vec![_945LoopW12 {
                    w12: W12 {
                        _01: "CC".to_string(),
                        _02: "72".to_string(),
                        _03: Some("72".to_string()),
                        _04: None,
                        _05: Some("CA".to_string()),
                        _06: None,
                        _07: Some("007117901645".to_string()),
                        _08: Some("UK".to_string()),
                        _09: Some("10071179016458".to_string()),
                        _10: Some("40550".to_string()),
                        _11: Some("799".to_string()),
                        _12: Some("G".to_string()),
                        _13: Some("L".to_string()),
                        ..Default::default()
                    },
                    n9: vec![
                        N9 {
                            _01: "LI".to_string(),
                            _02: "1000".to_string(),
                            ..Default::default()
                        },
                        N9 {
                            _01: "PC".to_string(),
                            _02: "989JAN281301".to_string(),
                            ..Default::default()
                        },
                        N9 {
                            _01: "LV".to_string(),
                            _02: "00100752782101847618".to_string(),
                            _03: Some("36".to_string()),
                            ..Default::default()
                        },
                        N9 {
                            _01: "LV".to_string(),
                            _02: "00100752782101847619".to_string(),
                            _03: Some("36".to_string()),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],

                ..Default::default()
            },
            _945LoopLX {
                lx: LX {
                    _01: "2".to_string(),
                },
                loop_w12: vec![_945LoopW12 {
                    w12: W12 {
                        _01: "CC".to_string(),
                        _02: "72".to_string(),
                        _03: Some("72".to_string()),
                        _04: None,
                        _05: Some("CA".to_string()),
                        _06: None,
                        _07: Some("007117901645".to_string()),
                        _08: Some("UK".to_string()),
                        _09: Some("10071179016458".to_string()),
                        _10: Some("40551".to_string()),
                        _11: Some("884".to_string()),
                        _12: Some("G".to_string()),
                        _13: Some("L".to_string()),
                        ..Default::default()
                    },
                    n9: vec![
                        N9 {
                            _01: "LI".to_string(),
                            _02: "1000".to_string(),
                            ..Default::default()
                        },
                        N9 {
                            _01: "PC".to_string(),
                            _02: "989JAN291301".to_string(),
                            ..Default::default()
                        },
                        N9 {
                            _01: "LV".to_string(),
                            _02: "00100752782101847620".to_string(),
                            _03: Some("72".to_string()),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],

                ..Default::default()
            },
            _945LoopLX {
                lx: LX {
                    _01: "3".to_string(),
                },
                loop_w12: vec![_945LoopW12 {
                    w12: W12 {
                        _01: "CC".to_string(),
                        _02: "96".to_string(),
                        _03: Some("96".to_string()),
                        _04: None,
                        _05: Some("CA".to_string()),
                        _06: None,
                        _07: Some("007117900070".to_string()),
                        _08: Some("UK".to_string()),
                        _09: Some("10071179000709".to_string()),
                        _10: Some("93724".to_string()),
                        _11: Some("1404".to_string()),
                        _12: Some("G".to_string()),
                        _13: Some("L".to_string()),
                        ..Default::default()
                    },
                    n9: vec![
                        N9 {
                            _01: "LI".to_string(),
                            _02: "2000".to_string(),
                            ..Default::default()
                        },
                        N9 {
                            _01: "PC".to_string(),
                            _02: "006OCT061301".to_string(),
                            ..Default::default()
                        },
                        N9 {
                            _01: "LV".to_string(),
                            _02: "00100752782101847621".to_string(),
                            _03: Some("72".to_string()),
                            ..Default::default()
                        },
                        N9 {
                            _01: "LV".to_string(),
                            _02: "00100752782101847622".to_string(),
                            _03: Some("24".to_string()),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                ..Default::default()
            },
        ],
        w03: Some(W03 {
            _01: "240".to_string(),
            _02: Some("15360".to_string()),
            _03: Some("LB".to_string()),
            ..Default::default()
        }),
        se: SE {
            _01: "32".to_string(),
            _02: "0001".to_string(),
        },
        ..Default::default()
    };

    let serialized = format!("{obj}");
    let expected = "ST*945*0001~\nW06*F*01766655*20131029*1035**9116*4896833*001001~\nN1*CN*FOOD DISTRIBUTING INC*91*10333648~\nN3*4820 BRADLEY DR~\nN4*JEFFERSON*LA*70121-3204~\nN1*SF*FORT WORTH*9*708066~\nN1*DE*J.R. Simplot*9*0377912820000~\nN9*SN*(Seal Number)~\nN9*ZZ*(Temperature Recording Device Number)~\nW27*H*TRUK*TRUCKING*CC***(Equipment ID)~\nLX*1~\nW12*CC*72*72**CA**007117901645*UK*10071179016458*40550*799*G*L~\nN9*LI*1000~\nN9*PC*989JAN281301~\nN9*LV*00100752782101847618*36~\nN9*LV*00100752782101847619*36~\nLX*2~\nW12*CC*72*72**CA**007117901645*UK*10071179016458*40551*884*G*L~\nN9*LI*1000~\nN9*PC*989JAN291301~\nN9*LV*00100752782101847620*72~\nLX*3~\nW12*CC*96*96**CA**007117900070*UK*10071179000709*93724*1404*G*L~\nN9*LI*2000~\nN9*PC*006OCT061301~\nN9*LV*00100752782101847621*72~\nN9*LV*00100752782101847622*24~\nW03*240*15360*LB~\nSE*32*0001~\n";
    assert_eq!(serialized, expected);
}

#[test]
fn parse_945() {
    let input = "ST*945*0001~W06*F*01766655*20131029*1035**9116*4896833*001001~N1*CN*FOOD DISTRIBUTING INC*91*10333648~N3*4820 BRADLEY DR~N4*JEFFERSON*LA*70121-3204~N1*SF*FORT WORTH*9*708066~N1*DE*J.R. Simplot*9*0377912820000~N9*SN*(Seal Number)~N9*ZZ*(Temperature Recording Device Number)~W27*H*TRUK*TRUCKING*CC***(Equipment ID)~LX*1~W12*CC*72*72**CA**007117901645*UK*10071179016458*40550*799*G*L~N9*LI*1000~N9*PC*989JAN281301~N9*LV*00100752782101847618*36~N9*LV*00100752782101847619*36~LX*2~W12*CC*72*72**CA**007117901645*UK*10071179016458*40551*884*G*L~N9*LI*1000~N9*PC*989JAN291301~N9*LV*00100752782101847620*72~LX*3~W12*CC*96*96**CA**007117900070*UK*10071179000709*93724*1404*G*L~N9*LI*2000~N9*PC*006OCT061301~N9*LV*00100752782101847621*72~N9*LV*00100752782101847622*24~W03*240*15360*LB~SE*32*0001~";

    let (rest, obj) = _945::parse(input).unwrap();
    assert_eq!(rest, "");

    // Verify the parsed structure
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
    assert_eq!(obj.n9[0]._01, "SN");

    assert!(obj.w27.is_some());
    assert_eq!(obj.w27.as_ref().unwrap()._01, "H");
}

#[test]
fn parse_945_minimal() {
    // Test with minimal required segments
    let input = "ST*945*0001~W06*F*12345~LX*1~W12*CC*10*10~SE*4*0001~";

    let (rest, obj) = _945::parse(input).unwrap();
    assert_eq!(rest, "");

    assert_eq!(obj.st._01, "945");
    assert_eq!(obj.w06._01, "F");
    assert_eq!(obj.loop_lx.len(), 1);
    assert_eq!(obj.loop_lx[0].loop_w12[0].w12._01, "CC");
    assert_eq!(obj.se._01, "4");
}

#[test]
fn full_transmission_945() {
    use crate::v004010::*;

    let obj = Transmission {
        isa: ISA {
            _01: "00".to_string(),
            _02: "          ".to_string(),
            _03: "00".to_string(),
            _04: "          ".to_string(),
            _05: "ZZ".to_string(),
            _06: "SENDER         ".to_string(),
            _07: "ZZ".to_string(),
            _08: "RECEIVER       ".to_string(),
            _09: "220524".to_string(),
            _10: "1120".to_string(),
            _11: "U".to_string(),
            _12: "00401".to_string(),
            _13: "000000001".to_string(),
            _14: "0".to_string(),
            _15: UsageIndicator::Production,
            _16: ">".to_string(),
        },
        functional_group: vec![FunctionalGroup {
            gs: GS {
                _01: "SW".to_string(),
                _02: "SENDER".to_string(),
                _03: "RECEIVER".to_string(),
                _04: "20220524".to_string(),
                _05: "1600".to_string(),
                _06: "1".to_string(),
                _07: "X".to_string(),
                _08: "004010".to_string(),
            },
            segments: vec![_945 {
                st: ST {
                    _01: "945".to_string(),
                    _02: "0001".to_string(),
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

    let serialized = format!("{obj}");
    let expected = "ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *220524*1120*U*00401*000000001*0*P*>~\nGS*SW*SENDER*RECEIVER*20220524*1600*1*X*004010~\nST*945*0001~\nW06*F*12345~\nLX*1~\nW12*CC*10*10~\nSE*4*0001~\nGE*1*1~\nIEA*1*000000001~\n";
    assert_eq!(serialized, expected);
}

#[test]
fn test_945_parse_1() {
    let str = r#"ISA*00* *00* *ZZ*YOUR-ID-HERE *ZZ*WAQE *200901*1409*U*00401*043000123*0*P*>~
GS*SW*YOUR-ID-HERE*WAQE*20200901*1409*000000123*X*004010~
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
    let (rest, _obj) = Transmission::<_945>::parse(str).unwrap();
    assert!(rest.is_empty());
}

#[test]
fn test_945_parse_2() {
    let str = r#"ISA*00* *00* *ZZ*WAQE *ZZ*YOUR-ID-HERE *200901*1409*U*00401*043000123*0*P*>~
GS*SW*WAQE*YOUR-ID-HERE*20200901*1409*000000123*X*004010~
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
    let (rest, _obj) = Transmission::<_945>::parse(str).unwrap();
    assert!(rest.is_empty());
}
