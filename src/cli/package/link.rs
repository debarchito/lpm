use crate::cli::colorize;
use crate::cli::package::utils::{link_exists, link_from_store};
use crate::config::structs;
use anyhow::{Context, Result};
use std::process;
use termcolor::{ColorChoice, StandardStream};

pub fn link(r#type: &str, packages: Vec<String>, lpm_toml: structs::LpmTOML) -> Result<()> {
  println!(); // Add a new line
  let mut stdout = colorize::Colorize {
    stream: StandardStream::stdout(ColorChoice::Always),
  };
  for package in packages.iter() {
    let file = lpm_toml.store.join(r#type).join(package);
    let dest = lpm_toml.target.join(r#type).join(package);
    if file.exists() {
      if !link_exists(&dest)? {
        link_from_store(file, dest)
          .with_context(|| format!("Failed to create a link \"{}\" from store", package))?;
        stdout.success()?;
        println!("[+] {}", package);
        stdout.reset()?;
      } else {
        println!("[?] \"{}\" is already linked to \"{}\"", package, r#type);
      }
    } else {
      stdout.failure()?;
      println!("[!] \"{}\" doesn't exist in store", package);
      stdout.reset()?;
    }
  }
  process::exit(1);
}
