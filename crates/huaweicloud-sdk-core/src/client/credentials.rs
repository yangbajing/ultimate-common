use serde::Deserialize;
use typed_builder::TypedBuilder;

use crate::model::UserPassword;

#[derive(Debug, Deserialize, TypedBuilder)]
pub struct Credentials {
    #[builder(setter(into))]
    pub ak: String,

    #[builder(setter(into))]
    pub sk: String,

    #[builder(default, setter(into))]
    pub endpoint: Option<String>,

    #[builder(default, setter(into))]
    pub iam_endpoint: Option<String>,

    #[builder(default, setter(into))]
    pub user_password: Option<UserPassword>,

    #[builder(default, setter(into))]
    pub secrity_token: Option<String>,

    #[builder(default = Some(900))]
    #[serde(default = "expired_at_default")]
    pub expired_at: Option<i64>,

    #[builder(default, setter(into))]
    pub auth_token: Option<String>,

    #[builder(default, setter(into))]
    pub region_id: Option<String>,

    #[builder(default, setter(into))]
    pub domain_id: Option<String>,

    #[builder(default, setter(into))]
    pub project_id: Option<String>,
}

impl Credentials {
    pub fn has_secrity_token(&self) -> bool {
        match self.secrity_token() {
            Some(st) => !st.is_empty(),
            _ => false,
        }
    }

    pub fn ak(&self) -> &str {
        &self.ak
    }
    pub fn sk(&self) -> &str {
        &self.sk
    }

    pub fn endpoint(&self) -> Option<&str> {
        self.endpoint.as_deref()
    }

    pub fn user_password(&self) -> Option<&UserPassword> {
        self.user_password.as_ref()
    }

    pub fn secrity_token(&self) -> Option<&str> {
        self.secrity_token.as_deref()
    }

    pub fn iam_endpoint(&self) -> Option<&str> {
        self.iam_endpoint.as_deref()
    }
    pub fn expired_at(&self) -> Option<&i64> {
        self.expired_at.as_ref()
    }
    pub fn auth_token(&self) -> Option<&str> {
        self.auth_token.as_deref()
    }
    pub fn region_id(&self) -> Option<&str> {
        self.region_id.as_deref()
    }

    pub fn project_id(&self) -> Option<&str> {
        self.project_id.as_deref()
    }
}

pub trait BasicCredential {
    #[allow(dead_code)]
    fn project_id(&self) -> Option<&str>;
}

pub trait GlobalCredential {
    #[allow(dead_code)]
    fn domain_id(&self) -> Option<&str>;
}

impl BasicCredential for Credentials {
    fn project_id(&self) -> Option<&str> {
        self.project_id.as_deref()
    }
}

impl GlobalCredential for Credentials {
    fn domain_id(&self) -> Option<&str> {
        self.domain_id.as_deref()
    }
}

fn expired_at_default() -> Option<i64> {
    Some(900)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_credentials_builder() {
        let b = Credentials::builder();
        let bc = b.ak("").sk("").build();
        assert_eq!(bc.ak(), "");
        assert_eq!(bc.auth_token(), None);
        assert_eq!(bc.expired_at(), Some(&900))
    }

    #[test]
    fn test_basic_credentials_json() -> anyhow::Result<()> {
        let bc: Credentials = serde_json::from_str(r#"{"ak":"abc","sk":""}"#)?;
        assert_eq!(bc.ak(), "abc");
        assert_eq!(bc.auth_token(), None);
        assert_eq!(bc.expired_at(), Some(&900));

        Ok(())
    }
}
