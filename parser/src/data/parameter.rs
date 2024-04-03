use crate::data::TypeWithAttributes;
use crate::{Attributes, Requirement};

#[derive(Debug, Clone, Default)]
pub struct Parameters {
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub ty: Option<ParameterType>,
    pub content: ParameterContent,
}

#[derive(Debug, Clone)]
pub enum ParameterType {
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Debug, Clone)]
pub struct ParameterContent {
    pub content: TypeWithAttributes,
    pub requirement: Requirement,
    pub attributes: Attributes,
}
