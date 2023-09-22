use crate::v004010::Transmission;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while;
use nom::character::complete::newline;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

pub mod dt;
pub mod tm;

pub fn is_equal_payload<T: PartialEq>(src: &Transmission<T>, target: &Transmission<T>) -> bool {
    let src_group = &src.functional_group;
    for src_item in src_group {
        let x = src_item.eq(target.functional_group.get(0).unwrap());
        if !x {
            return false;
        }
    }
    true
}

pub fn parse_line<'a>(input: &'a str, segment_name: &str) -> IResult<&'a str, Vec<&'a str>> {
    let tag_name = format!("{segment_name}*");
    let (rest, vars) = delimited(tag(tag_name.as_str()), take_until("~"), tag("~"))(input)?;
    let (_, vars) = separated_list0(
        tag("*"),
        take_while(|x: char| {
            x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
        }),
    )(vars)?;
    // look for trailing newline
    let (rest, _) = opt(newline)(rest)?;
    Ok((rest, vars))
}

pub trait Parser<I, O, E> {
    fn parse(str: I) -> IResult<I, O>;
}
