# X12-types
[![Latest Version](https://img.shields.io/crates/v/x12-types.svg)](https://crates.io/crates/x12-types)

This library provides bindings for the ASC X12 standard.

## Supported Bindings

* 003030
  * 998 - Set Cancellation
* 004010
  * 204 - Motor Carrier Load Tender
  * 214 - Transportation Carrier Shipment Status Message
  * 309 - U.S. Customs Manifest
  * 310 - Freight Receipt and Invoice (Ocean)
  * 315 - Status Details (Ocean)
  * 322 - Terminal Operations and Intermodal Ramp Activity
  * 404 - Rail Carrier Shipment Information
  * 810 - Invoice
  * 997 - Functional Acknowledgment
  * 998 - Set Cancellation
* 005010
  * 270 - Eligibility, Coverage or Benefit Inquiry
  * 271 - Eligibility, Coverage or Benefit Information
  * 276 - Health Claim Status Request
  * 820 - Payment Order/Remittance Advice
  * 834 - Benefit Enrollment and Maintenance (005010X220A1)
  * 835 - Health Care Claim Payment/Advice
  * 837 - Health Care Claim
  * 999 - Implementation Acknowledgment
* 005030
  * 404 - Rail Carrier Shipment Information

Something missing? Please open an issue.

## sample code

### Rendering X12

```rust
use x12_types::v004010::*;

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
        ...
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
// resulting string
//
// ISA*00*          *00*          *ZZ*SOURCE         *ZZ*TARGET         *220524*1120*U*00401*000000001*0*P*>~
// GS*QO*SOURCE*TARGET*20220524*1600*1*X*004010~
// ....
// GE*1*1~
// IEA*1*000000001~
```

### Parsing X12

```rust
use x12_types::v005010::*;

let str = r#"ISA*01*0000000000*01*0000000000*ZZ*ABCDEFGHIJKLMNO*ZZ*123456789012345*101127*1719*U*00400*000003438*0*P*>~
GS*HP*ABCCOM*01017*20110315*1005*1*X*004010X091A1~
ST*835*07504123~
BPR*H*5.75*C*NON************20110315~
...
SE*93*07504123~
GE*1*1~
IEA*1*004075123~"#;
    let (rest, obj) = Transmission::<_835>::parse(&str).unwrap();
    println!("{obj:?}");
// resulting string
//
// Transmission { isa: 
//  ISA { _01: "01", _02: "0000000000", _03: "01", _04: "0000000000", _05: "ZZ", _06: "ABCDEFGHIJKLMNO", _07: "ZZ", _08: "123456789012345", _09: "101127", _10: "1719", _11: "U", _12: "00400", _13: "000003438", _14: "0", _15: "P", _16: ">" }, 
//  functional_group: [
//    FunctionalGroup { 
//      gs: GS { _01: "HP", _02: "ABCCOM", _03: "01017", _04: "20110315", _05: "1005", _06: "1", _07: "X", _08: "004010X091A1" }, 
//      segments: [_835 { ...
```

More examples are located in the examples directory. Tests are embedded into each version directory.

## Usage from the CLI

We are also maintaining a CLI-tool for a more accessible way to consume EDIs.

[https://crates.io/crates/edi-cli](https://crates.io/crates/edi-cli)

## Contributions

Since the X12 is fairly huge, we only implement types on demand. So if you are missing some types, please open an issue or merge request.
