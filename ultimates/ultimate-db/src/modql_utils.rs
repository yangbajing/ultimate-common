use serde::Deserialize;
use ultimate_common::time::UtcDateTime;

pub fn time_to_sea_value(json_value: serde_json::Value) -> modql::filter::SeaResult<sea_query::Value> {
    Ok(UtcDateTime::deserialize(json_value)?.into())
}
