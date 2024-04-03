use crate::converter::path::path_values;
use crate::converter::request_body::request_body;
use crate::converter::response::response;
use crate::converter::schema::{enum_content, schema};
use crate::openapi::{Components, Info, OpenApi};
use parser::{Attributes, Literal, Object, Path, Tag, Type, TypeWithAttributes, Value};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

struct InfoConfig {
    info: HashMap<String, parser::Info>,
    default: Option<String>,
}

pub(crate) fn generate(objects: Vec<Object>, config: Option<String>) -> OpenApi {
    let (components, tags, paths, info) = components(&objects);

    let paths = path_values(paths);

    OpenApi {
        openapi: "3.1.0".to_string(),
        info: resolve_info(&info, config),
        servers: vec![],
        paths,
        components,
        tags,
    }
}

fn components(
    objects: &Vec<Object>,
) -> (Components, Vec<crate::openapi::Tag>, Vec<Path>, InfoConfig) {
    let mut cs = Components::default();
    let mut tags = Vec::new();
    let mut paths = Vec::new();

    let mut info = HashMap::new();
    let mut default_info = None;

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
                cs.request_bodies.insert(r.name.clone(), request_body(r));
            }
            Object::Enum(e) => {
                cs.schemas.insert(e.name.clone(), enum_content(&e.content));
            }
            Object::Path(p) => {
                paths.push(p.clone());
            }
            Object::Info(i) => {
                if i.is_default {
                    default_info = Some(i.config_name.to_string());
                }
                info.insert(i.config_name.to_string(), i.clone());
            }
        }
    }

    (
        cs,
        tags,
        paths,
        InfoConfig {
            info,
            default: default_info,
        },
    )
}

fn tag(tag: &Tag) -> crate::openapi::Tag {
    crate::openapi::Tag {
        name: tag.name.clone(),
        attributes: attributes(&tag.attributes),
    }
}

fn resolve_info(info_config: &InfoConfig, config: Option<String>) -> Info {
    let config = config.or(info_config.default.clone()).unwrap();
    let using = info_config.info.get(&config).unwrap();
    let info = extends_info(&info_config, using.clone());

    Info {
        title: info.title.unwrap(),
        version: info.version.unwrap(),
        description: info.description,
        terms_of_service: info.terms_of_service,
    }
}

fn extends_info(info_config: &InfoConfig, info: parser::Info) -> parser::Info {
    match info.base {
        None => info,
        Some(base) => {
            let base = info_config.info.get(&base).unwrap().clone();
            let base = extends_info(&info_config, base);
            parser::Info {
                is_default: true,
                config_name: "".to_string(),
                title: info.title.or(base.title),
                version: info.version.or(base.version),
                description: info.description.or(base.description),
                terms_of_service: info.terms_of_service.or(base.terms_of_service),
                summary: info.summary.or(base.summary),
                base: None,
            }
        }
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
