use async_trait::async_trait;

use crate::{api_url, model::user::User, Result};

use super::WeworkClient;

/// 通讯录管理
/// https://developer.work.weixin.qq.com/document/path/90193
#[async_trait]
pub trait WeworkContacts {
  /// 获取用户信息
  /// https://developer.work.weixin.qq.com/document/path/90196
  async fn get_user(&self, userid: &str) -> Result<User>;
}

static USER_GET: &str = api_url!("/user/get");

#[async_trait]
impl WeworkContacts for WeworkClient {
  async fn get_user(&self, userid: &str) -> Result<User> {
    let rb = self.client.get(USER_GET).query(&[("userid", userid)]);
    let user = self.send(rb).await?;
    Ok(user)
  }
}
