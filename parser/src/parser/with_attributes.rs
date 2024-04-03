use crate::data::{Attributes, Value};
use crate::parser::identifier::identifier;
use crate::parser::value::value;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, multispace1};
use nom::multi::many0;
use nom::IResult;
use std::collections::HashMap;

pub(super) fn with_attributes(s: &str) -> IResult<&str, Attributes> {
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("with")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, first_attribute) = attribute(s)?;
    let (s, attributes) = many0(comma_attribute)(s)?;

    let attributes = {
        let mut map = HashMap::from_iter(attributes.into_iter());
        map.insert(first_attribute.0, first_attribute.1);
        map
    };

    Ok((s, attributes))
}

fn comma_attribute(s: &str) -> IResult<&str, (String, Value)> {
    let (s, _) = multispace0(s)?;
    let (s, _) = char(',')(s)?;
    let (s, _) = multispace0(s)?;
    attribute(s)
}

fn attribute(s: &str) -> IResult<&str, (String, Value)> {
    let (s, identifier) = identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('=')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, value) = value(s)?;

    Ok((s, (identifier.to_string(), value)))
}
