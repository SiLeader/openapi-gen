use crate::converter::media_content;
use crate::converter::object::{attributes, type_with_attributes};
use crate::converter::schema::schema;
use crate::openapi::{MediaType, ParameterIn};
use parser::{
    Parameter, ParameterType, Requirement, Response, ResponseContent, TypeWithAttributes,
};
use std::collections::HashMap;

pub(super) fn response(res: &Response) -> crate::openapi::Response {
    response_content(&res.content)
}

fn response_content(content: &ResponseContent) -> crate::openapi::Response {
    crate::openapi::Response {
        headers: HashMap::from_iter(
            content
                .headers
                .parameters
                .iter()
                .map(|p| (p.name.clone(), parameter(p, None))),
        ),
        content: media_content(&content.content),
        attributes: attributes(&content.attributes),
    }
}

pub(super) fn parameter(
    param: &Parameter,
    pt: Option<ParameterType>,
) -> crate::openapi::ReferenceOr<crate::openapi::Parameter> {
    crate::openapi::ReferenceOr::Value(crate::openapi::Parameter {
        name: param.name.clone(),
        parameter_in: match &param.ty {
            None => match pt {
                None => panic!("Unknown type of parameter type. It may be generator bug"),
                Some(p) => match p {
                    ParameterType::Query => ParameterIn::Query,
                    ParameterType::Header => ParameterIn::Header,
                    ParameterType::Path => ParameterIn::Path,
                    ParameterType::Cookie => ParameterIn::Cookie,
                },
            },
            Some(p) => match p {
                ParameterType::Query => ParameterIn::Query,
                ParameterType::Header => ParameterIn::Header,
                ParameterType::Path => ParameterIn::Path,
                ParameterType::Cookie => ParameterIn::Cookie,
            },
        },
        required: param.content.requirement == Requirement::Required,
        schema: type_with_attributes(&param.content.content),
        attributes: attributes(&param.content.attributes),
    })
}
