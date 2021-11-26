use crate::config::structs::LpmTOML;
use anyhow::{Context, Result};
use std::env::set_current_dir;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::str;
mod structs;
use super::term;
use std::fs::{read_to_string, remove_dir_all};

//TODO: Fix Boilerplating

pub fn git(r#type: &str, arguments: Vec<String>, lpm_toml: LpmTOML) -> Result<()> {
  if !lpm_toml.config.git {
    term::pretty_print("\n[!] \"config.git\" is disabled", 0)?;
    term::pretty_print("[?] Enable it in ~/.lpm-store/lpm.toml", 2)?;
    exit(1);
  }
  term::pretty_print("\n[?] Spawning Git to handle cloning...", 2)?;
  let install_location = lpm_toml.store.join(r#type);
  set_current_dir(&install_location)?;
  let output = Command::new("git")
    .arg("clone")
    .args(arguments)
    .output()
    .context("Failed to spawn Git. Make sure the \"git\" binary is available via PATH")?;
  if !output.stderr.is_empty() {
    let msg = str::replace(str::from_utf8(&output.stderr)?, "\n", "");
    if msg.contains("fatal:") {
      term::pretty_print(&format!("[!] (from: git) {}", msg), 0)?;
    } else {
      term::pretty_print(&format!("[+] (from: git) {}", msg), 1)?;
      let package = str::replace(&str::replace(&msg, "'...", ""), "Cloning into '", "");
      let setup = Path::new(&install_location).join(&package).join(".lpm");
      if setup.exists() {
        if setup.metadata()?.is_file() {
          term::pretty_print("[?] Found lpm setup file (.lpm). Working...", 2)?;
          init(setup)?;
        } else {
          term::pretty_print(
            "[?] Found lpm setup (.lpm) but its not a file. Therefore, ending task",
            2,
          )?;
          remove_dir_all(package)?;
        }
      } else {
        term::pretty_print(
          "[?] Didn't find any lpm setup file (.lpm) to work with. Therefore, ending task",
          2,
        )?;
        remove_dir_all(package)?;
      }
    }
  } else {
    term::pretty_print("[?] Git didn't return anything. Therefore, ending task", 2)?;
  }
  Ok(())
}

// This function needs some real work
fn init(setup_path: PathBuf) -> Result<()> {
  let setup: structs::GitSetup =
    toml::from_str(&read_to_string(setup_path).context("Failed to read lpm setup (.lpm)")?)
      .context("Failed to parse lpm setup (.lpm). Probably syntax error")?;
  if let Some(requires) = setup.requires {
    if !requires.is_empty() {
      term::pretty_print(
        &format!("[?] (setup) This package requires: {}", requires.join(", ")),
        2,
      )?;
    } else {
      term::pretty_print("[?] (setup) Found empty \"requires\" field", 2)?;
    }
  }
  if let Some(scripts) = setup.scripts {
    if !scripts.is_empty() {
      for mut script in scripts.into_iter() {
        script = script.trim().to_string();
        if script.contains(&String::from(" ")) {
        } else {
          let _output = Command::new(&script)
            .output()
            .with_context(|| format!("Failed to spawn: {}", script))?;
        }
      }
    } else {
      term::pretty_print("[?] (setup) Found empty \"scripts\" field", 2)?;
    }
  }
  Ok(())
}
