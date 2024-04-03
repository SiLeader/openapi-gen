use crate::parser::identifier::identifier;
use crate::parser::literals::string_literal;
use crate::parser::parameter_definition::inferred_shorthand_parameter_definition;
use crate::parser::{shorthand_definition_head, wrapper_to_string};
use crate::{Parameter, ParameterType, Parameters};
use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

pub(in crate::parser) fn parameters_definition(
    context_type: ParameterType,
) -> impl Fn(&str) -> IResult<&str, Parameters> {
    move |s: &str| {
        let (s, _) = shorthand_definition_head("parameters")(s)?;
        let (s, _) = multispace0(s)?;
        delimited(
            char('{'),
            delimited(
                multispace0,
                parameters_content(context_type.clone()),
                multispace0,
            ),
            char('}'),
        )(s)
    }
}

fn parameters_content(context_type: ParameterType) -> impl Fn(&str) -> IResult<&str, Parameters> {
    move |s: &str| {
        let (s, parameters) =
            separated_list0(char(','), parameters_content_inner(context_type.clone()))(s)?;
        let (s, _) = opt(char(','))(s)?;

        Ok((s, Parameters { parameters }))
    }
}

fn parameters_content_inner(
    context_type: ParameterType,
) -> impl Fn(&str) -> IResult<&str, Parameter> {
    move |s: &str| {
        let (s, _) = multispace0(s)?;
        let (s, name) = alt((wrapper_to_string(string_literal), identifier))(s)?;
        let (s, _) = delimited(multispace0, char(':'), multispace0)(s)?;
        let (s, content) = inferred_shorthand_parameter_definition(context_type.clone())(s)?;
        let (s, _) = multispace0(s)?;

        Ok((
            s,
            Parameter {
                name,
                ty: Some(context_type.clone()),
                content,
            },
        ))
    }
}
