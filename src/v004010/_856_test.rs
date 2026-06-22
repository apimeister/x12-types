use crate::v004010::*;

#[test]
fn test_856_simple() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *231014*1831*U*00401*000000001*0*P*:~
GS*SH*SENDER*RECEIVER*20231014*1831*0001*X*004010~
ST*856*0001~
BSN*00*SHIP001*20231014*1831~
SE*4*0001~
GE*1*0001~
IEA*1*000000001~"#;

    let result = Transmission::<_856>::parse(str);
    match result {
        Ok((rest, obj)) => {
            println!("Success! Rest: '{}'", rest);
            println!("ISA sender: '{}'", obj.isa._06);
            println!("ISA receiver: '{}'", obj.isa._07);
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
}

#[test]
fn test_parse_856() {
    let str = r#"ISA*01* *00* *ZZ*SENDER *ZZ*RECEIVER *231014*1831*U*00401*000000001*0*P*:~
GS*SH*APP SENDER*APP RECEIVER*20231014*1831*0001*X*004010~
ST*856*0001~
BSN*00*SHIPMENT ID*20231014*1831~
HL*1**S~
TD1*CTN25*24****G*295.15625*LB~
TD5*B*2*ABCD*M*###5D~
REF*BM*REFERENCE ID~
DTM*011*20231014~
N1*ST*COMPANY*15*12345678~
HL*2*1*O~
PRF*0123456789~
N1*BY*BUYER COMPANY1*15*12345678~
HL*3*2*P~
MAN*GM*01234567891011121314~
HL*4*3*I~
LIN**IB*PRODUCT1*EN*PRODUCT2*PO*PRODUCT3*UK*001~
SN1**24*EA~
CTT*1*5~
SE*36*0001~
GE*1*0001~
IEA*1*000000001~"#;
    let result = Transmission::<_856>::parse(str);
    match result {
        Ok((rest, obj)) => {
            println!("Parsed 856: {:#?}", obj);
            println!("Rest: {}", rest);
            // Verify basic structure
            assert_eq!(obj.isa._06.trim(), "SENDER");
            assert_eq!(obj.isa._08.trim(), "RECEIVER");
            assert_eq!(obj.functional_group[0].gs._01, "SH");
            assert_eq!(obj.functional_group[0].segments[0].st._01, "856");
            assert_eq!(obj.functional_group[0].segments[0].bsn._01, "00");
            assert_eq!(obj.functional_group[0].segments[0].bsn._02, "SHIPMENT ID");

            // Verify HL loops are parsed
            assert!(!obj.functional_group[0].segments[0].loop_hl.is_empty());
            assert_eq!(obj.functional_group[0].segments[0].loop_hl[0].hl._01, "1");
            assert_eq!(obj.functional_group[0].segments[0].loop_hl[0].hl._03, "S");
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
            panic!("Parse error: {:?}", e);
        }
    }
}

#[test]
fn test_856_2() {
    // Use a 004010 compatible test data instead of the 004060 fixture
    let str = r#"ISA*00*          *00*          *16*SENDER1        *14*RECEIVER1      *071216*1406*U*00204*000000263*1*T*>~
GS*IN*SENDER1*RECEIVER1*20071216*1406*000000001*X*004010~
ST*856*0001~
BSN*00*01140824*20051015*1345*0001~
HL*1**S~
TD1*CTN25*2****G*45582*LB*1000*CF~
TD5*B*2*JBHT*M~
TD3*TL*ABCD*07213567******30394938483234~
REF*BM*01140824~
REF*CN*082131~
REF*CR*01082131~
DTM*011*200~
N1*ST*WAL-MART DC 6094J-JIT*UL*0078742035260~
N1*SF*SUPPLIER NAME~
HL*2*1*O~
PRF*9988776655***20051015~
REF*IA*211555050~
REF*DP*00005~
REF*MR*0073~
REF*IV*01140824~
N1*BY*WAL-MART STORES,INC.*UL*0078742000992~
HL*3*2*P~
MAN*GM*00000010012345614785~
HL*4*3*I~
LIN**UP*008815509183~
SN1**4*EA~
HL*5*3*I~
LIN**UP*008815547321~
SN1**9*EA~
HL*6*1*O~
PRF*2288115555***20051015~
REF*IA*211555050~
REF*DP*00005~
REF*MR*0073~
REF*IV*01140824~
N1*BY*WAL-MART STORES,INC.*UL*0078742000015~
HL*7*6*P~
MAN*GM*00000010012378945698~
HL*8*7*I~
LIN**UP*008815509183~
SN1**4*EA~
HL*9*7*I~
LIN**UP*008815547321~
SN1**9*EA~
CTT*9~
SE*44*0001~
GE*2*000000001~
IEA*1*000000263~"#;
    let result = Transmission::<_856>::parse(str);
    match result {
        Ok((rest, obj)) => {
            println!("Parsed 856_2: {:#?}", obj);
            println!("Rest: {}", rest);
            // Verify basic structure
            assert_eq!(obj.isa._06.trim(), "SENDER1");
            assert_eq!(obj.isa._08.trim(), "RECEIVER1");
            assert_eq!(obj.functional_group[0].gs._01, "IN");
            assert_eq!(obj.functional_group[0].segments[0].st._01, "856");
            assert_eq!(obj.functional_group[0].segments[0].bsn._01, "00");
            assert_eq!(obj.functional_group[0].segments[0].bsn._02, "01140824");

            // Verify HL loops are parsed
            assert!(!obj.functional_group[0].segments[0].loop_hl.is_empty());
            assert_eq!(obj.functional_group[0].segments[0].loop_hl[0].hl._01, "1");
            assert_eq!(obj.functional_group[0].segments[0].loop_hl[0].hl._03, "S");
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
            panic!("Parse error: {:?}", e);
        }
    }
}

#[test]
fn test_856_basic_structure() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *231014*1831*U*00401*000000001*0*P*:~
GS*SH*SENDER*RECEIVER*20231014*1831*0001*X*004010~
ST*856*0001~
BSN*00*SHIP001*20231014*1831~
HL*1**S~
N1*ST*SHIPPER COMPANY~
N1*SF*SUPPLIER NAME~
HL*2*1*O~
N1*BY*BUYER COMPANY~
HL*3*2*P~
MAN*GM*1234567890123456~
HL*4*3*I~
LIN**UP*123456789~
SN1**10*EA~
CTT*4~
SE*12*0001~
GE*1*0001~
IEA*1*000000001~"#;

    let (rest, obj) = Transmission::<_856>::parse(str).unwrap();
    assert!(rest.is_empty());

    // Verify basic structure
    assert_eq!(obj.isa._06.trim(), "SENDER");
    assert_eq!(obj.isa._08.trim(), "RECEIVER");
    assert_eq!(obj.functional_group[0].gs._01, "SH");
    assert_eq!(obj.functional_group[0].segments[0].st._01, "856");
    assert_eq!(obj.functional_group[0].segments[0].bsn._01, "00");
    assert_eq!(obj.functional_group[0].segments[0].bsn._02, "SHIP001");
}

#[test]
fn test_856_with_carrier_details() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *231014*1831*U*00401*000000001*0*P*:~
GS*SH*SENDER*RECEIVER*20231014*1831*0001*X*004010~
ST*856*0001~
BSN*00*SHIP002*20231014*1831~
HL*1**S~
TD1*CTN25*5****G*500*LB~
TD5*B*2*FEDEX*M~
REF*BM*REF123~
DTM*011*20231014~
N1*ST*SHIPPER COMPANY~
N1*SF*SUPPLIER NAME~
HL*2*1*O~
N1*BY*BUYER COMPANY~
HL*3*2*P~
MAN*GM*1234567890123456~
HL*4*3*I~
LIN**UP*123456789~
SN1**10*EA~
CTT*4~
SE*15*0001~
GE*1*0001~
IEA*1*000000001~"#;

    let (rest, obj) = Transmission::<_856>::parse(str).unwrap();
    assert!(rest.is_empty());

    // Verify carrier details
    let hl_loop = &obj.functional_group[0].segments[0].loop_hl[0];
    assert_eq!(hl_loop.td1[0]._01, Some("CTN25".to_string()));
    assert_eq!(hl_loop.td1[0]._02, Some("5".to_string()));
    assert_eq!(hl_loop.td5[0]._01, "B");
    assert_eq!(hl_loop.td5[0]._02, "2");
    assert_eq!(hl_loop.td5[0]._03, "FEDEX");
}

#[test]
fn test_856_with_line_items() {
    let str = r#"ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *231014*1831*U*00401*000000001*0*P*:~
GS*SH*SENDER*RECEIVER*20231014*1831*0001*X*004010~
ST*856*0001~
BSN*00*SHIP003*20231014*1831~
HL*1**S~
N1*ST*SHIPPER COMPANY~
N1*SF*SUPPLIER NAME~
HL*2*1*O~
N1*BY*BUYER COMPANY~
HL*3*2*P~
MAN*GM*1234567890123456~
HL*4*3*I~
LIN**UP*123456789~
SN1**10*EA~
HL*5*3*I~
LIN**UP*987654321~
SN1**5*EA~
CTT*5~
SE*13*0001~
GE*1*0001~
IEA*1*000000001~"#;
    let result = Transmission::<_856>::parse(str);
    match result {
        Ok((rest, obj)) => {
            println!("Parsed 856_with_line_items: {:#?}", obj);
            println!("Rest: {}", rest);

            // Verify line items
            let hl_loops = &obj.functional_group[0].segments[0].loop_hl;
            assert_eq!(hl_loops.len(), 5);

            // First item level (HL*4*3*I)
            let item1 = &hl_loops[3]; // Index 3 for HL*4
            assert_eq!(item1.lin[0]._02, "UP");
            assert_eq!(item1.lin[0]._03, "123456789");
            assert_eq!(item1.sn1[0]._02, "10");
            assert_eq!(item1.sn1[0]._03, "EA");

            // Second item level (HL*5*3*I)
            let item2 = &hl_loops[4]; // Index 4 for HL*5
            assert_eq!(item2.lin[0]._02, "UP");
            assert_eq!(item2.lin[0]._03, "987654321");
            assert_eq!(item2.sn1[0]._02, "5");
            assert_eq!(item2.sn1[0]._03, "EA");
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
            panic!("Parse error: {:?}", e);
        }
    }
}

#[test]
fn test_856_xy() {
    // source https://support.edifabric.com/hc/en-us/articles/360010386732-X12-856-Ship-Notice
    let str = r#"ISA*00*          *00*          *16*SENDER1        *14*RECEIVER1      *071216*1406*U*00204*000000263*1*T*>~
GS*IN*SENDER1*RECEIVER1*20071216*1406*000000001*X*004010~
ST*856*0001~
BSN*00*01140824*20051015*1345*0001~
HL*1**S~
TD1*CTN25*2****G*45582*LB*1000*CF~
TD5*B*2*JBHT*M~
TD3*TL*ABCD*07213567******30394938483234~
REF*BM*01140824~
REF*CN*082131~
REF*CR*01082131~
DTM*011*200~
N1*ST*WAL-MART DC 6094J-JIT*UL*0078742035260~
N1*SF*SUPPLIER NAME~
HL*2*1*O~
PRF*9988776655***20051015~
REF*IA*211555050~
REF*DP*00005~
REF*MR*0073~
REF*IV*01140824~
N1*BY*WAL-MART STORES,INC.*UL*0078742000992~
HL*3*2*P~
MAN*GM*00000010012345614785~
HL*4*3*I~
LIN**UP*008815509183~
SN1**4*EA~
HL*5*3*I~
LIN**UP*008815547321~
SN1**9*EA~
HL*6*1*O~
PRF*2288115555***20051015~
REF*IA*211555050~
REF*DP*00005~
REF*MR*0073~
REF*IV*01140824~
N1*BY*WAL-MART STORES,INC.*UL*0078742000015~
HL*7*6*P~
MAN*GM*00000010012378945698~
HL*8*7*I~
LIN**UP*008815509183~
SN1**4*EA~
HL*9*7*I~
LIN**UP*008815547321~
SN1**9*EA~
CTT*9~
SE*44*0001~
GE*2*000000001~
IEA*1*000000263~"#;
    let (rest, obj) = Transmission::<_856>::parse(str).unwrap();
    assert!(rest.is_empty());
    println!("{obj:?}");
}

#[test]
fn test_856_xx() {
    // source https://www.fishersci.com/content/dam/fishersci/en_US/documents/programs/scientific/brochures-and-catalogs/guides/fisher-scientific-edi-specifications-856-advance-ship-notice-supplier-guide.pdf
    let str = r#"ISA*00* *00* *ZZ*7777777777 *14*004321519IBMP*070920*0849*U*00401*000012152*0*P*>~
GS*SH*7777777777*004321519*20070920*0849*12467*X*004010~
ST*856*12159~
BSN*00*222253*20070919*115902*0001~
HL*1**S~
DTM*011*20070919~
HL*2*1*O~
PRF*DR3469500~
HL*3*2*P~
MAN*SM*1Z54242X0357922222~
HL*4*3*I~
LIN*001*CB*25901~
SN1*001*2*EA~
CTT*4~
SE*13*12159~
GE*1*12467~
IEA*1*000012152~"#;
    let (rest, obj) = Transmission::<_856>::parse(str).unwrap();
    assert!(rest.is_empty());
    println!("{obj:?}");
}
