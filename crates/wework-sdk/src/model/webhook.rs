use serde::{
    ser::{self, SerializeStruct},
    Deserialize, Serialize,
};
use strum::AsRefStr;
use typed_builder::TypedBuilder;

use super::msg::{MsgMarkdown, MsgText, MsgType};

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(strip_option)))]
pub struct WebhookMsg {
    #[builder(setter(!strip_option))]
    pub msgtype: MsgType,
    #[builder(default)]
    pub text: Option<MsgText>,
    #[builder(default)]
    pub markdown: Option<MsgMarkdown>,
    #[builder(default)]
    pub image: Option<MsgImage>,
}

impl Serialize for WebhookMsg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ser = serializer.serialize_struct("WebhookMsg", 2)?;
        ser.serialize_field("msgtype", &self.msgtype)?;
        match self.msgtype {
            MsgType::text if self.text.is_some() => {
                ser.serialize_field("text", self.text.as_ref().unwrap())?;
            }
            MsgType::markdown if self.markdown.is_some() => {
                ser.serialize_field("markdown", self.markdown.as_ref().unwrap())?;
            }
            MsgType::image if self.image.is_some() => {
                ser.serialize_field("image", self.image.as_ref().unwrap())?;
            }
            _ => {
                return Err(ser::Error::custom(format!(
                    "The msgtype is [{}], but the field `{}` is not set.",
                    self.msgtype, self.msgtype
                )));
            }
        }
        ser.end()
    }
}

#[derive(Debug, Deserialize, AsRefStr)]
pub enum WebhookUploadType {
    #[strum(serialize = "voice")]
    Voice,
    #[strum(serialize = "file")]
    File,
}

#[derive(Debug, Deserialize)]
pub struct WebhookUploaded {
    #[serde(rename = "type")]
    pub typ: WebhookUploadType,
    pub media_id: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MsgImage {
    base64: String,
    md5: String,
}
