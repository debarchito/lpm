use anyhow::{Context, Result};
use std::fs::{create_dir_all, read_to_string, write};
use std::path::{Path, PathBuf};
pub mod structs;

pub fn init() -> Result<structs::LpmTOML> {
  let home_dir = dirs::home_dir().context("Failed to locate the home directory")?;
  let lpm_store_path = home_dir.join(".lpm-store");
  let lpm_toml_path = lpm_store_path.join("lpm.toml");
  if !lpm_toml_path.exists() {
    create_lpm_store(&lpm_store_path)?;
    Ok(init_lpm_store(home_dir, lpm_store_path, lpm_toml_path)?)
  } else {
    let lpm_toml: structs::LpmTOML =
      toml::from_str(&read_to_string(lpm_toml_path).context("Failed to read \"lpm.toml\"")?)
        .context("Failed to parse \"lpm.toml\". Probably syntax error")?;
    Ok(lpm_toml)
  }
}

fn create_lpm_store(lpm_store_path: &Path) -> Result<()> {
  create_dir_all(lpm_store_path.join("plugins"))
    .context("Failed to create \"plugins\" directory in store")?;
  create_dir_all(lpm_store_path.join("colors"))
    .context("Failed to create \"colors\" directory in store")?;
  create_dir_all(lpm_store_path.join("fonts"))
    .context("Failed to create \"fonts\" directory in store")?;
  Ok(())
}

fn init_lpm_store(
  home_dir: PathBuf,
  lpm_store_path: PathBuf,
  lpm_toml_path: PathBuf,
) -> Result<structs::LpmTOML> {
  let lpm_toml = structs::LpmTOML {
    store: lpm_store_path,
    target: home_dir.join(".config").join("lite-xl"),
    config: structs::LpmTOMLConfig {
      git: true,
      decentralize: false,
    },
  };
  create_lpm_toml(lpm_toml_path, &lpm_toml)?;
  Ok(lpm_toml)
}

fn create_lpm_toml(lpm_toml_path: PathBuf, lpm_toml: &structs::LpmTOML) -> Result<()> {
  write(
    lpm_toml_path,
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
