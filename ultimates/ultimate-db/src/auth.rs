use modql::field::Fields;
use serde::{Deserialize, Serialize};

#[derive(Fields)]
pub struct LoginBy {
    pub username: Option<String>,
    pub phone: Option<String>,
}

impl From<&LoginByPasswordReq> for LoginBy {
    fn from(req: &LoginByPasswordReq) -> Self {
        Self { username: req.username.clone(), phone: req.phone.clone() }
    }
}

#[derive(Deserialize)]
pub struct LoginByPasswordReq {
    pub username: Option<String>,
    pub phone: Option<String>,
    #[serde(skip_serializing)]
    pub pwd: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginResp {
    pub token: String,
}
