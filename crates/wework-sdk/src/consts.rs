use crate::api_url;

pub static GETTOKEN: &str = api_url!("/gettoken");

#[macro_export]
macro_rules! api_url {
    ($($x:expr),* $(,)?) => {
        concat!("https://qyapi.weixin.qq.com/cgi-bin", $($x, )*)
    };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_api_url() {
    assert_eq!(GETTOKEN, "https://qyapi.weixin.qq.com/cgi-bin/gettoken");
    assert_eq!(api_url!("/message", "/send"), "https://qyapi.weixin.qq.com/cgi-bin/message/send");
  }
}
