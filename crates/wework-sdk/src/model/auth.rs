use serde::{Deserialize, Serialize};
use std::convert::AsRef;
use strum::AsRefStr;
use typed_builder::TypedBuilder;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct TfaCode {
    pub userid: String,
    pub tfa_code: String,
}

#[derive(Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    #[strum(serialize = "code")]
    Code,
}

#[derive(Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizeScope {
    /// snsapi_base：静默授权，可获取成员的基础信息（UserId与DeviceId）；
    #[strum(serialize = "snsapi_base")]
    SnsapiBase,
    /// snsapi_privateinfo：手动授权，可获取成员的详细信息，包含头像、二维码等敏感信息。
    #[strum(serialize = "snsapi_privateinfo")]
    SnsapiPrivateinfo,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct AuthorizeReq {
    /// 企业的CorpID
    pub appid: String,
    /// 授权后重定向的回调链接地址，发送前需要使用urlencode对链接进行处理
    pub redirect_uri: String,
    /// 返回类型，此时固定为：code
    #[builder(default = ResponseType::Code)]
    pub response_type: ResponseType,
    /// 应用授权作用域。
    pub scope: AuthorizeScope,
    /// 重定向后会带上state参数，企业可以填写a-zA-Z0-9的参数值，长度不可超过128个字节
    #[builder(setter(strip_option))]
    pub state: Option<String>,
    /// 应用agentid，建议填上该参数（对于第三方应用和代开发自建应用，
    /// 在填写该参数的情况下或者在工作台、聊天工具栏、应用会话内发起oauth2请求的场景中，
    /// 会触发接口许可的自动激活）。snsapi_privateinfo时必填否则报错；
    #[builder(setter(strip_option))]
    pub agentid: Option<String>,
    /// 终端使用此参数判断是否需要带上身份信息
    #[builder(default = true)]
    pub wechat_redirect: bool,
}

static OAUTH2_AUTHORIZE: &str = "https://open.weixin.qq.com/connect/oauth2/authorize";

impl AuthorizeReq {
    pub fn make_url(&self) -> Result<Url, url::ParseError> {
        let mut params = vec![
            ("appid", self.appid.as_str()),
            ("redirect_uri", self.redirect_uri.as_str()),
            ("response_type", self.response_type.as_ref()),
            ("scope", self.scope.as_ref()),
        ];
        if let Some(state) = self.state.as_deref() {
            params.push(("state", state));
        }
        if let Some(agentid) = self.agentid.as_deref() {
            params.push(("agentid", agentid));
        }
        let mut url = Url::parse_with_params(OAUTH2_AUTHORIZE, &params)?;
        if self.wechat_redirect {
            url.set_fragment(Some("wechat_redirect"));
        }
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthorizeReq, AuthorizeScope};

    #[test]
    fn test_make_url() {
        let req = AuthorizeReq::builder()
            .appid("wxCorpId")
            .redirect_uri("http://api.3dept.com/cgi-bin/query?action=get")
            .agentid("101010")
            .scope(AuthorizeScope::SnsapiPrivateinfo)
            .state("abcdefg")
            .build();

        let url = req.make_url().unwrap();
        println!("url is {url}");
    }
}
