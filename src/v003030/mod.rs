//! v003030 repesents all entities of the 003030 specification.

use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use x12_types_macros::DisplayX12;
mod segment;
pub use segment::*;

use crate::util::Parser;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Transmission<T> {
    pub isa: ISA,
    pub functional_group: Vec<FunctionalGroup<T>>,
    pub iea: IEA,
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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct FunctionalGroup<T> {
    pub gs: GS,
    pub segments: Vec<T>,
    pub ge: GE,
}

/// 998 - Set Cancellation NEW
///
/// This Draft Standard for Trial Use contains the format and establishes the data contents of the Set Cancellation Transaction Set (998) for use within the context of an Electronic Data Interchange (EDI) environment. The transaction set can be used to request the deletion of a previously transmitted transaction set and will indicate the reason for this action, such as diversion or cancelled bill.
///
/// POS | ID | NAME | REQ | MAX | REPEAT
/// ----|----|------|-----|-----|-------
/// 0010 | ST | Transaction Set Header | M | 1
/// 0020 | ZD | Transaction Set Deletion - ID, Reason, and Source | M | 1
/// 0030 | SE | Transaction Set Trailer | M | 1
#[derive(Serialize, Deserialize, Clone, Default, Debug, DisplayX12)]
pub struct _998 {
    pub st: ST,
    pub zd: ZD,
    pub se: SE,
}

impl<'a> Parser<&'a str, _998, nom::error::Error<&'a str>> for _998 {
    fn parse(input: &'a str) -> IResult<&'a str, _998> {
        let mut output = _998::default();
        let (input, obj) = ST::parse(input)?;
        output.st = obj;
        let (input, obj) = ZD::parse(input)?;
        output.zd = obj;
        let (input, obj) = SE::parse(input)?;
        output.se = obj;
        Ok((input, output))
    }
}
