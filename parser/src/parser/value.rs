use crate::data::Value;
use crate::parser::identifier::identifier;
use crate::parser::literals::literal;
use crate::parser::wrapper;
use nom::branch::alt;
use nom::IResult;

pub(super) fn value(s: &str) -> IResult<&str, Value> {
    alt((
        wrapper(literal, Value::Immediate),
        wrapper(identifier, Value::Identifier),
    ))(s)
}
