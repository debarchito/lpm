use crate::cli::colorize;
use crate::config::structs::LpmTOML;
use anyhow::{Context, Result};
use std::env::set_current_dir;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::str;
use termcolor::{ColorChoice, StandardStream};
mod structs;
use std::fs::read_to_string;

pub fn git(r#type: &str, arguments: Vec<String>, lpm_toml: LpmTOML) -> Result<()> {
  let mut stdout = colorize::Colorize {
    stream: StandardStream::stdout(ColorChoice::Always),
  };
  if !lpm_toml.config.git {
    stdout.failure()?;
    println!("\n[!] \"config.git\" is disabled");
    stdout.info()?;
    println!("[?] Enable it in ~/.lpm-store/lpm.toml");
    stdout.reset()?;
    exit(1);
  }
  stdout.info()?;
  println!("\n[?] Spawning Git to handle cloning...");
  stdout.reset()?;
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
      stdout.failure()?;
      println!("[!] (from: git) {}", msg);
      stdout.reset()?;
    } else {
      stdout.success()?;
      println!("[+] (from: git) {}", msg);
      stdout.reset()?;
      let package = str::replace(&str::replace(&msg, "'...", ""), "Cloning into '", "");
      let setup = Path::new(&install_location).join(package).join(".lpm");
      if setup.exists() {
        if setup.metadata()?.is_file() {
          stdout.info()?;
          println!("[?] Found lpm setup file (.lpm). Working...");
          stdout.reset()?;
          init(setup, stdout)?;
        } else {
          stdout.info()?;
          println!("[?] Found lpm setup (.lpm) but its not a file. Therefore, ending task");
          stdout.reset()?;
        }
      } else {
        stdout.info()?;
        println!("[?] Didn't find any lpm setup file (.lpm) to work with. Therefore, ending task");
        stdout.reset()?;
      }
    }
  } else {
    stdout.info()?;
    println!("[?] Git didn't return anything. Therefore, ending task");
    stdout.reset()?;
  }
  Ok(())
}

// This function needs some real work
fn init(setup_path: PathBuf, mut stdout: colorize::Colorize) -> Result<()> {
  let setup: structs::GitSetup =
    toml::from_str(&read_to_string(setup_path).context("Failed to read lpm setup (.lpm)")?)
      .context("Failed to parse lpm setup (.lpm). Probably syntax error")?;
  if let Some(requires) = setup.requires {
    if !requires.is_empty() {
      stdout.info()?;
      println!("[?] (setup) This package requires: {}", requires.join(", "));
      stdout.reset()?;
    } else {
      stdout.info()?;
      println!("[?] (setup) Found empty \"requires\" field");
      stdout.reset()?;
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
      stdout.info()?;
      println!("[?] (setup) Found empty \"scripts\" field");
      stdout.reset()?;
    }
  }
  Ok(())
}
