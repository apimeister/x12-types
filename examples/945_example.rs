use x12_types::util::Parser;
use x12_types::v004010::*;

fn main() {
    // Create a simple 945 Warehouse Shipping Advice
    let warehouse_shipping_advice = _945 {
        st: ST {
            _01: "945".to_string(),
            _02: "0001".to_string(),
        },
        w06: W06 {
            _01: "F".to_string(),              // Full shipment
            _02: Some("WH001234".to_string()), // Warehouse order number
            _03: Some("20231215".to_string()), // Ship date
            _04: Some("1430".to_string()),     // Ship time
            _06: Some("SHIP001".to_string()),  // Shipment ID
            ..Default::default()
        },
        loop_n1: vec![
            // Ship-to information
            _945LoopN1 {
                n1: N1 {
                    _01: "ST".to_string(), // Ship-to
                    _02: Some("ABC COMPANY".to_string()),
                    _03: Some("91".to_string()), // Assigned by seller
                    _04: Some("123456".to_string()),
                    ..Default::default()
                },
                n2: vec![],
                n3: vec![N3 {
                    _01: "123 MAIN STREET".to_string(),
                    ..Default::default()
                }],
                n4: Some(N4 {
                    _01: Some("ANYTOWN".to_string()),
                    _02: Some("NY".to_string()),
                    _03: Some("12345".to_string()),
                    ..Default::default()
                }),
            },
            // Ship-from (warehouse) information
            _945LoopN1 {
                n1: N1 {
                    _01: "SF".to_string(), // Ship-from
                    _02: Some("WAREHOUSE DEPOT".to_string()),
                    _03: Some("9".to_string()),
                    _04: Some("DEPOT001".to_string()),
                    ..Default::default()
                },
                n2: vec![],
                n3: vec![],
                n4: None,
            },
        ],
        n9: vec![N9 {
            _01: "BM".to_string(), // Bill of lading number
            _02: "BOL123456".to_string(),
            ..Default::default()
        }],
        g62: vec![G62 {
            _01: Some("11".to_string()), // Ship date
            _02: Some("20231215".to_string()),
            ..Default::default()
        }],
        w27: Some(W27 {
            _01: "M".to_string(),          // Motor (truck)
            _02: Some("FEDX".to_string()), // FedEx
            _03: Some("GROUND".to_string()),
            _04: Some("PP".to_string()), // Prepaid
            ..Default::default()
        }),
        loop_lx: vec![_945LoopLX {
            lx: LX {
                _01: "1".to_string(), // Line item 1
            },
            w12: W12 {
                _01: "CC".to_string(),                 // Complete shipment
                _02: Some("10".to_string()),           // Quantity ordered
                _03: Some("10".to_string()),           // Quantity shipped
                _04: Some("EA".to_string()),           // Each
                _05: Some("EA".to_string()),           // Each
                _07: Some("UP".to_string()),           // UPC
                _08: Some("123456789012".to_string()), // UPC code
                _09: Some("VN".to_string()),           // Vendor part number
                _10: Some("WIDGET001".to_string()),
                ..Default::default()
            },
            n9: vec![
                N9 {
                    _01: "LI".to_string(), // Line item
                    _02: "1".to_string(),
                    ..Default::default()
                },
                N9 {
                    _01: "PO".to_string(), // Purchase order
                    _02: "PO123456".to_string(),
                    ..Default::default()
                },
            ],
        }],
        w03: Some(W03 {
            _01: "1".to_string(),          // Total quantity
            _02: Some("25.5".to_string()), // Total weight
            _03: Some("LB".to_string()),   // Pounds
            ..Default::default()
        }),
        se: SE {
            _01: "15".to_string(), // Number of segments
            _02: "0001".to_string(),
        },
    };

    // Create a full transmission
    let transmission = Transmission {
        isa: ISA {
            _01: "00".to_string(),
            _02: "          ".to_string(),
            _03: "00".to_string(),
            _04: "          ".to_string(),
            _05: "ZZ".to_string(),
            _06: "WAREHOUSE      ".to_string(),
            _07: "ZZ".to_string(),
            _08: "SUPPLIER       ".to_string(),
            _09: "231215".to_string(),
            _10: "1430".to_string(),
            _11: "U".to_string(),
            _12: "00401".to_string(),
            _13: "000000001".to_string(),
            _14: "0".to_string(),
            _15: "P".to_string(),
            _16: ">".to_string(),
        },
        functional_group: vec![FunctionalGroup {
            gs: GS {
                _01: "SW".to_string(), // Warehouse shipping advice
                _02: "WAREHOUSE".to_string(),
                _03: "SUPPLIER".to_string(),
                _04: "20231215".to_string(),
                _05: "1430".to_string(),
                _06: "1".to_string(),
                _07: "X".to_string(),
                _08: "004010".to_string(),
            },
            segments: vec![warehouse_shipping_advice],
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

    // Serialize to X12 format
    let x12_output = format!("{}", transmission);
    println!("Generated X12-945 Warehouse Shipping Advice:");
    println!("{}", x12_output);

    // Parse it back to verify
    let (remaining, parsed) = Transmission::<_945>::parse(&x12_output).unwrap();
    assert!(remaining.is_empty());

    println!("\nParsed successfully!");
    println!(
        "Transaction type: {}",
        parsed.functional_group[0].segments[0].st._01
    );
    println!(
        "Shipment ID: {:?}",
        parsed.functional_group[0].segments[0].w06._06
    );
    println!(
        "Ship-to: {:?}",
        parsed.functional_group[0].segments[0].loop_n1[0].n1._02
    );
    println!(
        "Number of line items: {}",
        parsed.functional_group[0].segments[0].loop_lx.len()
    );
}
