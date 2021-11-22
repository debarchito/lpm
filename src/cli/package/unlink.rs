use crate::cli::colorize;
use crate::cli::package::utils::link_exists;
use crate::config::structs;
use anyhow::{Context, Result};
use std::{fs::remove_dir_all, process};
use termcolor::{ColorChoice, StandardStream};

pub fn unlink(r#type: &str, packages: Vec<String>, lpm_toml: structs::LpmTOML) -> Result<()> {
  println!(); // Add a new line
  let mut stdout = colorize::Colorize {
    stream: StandardStream::stdout(ColorChoice::Always),
  };
  for package in packages.iter() {
    let dest = lpm_toml.target.join(r#type).join(package);
    if link_exists(&dest)? {
      remove_dir_all(dest)
        .with_context(|| format!("Failed to unlink \"{}\" from store", package))?;
      stdout.success()?;
      println!("[-] {}", package);
    } else {
      stdout.failure()?;
      println!("[!] \"{}\" doesn't exist in \"{}\"", package, r#type);
    }
  }
  stdout.reset()?;
  process::exit(1);
}