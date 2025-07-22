use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// L0 - Line Item - Quantity and Weight
///
/// To specify quantity, weight, volume, and type of service for a line item including applicable "quantity/rate-as" data
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 213 | Lading Line Item Number | 1 | O | N0 | 1/3
/// 02 | 220 | Billed/Rated-as Quantity | 1 | X | R | 1/11
/// 03 | 221 | Billed/Rated-as Qualifier | 1 | X | ID | 2/2
/// 04 | 81 | Weight | 1 | X | R | 1/10
/// 05 | 187 | Weight Qualifier | 1 | X | ID | 1/2
/// 06 | 183 | Volume | 1 | X | R | 1/8
/// 07 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 08 | 80 | Lading Quantity | 1 | X/Z | N0 | 1/7
/// 09 | 211 | Packaging Form Code | 1 | X | ID | 3/3
/// 10 | 458 | Dunnage Description | 1 | O | AN | 2/25
/// 11 | 188 | Weight Unit Code | 1 | O | ID | 1/1
/// 12 | 56 | Type of Service Code | 1 | O | ID | 2/2
/// 13 | 380 | Quantity | 1 | X/Z | R | 1/15
/// 14 | 211 | Packaging Form Code | 1 | O | ID | 3/3
/// 15 | 1073 | Yes/No Condition or Response Code | 1 | X/Z | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct L0 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
    #[serde(rename = "15")]
    pub _15: Option<String>,
}

/// L1 - Rate and Charges
///
/// To specify rate and charges detail relative to a line item including freight charges, advances, special charges, and entitlements
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 213 | Lading Line Item Number | 1 | O | N0 | 1/3
/// 02 | 60 | Freight Rate | 1 | X | R | 1/9
/// 03 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 04 | 58 | Charge | 1 | X | N2 | 1/12
/// 05 | 191 | Advances | 1 | X | N2 | 1/9
/// 06 | 117 | Prepaid Amount | 1 | X | N2 | 1/9
/// 07 | 120 | Rate Combination Point Code | 1 | O | AN | 3/9
/// 08 | 150 | Special Charge or Allowance Code | 1 | O | ID | 3/3
/// 09 | 121 | Rate Class Code | 1 | O | ID | 1/3
/// 10 | 39 | Entitlement Code | 1 | O | ID | 1/1
/// 11 | 16 | Charge Method of Payment | 1 | O | ID | 1/1
/// 12 | 276 | Special Charge Description | 1 | O | AN | 2/25
/// 13 | 257 | Tariff Application Code | 1 | O | ID | 1/1
/// 14 | 74 | Declared Value | 1 | X | N2 | 2/12
/// 15 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 16 | 372 | Lading Liability Code | 1 | O | ID | 1/1
/// 17 | 220 | Billed/Rated-as Quantity | 1 | X | R | 1/11
/// 18 | 221 | Billed/Rated-as Qualifier | 1 | X | ID | 2/2
/// 19 | 954 | Percent | 1 | O/Z | R | 1/10
/// 20 | 100 | Currency Code | 1 | O/Z | ID | 3/3
/// 21 | 610 | Amount | 1 | O/Z | N2 | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct L1 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
    #[serde(rename = "15")]
    pub _15: Option<String>,
    #[serde(rename = "16")]
    pub _16: Option<String>,
    #[serde(rename = "17")]
    pub _17: Option<String>,
    #[serde(rename = "18")]
    pub _18: Option<String>,
    #[serde(rename = "19")]
    pub _19: Option<String>,
    #[serde(rename = "20")]
    pub _20: Option<String>,
    #[serde(rename = "21")]
    pub _21: Option<String>,
}

/// L3 - Total Weight and Charges
///
/// To specify the total shipment in terms of weight, volume, rates, charges, advances, and prepaid amounts applicable to one or more line items
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 81 | Weight | 1 | X | R | 1/10
/// 02 | 187 | Weight Qualifier | 1 | X | ID | 1/2
/// 03 | 60 | Freight Rate | 1 | X | R | 1/9
/// 04 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
/// 05 | 58 | Charge | 1 | O/Z | N2 | 1/12
/// 06 | 191 | Advances | 1 | O | N2 | 1/9
/// 07 | 117 | Prepaid Amount | 1 | O | N2 | 1/9
/// 08 | 150 | Special Charge or Allowance Code | 1 | O | ID | 3/3
/// 09 | 183 | Volume | 1 | X | R | 1/8
/// 10 | 184 | Volume Unit Qualifier | 1 | X | ID | 1/1
/// 11 | 80 | Lading Quantity | 1 | O | N0 | 1/7
/// 12 | 188 | Weight Unit Code | 1 | O | ID | 1/1
/// 13 | 171 | Tariff Number | 1 | O | AN | 1/7
/// 14 | 74 | Declared Value | 1 | X | N2 | 2/12
/// 15 | 122 | Rate/Value Qualifier | 1 | X | ID | 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct L3 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
    #[serde(rename = "15")]
    pub _15: Option<String>,
}

/// L4 - Measurement
///
/// To describe physical ddimensions and quantities
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct L4 {
    /// 82 - Length
    #[serde(rename = "01")]
    pub _01: String,
    /// 189 - Width
    #[serde(rename = "02")]
    pub _02: String,
    /// 65 - Height
    #[serde(rename = "03")]
    pub _03: String,
    /// 90 - Measurement Unit Qualifier
    #[serde(rename = "04")]
    pub _04: String,
    /// 380 - Quantity
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 271 - Industry Code
    #[serde(rename = "06")]
    pub _06: Option<String>,
}

/// L5 - Description, Marks and Numbers
///
/// To specify the line item in terms of description, quantity, packaging, and marks and numbers
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 213 | Lading Line Item Number | 1 | O | N0 | 1/3
/// 02 | 79 | Lading Description | 1 | O | AN | 1/50
/// 03 | 22 | Commodity Code | 1 | X | AN | 1/30
/// 04 | 23 | Commodity Code Qualifier | 1 | X | ID | 1/1
/// 05 | 103 | Packaging Code | 1 | O | AN | 3/5
/// 06 | 87 | Marks and Numbers | 1 | X | AN | 1/48
/// 07 | 88 | Marks and Numbers Qualifier | 1 | O | ID | 1/2
/// 08 | 23 | Commodity Code Qualifier | 1 | X | ID | 1/1
/// 09 | 22 | Commodity Code | 1 | X | AN | 1/30
/// 10 | 595 | Compartment ID Code | 1 | O | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct L5 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
}

/// L7 - Tariff Reference
///
/// To reference details of the tariff used to arrive at applicable rates or charge
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 213 | Lading Line Item Number | O |  | N0 1/3
/// 02 | 168 | Tariff Agency Code | O |  | ID 1/4
/// 03 | 171 | Tariff Number | O |  | AN 1/7
/// 04 | 172 | Tariff Section | O |  | AN 1/2
/// 05 | 169 | Tariff Item Number | O |  | AN 1/16
/// 06 | 170 | Tariff Item Part | O |  | N0 1/2
/// 07 | 59 | Freight Class Code | O |  | AN 2/5
/// 08 | 173 | Tariff Supplement Identifier | O |  | AN 1/4
/// 09 | 46 | Ex Parte | O |  | AN 4/4
/// 10 | 373 | Date | O |  | DT 8/8
/// 11 | 119 | Rate Basis Number | O |  | AN 1/6
/// 12 | 227 | Tariff Column | O |  | AN 1/2
/// 13 | 294 | Tariff Distance | O |  | N0 1/5
/// 14 | 295 | Distance Qualifier | O |  | ID 1/1
/// 15 | 19 | City Name | O |  | AN 2/30
/// 16 | 156 | State or Province Code | O |  | ID 2/2
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct L7 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
    #[serde(rename = "15")]
    pub _15: Option<String>,
    #[serde(rename = "16")]
    pub _16: Option<String>,
}

/// L11 - Business Instructions and Reference Number
///
/// To specify instructions in this business relationship or a reference number
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 127 | Reference Identification | 1 | X | AN | 1/30
/// 02 | 128 | Reference Identification Qualifier | 1 | X | ID | 2/3
/// 03 | 352 | Description | 1 | X | AN | 1/80
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct L11 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// LAD - Lading Detail
///
/// To transmit detailed lading data pertinent to a pickup or delivery
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 211 | Packaging Form Code | 1 | X | ID | 3/3
/// 02 | 80 | Lading Quantity | 1 | X | N0 | 1/7
/// 03 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 04 | 395 | Unit Weight | 1 | X | R | 1/8
/// 05 | 188 | Weight Unit Code | 1 | X | ID | 1/1
/// 06 | 81 | Weight | 1 | X | R | 1/10
/// 07 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 08 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 09 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 10 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 11 | 235 | Product/Service ID Qualifier | 1 | X | ID | 2/2
/// 12 | 234 | Product/Service ID | 1 | X | AN | 1/48
/// 13 | 79 | Lading Description | 1 | O | AN | 1/50
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LAD {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
}

/// LE - Loop Trailer
///
/// To indicate that the loop immediately preceding this segment is complete
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 447 | Loop Identifier Code | 1 | M | AN | 1/6
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LE {
    #[serde(rename = "01")]
    pub _01: String,
}

/// LEP - EPA Required Data
///
/// To specify the Environmental Protection Agency (EPA) information relating to shipments of hazardous material
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 806 | EPA Waste Stream Number Code | 1 | O | ID | 4/6
/// 02 | 807 | Waste Characteristics Code | 1 | O | ID | 12/16
/// 03 | 156 | State or Province Code NEW | 1 | X/Z | ID | 2/2
/// 04 | 127 | Reference Identification NEW | 1 | X/Z | AN | 1/30
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LEP {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// LFH - Freeform Hazardous Material Information
///
/// To uniquely identify the variable information required by government regulation covering the transportation of hazardous material shipments
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 808 | Hazardous Material Shipment Information Qualifier | 1 | M | ID | 3/3
/// 02 | 809 | Hazardous Material Shipment Information | 1 | M | AN | 1/25
/// 03 | 809 | Hazardous Material Shipment Information | 1 | O | AN | 1/25
/// 04 | 1023 | Hazard Zone Code | 1 | O | ID | 1/1
/// 05 | 355 | Unit or Basis for Measurement Code NEW | 1 | X | ID | 2/2
/// 06 | 380 | Quantity NEW | 1 | X/Z | R | 1/15
/// 07 | 380 | Quantity NEW | 1 | O/Z | R | 1/15
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LFH {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
}

/// LH1 - Hazardous Identification Information
///
/// To specify the hazardous commodity identification reference number and quantity
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 355 | Unit or Basis for Measurement Code | 1 | M | ID | 2/2
/// 02 | 80 | Lading Quantity | 1 | M | N0 | 1/7
/// 03 | 277 | UN/NA Identification Code | 1 | O | ID | 6/6
/// 04 | 200 | Hazardous Materials Page | 1 | O | AN | 1/6
/// 05 | 22 | Commodity Code | 1 | O | AN | 1/30
/// 06 | 355 | Unit or Basis for Measurement Code | 1 | O | ID | 2/2
/// 07 | 380 | Quantity | 1 | O | R | 1/15
/// 08 | 595 | Compartment ID Code | 1 | O | ID | 1/1
/// 09 | 665 | Residue Indicator Code | 1 | O | ID | 1/1
/// 10 | 254 | Packing Group Code | 1 | O | ID | 1/3
/// 11 | 1375 | Interim Hazardous Material Regulatory Number | 1 | O | AN | 1/5
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LH1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
}

/// LH2 - Hazardous Classification Information
///
/// To specify the hazardous notation and endorsement information
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 215 | Hazardous Classification | 1 | O | ID | 1/30
/// 02 | 983 | Hazardous Class Qualifier | 1 | O | ID | 1/1
/// 03 | 218 | Hazardous Placard Notation | 1 | O | ID | 14/40
/// 04 | 222 | Hazardous Endorsement | 1 | O | ID | 4/25
/// 05 | 759 | Reportable Quantity Code | 1 | O | ID | 2/2
/// 06 | 355 | Unit or Basis for Measurement Code | 1 | X/Z | ID | 2/2
/// 07 | 408 | Temperature | 1 | X | R | 1/4
/// 08 | 355 | Unit or Basis for Measurement Code NEW | 1 | X/Z | ID | 2/2
/// 09 | 408 | Temperature NEW | 1 | X | R | 1/4
/// 10 | 355 | Unit or Basis for Measurement Code NEW | 1 | X/Z | ID | 2/2
/// 11 | 408 | Temperature NEW | 1 | X | R | 1/4
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LH2 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
}

/// LH3 - Hazardous Material Shipping Name
///
/// To specify the hazardous material shipping name and additional descriptive requirements
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 224 | Hazardous Material Shipping Name | 1 | X | AN | 1/25
/// 02 | 984 | Hazardous Material Shipping Name Qualifier | 1 | X | ID | 1/1
/// 03 | 985 | N.O.S. Indicator Code | 1 | O | ID | 3/3
/// 04 | 1073 | Yes/No Condition or Response Code | 1 | O/Z | ID | 1/1
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LH3 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// LH4 - Canadian Dangerous Requirements
///
/// To specify additional Transport Canada requirements covering transportation of dangerous goods in Canada
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// --- | --- | ---- | ------ | --- | ---- | -----
/// 01 | 238 | Emergency Response Plan Number | 1 | O | AN | 1/12
/// 02 | 364 | Communication Number | 1 | O | AN | 1/80
/// 03 | 254 | Packing Group Code | 1 | O | ID | 1/3
/// 04 | 230 | Subsidiary Classification | 1 | O | ID | 1/3
/// 05 | 230 | Subsidiary Classification | 1 | O | ID | 1/3
/// 06 | 230 | Subsidiary Classification | 1 | O | ID | 1/3
/// 07 | 271 | Subsidiary Risk Indicator | 1 | O | ID | 1/2
/// 08 | 267 | Net Explosive Quantity | 1 | X | N0 | 1/6
/// 09 | 805 | Canadian Hazardous Notation | 1 | O | AN | 1/25
/// 10 | 986 | Special Commodity Indicator Code | 1 | O | ID | 1/1
/// 11 | 364 | Communication Number | 1 | O/Z | AN | 1/80
/// 12 | 355 | Unit or Basis for Measurement Code NEW | 1 | X | ID | 2/2
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplaySegment, ParseSegment)]
pub struct LH4 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
}

/// LH6 - Hazardous Certification
///
/// To specify the name of the person certifying that the shipment complies with the regulations and/or the actual certification
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 93 | Name | 1 | O | AN | 1/60
/// 02 | 272 | Hazardous Certification Code | 1 | X | ID | 1/1
/// 03 | 273 | Hazardous Certification Declaration | 1 | X | AN | 1/25
/// 04 | 273 | Hazardous Certification Declaration | 1 | O | AN | 1/25
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LH6 {
    /// 93 - Name
    ///
    /// Free-form name
    /// - TYPE=AN
    /// - MIN=1
    /// - MAX=60
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
    #[serde(rename = "04")]
    pub _04: Option<String>,
}

/// LHR - Hazardous Material Identifying Reference Numbers
///
/// To transmit specific hazardous material reference numbers
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 128 | Reference Identification Qualifier | 1 | M | ID | 2/3
/// 02 | 127 | Reference Identification | 1 | M | AN | 1/30
/// 03 | 373 | Date NEW | 1 | O | DT | 8/8
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LHR {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    /// 373 - Date
    ///
    /// Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year
    /// - TYPE=DT
    /// - MIN=8
    /// - MAX=8
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// LHT - Transborder Hazardous Requirements
///
/// To specify the placard information required by the second government agency when shipment is to cross into another country
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 215 | Hazardous Classification | 1 | O | ID | 1/30
/// 02 | 218 | Hazardous Placard Notation | 1 | O | ID | 14/40
/// 03 | 222 | Hazardous Endorsement | 1 | O | ID | 4/25
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LHT {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// LM - Code Source Information
///
/// To transmit standard code list identification information
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LM {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// LQ - Industry Code
///
/// Code to transmit standard industry codes
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LQ {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: Option<String>,
}

/// LS - Loop Header
///
/// To indicate that the next segment begins a loop
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 447 | Loop Identifier Code | 1 | M | AN | 1/6
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LS {
    #[serde(rename = "01")]
    pub _01: String,
}

/// LX - Assigned Number
///
/// To reference a line number in a transaction set
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 554 | Assigned Number | 1 | M | N0 | 1/6
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LX {
    #[serde(rename = "01")]
    pub _01: String,
}

/// LIN - Item Identification
///
/// To specify basic item identification data
///
/// REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX
/// ----|----|-------|--------|----|------|-------
/// 01 | 350 | Assigned Identification | 1 | O | AN | 1/20
/// 02 | 235 | Product/Service ID Qualifier | 1 | M | ID | 2/2
/// 03 | 234 | Product/Service ID | 1 | M | AN | 1/48
/// 04 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 05 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 06 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 07 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 08 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 09 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 10 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 11 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 12 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 13 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 14 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 15 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 16 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 17 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 18 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 19 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 20 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 21 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 22 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 23 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 24 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 25 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 26 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 27 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 28 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 29 | 234 | Product/Service ID | 1 | O | AN | 1/48
/// 30 | 235 | Product/Service ID Qualifier | 1 | O | ID | 2/2
/// 31 | 234 | Product/Service ID | 1 | O | AN | 1/48
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct LIN {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: String,
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    #[serde(rename = "06")]
    pub _06: Option<String>,
    #[serde(rename = "07")]
    pub _07: Option<String>,
    #[serde(rename = "08")]
    pub _08: Option<String>,
    #[serde(rename = "09")]
    pub _09: Option<String>,
    #[serde(rename = "10")]
    pub _10: Option<String>,
    #[serde(rename = "11")]
    pub _11: Option<String>,
    #[serde(rename = "12")]
    pub _12: Option<String>,
    #[serde(rename = "13")]
    pub _13: Option<String>,
    #[serde(rename = "14")]
    pub _14: Option<String>,
    #[serde(rename = "15")]
    pub _15: Option<String>,
    #[serde(rename = "16")]
    pub _16: Option<String>,
    #[serde(rename = "17")]
    pub _17: Option<String>,
    #[serde(rename = "18")]
    pub _18: Option<String>,
    #[serde(rename = "19")]
    pub _19: Option<String>,
    #[serde(rename = "20")]
    pub _20: Option<String>,
    #[serde(rename = "21")]
    pub _21: Option<String>,
    #[serde(rename = "22")]
    pub _22: Option<String>,
    #[serde(rename = "23")]
    pub _23: Option<String>,
    #[serde(rename = "24")]
    pub _24: Option<String>,
    #[serde(rename = "25")]
    pub _25: Option<String>,
    #[serde(rename = "26")]
    pub _26: Option<String>,
    #[serde(rename = "27")]
    pub _27: Option<String>,
    #[serde(rename = "28")]
    pub _28: Option<String>,
    #[serde(rename = "29")]
    pub _29: Option<String>,
    #[serde(rename = "30")]
    pub _30: Option<String>,
    #[serde(rename = "31")]
    pub _31: Option<String>,
}
