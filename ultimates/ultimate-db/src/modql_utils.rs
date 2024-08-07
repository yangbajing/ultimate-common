use serde::Deserialize;
use ultimate_common::time::UtcDateTime;

pub fn time_to_sea_value(json_value: serde_json::Value) -> modql::filter::SeaResult<sea_query::Value> {
    Ok(UtcDateTime::deserialize(json_value)?.into())
}

#[cfg(feature = "utoipa")]
pub fn op_vals_obj_schema() -> utoipa::openapi::Object {
    utoipa::openapi::    ObjectBuilder::new()
        .schema_type(utoipa::openapi::SchemaType::Object)
        .description(Some("支持的详细操作条件见：https://github.com/jeremychone/rust-modql?tab=readme-ov-file#opvaltype-conditional-operators"))
        .build()
}
