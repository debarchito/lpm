use anyhow::{Context, Result};
use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;

pub mod structs;

pub fn init() -> Result<structs::LpmTOML> {
  if !Path::new("lpm.toml").exists() {
    Ok(create_lpm_store()?)
  } else {
    let lpm_toml: structs::LpmTOML =
      toml::from_str(&read_to_string("lpm.toml").context("Failed to read \"lpm.toml\"")?)
        .context("Failed to parse \"lpm.toml\". Probably syntax error")?;
    Ok(lpm_toml)
  }
}

fn create_lpm_store() -> Result<structs::LpmTOML> {
  let home_dir = dirs::home_dir().context("Failed to locate the home directory")?;
  let lpm_toml = structs::LpmTOML {
    store: home_dir.join(".lpm-store"),
    target: home_dir.join(".config").join("lite-xl"),
    config: structs::LpmTOMLConfig {
      git: true,
      decentralize: false,
    },
  };
  create_dir_all(lpm_toml.store.join("plugins"))
    .context("Failed to create \"plugins\" directory in store")?;
  create_dir_all(lpm_toml.store.join("colors"))
    .context("Failed to create \"colors\" directory in store")?;
  create_dir_all(lpm_toml.store.join("fonts"))
    .context("Failed to create \"fonts\" directory in store")?;
  create_lpm_toml(&lpm_toml)?;
  Ok(lpm_toml)
}

fn create_lpm_toml(lpm_toml: &structs::LpmTOML) -> Result<()> {
  write(
    "lpm.toml",
    format!(
      r#"store = {:?}
target = {:?}

[config]
git = {}
decentralize = {}"#,
      lpm_toml.store, lpm_toml.target, lpm_toml.config.git, lpm_toml.config.decentralize,
    ),
  )
  .context("Failed to create \"lpm.toml\"")?;
  Ok(())
}
