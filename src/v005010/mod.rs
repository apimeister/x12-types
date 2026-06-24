//! v005010 repesents all entities of the 005010 specification.

use crate::util::Parser;
use nom::{
    combinator::{opt, peek},
    IResult, Parser as _,
};
pub mod element;
pub use segment::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

mod _148_doc;
pub use _148_doc::*;
#[cfg(test)]
mod _148_test;

mod _204_doc;
pub use _204_doc::*;
#[cfg(test)]
mod _204_test;

mod _210_doc;
pub use _210_doc::*;
#[cfg(test)]
mod _210_test;

mod _211_doc;
pub use _211_doc::*;
#[cfg(test)]
mod _211_test;

mod _212_doc;
pub use _212_doc::*;
#[cfg(test)]
mod _212_test;

mod _216_doc;
pub use _216_doc::*;
#[cfg(test)]
mod _216_test;

mod _217_doc;
pub use _217_doc::*;
#[cfg(test)]
mod _217_test;

mod _214_doc;
pub use _214_doc::*;
#[cfg(test)]
mod _214_test;

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

mod _274_doc;
pub use _274_doc::*;
#[cfg(test)]
mod _274_test;

mod _275_doc;
pub use _275_doc::*;
#[cfg(test)]
mod _275_test;

mod _278_doc;
pub use _278_doc::*;
#[cfg(test)]
mod _278_test;

mod _300_doc;
pub use _300_doc::*;
#[cfg(test)]
mod _300_test;

mod _301_doc;
pub use _301_doc::*;
#[cfg(test)]
mod _301_test;

mod _303_doc;
pub use _303_doc::*;
#[cfg(test)]
mod _303_test;

mod _304_doc;
pub use _304_doc::*;
#[cfg(test)]
mod _304_test;

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

mod _350_doc;
pub use _350_doc::*;
#[cfg(test)]
mod _350_test;

mod _404_doc;
pub use _404_doc::*;
#[cfg(test)]
mod _404_test;

mod _417_doc;
pub use _417_doc::*;
#[cfg(test)]
mod _417_test;

mod _425_doc;
pub use _425_doc::*;
#[cfg(test)]
mod _425_test;

mod _353_doc;
pub use _353_doc::*;
#[cfg(test)]
mod _353_test;

mod _810_doc;
pub use _810_doc::*;
#[cfg(test)]
mod _810_test;

mod _811_doc;
pub use _811_doc::*;
#[cfg(test)]
mod _811_test;

mod _820_doc;
pub use _820_doc::*;
#[cfg(test)]
mod _820_test;

mod _821_doc;
pub use _821_doc::*;
#[cfg(test)]
mod _821_test;

mod _822_doc;
pub use _822_doc::*;
#[cfg(test)]
mod _822_test;

mod _823_doc;
pub use _823_doc::*;
#[cfg(test)]
mod _823_test;

mod _824_doc;
pub use _824_doc::*;
#[cfg(test)]
mod _824_test;

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

mod _846_doc;
pub use _846_doc::*;
#[cfg(test)]
mod _846_test;

mod _850_doc;
pub use _850_doc::*;
#[cfg(test)]
mod _850_test;

mod _855_doc;
pub use _855_doc::*;
#[cfg(test)]
mod _855_test;

mod _856_doc;
pub use _856_doc::*;
#[cfg(test)]
mod _856_test;

mod _857_doc;
pub use _857_doc::*;
#[cfg(test)]
mod _857_test;

mod _860_doc;
pub use _860_doc::*;
#[cfg(test)]
mod _860_test;

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

mod _999_doc;
pub use _999_doc::*;
#[cfg(test)]
mod _999_test;

mod _830_doc;
pub use _830_doc::*;
#[cfg(test)]
mod _830_test;

mod _852_doc;
pub use _852_doc::*;
#[cfg(test)]
mod _852_test;

mod _861_doc;
pub use _861_doc::*;
#[cfg(test)]
mod _861_test;

mod _862_doc;
pub use _862_doc::*;
#[cfg(test)]
mod _862_test;

mod _943_doc;
pub use _943_doc::*;
#[cfg(test)]
mod _943_test;

mod _944_doc;
pub use _944_doc::*;
#[cfg(test)]
mod _944_test;

mod _990_doc;
pub use _990_doc::*;
#[cfg(test)]
mod _990_test;

mod _163_doc;
pub use _163_doc::*;
#[cfg(test)]
mod _163_test;

mod _180_doc;
pub use _180_doc::*;
#[cfg(test)]
mod _180_test;

mod _753_doc;
pub use _753_doc::*;
#[cfg(test)]
mod _753_test;

mod _754_doc;
pub use _754_doc::*;
#[cfg(test)]
mod _754_test;

mod _812_doc;
pub use _812_doc::*;
#[cfg(test)]
mod _812_test;

mod _816_doc;
pub use _816_doc::*;
#[cfg(test)]
mod _816_test;

mod _832_doc;
pub use _832_doc::*;
#[cfg(test)]
mod _832_test;

mod _840_doc;
pub use _840_doc::*;
#[cfg(test)]
mod _840_test;

mod _843_doc;
pub use _843_doc::*;
#[cfg(test)]
mod _843_test;

mod _845_doc;
pub use _845_doc::*;
#[cfg(test)]
mod _845_test;

mod _864_doc;
pub use _864_doc::*;
#[cfg(test)]
mod _864_test;

mod _866_doc;
pub use _866_doc::*;
#[cfg(test)]
mod _866_test;

mod _865_doc;
pub use _865_doc::*;
#[cfg(test)]
mod _865_test;

mod _867_doc;
pub use _867_doc::*;
#[cfg(test)]
mod _867_test;

mod _869_doc;
pub use _869_doc::*;
#[cfg(test)]
mod _869_test;

mod _870_doc;
pub use _870_doc::*;
#[cfg(test)]
mod _870_test;

mod _875_doc;
pub use _875_doc::*;
#[cfg(test)]
mod _875_test;

mod _888_doc;
pub use _888_doc::*;
#[cfg(test)]
mod _888_test;

mod _889_doc;
pub use _889_doc::*;
#[cfg(test)]
mod _889_test;

mod _880_doc;
pub use _880_doc::*;
#[cfg(test)]
mod _880_test;

#[cfg(test)]
mod element_test;
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
