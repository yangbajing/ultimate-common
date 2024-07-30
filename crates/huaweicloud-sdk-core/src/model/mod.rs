pub mod auth;
pub mod token;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Debug, Deserialize)]
pub struct Credential {
    pub access: String,
    pub expires_at: String,
    pub secret: String,
    pub securitytoken: String,
}

/// 认证信息。
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Auth {
    pub identity: Identity,
}

/// 认证参数。
#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct Identity {
    pub methods: Vec<IdentityMethod>,

    #[builder(default)]
    pub password: Option<IdentityPassword>,

    #[builder(default)]
    pub token: Option<IdentityToken>,

    #[builder(default)]
    pub policy: Option<IdentityPolicy>,
}

impl Default for Identity {
    fn default() -> Self {
        Self {
            methods: vec![IdentityMethod::TOKEN],
            password: Default::default(),
            token: Some(IdentityToken::duration_seconds(15 * 60)),
            policy: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IdentityPolicy {
    pub version: PolicyVersion,
    pub statement: Vec<PolicyStatement>,
}

impl Default for IdentityPolicy {
    fn default() -> Self {
        Self { version: PolicyVersion::V1_1, statement: Default::default() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PolicyVersion {
    #[serde(rename = "1.1")]
    V1_1,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyStatement {
    pub effect: Effect,
    pub action: Option<Vec<String>>,
    // 资源规则
    pub resource: Option<Vec<String>>,
    // TODO 用 linkedmap？
    pub condition: Option<HashMap<String, HashMap<String, Vec<String>>>>,
}

impl PolicyStatement {
    pub fn builder() -> PolicyStatementBuilder {
        PolicyStatementBuilder(PolicyStatement::default())
    }
}

pub struct PolicyStatementBuilder(PolicyStatement);

impl PolicyStatementBuilder {
    pub fn effect(mut self, effect: Effect) -> Self {
        self.0.effect = effect;
        self
    }

    pub fn append_action(mut self, v: impl Into<String>) -> Self {
        if let Some(action) = self.0.action.as_mut() {
            action.push(v.into());
        } else {
            self.0.action = Some(vec![v.into()]);
        }
        self
    }

    pub fn append_resource(mut self, v: impl Into<String>) -> Self {
        if let Some(resource) = self.0.resource.as_mut() {
            resource.push(v.into());
        } else {
            self.0.resource = Some(vec![v.into()]);
        }
        self
    }

    pub fn append_condition(mut self, k: impl Into<String>, vs: HashMap<String, Vec<String>>) -> Self {
        if let Some(condition) = self.0.condition.as_mut() {
            condition.insert(k.into(), vs);
        } else {
            let mut m = HashMap::new();
            m.insert(k.into(), vs);
            self.0.condition = Some(m);
        }
        self
    }

    pub fn build(self) -> PolicyStatement {
        PolicyStatement {
            effect: self.0.effect,
            action: self.0.action,
            resource: self.0.resource,
            condition: self.0.condition,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Effect {
    Allow,
    #[default]
    Deny,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IdentityMethod {
    TOKEN,
    PASSWORD,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityToken {
    // 即token，若请求Header中不传X-Auth-Token，则须填此参数。
    id: Option<String>,
    // 临时访问密钥和securitytoken的有效期，时间单位为秒。取值范围：15分钟 ~ 24小时 ，默认为15分钟。
    duration_seconds: Option<i32>,
}

impl Default for IdentityToken {
    fn default() -> Self {
        Self { id: None, duration_seconds: Some(15 * 60) }
    }
}
impl IdentityToken {
    pub fn new(id: impl Into<String>, duration_seconds: i32) -> Self {
        Self { id: Some(id.into()), duration_seconds: Some(duration_seconds) }
    }

    pub fn duration_seconds(duration_seconds: i32) -> Self {
        Self { id: None, duration_seconds: Some(duration_seconds) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityPassword {
    pub user: UserPassword,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UserPassword {
    pub domain: UserDomain,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UserDomain {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenScope {
    pub domain: UserDomain,
}

impl TokenScope {
    pub fn from_domain_id(id: impl Into<String>) -> Self {
        Self { domain: UserDomain { id: Some(id.into()), name: None } }
    }
    pub fn from_domain_name(name: impl Into<String>) -> Self {
        Self { domain: UserDomain { id: None, name: Some(name.into()) } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_default() {
        let identity = Identity::default();
        assert_eq!(vec![IdentityMethod::TOKEN], identity.methods);
        assert_eq!(Some(15 * 60), identity.token.unwrap().duration_seconds);
    }

    #[test]
    fn test_serialize_json() -> anyhow::Result<()> {
        let mut policy = IdentityPolicy::default();

        let mut se = HashMap::new();
        se.insert(
            "g:DomainName".into(),
            vec!["DomainNameExample".to_string()], //示例，表示限制条件值，根据实际情况填写
        );

        policy.statement = vec![PolicyStatement::builder()
            .effect(Effect::Allow)
            .append_action("obs:object:GetObject")
            .append_resource("OBS:*:*:object:*")
            .append_condition("StringEquals", se)
            .build()];
        let auth = Auth {
            identity: Identity {
                methods: vec![IdentityMethod::TOKEN],
                token: Some(IdentityToken { id: None, duration_seconds: Some(15 * 60) }),
                policy: Some(policy),
                password: None,
            },
        };
        let value = serde_json::to_value(auth)?;
        println!("json value is {value}");

        let text = value.to_string();
        assert!(text.contains(r#""Statement":[{"Effect":"Allow","Action":["obs:object:GetObject"],"Resource":["OBS:*:*:object:*"],"Condition":{"StringEquals":{"g:DomainName":["DomainNameExample"]}}}]"#));
        Ok(())
    }
}
