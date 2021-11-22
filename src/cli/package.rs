use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};
use std::process;

use crate::cli::colorize;
use crate::config::structs;
use anyhow::{Context, Result};
use termcolor::{ColorChoice, StandardStream};
mod registry;
mod utils;

#[cfg(target_os = "windows")]
fn link_from_store(src: PathBuf, dest: PathBuf) -> Result<()> {
  junction::create(src, dest)?;
  Ok(())
}
#[cfg(target_os = "windows")]
fn link_exists(link: &PathBuf) -> Result<bool> {
  let exists: bool = junction::exists(link)?;
  Ok(exists);
}

#[cfg(not(target_os = "windows"))]
fn link_from_store(src: PathBuf, dest: PathBuf) -> Result<()> {
  std::os::unix::fs::symlink(src, dest)?;
  Ok(())
}
#[cfg(not(target_os = "windows"))]
fn link_exists(link: &Path) -> Result<bool> {
  let exists: bool = std::fs::symlink_metadata(link)?.is_symlink();
  Ok(exists)
}

pub fn list(r#type: &str, lpm_toml: structs::LpmTOML, global: bool) -> Result<()> {
  let mut stdout = colorize::Colorize {
    stream: StandardStream::stdout(ColorChoice::Always),
  };
  stdout.success()?;
  println!("\n[?] List of {}: \n", r#type);
  stdout.reset()?;
  let point_to = if global {
    lpm_toml.store.join(r#type)
  } else {
    lpm_toml.target.join(r#type)
  };
  let mut index = 0;
  for entry in std::fs::read_dir(point_to)? {
    let path = entry?.path();
    if path.is_dir() {
      println!(
        "{}/",
        path
          .file_name()
          .with_context(|| format!("Failed to extract the filename for path: {:?}", path))?
          .to_str()
          .with_context(|| format!(
            "Failed to convert filename (osstr) to str for path: {:?}",
            path
          ))?
      );
      index += 1;
    }
  }
  if index == 0 {
    println!("No {} (T-T)", r#type);
  }
  process::exit(1);
}

pub fn link(r#type: &str, packages: Vec<String>, lpm_toml: structs::LpmTOML) -> Result<()> {
  // Subject to Change
  println!();
  let mut stdout = colorize::Colorize {
    stream: StandardStream::stdout(ColorChoice::Always),
  };
  for package in packages.iter() {
    let file = lpm_toml.store.join(r#type).join(package);
    let dest = lpm_toml.target.join(r#type).join(package);
    if file.exists() {
      if !link_exists(&dest)? {
        link_from_store(file, dest)
          .with_context(|| format!("Failed to create a link from store for \"{}\"", package))?;
        stdout.success()?;
        println!("[+] {}", package);
      } else {
        stdout.reset()?;
        println!("[?] \"{}\" is already linked to \"{}\"", package, r#type);
      }
    } else {
      stdout.failure()?;
      println!("[!] \"{}\" doesn't exist in store", package);
    }
  }
  stdout.reset()?;
  process::exit(1);
}

pub fn unlink(r#type: &str, packages: Vec<String>, lpm_toml: structs::LpmTOML) -> Result<()> {
  //Subject to change
  println!();
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

pub fn install(r#type: &str, packages: Vec<String>) -> Result<()> {
  let _packages = utils::verify(r#type, packages)?;
  /*for (package_name, registry_info) in packages.iter() {
  }*/
  Ok(())
}
