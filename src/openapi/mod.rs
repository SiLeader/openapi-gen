use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize)]
pub struct OpenApi {
    pub openapi: String,
    pub info: Info,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    pub paths: Paths,
    pub components: Components,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

pub type Attributes = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, Serialize)]
pub struct Info {
    pub title: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Server {
    pub url: String,
    #[serde(flatten)]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Serialize)]
pub struct Paths {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub content: HashMap<String, Path>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub schemas: HashMap<String, ReferenceOr<Schema>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub responses: HashMap<String, Response>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, Parameter>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub request_body: HashMap<String, ReferenceOr<RequestBody>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, Parameter>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub name: String,
    #[serde(flatten)]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Serialize)]
pub struct Path {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ReferenceOr<Parameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub operation_id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ReferenceOr<Parameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBody>,
    pub responses: Responses,
    #[serde(flatten)]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Serialize)]
pub struct Responses {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<ReferenceOr<Response>>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub code: HashMap<String, ReferenceOr<Response>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Schema {
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,

        #[serde(rename = "enum")]
        #[serde(skip_serializing_if = "Option::is_none")]
        selection: Option<Vec<String>>,
        #[serde(flatten)]
        attributes: Attributes,
    },
    Object {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        required: Vec<String>,
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        properties: HashMap<String, ReferenceOr<Schema>>,
        #[serde(flatten)]
        attributes: Attributes,
    },
    Integer {
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(flatten)]
        attributes: Attributes,
    },
    Boolean {
        #[serde(flatten)]
        attributes: Attributes,
    },
    Array {
        items: Box<ReferenceOr<Schema>>,
        #[serde(flatten)]
        attributes: Attributes,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct Response {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, ReferenceOr<Parameter>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub content: HashMap<String, MediaType>,
    #[serde(flatten)]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Serialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub parameter_in: ParameterIn,
    pub required: bool,
    pub schema: ReferenceOr<Schema>,
    #[serde(flatten)]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ReferenceOr<T>
where
    T: Debug + Clone + Serialize,
{
    Ref {
        #[serde(rename = "$ref")]
        ref_path: String,
    },
    Value(T),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ParameterIn {
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestBody {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub content: HashMap<String, MediaType>,
    #[serde(flatten)]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Serialize)]
pub struct MediaType {
    pub schema: ReferenceOr<Schema>,
}
