use crate::converter::path::path_values;
use crate::converter::request_body::request_body;
use crate::converter::response::response;
use crate::converter::schema::{enum_content, schema};
use crate::openapi::{Components, Info, OpenApi};
use parser::{Attributes, Literal, Object, Path, Tag, Type, TypeWithAttributes, Value};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

pub(crate) fn generate(objects: Vec<Object>) -> OpenApi {
    let (components, tags, paths) = components(&objects);

    let paths = path_values(paths);

    OpenApi {
        openapi: "3.1.0".to_string(),
        info: Info {
            title: "".to_string(),
            attributes: Default::default(),
        },
        servers: vec![],
        paths,
        components,
        tags,
    }
}

fn components(objects: &Vec<Object>) -> (Components, Vec<crate::openapi::Tag>, Vec<Path>) {
    let mut cs = Components::default();
    let mut tags = Vec::new();
    let mut paths = Vec::new();

    for o in objects {
        match o {
            Object::Schema(s) => {
                cs.schemas.insert(s.name.clone().unwrap(), schema(s));
            }
            Object::Tag(t) => tags.push(tag(t)),
            Object::Response(r) => {
                cs.responses.insert(r.name.clone(), response(r));
            }
            Object::RequestBody(r) => {
                cs.request_body.insert(r.name.clone(), request_body(r));
            }
            Object::Enum(e) => {
                cs.schemas.insert(e.name.clone(), enum_content(&e.content));
            }
            Object::Path(p) => {
                paths.push(p.clone());
            }
        }
    }

    (cs, tags, paths)
}

fn tag(tag: &Tag) -> crate::openapi::Tag {
    crate::openapi::Tag {
        name: tag.name.clone(),
        attributes: attributes(&tag.attributes),
    }
}

pub(super) fn type_with_attributes(
    twa: &TypeWithAttributes,
) -> crate::openapi::ReferenceOr<crate::openapi::Schema> {
    match &twa.target_type {
        Type::Integer { format } => {
            crate::openapi::ReferenceOr::Value(crate::openapi::Schema::Integer {
                format: format.clone(),
                attributes: attributes(&twa.attributes),
            })
        }
        Type::String { format } => {
            crate::openapi::ReferenceOr::Value(crate::openapi::Schema::String {
                format: format.clone(),
                attributes: attributes(&twa.attributes),
                selection: None,
            })
        }
        Type::Float => {
            todo!()
        }
        Type::Bool => crate::openapi::ReferenceOr::Value(crate::openapi::Schema::Boolean {
            attributes: attributes(&twa.attributes),
        }),
        Type::List { item_type } => {
            crate::openapi::ReferenceOr::Value(crate::openapi::Schema::Array {
                items: Box::new(type_with_attributes(&item_type)),
                attributes: attributes(&twa.attributes),
            })
        }
        Type::Object => crate::openapi::ReferenceOr::Value(crate::openapi::Schema::Object {
            required: vec![],
            properties: HashMap::new(),
            attributes: attributes(&twa.attributes),
        }),
        Type::Schema(r) => r.to_reference_or(),
        Type::Enum(e) => enum_content(e),
    }
}

pub(super) fn attributes(attr: &Attributes) -> crate::openapi::Attributes {
    attr.iter()
        .map(|(key, value)| (key.clone(), value_to_json(value.clone())))
        .collect()
}

fn value_to_json(value: Value) -> serde_json::Value {
    match value {
        Value::Immediate(value) => literal_to_json(value),
        Value::Identifier(_) => serde_json::Value::Null,
    }
}

fn literal_to_json(literal: Literal) -> serde_json::Value {
    match literal {
        Literal::String(value) => serde_json::Value::from(value.as_str()),
        Literal::Int(value) => serde_json::Value::from(value),
        Literal::Bool(value) => serde_json::Value::from(value),
        Literal::Float(value) => serde_json::Value::from(value),
        Literal::List(value) => {
            serde_json::Value::from_iter(value.into_iter().map(|v| literal_to_json(v)))
        }
    }
}

pub(super) trait ToReferenceOr {
    type Output: Clone + Debug + Serialize;

    fn to_reference_or(&self) -> crate::openapi::ReferenceOr<Self::Output>;
}
