//TODO: Download the file
#![allow(unused_variables)]
use super::{link, search};
use crate::config::structs;
use anyhow::{Context, Result};
use std::path::Path;
use url::Url;

fn download_file(url: Url, file_loc: &Path) -> Result<()> { todo!() }

pub fn install(r#type: &str, packages: Vec<String>, lpm_toml: structs::LpmTOML) -> Result<()> {
  let file_loc = lpm_toml.store.join(r#type);

  for package in packages.iter() {
    let url = search::search(r#type, package)
      .with_context(|| format!("Package \"{}\" not found.", package))?;
    download_file(url, &file_loc)?;
  }
  link::link(r#type, packages, lpm_toml)?;
  Ok(())
}
