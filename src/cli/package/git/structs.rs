use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitSetup {
  pub requires: Option<Vec<String>>,
  pub scripts: Option<Vec<String>>,
}
