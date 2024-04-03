use crate::data::{RequestBody, RequestBodyContent};
use crate::parser::response_definition::content_assignment;
use crate::parser::with_attributes::with_attributes;
use crate::parser::{definition_head, shorthand_definition_head};
use nom::character::complete::{char, multispace0};
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::IResult;

pub(super) fn request_body_definition(s: &str) -> IResult<&str, RequestBody> {
    let (s, name) = definition_head("requestBody")(s)?;
    let (s, content) = request_body_content(s)?;

    Ok((s, RequestBody { name, content }))
}

pub(super) fn shorthand_request_body_definition(s: &str) -> IResult<&str, RequestBodyContent> {
    let (s, _) = shorthand_definition_head("requestBody")(s)?;
    let (s, content) = request_body_content(s)?;

    Ok((s, content))
}

fn request_body_content(s: &str) -> IResult<&str, RequestBodyContent> {
    let (s, attributes) = opt(with_attributes)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, content) = delimited(
        char('{'),
        delimited(multispace0, content_assignment, multispace0),
        char('}'),
    )(s)?;

    Ok((
        s,
        RequestBodyContent {
            content,
            attributes: attributes.unwrap_or_default(),
        },
    ))
}
