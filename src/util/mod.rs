use nom::IResult;

use crate::v004010::Transmission;

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

pub trait Parser<I,O,E>{
    fn parse(str: I) -> IResult<I, O>;
}