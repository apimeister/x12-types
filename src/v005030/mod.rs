//! v005030 repesents all entities of the 005030 specification.

use crate::util::Parser;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod element;
mod segment;
pub use segment::*;

mod _404_doc;
pub use _404_doc::*;
#[cfg(test)]
mod _404_test;
#[cfg(test)]
mod segments_test;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Transmission<T> {
    pub isa: ISA,
    pub functional_group: Vec<FunctionalGroup<T>>,
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
        let (input, t_obj) = T::parse(input)?;
        let (input, ge) = GE::parse(input)?;
        let fg = FunctionalGroup {
            gs,
            segments: vec![t_obj],
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct FunctionalGroup<T> {
    pub gs: GS,
    pub segments: Vec<T>,
    pub ge: GE,
}
