use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub errcode: i32,
    pub errmsg: String,
    pub access_token: String,
    pub expires_in: u64,
}
