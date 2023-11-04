# how a new segment should look like?

```
/// ST - Transaction Set Header
#[derive(Serialize, Deserialize, Clone, Default, Debug, Validate, PartialEq, Eq)]
pub struct ST {
    #[serde(rename = "01")]
    pub _01: String,
    #[serde(rename = "02")]
    pub _02: String,
    #[serde(rename = "03")]
    pub _03: Option<String>,
}
```