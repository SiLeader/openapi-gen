use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{all_consuming, opt};
use nom::multi::separated_list0;
use nom::IResult;

use crate::data::SourceFileContent;
use crate::parser::identifier::identifier;
use crate::parser::import_statement::import_statement;
use crate::parser::object_definition::object_definition;

mod enum_definition;
mod identifier;
mod import_statement;
mod info_definition;
mod literals;
mod object_definition;
mod parameter_definition;
mod path_definition;
mod request_body_definition;
mod response_definition;
mod schema_definition;
mod tag_definition;
mod value;
mod with_attributes;

pub fn parse(input: &str) -> Result<SourceFileContent, nom::Err<nom::error::Error<&str>>> {
    let (_, s) = parse_impl(input)?;
    Ok(s)
}

fn parse_impl(s: &str) -> IResult<&str, SourceFileContent> {
    all_consuming(source_file_content)(s)
}

fn source_file_content(s: &str) -> IResult<&str, SourceFileContent> {
    let (s, imports) = separated_list0(multispace1, import_statement)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, objects) = separated_list0(multispace1, object_definition)(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, SourceFileContent { imports, objects }))
}

fn wrapper<'a, O, F, Tr, O2>(f: F, t: Tr) -> impl Fn(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O2>,
    Tr: Fn(O2) -> O,
{
    move |s| {
        let (s, o) = f(s)?;
        Ok((s, t(o)))
    }
}

fn wrapper_to_string<F>(f: F) -> impl Fn(&str) -> IResult<&str, String>
where
    F: Fn(&str) -> IResult<&str, &str>,
{
    move |s| {
        let (s, o) = f(s)?;
        Ok((s, o.to_string()))
    }
}

fn definition_head_with_parser(
    type_name: &'static str,
    parser: impl Fn(&str) -> IResult<&str, String>,
) -> impl Fn(&str) -> IResult<&str, String> {
    move |s| {
        let (s, _) = shorthand_definition_head(type_name)(s)?;
        let (s, _) = multispace1(s)?;
        let (s, name) = parser(s)?;
        Ok((s, name))
    }
}

fn definition_head(type_name: &'static str) -> impl Fn(&str) -> IResult<&str, String> {
    definition_head_with_parser(type_name, identifier)
}

fn shorthand_definition_head(type_name: &'static str) -> impl Fn(&str) -> IResult<&str, ()> {
    move |s| {
        let (s, _) = tag(type_name)(s)?;
        Ok((s, ()))
    }
}

fn opt_permutation<P1, P2, O1, O2>(
    parsers: (P1, P2),
) -> impl Fn(&str) -> IResult<&str, (Option<O1>, Option<O2>)>
where
    P1: Fn(&str) -> IResult<&str, O1> + Copy,
    P2: Fn(&str) -> IResult<&str, O2> + Copy,
{
    let p1 = parsers.0;
    let p2 = parsers.1;
    move |s: &str| {
        let (s, o11) = opt(p1)(s)?;
        let (s, o2) = opt(p2)(s)?;
        let (s, o1) = match o11 {
            Some(o11) => (s, Some(o11)),
            None => opt(p1)(s)?,
        };

        Ok((s, (o1, o2)))
    }
}
