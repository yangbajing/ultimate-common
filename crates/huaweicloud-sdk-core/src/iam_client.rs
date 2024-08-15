use crate::model::auth::AuthResp;
use crate::model::token::{TokenReq, TokenResp};
use crate::{client::ClientConfig, model::auth::AuthReq};
use crate::{Result, SdkError};
use reqwest::Method;
use std::sync::Arc;

static API_TOKENS: &str = "/v3/auth/tokens";
static API_SECURITYTOKENS: &str = "/v3.0/OS-CREDENTIAL/securitytokens";
static X_SUBJECT_TOKEN: &str = "X-Subject-Token";

pub struct IamClient {
  client_config: Arc<ClientConfig>,
}

impl IamClient {
  pub fn new(client_config: Arc<ClientConfig>) -> Self {
    Self { client_config }
  }

  pub async fn tokens(&self, token_req: &TokenReq) -> Result<TokenResp> {
    let rb = self
      .client_config
      .create_http_client_builder()?
      .method(Method::POST)
      .resource_path(API_TOKENS)
      .json(serde_json::to_value(token_req)?)
      .build(&self.client_config)?;
    let resp = rb.send().await?;
    if resp.status().is_success() {
      let subject_token =
        if let Some(st) = resp.headers().get(X_SUBJECT_TOKEN) { st.to_str()?.to_string() } else { String::from("") };
      let res = resp.json::<TokenResp>().await?.with_subject_token(subject_token);
      Ok(res)
    } else {
      let err_msg = resp.text().await?;
      Err(SdkError::api_error(API_TOKENS, err_msg))
    }
  }

  pub async fn securitytokens(&self, auth_req: &AuthReq) -> Result<AuthResp> {
    let rb = self
      .client_config
      .create_http_client_builder()?
      .method(Method::POST)
      .resource_path(API_SECURITYTOKENS)
      .json(serde_json::to_value(auth_req)?)
      .build(&self.client_config)?;
    let resp = rb.send().await?;
    if resp.status().is_success() {
      let auth_resp = resp.json().await?;
      Ok(auth_resp)
    } else {
      let err_msg = resp.text().await?;
      Err(SdkError::api_error(API_SECURITYTOKENS, err_msg))
    }
  }
}
