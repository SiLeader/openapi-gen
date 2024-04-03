use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0};
use nom::combinator::opt;
use nom::error::ErrorKind;
use nom::sequence::delimited;
use nom::IResult;

use crate::data::{Response, ResponseContent, TypeWithAttributes};
use crate::parser::parameter_definition::parameters_definition;
use crate::parser::schema_definition::data_type;
use crate::parser::with_attributes::with_attributes;
use crate::parser::{definition_head, opt_permutation, shorthand_definition_head};
use crate::{ParameterType, Parameters};

pub(super) fn response_definition(s: &str) -> IResult<&str, Response> {
    let (s, name) = definition_head("response")(s)?;
    let (s, content) = response_contents(s)?;

    Ok((s, Response { name, content }))
}

pub(super) fn shorthand_response_definition(s: &str) -> IResult<&str, ResponseContent> {
    let (s, _) = shorthand_definition_head("response")(s)?;
    response_contents(s)
}

fn response_contents(s: &str) -> IResult<&str, ResponseContent> {
    let (s, attributes) = opt(with_attributes)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, (headers, content)) = delimited(
        char('{'),
        delimited(multispace0, response_contents_impl, multispace0),
        char('}'),
    )(s)?;

    Ok((
        s,
        ResponseContent {
            attributes: attributes.unwrap_or_default(),
            headers,
            content,
        },
    ))
}

fn response_contents_impl(s: &str) -> IResult<&str, (Parameters, TypeWithAttributes)> {
    let (s, (ha, ca)) = opt_permutation((headers_assignment, content_assignment))(s)?;
    match ca {
        None => Err(nom::Err::Error(nom::error::Error::new(
            s,
            ErrorKind::Permutation,
        ))),
        Some(ca) => Ok((s, (ha.unwrap_or_default(), ca))),
    }
}

fn headers_assignment(s: &str) -> IResult<&str, Parameters> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("headers")(s)?;
    let (s, _) = delimited(multispace0, char('='), multispace0)(s)?;
    parameters_definition(ParameterType::Header)(s)
}

pub(super) fn content_assignment(s: &str) -> IResult<&str, TypeWithAttributes> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("content")(s)?;
    let (s, _) = delimited(multispace0, char('='), multispace0)(s)?;
    let (s, t) = data_type(s)?;
    let (s, _) = multispace0(s)?;
    Ok((s, t))
}
