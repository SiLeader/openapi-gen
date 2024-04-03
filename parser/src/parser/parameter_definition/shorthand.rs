use crate::parser::parameter_definition::parameter_content;
use crate::parser::shorthand_definition_head;
use crate::{ParameterContent, ParameterType};
use nom::branch::alt;
use nom::IResult;

pub(in crate::parser) fn inferred_shorthand_parameter_definition(
    context_type: ParameterType,
) -> impl Fn(&str) -> IResult<&str, ParameterContent> {
    move |s: &str| {
        alt((
            shorthand_parameter_definition,
            match context_type.clone() {
                ParameterType::Query => shorthand_query_definition,
                ParameterType::Header => shorthand_header_definition,
                ParameterType::Path => shorthand_path_parameter_definition,
                ParameterType::Cookie => shorthand_cookie_definition,
            },
        ))(s)
    }
}

fn shorthand_parameter_definition(s: &str) -> IResult<&str, ParameterContent> {
    let (s, _) = shorthand_definition_head("parameter")(s)?;
    parameter_content(s)
}

fn shorthand_query_definition(s: &str) -> IResult<&str, ParameterContent> {
    let (s, _) = shorthand_definition_head("query")(s)?;
    parameter_content(s)
}

fn shorthand_path_parameter_definition(s: &str) -> IResult<&str, ParameterContent> {
    let (s, _) = shorthand_definition_head("pathParameter")(s)?;
    parameter_content(s)
}

fn shorthand_header_definition(s: &str) -> IResult<&str, ParameterContent> {
    let (s, _) = shorthand_definition_head("header")(s)?;
    parameter_content(s)
}

fn shorthand_cookie_definition(s: &str) -> IResult<&str, ParameterContent> {
    let (s, _) = shorthand_definition_head("cookie")(s)?;
    parameter_content(s)
}
