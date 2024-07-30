pub mod auth;
pub mod msg;
pub mod token;
pub mod user;
pub mod webhook;

use crate::{Error, Result};
use serde::{de::DeserializeOwned, Deserialize};

use self::user::UserGender;

#[derive(Debug, Deserialize)]
pub struct CorpUserInfo {
    pub userid: String,
    pub user_ticket: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NonCorpUserInfo {
    pub openid: String,
    pub external_userid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserDetailInfo {
    pub userid: String,
    pub gender: UserGender,
    pub avatar: String,
    pub qr_code: Option<String>,
    pub mobile: Option<u64>,
    pub email: Option<String>,
    pub biz_mail: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UserInfo {
    CorpUserInfo { userid: String, user_ticket: Option<String> },
    NonCorpUserInfo { openid: String, external_userid: Option<String> },
}

impl UserInfo {
    pub fn is_corp(&self) -> bool {
        match self {
            UserInfo::CorpUserInfo { .. } => true,
            UserInfo::NonCorpUserInfo { .. } => false,
        }
    }

    #[inline]
    pub fn is_non_corp(&self) -> bool {
        !self.is_corp()
    }
}

pub struct ApiResult {
    inner: serde_json::Value,
    errcode: i32,
    errmsg: String,
}

impl ApiResult {
    pub fn new(inner: serde_json::Value) -> Self {
        let errcode = inner.get("errcode").and_then(|v| v.as_i64()).map(|i| i as i32).unwrap_or(400);
        let errmsg = inner.get("errmsg").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        Self { inner, errcode, errmsg }
    }

    pub fn errcode(&self) -> i32 {
        self.errcode
    }

    pub fn errmsg(&self) -> &str {
        &self.errmsg
    }

    pub fn extract<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        if self.errcode() != 0 {
            return Err(Error::WeworkError { errcode: self.errcode, errmsg: self.errmsg, json: self.inner });
        }
        let v = serde_json::from_value::<T>(self.inner)?;
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::UserInfo;

    #[test]
    fn test_user_info() {
        let corp_json_text = r#"{
            "errcode": 400,
            "errmsg": "ddd",
            "userid": "userid"
        }"#;
        let non_corp_json_text = r#"{
            "errcode": 400,
            "errmsg": "ddd",
            "openid": "openid",
            "external_userid": "external_userid"
        }"#;
        let user_info = serde_json::from_str::<UserInfo>(corp_json_text).unwrap();
        println!("user info is {:?}", user_info);

        let user_info = serde_json::from_str::<UserInfo>(non_corp_json_text).unwrap();
        println!("user info is {:?}", user_info);
    }
}
