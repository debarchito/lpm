use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

use crate::cli::structs;
use anyhow::{Context, Result};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[cfg(target_os = "windows")]
fn link_from_store(src: PathBuf, dest: PathBuf) -> Result<()> {
  junction::create(src, dest)?;
  Ok(())
}
#[cfg(not(target_os = "windows"))]
fn link_from_store(src: PathBuf, dest: PathBuf) -> Result<()> {
  std::os::unix::fs::symlink(src, dest)?;
  Ok(())
}

#[cfg(target_os = "windows")]
fn link_exists(link: &PathBuf) -> Result<bool> {
  if junction::exists(link)? {
    Ok(true)
  } else {
    Ok(false)
  }
}
#[cfg(not(target_os = "windows"))]
fn link_exists(link: &PathBuf) -> Result<bool> {
  if std::fs::symlink_metadata(link)?.is_symlink() {
    Ok(true)
  } else {
    Ok(false)
  }
}

pub fn list(r#type: &str, lpm_toml: structs::LpmTOML) -> Result<()> {
  let mut stdout = StandardStream::stdout(ColorChoice::Always);
  stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
  writeln!(&mut stdout, "\n[?] List of {}: \n", r#type)?;
  stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
  for entry in std::fs::read_dir(lpm_toml.store.join(r#type))? {
    let path = entry?.path();
    if path.is_dir() {
      writeln!(
        &mut stdout,
        "{}/",
        path
          .file_name()
          .with_context(|| format!("Failed to extract the filename for: {:?}", path))?
          .to_str()
          .with_context(|| format!("Failed to convert filename<Osstr> to str for: {:?}", path))?
      )?;
    }
  }
  process::exit(1);
}

pub fn link(r#type: &str, packages: Vec<String>, lpm_toml: structs::LpmTOML) -> Result<()> {
  println!("");
  let mut stdout = StandardStream::stdout(ColorChoice::Always);
  for package in packages.iter() {
    let file = lpm_toml.store.join(r#type).join(package);
    let dest = lpm_toml.target.join(r#type).join(package);
    if Path::new(&file).exists() {
      if !link_exists(&dest)? {
        link_from_store(file, dest)
          .with_context(|| format!("Failed to create a link from store for \"{}\"", package))?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut stdout, "[+] {}", package)?;
      } else {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        writeln!(
          &mut stdout,
          "[?] \"{}\" is already linked to \"{}\"",
          package, r#type
        )?;
      }
    } else {
      stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
      writeln!(&mut stdout, "[!] \"{}\" doesn't exist in store", package)?;
    }
  }
  stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
  process::exit(1);
}

pub fn unlink(r#type: &str, packages: Vec<String>, lpm_toml: structs::LpmTOML) -> Result<()> {
  println!("");
  let mut stdout = StandardStream::stdout(ColorChoice::Always);
  for package in packages.iter() {
    let dest = lpm_toml.target.join(r#type).join(package);
    if link_exists(&dest)? {
      std::fs::remove_dir_all(dest)
        .with_context(|| format!("Failed to unlink \"{}\" from store", package))?;
      stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
      writeln!(&mut stdout, "[-] {}", package)?;
    } else {
      stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
      writeln!(
        &mut stdout,
        "[!] \"{}\" doesn't exist in \"{}\"",
        package, r#type
      )?;
    }
  }
  stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
  process::exit(1);
}
