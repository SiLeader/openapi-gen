use crate::parser::definition_head;
use crate::parser::identifier::identifier;
use crate::parser::literals::string_literal;
use crate::Info;
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::opt;
use nom::sequence::{delimited, preceded};
use nom::IResult;

pub(super) fn info_definition(s: &str) -> IResult<&str, Info> {
    let (s, default_spec) = opt(default_spec)(s)?;
    let (s, name) = definition_head("info")(s)?;
    let (s, base_info) = opt(extends)(s)?;
    let (s, _) = multispace0(s)?;
    delimited(
        char('{'),
        delimited(
            multispace0,
            info_content(default_spec.is_some(), name, base_info),
            multispace0,
        ),
        char('}'),
    )(s)
}

fn extends(s: &str) -> IResult<&str, String> {
    alt((extends_colon, extends_reserved_word))(s)
}

fn extends_colon(s: &str) -> IResult<&str, String> {
    let (s, _) = multispace0(s)?;
    let (s, _) = char(':')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, base) = identifier(s)?;

    Ok((s, base))
}

fn extends_reserved_word(s: &str) -> IResult<&str, String> {
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("extends")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, base) = identifier(s)?;

    Ok((s, base))
}

fn info_content(
    is_default: bool,
    config_name: String,
    base: Option<String>,
) -> impl Fn(&str) -> IResult<&str, Info> {
    move |s: &str| {
        let (s, (title, summary, description, version)) = permutation((
            assignment_option("title"),
            assignment_option("summary"),
            assignment_option("description"),
            assignment_option("version"),
        ))(s)?;
        Ok((
            s,
            Info {
                is_default,
                config_name: config_name.to_string(),
                title,
                summary,
                description,
                terms_of_service: None,
                version,
                base: base.clone(),
            },
        ))
    }
}

fn assignment_option(name: &'static str) -> impl Fn(&str) -> IResult<&str, Option<String>> {
    move |s: &str| opt(preceded(tag(name), tag_with_assignment))(s)
}

fn tag_with_assignment(s: &str) -> IResult<&str, String> {
    let (s, _) = delimited(multispace0, char('='), multispace0)(s)?;
    let (s, value) = string_literal(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, value.to_string()))
}

fn default_spec(s: &str) -> IResult<&str, ()> {
    let (s, _) = tag("default")(s)?;
    let (s, _) = multispace1(s)?;

    Ok((s, ()))
}
