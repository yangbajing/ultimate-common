use super::{Credentials, HttpClientBuilder};

use crate::digest;
use crate::{Result, SDK_DATE_FORMAT};
use crate::{ALGORITHM, HEADER_CONTENT_SHA_256, HEADER_X_SDK_DATE};

use chrono::Utc;
use config::FileFormat;
use reqwest::header::{self, HeaderMap};
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

/// 使用示例
/// ```
/// use config::{Config, Source};
/// use huaweicloud_sdk_core::client::ClientConfig;
///
/// fn load() {
///     let cc: ClientConfig = Config::builder()
///         .add_source(config::Environment::default())
///         .build().unwrap()
///         .try_deserialize().unwrap();
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    credentials: Credentials,
    _underlying: Option<config::Value>,
}

impl ClientConfig {
    pub fn create_http_client_builder(&self) -> Result<HttpClientBuilder> {
        HttpClientBuilder::new(self.credentials().endpoint().unwrap())
    }

    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    pub fn credentials_mut(&mut self) -> &mut Credentials {
        &mut self.credentials
    }

    pub fn underlying(&self) -> Option<&config::Value> {
        self._underlying.as_ref()
    }
}

impl ClientConfig {
    pub fn load_from_file(file: &str) -> Result<Self> {
        let app_file_source = config::File::new(file, FileFormat::Toml);
        let cc: ClientConfig = config::Config::builder().add_source(app_file_source).build()?.get("hw")?;

        Ok(cc)
    }
}

pub fn sign_headers(client_client: &ClientConfig, builder: &mut HttpClientBuilder) -> Result<()> {
    prepare_host(builder)?;
    prepare_date(builder)?;

    let signed_headers = signed_headers(&builder.headers);
    let canonical_str = canonical_string(builder, &signed_headers)?;

    let string_to_sign = string_to_sign(&canonical_str, builder.headers[HEADER_X_SDK_DATE].to_str()?)?;
    let signature = digest::hmac_sha256_string(client_client.credentials.sk().as_bytes(), string_to_sign.as_bytes())?;
    let auth_value = auth_header_value(client_client.credentials.ak(), &signed_headers, &signature).parse()?;
    builder.header(header::AUTHORIZATION, auth_value);
    builder.header(header::CONTENT_LENGTH, builder.body.len().to_string().parse()?);
    Ok(())
}

// headers 添加 X-Sdk-Date
fn prepare_date(builder: &mut HttpClientBuilder) -> Result<()> {
    if !builder.headers.contains_key(HEADER_X_SDK_DATE) {
        let now = Utc::now();
        let sdk_date = now.format(SDK_DATE_FORMAT).to_string();
        builder.header(HEADER_X_SDK_DATE, sdk_date.try_into()?);
    }
    Ok(())
}

// headers 添加 HOST
fn prepare_host(builder: &mut HttpClientBuilder) -> Result<()> {
    // let mut have_host = false;
    // for key in builder.headers.keys() {
    //     if key.as_str().eq_ignore_ascii_case("host") {
    //         have_host = true;
    //         break;
    //     }
    // }
    let have_host = builder.headers.iter().any(|(key, _)| key.as_str().eq_ignore_ascii_case("host"));

    if !have_host && builder.url.has_host() {
        builder.header(reqwest::header::HOST, builder.url.host_str().unwrap().try_into()?);
    }

    Ok(())
}

fn auth_header_value(key: &str, signed_headers: &[String], signature: &str) -> String {
    format!("{} Access={}, SignedHeaders={}, Signature={}", ALGORITHM, key, signed_headers.join(";"), signature)
}

fn string_to_sign(canonical_str: &str, header_x_sdk_date: &str) -> Result<String> {
    let hashed = digest::sha256_string(canonical_str.as_bytes());
    Ok(format!("{}\n{}\n{}", ALGORITHM, header_x_sdk_date, hashed))
}

fn canonical_string(builder: &HttpClientBuilder, signed_headers: &[String]) -> Result<String> {
    let canonical_headers = canonical_headers(&builder.headers, signed_headers);
    let hexencode = if let Some(hv) = builder.headers.get(HEADER_CONTENT_SHA_256) {
        hv.to_str()?.to_string()
    } else {
        digest::sha256_string(builder.body.as_bytes())
    };
    Ok(format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        builder.method.as_str(),
        canonical_uri(&builder.url),
        canonical_query_string(&builder.url),
        canonical_headers,
        signed_headers.join(";"),
        hexencode
    ))
}

fn canonical_headers(headers: &HeaderMap, signed_headers: &[String]) -> String {
    let a: Vec<_> = signed_headers
        .iter()
        .map(|key| {
            let k = key.trim();
            let value = &headers[k];
            let v = value.to_str().unwrap_or("").trim();
            format!("{}:{}", k, v)
        })
        .collect();
    a.join("\n") + "\n"
}

fn canonical_uri(url: &Url) -> String {
    let path = url.path();
    if path.ends_with('/') {
        path.to_string()
    } else {
        format!("{}/", path)
    }
}

fn canonical_query_string(url: &Url) -> String {
    let qs = url.query_pairs().collect::<HashMap<_, _>>();
    let mut keys: Vec<_> = qs.keys().map(|k| k.as_ref()).collect();
    keys.sort();
    let items: Vec<_> = keys
        .into_iter()
        .map(|k| {
            let value = &qs[k];
            format!("{}={}", urlencoding::encode(k), urlencoding::encode(value))
        })
        .collect();
    items.join("&")
}

// 将 headers key 按字典序排序
fn signed_headers(headers: &HeaderMap) -> Vec<String> {
    let mut v: Vec<String> = headers.iter().map(|(key, _)| key.to_string()).collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use config::{Config, FileFormat};

    #[test]
    fn test_canonical_request() -> Result<()> {
        let text = r#"[hw]
project_id = "nicoh8paich7aeheer2oocahZa7da0nu"

# HW.CREDENTIALS
[hw.credentials]
ak = "Bahvahbahre5tae1aiye"
sk = "EephooKohTh1iechapia0aem0bi2We7eeka9di3i"
endpoint = "https://vpc.cn-southwest-2.myhuaweicloud.com"

# HW.USER_PASSWORD
[hw.user_password]
domain.id = "Ohvoh8aoQuee7aeH2ahphohgee0jeiNe"
domain.name = "hwtestuser"
name = "hwtestuser"
password = "O{%B>~-ZLvL3B5OL;i"
"#;

        let client_config: ClientConfig =
            config::Config::builder().add_source(config::File::from_str(text, FileFormat::Toml)).build()?.get("hw")?;
        let mut client_builder = client_config.create_http_client_builder()?;

        sign_headers(&client_config, &mut client_builder)?;

        let request = client_builder.build(&client_config)?;

        println!("RequestBuilder is {:?}", request);

        Ok(())
    }

    #[test]
    fn test_from_file_path() -> anyhow::Result<()> {
        let c = Config::builder().add_source(config::File::new(CONFIG_FILE_PATH, FileFormat::Toml)).build()?;
        // println!("Config is {:?}", c.get_table("CREDENTIALS").unwrap());
        let cc: ClientConfig = c.get::<config::Value>("hw")?.try_deserialize()?;
        println!("ClientConfig is {cc:?}");
        assert!(!cc.credentials.ak().is_empty());
        assert!(!cc.credentials.sk().is_empty());
        Ok(())
    }

    // crates/huaweicloud-sdk-core/
    const CONFIG_FILE_PATH: &str = "docs/config/application-example.toml";
}
