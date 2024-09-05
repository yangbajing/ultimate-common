use std::{env, path::PathBuf};

fn main() {
  println!("cargo::rerun-if-changed=proto/fruitbox_iam/");

  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  tonic_build::configure()
    .file_descriptor_set_path(out_dir.join("fruitbox_descriptor.bin"))
    .extern_path(".ultimate_api", "::ultimate_api")
    .type_attribute(".fruitbox_iam.v1", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile(&["proto/fruitbox_iam/v1/auth.proto", "proto/fruitbox_iam/v1/user.proto"], &["proto"])
    .unwrap();
}
