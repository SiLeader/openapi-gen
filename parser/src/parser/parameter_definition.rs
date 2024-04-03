use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0};
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::IResult;

use crate::parser::schema_definition::{data_type, requirement_spec};
use crate::parser::with_attributes::with_attributes;
use crate::{Parameter, ParameterContent, ParameterType, Requirement, TypeWithAttributes};

mod full;
mod parameters;
mod shorthand;

pub(super) use parameters::*;
pub(super) use shorthand::*;

fn parameter_definition_inner(
    name: String,
    ty: Option<ParameterType>,
) -> impl Fn(&str) -> IResult<&str, Parameter> {
    move |s: &str| {
        let (s, content) = parameter_content(s)?;

        Ok((
            s,
            Parameter {
                name: name.clone(),
                content,
                ty: ty.clone(),
            },
        ))
    }
}

fn parameter_content(s: &str) -> IResult<&str, ParameterContent> {
    let (s, attributes) = opt(with_attributes)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, (requirement, content)) = delimited(char('{'), parameter_content_inner, char('}'))(s)?;

    Ok((
        s,
        ParameterContent {
            content,
            requirement,
            attributes: attributes.unwrap_or_default(),
        },
    ))
}

fn parameter_content_inner(s: &str) -> IResult<&str, (Requirement, TypeWithAttributes)> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("content")(s)?;
    let (s, _) = delimited(multispace0, char('='), multispace0)(s)?;
    let (s, rs) = opt(requirement_spec)(s)?;
    let (s, content) = data_type(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, (rs.unwrap_or_default(), content)))
}
