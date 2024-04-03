use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0};
use nom::combinator::opt;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::delimited;

use crate::data::Enum;
use crate::EnumContent;
use crate::parser::{definition_head, shorthand_definition_head, wrapper};
use crate::parser::identifier::identifier;
use crate::parser::literals::string_literal;
use crate::parser::with_attributes::with_attributes;

pub(super) fn enum_definition(s: &str) -> IResult<&str, Enum> {
    let (s, name) = definition_head("enum")(s)?;
    let (s, content) = enum_definition_impl(s)?;

    Ok((s, Enum { name, content }))
}

pub(super) fn shorthand_enum_definition(s: &str) -> IResult<&str, EnumContent> {
    let (s, _) = shorthand_definition_head("enum")(s)?;
    enum_definition_impl(s)
}

fn enum_definition_impl(s: &str) -> IResult<&str, EnumContent> {
    let (s, attributes) = opt(with_attributes)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, selection) = delimited(
        char('{'),
        delimited(multispace0, enum_contents, multispace0),
        char('}'),
    )(s)?;

    Ok((
        s,
        EnumContent {
            attributes: attributes.unwrap_or_default(),
            selection,
        },
    ))
}

fn enum_contents(s: &str) -> IResult<&str, Vec<String>> {
    let (s, c) = separated_list0(tag(","), enum_content)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = opt(char(','))(s)?;

    Ok((s, c.into_iter().filter(|cc| !cc.is_empty()).collect()))
}

fn enum_content(s: &str) -> IResult<&str, String> {
    let (s, _) = multispace0(s)?;
    let (s, c) = alt((wrapper(string_literal, |s| s.to_string()), identifier))(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, c))
}
