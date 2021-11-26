use super::term;
use crate::cli::package::utils::link_exists;
use crate::config::structs;
use anyhow::{Context, Result};
use std::{fs::remove_dir_all, process};

pub fn unlink(r#type: &str, packages: Vec<String>, lpm_toml: structs::LpmTOML) -> Result<()> {
  println!(); // Add a new line
  for package in packages.iter() {
    let dest = lpm_toml.target.join(r#type).join(package);
    if link_exists(&dest)? {
      remove_dir_all(dest)
        .with_context(|| format!("Failed to unlink \"{}\" from store", package))?;
      term::pretty_print(&format!("[-] {}", package), 1)?;
    } else {
      term::pretty_print(
        &format!("[!] \"{}\" doesn't exist in \"{}\"", package, r#type),
        0,
      )?;
    }
  }
  process::exit(1);
}
