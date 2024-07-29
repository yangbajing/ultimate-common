use async_trait::async_trait;
use serde_json::{json, Value};

use crate::{
  api_url,
  model::{auth::TfaCode, UserDetailInfo, UserInfo},
  Result,
};

use super::WeworkClient;

#[async_trait]
pub trait WeworkAuths {
  /// 获取访问用户身份
  /// https://developer.work.weixin.qq.com/document/path/91023
  async fn getuserinfo(&self, code: &str) -> Result<UserInfo>;

  /// 获取访问用户敏感信息
  /// https://developer.work.weixin.qq.com/document/path/95833
  async fn getuserdetail(&self, user_tiket: &str) -> Result<UserDetailInfo>;

  /// 获取用户二次验证信息
  /// https://developer.work.weixin.qq.com/document/path/99499
  async fn get_tfa_info(&self, code: &str) -> Result<TfaCode>;

  /// 登录二次验证
  /// https://developer.work.weixin.qq.com/document/path/99521
  async fn authsucc(&self, userid: &str) -> Result<()>;

  /// 使用二次验证
  /// https://developer.work.weixin.qq.com/document/path/99500
  async fn tfa_succ(&self, tfa: &TfaCode) -> Result<()>;
}

static AUTH_GETUSERINFO: &str = api_url!("/auth/getuserinfo");
static AUTH_GETUSERDETAIL: &str = api_url!("/auth/getuserdetail");
static GET_TFA_INFO: &str = api_url!("/auth/get_tfa_info");
static AUTHSUCC: &str = api_url!("/user/authsucc");
static TFA_SUCC: &str = api_url!("/user/tfa_succ");

#[async_trait]
impl WeworkAuths for WeworkClient {
  async fn getuserinfo(&self, code: &str) -> Result<UserInfo> {
    let rb = self.client.get(AUTH_GETUSERINFO).query(&[("code", code)]);
    let ret = self.send(rb).await?;
    Ok(ret)
  }

  async fn getuserdetail(&self, user_tiket: &str) -> Result<UserDetailInfo> {
    let rb = self.client.post(AUTH_GETUSERDETAIL).json(&json!({"user_tiket": user_tiket}));
    let ret = self.send(rb).await?;
    Ok(ret)
  }

  async fn get_tfa_info(&self, code: &str) -> Result<TfaCode> {
    let rb = self.client.post(GET_TFA_INFO).json(&json!({"code": code}));
    let ret = self.send(rb).await?;
    Ok(ret)
  }

  async fn authsucc(&self, userid: &str) -> Result<()> {
    let rb = self.client.get(AUTHSUCC).query(&[("userid", userid)]);
    self.send::<Value>(rb).await?;
    Ok(())
  }

  async fn tfa_succ(&self, tfa: &TfaCode) -> Result<()> {
    let rb = self.client.post(TFA_SUCC).json(tfa);
    self.send::<Value>(rb).await?;
    Ok(())
  }
}
