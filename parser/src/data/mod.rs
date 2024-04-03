use std::fmt::Debug;

pub use import::*;
pub use info::*;
pub use parameter::*;
pub use path::*;
pub use request_body::*;
pub use response::*;
pub use schema::*;
pub use tag::*;

mod import;
mod info;
mod parameter;
mod path;
mod request_body;
mod response;
mod schema;
mod tag;

#[derive(Debug, Clone)]
pub enum ReferenceOr<T>
where
    T: Debug + Clone,
{
    Ref(String),
    Value(T),
}

#[derive(Debug, Clone)]
pub enum Object {
    Schema(Schema),
    Tag(Tag),
    Response(Response),
    Path(Path),
    RequestBody(RequestBody),
    Enum(Enum),
    Info(Info),
}

#[derive(Debug, Clone)]
pub enum Value {
    Immediate(Literal),
    Identifier(String),
}

#[derive(Debug, Clone)]
pub struct SourceFileContent {
    pub imports: Vec<Import>,
    pub objects: Vec<Object>,
}
