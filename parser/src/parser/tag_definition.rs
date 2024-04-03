use nom::combinator::opt;
use nom::IResult;

use crate::data::Tag;
use crate::parser::definition_head;
use crate::parser::with_attributes::with_attributes;

pub(super) fn tag_definition(s: &str) -> IResult<&str, Tag> {
    let (s, name) = definition_head("tag")(s)?;
    let (s, attributes) = opt(with_attributes)(s)?;

    Ok((
        s,
        Tag {
            name,
            attributes: attributes.unwrap_or_default(),
        },
    ))
}
