//! v005010 repesents all entities of the 005010 specification.

use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    IResult, Parser as _,
};
pub use segment::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

mod _270_doc;
pub use _270_doc::*;
#[cfg(test)]
mod _270_test;

mod _271_doc;
pub use _271_doc::*;
#[cfg(test)]
mod _271_test;

mod _276_doc;
pub use _276_doc::*;
#[cfg(test)]
mod _276_test;

mod _277_doc;
pub use _277_doc::*;
#[cfg(test)]
mod _277_test;

mod _278_doc;
pub use _278_doc::*;
#[cfg(test)]
mod _278_test;

mod _820_doc;
pub use _820_doc::*;
#[cfg(test)]
mod _820_test;

mod _834_doc;
pub use _834_doc::*;
#[cfg(test)]
mod _834_test;

mod _835_doc;
pub use _835_doc::*;
#[cfg(test)]
mod _835_test;

mod _837_doc;
pub use _837_doc::*;
#[cfg(test)]
mod _837_test;

mod _850_doc;
pub use _850_doc::*;
#[cfg(test)]
mod _850_test;

mod _855_doc;
pub use _855_doc::*;
#[cfg(test)]
mod _855_test;

mod _999_doc;
pub use _999_doc::*;
#[cfg(test)]
mod _999_test;

mod segment;
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
        let mut loop_rest = input;
        let mut obj_arr = vec![];
        while peek(opt(ST::parse)).parse(loop_rest)?.1.is_some() {
            let (input, t_obj) = T::parse(loop_rest)?;
            loop_rest = input;
            obj_arr.push(t_obj);
        }
        let input = loop_rest;
        let (input, ge) = GE::parse(input)?;
        let fg = FunctionalGroup {
            gs,
            segments: obj_arr,
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
