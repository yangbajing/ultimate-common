use modql::filter::{FilterNodes, OpValsString};
use serde::{Deserialize, Serialize};

#[derive(FilterNodes)]
pub struct LoginFilter {
    pub username: Option<OpValsString>,
    pub email: Option<OpValsString>,
    pub phone: Option<OpValsString>,
}

impl From<&LoginByPasswordReq> for LoginFilter {
    fn from(req: &LoginByPasswordReq) -> Self {
        Self {
            username: req.username.clone().map(OpValsString::from),
            email: req.email.clone().map(OpValsString::from),
            phone: req.phone.clone().map(OpValsString::from),
        }
    }
}

#[derive(Deserialize)]
pub struct LoginByPasswordReq {
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(skip_serializing)]
    pub pwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResp {
    pub token: String,
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Bearer,
}
