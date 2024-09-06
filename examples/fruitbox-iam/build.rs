use std::{env, path::PathBuf};

static ENUM_ATTR: &str =
  "#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, enum_iterator::Sequence, sqlx::Type)]";

static MESSAGE_ATTR: &str = "#[derive(serde::Serialize, serde::Deserialize)]";

static BASE_PACKAGE: &str = ".fruitbox_iam.v1";

fn main() {
  println!("cargo::rerun-if-changed=proto/fruitbox_iam/");

  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  let enums = ["UserStatus"];
  let messages = [
    "SigninRequest",
    "SigninReplay",
    "UserDto",
    "FindUserRequest",
    "CreateUserRequest",
    "UpdateUserRequest",
    "PageUserRequest",
    "FilterUserRequest",
    "PageUserReply",
    "DeleteUserReply",
    "UserReply",
  ];

  let mut builder = tonic_build::configure()
    .file_descriptor_set_path(out_dir.join("fruitbox_descriptor.bin"))
    .compile_well_known_types(true)
    .extern_path(".ultimate_api", "::ultimate_api");

  builder = enums.iter().fold(builder, |b, e| b.enum_attribute(format!("{}.{}", BASE_PACKAGE, e), ENUM_ATTR));
  builder = messages.iter().fold(builder, |b, m| b.message_attribute(format!("{}.{}", BASE_PACKAGE, m), MESSAGE_ATTR));

  builder.compile(&["proto/fruitbox_iam/v1/auth.proto", "proto/fruitbox_iam/v1/user.proto"], &["proto"]).unwrap();
}
