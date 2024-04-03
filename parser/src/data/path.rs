use std::collections::HashMap;
use std::fmt::Debug;

use crate::data::parameter::Parameters;
use crate::data::response::ResponseContent;
use crate::data::{Attributes, RequestBodyContent};
use crate::ReferenceOr;

#[derive(Debug, Clone)]
pub struct Path {
    pub name: String,
    pub attributes: Attributes,
    pub content: PathContent,
}

#[derive(Debug, Clone, Default)]
pub struct PathContent {
    pub parameters: Parameters,
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub patch: Option<Operation>,
    pub head: Option<Operation>,
    pub trace: Option<Operation>,
}

#[derive(Debug, Clone)]
pub enum HttpMethod {
    Default,
    Get,
    Post,
    Put,
    Delete,
    Options,
    Head,
    Trace,
    Patch,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub name: String,
    pub method: HttpMethod,
    pub parameters: Parameters,
    pub content: OperationContent,
    pub request_body: Option<ReferenceOr<RequestBodyContent>>,
}

#[derive(Debug, Clone, Default)]
pub struct OperationContent {
    pub default: Option<ReferenceOr<ResponseContent>>,
    pub response: HashMap<String, ReferenceOr<ResponseContent>>,
}
