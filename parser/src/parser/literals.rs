use crate::data::Literal;
use crate::parser::wrapper;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_until};
use nom::character::complete::{char, digit1, hex_digit1, oct_digit1};
use nom::error::{ErrorKind, ParseError};
use nom::sequence::delimited;
use nom::IResult;

pub(super) fn literal(s: &str) -> IResult<&str, Literal> {
    alt((
        wrapper(string_literal, |v| Literal::String(v.to_string())),
        wrapper(integer_literal, Literal::Int),
        wrapper(bool_literal, Literal::Bool),
    ))(s)
}

// string
pub(super) fn string_literal(s: &str) -> IResult<&str, &str> {
    alt((simple_string_literal, raw_string_literal))(s)
}

fn simple_string_literal(s: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"'))(s)
}

fn raw_string_literal(s: &str) -> IResult<&str, &str> {
    delimited(tag("r#\""), take_until("\"#"), tag("\"#"))(s)
}

// integer
pub(super) fn integer_literal(s: &str) -> IResult<&str, i64> {
    alt((
        binary_integer_literal,
        octal_integer_literal,
        hexadecimal_integer_literal,
        decimal_integer_literal,
    ))(s)
}

fn decimal_integer_literal(s: &str) -> IResult<&str, i64> {
    let (s, n) = digit1(s)?;
    let n = i64::from_str_radix(n, 10)
        .map_err(|_| nom::Err::Error(nom::error::Error::from_error_kind(s, ErrorKind::Digit)))?;

    Ok((s, n))
}

fn hexadecimal_integer_literal(s: &str) -> IResult<&str, i64> {
    let (s, _) = tag("0x")(s)?;
    let (s, n) = hex_digit1(s)?;
    let n = i64::from_str_radix(n, 16)
        .map_err(|_| nom::Err::Error(nom::error::Error::from_error_kind(s, ErrorKind::Digit)))?;

    Ok((s, n))
}

fn octal_integer_literal(s: &str) -> IResult<&str, i64> {
    let (s, _) = tag("0o")(s)?;
    let (s, n) = oct_digit1(s)?;
    let n = i64::from_str_radix(n, 8)
        .map_err(|_| nom::Err::Error(nom::error::Error::from_error_kind(s, ErrorKind::Digit)))?;

    Ok((s, n))
}

fn binary_integer_literal(s: &str) -> IResult<&str, i64> {
    let (s, _) = tag("0b")(s)?;
    let (s, n) = is_a("01")(s)?;
    let n = i64::from_str_radix(n, 2)
        .map_err(|_| nom::Err::Error(nom::error::Error::from_error_kind(s, ErrorKind::Digit)))?;

    Ok((s, n))
}

// bool
pub(super) fn bool_literal(s: &str) -> IResult<&str, bool> {
    let (s, v) = alt((tag("true"), tag("false")))(s)?;
    Ok((s, v == "true"))
}
