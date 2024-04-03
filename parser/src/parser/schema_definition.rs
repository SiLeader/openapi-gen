use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0};
use nom::character::streaming::multispace1;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

use crate::data::{Requirement, Schema, SchemaContent, SchemaField, Type, TypeWithAttributes};
use crate::parser::enum_definition::shorthand_enum_definition;
use crate::parser::identifier::identifier;
use crate::parser::with_attributes::with_attributes;
use crate::parser::{definition_head, shorthand_definition_head};
use crate::ReferenceOr;

pub(super) fn schema_definition(s: &str) -> IResult<&str, Schema> {
    let (s, name) = definition_head("schema")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, content) = alt((brace_schema_definition, assignment_schema_definition))(s)?;

    Ok((
        s,
        Schema {
            name: Some(name),
            content,
        },
    ))
}

pub(super) fn shorthand_schema_definition(s: &str) -> IResult<&str, Schema> {
    let (s, _) = shorthand_definition_head("schema")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, content) = alt((brace_schema_definition, assignment_schema_definition))(s)?;

    Ok((
        s,
        Schema {
            name: None,
            content,
        },
    ))
}

fn brace_schema_definition(s: &str) -> IResult<&str, SchemaContent> {
    let (s, attributes) = opt(with_attributes)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, fields) = delimited(
        char('{'),
        delimited(multispace0, brace_schema_content, multispace0),
        char('}'),
    )(s)?;

    Ok((
        s,
        SchemaContent::Definition {
            fields,
            attributes: attributes.unwrap_or_default(),
        },
    ))
}

fn brace_schema_content(s: &str) -> IResult<&str, Vec<SchemaField>> {
    let (s, res) = delimited(
        multispace0,
        separated_list0(tag(","), data_content),
        multispace0,
    )(s)?;

    let (s, _) = opt(char(','))(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, res))
}

fn data_content(s: &str) -> IResult<&str, SchemaField> {
    delimited(multispace0, data_content_impl, multispace0)(s)
}

fn data_content_impl(s: &str) -> IResult<&str, SchemaField> {
    let (s, name) = identifier(s)?;
    let (s, _) = delimited(multispace0, tag(":"), multispace0)(s)?;
    let (s, requirement_spec) = opt(requirement_spec)(s)?;
    let (s, ty) = data_type(s)?;

    Ok((
        s,
        SchemaField {
            name,
            requirement: requirement_spec.unwrap_or_default(),
            target_type: ty,
        },
    ))
}

pub(super) fn requirement_spec(s: &str) -> IResult<&str, Requirement> {
    let (s, requirement_spec) = alt((tag("required"), tag("optional")))(s)?;
    let (s, _) = multispace1(s)?;

    let req = match requirement_spec {
        "required" => Requirement::Required,
        "optional" => Requirement::Optional,
        _ => Requirement::default(),
    };

    Ok((s, req))
}

fn assignment_schema_definition(s: &str) -> IResult<&str, SchemaContent> {
    let (s, _) = char('=')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, dt) = data_type(s)?;

    Ok((s, SchemaContent::Typedef(Box::new(dt))))
}

pub(super) fn data_type(s: &str) -> IResult<&str, TypeWithAttributes> {
    alt((
        primitive_data_type("String", Type::string()),
        primitive_data_type("Object", Type::Object),
        primitive_data_type("Bool", Type::Bool),
        primitive_data_type("Int32", Type::int32()),
        primitive_data_type("Int64", Type::int64()),
        primitive_data_type("Float", Type::Float),
        primitive_data_type("Number", Type::Float),
        primitive_data_type("Int", Type::int()),
        primitive_data_type("DateTime", Type::datetime()),
        primitive_data_type("Date", Type::date()),
        primitive_data_type("Time", Type::time()),
        primitive_data_type("Duration", Type::duration()),
        primitive_data_type("Email", Type::email()),
        primitive_data_type("Uuid", Type::uuid()),
        primitive_data_type("Uri", Type::uri()),
        list_type,
        shorthand_schema_type,
        shorthand_enum_type,
        schema_name_type,
    ))(s)
}

fn primitive_data_type(
    ty: &'static str,
    out: Type,
) -> impl Fn(&str) -> IResult<&str, TypeWithAttributes> {
    move |s| {
        let (s, _) = tag(ty)(s)?;
        let (s, attributes) = opt(with_attributes)(s)?;
        let wta = TypeWithAttributes {
            target_type: out.clone(),
            attributes: attributes.unwrap_or_default(),
        };
        Ok((s, wta))
    }
}

fn schema_name_type(s: &str) -> IResult<&str, TypeWithAttributes> {
    let (s, identifier) = identifier(s)?;
    Ok((
        s,
        TypeWithAttributes {
            target_type: Type::Schema(ReferenceOr::Ref(identifier)),
            attributes: HashMap::new(),
        },
    ))
}

fn shorthand_schema_type(s: &str) -> IResult<&str, TypeWithAttributes> {
    let (s, schema) = shorthand_schema_definition(s)?;
    Ok((
        s,
        TypeWithAttributes {
            target_type: Type::Schema(ReferenceOr::Value(schema.content)),
            attributes: HashMap::new(),
        },
    ))
}

fn shorthand_enum_type(s: &str) -> IResult<&str, TypeWithAttributes> {
    let (s, en) = shorthand_enum_definition(s)?;
    Ok((
        s,
        TypeWithAttributes {
            target_type: Type::Enum(en),
            attributes: HashMap::new(),
        },
    ))
}

fn list_type(s: &str) -> IResult<&str, TypeWithAttributes> {
    let (s, _) = tag("List")(s)?;
    let (s, item_type) = delimited(
        multispace0,
        delimited(
            char('<'),
            delimited(multispace0, data_type, multispace0),
            char('>'),
        ),
        multispace0,
    )(s)?;
    let (s, attributes) = opt(with_attributes)(s)?;

    Ok((
        s,
        TypeWithAttributes {
            target_type: Type::List {
                item_type: Box::new(item_type),
            },
            attributes: attributes.unwrap_or_default(),
        },
    ))
}
