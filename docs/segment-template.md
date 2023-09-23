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

impl<'a> Parser<&'a str, ST, nom::error::Error<&'a str>> for ST {
    fn parse(input: &'a str) -> IResult<&'a str, ST> {
        let (rest, vars) = crate::util::parse_line(input, "ST")?;
        let mut obj = ST {
            _01: vars.first().unwrap().to_string(),
            _02: vars.get(1).unwrap().to_string(),
            _03: vars.get(2).map(util::unborrow_string),
        };
        Ok((rest, obj))
    }
}
```