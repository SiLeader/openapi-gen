use crate::converter::object::{attributes, type_with_attributes, ToReferenceOr};
use parser::{EnumContent, ReferenceOr, Requirement, Schema, SchemaContent, TypeWithAttributes};
use std::collections::HashMap;

pub(super) fn schema(schema: &Schema) -> crate::openapi::ReferenceOr<crate::openapi::Schema> {
    schema_content(&schema.content)
}

pub(super) fn schema_content(
    content: &SchemaContent,
) -> crate::openapi::ReferenceOr<crate::openapi::Schema> {
    match &content {
        SchemaContent::Typedef(ty) => type_with_attributes(&ty),
        SchemaContent::Definition {
            fields,
            attributes: attr,
        } => crate::openapi::ReferenceOr::Value(crate::openapi::Schema::Object {
            required: fields
                .iter()
                .filter(|f| f.requirement == Requirement::Required)
                .map(|f| f.name.clone())
                .collect(),
            properties: HashMap::from_iter(
                fields
                    .iter()
                    .map(|f| (f.name.clone(), type_with_attributes(&f.target_type))),
            ),
            attributes: attributes(attr),
        }),
    }
}

pub(super) fn enum_content(
    content: &EnumContent,
) -> crate::openapi::ReferenceOr<crate::openapi::Schema> {
    crate::openapi::ReferenceOr::Value(crate::openapi::Schema::String {
        format: None,
        selection: Some(content.selection.clone()),
        attributes: attributes(&content.attributes),
    })
}

impl ToReferenceOr for ReferenceOr<SchemaContent> {
    type Output = crate::openapi::Schema;

    fn to_reference_or(&self) -> crate::openapi::ReferenceOr<Self::Output> {
        match self {
            ReferenceOr::Ref(r) => crate::openapi::ReferenceOr::Ref {
                ref_path: format!("#/components/schemas/{r}"),
            },
            ReferenceOr::Value(v) => schema_content(v),
        }
    }
}
