use crate::converter::media_content;
use crate::converter::object::attributes;
use parser::{RequestBody, RequestBodyContent};

pub(super) fn request_body(
    request_body: &RequestBody,
) -> crate::openapi::ReferenceOr<crate::openapi::RequestBody> {
    request_body_content(&request_body.content)
}

pub(super) fn request_body_content(
    content: &RequestBodyContent,
) -> crate::openapi::ReferenceOr<crate::openapi::RequestBody> {
    crate::openapi::ReferenceOr::Value(crate::openapi::RequestBody {
        content: media_content(&content.content),
        attributes: attributes(&content.attributes),
    })
}
