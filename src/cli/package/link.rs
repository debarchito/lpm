use super::term;
use super::utils::{link_exists, link_from_store};
use crate::config::structs;
use anyhow::{Context, Result};
use std::process;

pub fn link(r#type: &str, packages: Vec<String>, lpm_toml: structs::LpmTOML) -> Result<()> {
  println!(); // Add a new line
  for package in packages.iter() {
    let file = lpm_toml.store.join(r#type).join(package);
    let dest = lpm_toml.target.join(r#type).join(package);
    if file.exists() {
      if !link_exists(&dest)? {
        link_from_store(file, dest)
          .with_context(|| format!("Failed to create a link \"{}\" from store", package))?;
        term::pretty_print(&format!("[+] {}", package), 1)?;
      } else {
        println!("[?] \"{}\" is already linked to \"{}\"", package, r#type);
      }
    } else {
      term::pretty_print(&format!("[!] \"{}\" doesn't exist in store", package), 0)?;
    }
  }
  process::exit(1);
}
