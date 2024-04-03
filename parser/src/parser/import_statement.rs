use nom::bytes::complete::tag;
use nom::character::streaming::multispace1;
use nom::IResult;

use crate::data::Import;
use crate::parser::literals::string_literal;

pub(super) fn import_statement(s: &str) -> IResult<&str, Import> {
    let (s, _) = tag("import")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, file) = string_literal(s)?;

    Ok((
        s,
        Import {
            file: file.to_string(),
        },
    ))
}
