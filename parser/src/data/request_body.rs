use crate::data::schema::TypeWithAttributes;
use crate::data::Attributes;

#[derive(Debug, Clone)]
pub struct RequestBody {
    pub name: String,
    pub content: RequestBodyContent,
}

#[derive(Debug, Clone)]
pub struct RequestBodyContent {
    pub content: TypeWithAttributes,
    pub attributes: Attributes,
}
