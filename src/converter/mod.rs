use crate::converter::object::type_with_attributes;
use crate::openapi::MediaType;
use parser::TypeWithAttributes;
use std::collections::HashMap;

mod object;
mod path;
mod request_body;
mod response;
mod schema;

pub(crate) use object::generate;

fn media_content(twa: &TypeWithAttributes) -> HashMap<String, MediaType> {
    let mut hm = HashMap::new();
    hm.insert("application/json".to_string(), media_type(&twa));
    hm
}

fn media_type(twa: &TypeWithAttributes) -> MediaType {
    MediaType {
        schema: type_with_attributes(twa),
    }
}
