#![allow(dead_code)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegistryInfo {
  r#type: String,
  src: Option<String>,
  args: Option<Vec<String>>,
  rename: Option<String>,
  target: String,
  external: Option<Vec<String>>,
  scripts: Option<Vec<String>>,
}
