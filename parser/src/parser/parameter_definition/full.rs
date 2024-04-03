use crate::parser::definition_head;
use crate::parser::parameter_definition::parameter_definition_inner;
use crate::{Parameter, ParameterType};
use nom::IResult;

#[allow(dead_code)]
pub(in crate::parser) fn parameter_definition_with_type(
    ty: ParameterType,
) -> impl Fn(&str) -> IResult<&str, Parameter> {
    move |s: &str| {
        let (s, name) = definition_head("parameter")(s)?;
        parameter_definition_inner(name, Some(ty.clone()))(s)
    }
}

#[allow(dead_code)]
pub(in crate::parser) fn parameter_definition(s: &str) -> IResult<&str, Parameter> {
    let (s, name) = definition_head("parameter")(s)?;
    parameter_definition_inner(name, None)(s)
}

#[allow(dead_code)]
pub(in crate::parser) fn query_definition(s: &str) -> IResult<&str, Parameter> {
    let (s, name) = definition_head("query")(s)?;
    parameter_definition_inner(name, Some(ParameterType::Query))(s)
}

#[allow(dead_code)]
pub(in crate::parser) fn path_parameter_definition(s: &str) -> IResult<&str, Parameter> {
    let (s, name) = definition_head("pathParameter")(s)?;
    parameter_definition_inner(name, Some(ParameterType::Path))(s)
}

#[allow(dead_code)]
pub(in crate::parser) fn header_definition(s: &str) -> IResult<&str, Parameter> {
    let (s, name) = definition_head("header")(s)?;
    parameter_definition_inner(name, Some(ParameterType::Header))(s)
}

#[allow(dead_code)]
pub(in crate::parser) fn cookie_definition(s: &str) -> IResult<&str, Parameter> {
    let (s, name) = definition_head("cookie")(s)?;
    parameter_definition_inner(name, Some(ParameterType::Cookie))(s)
}
