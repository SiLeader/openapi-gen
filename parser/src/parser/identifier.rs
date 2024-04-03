use nom::bytes::complete::take_while;
use nom::IResult;

fn is_identifier_prefix(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier_body(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub(super) fn identifier(s: &str) -> IResult<&str, String> {
    let (s, prefix) = take_while(is_identifier_prefix)(s)?;
    let (s, body) = take_while(is_identifier_body)(s)?;

    Ok((s, prefix.to_string() + body))
}
