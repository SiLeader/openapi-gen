use std::collections::HashMap;

use crate::data::Value;
use crate::ReferenceOr;

#[derive(Debug, Clone)]
pub struct Schema {
    pub name: Option<String>,
    pub content: SchemaContent,
}

#[derive(Debug, Clone)]
pub enum SchemaContent {
    Typedef(Box<TypeWithAttributes>),
    Definition {
        fields: Vec<SchemaField>,
        attributes: Attributes,
    },
}

#[derive(Debug, Clone)]
pub struct SchemaField {
    pub name: String,
    pub requirement: Requirement,
    pub target_type: TypeWithAttributes,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum Requirement {
    #[default]
    Required,
    Optional,
}

#[derive(Debug, Clone)]
pub struct TypeWithAttributes {
    pub target_type: Type,
    pub attributes: Attributes,
}

#[derive(Debug, Clone)]
pub enum Type {
    Integer { format: Option<String> },
    String { format: Option<String> },
    Float,
    Bool,
    List { item_type: Box<TypeWithAttributes> },
    Object,
    Schema(ReferenceOr<SchemaContent>),
    Enum(EnumContent),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Int(i64),
    Bool(bool),
    Float(f64),
    List(Vec<Literal>),
}

pub type Attributes = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub content: EnumContent,
}

#[derive(Debug, Clone)]
pub struct EnumContent {
    pub selection: Vec<String>,
    pub attributes: Attributes,
}

impl Type {
    pub(crate) fn int32() -> Self {
        Type::Integer {
            format: Some("int32".to_string()),
        }
    }

    pub(crate) fn int64() -> Self {
        Type::Integer {
            format: Some("int64".to_string()),
        }
    }

    pub(crate) fn string() -> Self {
        Type::String { format: None }
    }

    pub(crate) fn int() -> Self {
        Type::Integer { format: None }
    }

    pub(crate) fn datetime() -> Self {
        Type::String {
            format: Some("date-time".to_string()),
        }
    }

    pub(crate) fn date() -> Self {
        Type::String {
            format: Some("date".to_string()),
        }
    }

    pub(crate) fn time() -> Self {
        Type::String {
            format: Some("time".to_string()),
        }
    }

    pub(crate) fn duration() -> Self {
        Type::String {
            format: Some("duration".to_string()),
        }
    }

    pub(crate) fn email() -> Self {
        Type::String {
            format: Some("email".to_string()),
        }
    }

    pub(crate) fn uuid() -> Self {
        Type::String {
            format: Some("uuid".to_string()),
        }
    }

    pub(crate) fn uri() -> Self {
        Type::String {
            format: Some("uri".to_string()),
        }
    }
}
