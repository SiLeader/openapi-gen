use crate::converter::media_content;
use crate::converter::object::{attributes, ToReferenceOr};
use crate::converter::response::parameter;
use crate::converter::schema::schema_content;
use crate::openapi::{ParameterIn, Paths, ReferenceOr};
use parser::{
    Operation, Parameter, ParameterType, Parameters, Path, Response, ResponseContent, SchemaContent,
};
use std::collections::HashMap;

pub(super) fn path_values(paths: Vec<Path>) -> Paths {
    let mut ps = HashMap::new();

    for p in paths {
        ps.insert(p.name.clone(), path(p));
    }

    Paths { content: ps }
}

fn path(path: Path) -> crate::openapi::Path {
    crate::openapi::Path {
        parameters: parameters(path.content.parameters),
        get: operation(path.content.get),
        post: operation(path.content.post),
        put: operation(path.content.put),
        delete: operation(path.content.delete),
        options: operation(path.content.options),
        head: operation(path.content.head),
        patch: operation(path.content.patch),
        trace: operation(path.content.trace),
    }
}

fn parameters(
    parameters: Parameters,
) -> Vec<crate::openapi::ReferenceOr<crate::openapi::Parameter>> {
    parameters
        .parameters
        .into_iter()
        .map(|p| parameter(&p, None))
        .collect()
}

fn operation(operation: Option<Operation>) -> Option<crate::openapi::Operation> {
    match operation {
        None => None,
        Some(op) => Some(crate::openapi::Operation {
            operation_id: op.name,
            tags: vec![],
            parameters: parameters(op.parameters),
            request_body: None,
            responses: crate::openapi::Responses {
                default: op.content.default.map(|r| r.to_reference_or()),
                code: op
                    .content
                    .response
                    .into_iter()
                    .map(|(name, r)| (name, r.to_reference_or()))
                    .collect(),
            },
            attributes: Default::default(),
        }),
    }
}

fn response(response: &ResponseContent) -> ReferenceOr<crate::openapi::Response> {
    ReferenceOr::Value(crate::openapi::Response {
        headers: response
            .headers
            .parameters
            .iter()
            .map(|p| (p.name.clone(), parameter(p, Some(ParameterType::Header))))
            .collect(),
        content: media_content(&response.content),
        attributes: attributes(&response.attributes),
    })
}

impl ToReferenceOr for parser::ReferenceOr<ResponseContent> {
    type Output = crate::openapi::Response;

    fn to_reference_or(&self) -> ReferenceOr<Self::Output> {
        match self {
            parser::ReferenceOr::Ref(r) => ReferenceOr::Ref {
                ref_path: format!("#/components/responses/{r}"),
            },
            parser::ReferenceOr::Value(v) => response(v),
        }
    }
}
