use serde::Deserialize;
use ultimate_common::time::UtcDateTime;

pub fn time_to_sea_value(json_value: serde_json::Value) -> modql::filter::SeaResult<sea_query::Value> {
    Ok(UtcDateTime::deserialize(json_value)?.into())
}

#[cfg(feature = "utoipa")]
pub fn op_vals_integer_schema() -> utoipa::openapi::Object {
    utoipa::openapi::ObjectBuilder::new()
        .schema_type(utoipa::openapi::SchemaType::Object)
        .description(Some("opvalfloat64"))
        .build()
}

#[cfg(feature = "utoipa")]
pub fn op_vals_string_schema() -> utoipa::openapi::Object {
    utoipa::openapi::ObjectBuilder::new()
        .schema_type(utoipa::openapi::SchemaType::String)
        .description(Some("https://github.com/jeremychone/rust-modql?tab=readme-ov-file#opvalstring-operators"))
        .build()
}

#[cfg(feature = "utoipa")]
pub fn op_vals_bool_schema() -> utoipa::openapi::Object {
    utoipa::openapi::ObjectBuilder::new()
        .schema_type(utoipa::openapi::SchemaType::Boolean)
        .description(Some("https://github.com/jeremychone/rust-modql?tab=readme-ov-file#opvalbool-operators"))
        .build()
}
