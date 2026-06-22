//! v004010 repesents all entities of the 004010 specification.

use crate::util::Parser;
use nom::multi::many0;
use nom::IResult;
use nom::Parser as _;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use validator::Validate;

pub mod segment;
pub use segment::*;

mod _204_doc;
pub use _204_doc::*;
#[cfg(test)]
mod _204_test;

mod _214_doc;
pub use _214_doc::*;
#[cfg(test)]
mod _214_test;

mod _301_doc;
pub use _301_doc::*;
#[cfg(test)]
mod _301_test;

mod _309_doc;
pub use _309_doc::*;
#[cfg(test)]
mod _309_test;

mod _310_doc;
pub use _310_doc::*;
#[cfg(test)]
mod _310_test;

mod _315_doc;
pub use _315_doc::*;
#[cfg(test)]
mod _315_test;

mod _322_doc;
pub use _322_doc::*;
#[cfg(test)]
mod _322_test;

mod _404_doc;
pub use _404_doc::*;
#[cfg(test)]
mod _404_test;

mod _810_doc;
pub use _810_doc::*;
#[cfg(test)]
mod _810_test;

mod _856_doc;
pub use _856_doc::*;
#[cfg(test)]
mod _856_test;

mod _940_doc;
pub use _940_doc::*;
#[cfg(test)]
mod _940_test;

mod _945_doc;
pub use _945_doc::*;
#[cfg(test)]
mod _945_test;

mod _997_doc;
pub use _997_doc::*;
#[cfg(test)]
mod _997_test;

mod _998_doc;
pub use _998_doc::*;
#[cfg(test)]
mod _998_test;

#[cfg(test)]
mod segments_test;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, Validate)]
pub struct Transmission<T> {
    #[validate(nested)]
    pub isa: ISA,
    pub functional_group: Vec<FunctionalGroup<T>>,
    #[validate(nested)]
    pub iea: IEA,
}

impl<'a, T: Default + Parser<&'a str, T, nom::error::Error<&'a str>>>
    Parser<&'a str, Transmission<T>, nom::error::Error<&'a str>> for Transmission<T>
{
    fn parse(input: &'a str) -> IResult<&'a str, Transmission<T>> {
        let mut output = Transmission::default();
        let (input, obj) = ISA::parse(input)?;
        output.isa = obj;
        // functional group
        let (input, gs) = GS::parse(input)?;
        let (input, t_obj) = many0(T::parse).parse(input)?;
        // let (input, t_obj) = T::parse(input)?;
        let (input, ge) = GE::parse(input)?;
        let fg = FunctionalGroup {
            gs,
            segments: t_obj,
            // segments: vec![t_obj],
            ge,
        };
        output.functional_group.push(fg);
        let (input, obj) = IEA::parse(input)?;
        output.iea = obj;
        Ok((input, output))
    }
}

impl<T: Display> Display for Transmission<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![];
        lines.push(format!("{}", self.isa));
        for fg in &self.functional_group {
            lines.push(format!("{}", fg.gs));
            for segment in &fg.segments {
                lines.push(format!("{segment}"));
            }
            lines.push(format!("{}", fg.ge));
        }
        lines.push(format!("{}", self.iea));
        let all = lines.join("");
        write!(f, "{all}")
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, Validate)]
pub struct FunctionalGroup<T> {
    #[validate(nested)]
    pub gs: GS,
    pub segments: Vec<T>,
    #[validate(nested)]
    pub ge: GE,
}
