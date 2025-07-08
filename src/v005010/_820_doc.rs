use crate::util::Parser;
use nom::IResult;
use serde::{Deserialize, Serialize};
use x12_types_macros::DisplayX12;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayX12)]
pub struct _820 {}

impl<'a> Parser<&'a str, _820, nom::error::Error<&'a str>> for _820 {
    fn parse(input: &'a str) -> IResult<&'a str, _820> {
        Ok((input, _820 {}))
    }
}
