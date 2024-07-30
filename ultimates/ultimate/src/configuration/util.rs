use std::{env, path::Path};

use config::{builder::DefaultState, Config, ConfigBuilder, Environment, File, FileFormat};

use tracing::debug;
use ultimate_common::runtime;

use super::Result;

pub fn load_config() -> Result<Config> {
    let files = if let Ok(profiles_active) = env::var("FUSION__PROFILES__ACTIVE") {
        vec![
            format!("app-{profiles_active}.yaml"),
            format!("app-{profiles_active}.yml"),
            format!("app-{profiles_active}.toml"),
        ]
    } else {
        vec!["app.yaml".to_string(), "app.yml".to_string(), "app.toml".to_string()]
    };
    debug!("Load files: {:?}", files);

    let mut config_file = String::new();
    let mut b = Config::builder().add_source(load_default_source());

    for file in &files {
        if let Ok(path) = runtime::cargo_manifest_dir().map(|dir| dir.join("resources").join(file)) {
            if path.exists() {
                config_file = format!("{}", path.display());
                b = b.add_source(File::from(path));
                break;
            }
        }
    }

    for file in &files {
        let path = Path::new(file);
        if path.exists() {
            config_file = format!("{}", path.display());
            b = b.add_source(File::from(path));
            break;
        }
    }

    // load from file of env
    if let Ok(file) = std::env::var("FUSION_CONFIG_FILE") {
        let path = Path::new(&file);
        if path.exists() {
            b = b.add_source(File::from(path));
        }
        config_file = file;
    }

    b = add_enviroment(b);

    {
        let ss = format!(r#"ultimate.config_file: "{}""#, config_file);
        b = b.add_source(File::from_str(&ss, FileFormat::Yaml));
    }

    let c = b.build()?;
    Ok(c)
}

pub fn load_default_source() -> File<config::FileSourceString, FileFormat> {
    let text = include_str!("default.toml");
    File::from_str(text, FileFormat::Toml)
}

pub fn add_enviroment(b: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
    let mut env = Environment::default();
    env = env.separator("__");
    b.add_source(env)
}
