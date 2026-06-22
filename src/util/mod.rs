use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while;
use nom::character::complete::newline;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use nom::Parser as _;

pub mod dt;
pub mod tm;

/// Compare two transmissions by their functional-group payload.
///
/// Gated behind `v004010` because it references that version's `Transmission`
/// type; without this gate `util` would not compile when `v004010` is disabled.
#[cfg(feature = "v004010")]
pub fn is_equal_payload<T: PartialEq>(
    src: &crate::v004010::Transmission<T>,
    target: &crate::v004010::Transmission<T>,
) -> bool {
    let Some(target_first) = target.functional_group.first() else {
        return src.functional_group.is_empty();
    };
    src.functional_group
        .iter()
        .all(|item| item.eq(target_first))
}

pub fn parse_line<'a>(input: &'a str, segment_name: &str) -> IResult<&'a str, Vec<&'a str>> {
    let tag_name = format!("{segment_name}*");
    let (rest, vars) = delimited(tag(tag_name.as_str()), take_until("~"), tag("~")).parse(input)?;
    let (_, vars) = separated_list0(
        tag("*"),
        take_while(|x: char| {
            x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
        }),
    )
    .parse(vars)?;
    // look for trailing newline
    let (rest, _) = opt(newline).parse(rest)?;
    Ok((rest, vars))
}

pub trait Parser<I, O, E> {
    fn parse(str: I) -> IResult<I, O>;
}

pub fn unborrow_string(input: &&str) -> String {
    input.to_string()
}
