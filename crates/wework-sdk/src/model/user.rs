use serde::{de::Visitor, Deserialize};
use serde_json::Value;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
pub struct User {
  pub userid: String,
  pub name: String,
  pub department: Vec<i32>,
  pub order: Vec<i32>,
  pub position: String,
  pub mobile: String,
  pub gender: UserGender,
  pub email: String,
  pub biz_mail: String,
  pub is_leader_in_dept: Vec<i32>,
  pub direct_leader: Vec<String>,
  pub avatar: String,
  pub thumb_avatar: String,
  pub telephone: String,
  pub alias: String,
  pub address: String,
  pub open_userid: Option<String>,
  pub main_department: Option<i32>,
  pub status: UserStatus,
  pub hide_mobile: i32,
  pub qr_code: String,
  pub extattr: UserExtattr,
  pub external_profile: Value,
}

#[derive(Debug, Deserialize)]
pub struct UserExtattr {
  pub attrs: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct UserExternalProfile {
  pub external_corp_name: String,
  pub external_attr: Vec<Value>,
}

#[derive(Debug, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum UserStatus {
  Actived = 1,
  Disabled = 2,
  Unactive = 4,
  Exited = 5,
}
#[derive(Debug, PartialEq)]
pub enum UserGender {
  Undefined = 0,
  Male = 1,
  Female = 2,
}

impl From<u8> for UserGender {
  fn from(value: u8) -> Self {
    match value {
      1 => UserGender::Male,
      2 => UserGender::Female,
      _ => UserGender::Undefined,
    }
  }
}
impl From<&str> for UserGender {
  fn from(value: &str) -> Self {
    match value {
      "1" => UserGender::Male,
      "2" => UserGender::Female,
      _ => UserGender::Undefined,
    }
  }
}

impl<'de> Deserialize<'de> for UserGender {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct UserGenderVisitor;
    impl<'de> Visitor<'de> for UserGenderVisitor {
      type Value = UserGender;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Invalid gender, expect 1 or 2, 0")
      }

      fn visit_u8<E>(self, v: u8) -> std::result::Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(v.into())
      }

      fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(v.into())
      }
    }
    deserializer.deserialize_any(UserGenderVisitor)
  }
}
