use crate::data::schema::Attributes;

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub attributes: Attributes,
}
