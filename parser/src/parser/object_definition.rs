use nom::branch::alt;
use nom::IResult;

use crate::data::Object;
use crate::parser::enum_definition::enum_definition;
use crate::parser::info_definition::info_definition;
use crate::parser::path_definition::path_definition;
use crate::parser::request_body_definition::request_body_definition;
use crate::parser::response_definition::response_definition;
use crate::parser::schema_definition::schema_definition;
use crate::parser::tag_definition::tag_definition;
use crate::parser::wrapper;

pub(super) fn object_definition(s: &str) -> IResult<&str, Object> {
    alt((
        wrapper(schema_definition, Object::Schema),
        wrapper(enum_definition, Object::Enum),
        wrapper(tag_definition, Object::Tag),
        wrapper(response_definition, Object::Response),
        wrapper(path_definition, Object::Path),
        wrapper(request_body_definition, Object::RequestBody),
        wrapper(info_definition, Object::Info),
    ))(s)
}
