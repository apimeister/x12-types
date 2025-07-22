use serde::{Deserialize, Serialize};
use x12_types_macros::{DisplaySegment, ParseSegment};

/// F9 - Origin Station
///
/// To identify the station where the shipment originates
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct F9 {
    #[serde(rename = "01")]
    pub _01: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: String,
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "04")]
    pub _04: Option<String>,
    #[serde(rename = "05")]
    pub _05: Option<String>,
    /// 19 - City Name
    ///
    /// Free-form text for city name
    /// - TYPE=AN
    /// - MIN=2
    /// - MAX=30
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
    /// 26 - Country Code
    ///
    /// Code identifying the country
    /// - TYPE=ID
    /// - MIN=2
    /// - MAX=3
    #[serde(rename = "12")]
    pub _12: Option<String>,
}

/// FA1 - Type of Financial Accounting Data
///
/// To specify the organization controlling the content of the accounting citation, and the purpose associated with the accounting citation
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct FA1 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: Option<String>,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}

/// FA2 - Accounting Data
///
/// To specify the detailed accounting data
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct FA2 {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
}

/// FOB - F.O.B. Related Instructions
///
/// To specify transportation instructions relating to shipment
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplaySegment, ParseSegment,
)]
pub struct FOB {
    #[serde(rename = "01")]
    pub _01: String,
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
}
