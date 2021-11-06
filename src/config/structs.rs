use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct LpmTOML {
  pub store: PathBuf,
  pub target: PathBuf,
  pub config: LpmTOMLConfig,
}

#[derive(Debug, Deserialize)]
pub struct LpmTOMLConfig {
  pub path: PathBuf,
  pub git: bool,
  pub decentralize: bool,
}
