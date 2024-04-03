use crate::data::schema::TypeWithAttributes;
use crate::data::Attributes;
use crate::Parameters;

#[derive(Debug, Clone)]
pub struct Response {
    pub name: String,
    pub content: ResponseContent,
}

#[derive(Debug, Clone)]
pub struct ResponseContent {
    pub attributes: Attributes,
    pub headers: Parameters,
    pub content: TypeWithAttributes,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub schema: TypeWithAttributes,
}
