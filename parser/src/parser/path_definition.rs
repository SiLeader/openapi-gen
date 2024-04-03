use nom::character::complete::{char, multispace0};
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::IResult;

use crate::data::Path;
use crate::parser::literals::string_literal;
use crate::parser::path_definition::operation::path_content;
use crate::parser::with_attributes::with_attributes;
use crate::parser::{definition_head_with_parser, wrapper_to_string};

mod operation;

pub(super) fn path_definition(s: &str) -> IResult<&str, Path> {
    let (s, name) = definition_head_with_parser("path", wrapper_to_string(string_literal))(s)?;
    let (s, attributes) = opt(with_attributes)(s)?;
    let (s, _) = multispace0(s)?;

    let (s, content) = delimited(
        char('{'),
        delimited(multispace0, path_content, multispace0),
        char('}'),
    )(s)?;

    Ok((
        s,
        Path {
            name,
            attributes: attributes.unwrap_or_default(),
            content,
        },
    ))
}
