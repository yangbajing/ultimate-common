use crate::{consts, model::token::AccessToken, utils, Result, WeworkConfig};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use std::{sync::Arc, time::SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, error};

pub struct WeworkClient {
  config: Arc<WeworkConfig>,
  pub(crate) client: Client,
  at: Arc<RwLock<AT>>,
}

impl WeworkClient {
  pub async fn create(config: WeworkConfig) -> Result<Self> {
    let config = Arc::new(config);
    let client = reqwest::Client::builder().build()?;
    let at = Arc::new(RwLock::new(AT::default()));
    let wc = Self { config, client, at };
    wc.init().await;
    Ok(wc)
  }

  async fn init(&self) {
    set_access_token(self.at.clone(), self.config.clone()).await;
    self.interval_reset_ak().await;
  }

  pub fn config(&self) -> &WeworkConfig {
    &self.config
  }

  pub async fn send<T>(&self, rb: RequestBuilder) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let at = self.at.read().await;
    self._send(rb, at.access_token()).await
  }

  #[inline]
  pub(crate) async fn _send<T>(&self, rb: RequestBuilder, access_token: &str) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let request = rb.query(&[("access_token", access_token)]).build()?;
    let json = self.client.execute(request).await?.json().await?;
    utils::extract(json)
  }

  #[inline]
  pub(crate) async fn _send_direct<T>(&self, rb: RequestBuilder) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let json = self.client.execute(rb.build()?).await?.json().await?;
    utils::extract(json)
  }

  pub(crate) async fn interval_reset_ak(&self) {
    let at = self.at.clone();
    let config = self.config.clone();
    tokio::spawn(async move {
      let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
      loop {
        let _inst = interval.tick().await;
        let is_expires;
        {
          let ar = at.read().await;
          is_expires = ar.is_expires();
        }
        debug!("Determine if 'access_token' has expired, expires: {}", is_expires);
        if !is_expires {
          continue;
        }

        // 已过期，重新设置
        set_access_token(at.clone(), config.clone()).await
      }
    });
  }

  /// 获取 access_token。通常不需要手动获取，在调用其它API时会自动获取并判断有效期。
  pub async fn gettoken(&self) -> Result<AccessToken> {
    gettoken(self.config().corp_id(), self.config().secret()).await
  }
}

async fn set_access_token(at: Arc<RwLock<AT>>, config: Arc<WeworkConfig>) {
  let mut aw = at.write().await;
  match gettoken(config.corp_id(), config.secret()).await {
    Ok(t) => {
      aw.access_token = t.access_token;
      // 设置过期时间为实际过期时间短2分钟，以避免在过期临界点时因时差问题造成调用认证失败
      aw.expires_at = SystemTime::now() + std::time::Duration::from_secs(t.expires_in - 120);
    }
    Err(e) => error!("gettoken error: {}", e.to_string()),
  }
}

async fn gettoken(coprid: &str, corpsecret: &str) -> Result<AccessToken> {
  let at: AccessToken = reqwest::Client::new()
    .get(consts::GETTOKEN)
    .query(&[("corpid", coprid), ("corpsecret", corpsecret)])
    .send()
    .await?
    .json()
    .await?;
  debug!("{:?} Get access token success", SystemTime::now());
  Ok(at)
}

struct AT {
  access_token: String,
  expires_at: SystemTime,
}

impl AT {
  pub fn access_token(&self) -> &str {
    &self.access_token
  }

  pub fn is_expires(&self) -> bool {
    self.expires_at < SystemTime::now()
  }
}

impl Default for AT {
  fn default() -> Self {
    Self {
      access_token: Default::default(),
      expires_at: SystemTime::UNIX_EPOCH, // 初始化为此值可确保第一次使用时为过期状态
    }
  }
}
