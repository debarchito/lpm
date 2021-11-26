use super::term;
use crate::config::structs;
use anyhow::{Context, Result};
use std::{fs::remove_dir_all, process};

// This also needs some work
pub fn remove(r#type: &str, packages: Vec<String>, lpm_toml: structs::LpmTOML) -> Result<()> {
  println!(); // Add a new line
  for package in packages.iter() {
    let file = lpm_toml.store.join(r#type).join(package);
    if file.exists() {
      remove_dir_all(file)
        .with_context(|| format!("Failed to remove \"{}\" from store", package))?;
      term::pretty_print(&format!("[-] {}", package), 1)?;
    } else {
      term::pretty_print(&format!("[!] \"{}\" doesn't exist in store", package), 0)?;
    }
  }
  process::exit(1);
}
