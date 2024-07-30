use serde::de::DeserializeOwned;
use tracing::trace;

use crate::{Error, Result};

/// 从 API 接口返回的 JSON 数据中构造结果。当 errcode: 0，返回 `Ok(T)`，非 0 则返回 `Err(Error::WeworkError)`，
pub fn extract<T>(json: serde_json::Value) -> Result<T>
where
    T: DeserializeOwned,
{
    let errcode = json.get("errcode").and_then(|v| v.as_i64()).map(|i| i as i32).unwrap_or(400);
    let errmsg = json.get("errmsg").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    trace!("extract original json is {}", json);
    if errcode == 0 {
        let v = serde_json::from_value::<T>(json)?;
        Ok(v)
    } else {
        Err(Error::WeworkError { errcode, errmsg, json })
    }
}
