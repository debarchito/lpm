#![allow(unused_variables)]

use anyhow::Result;
use structopt::StructOpt;

use crate::config::structs;
pub mod colorize;
mod enums;
mod package;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Lite-XL Package Manager (lpm)",
  about = "A package manager for the Lite-XL code editor"
)]
struct Lpm {
  #[structopt(subcommand)]
  cmd: enums::Command,
}

pub fn init(lpm_toml: structs::LpmTOML) -> Result<()> {
  let lpm = Lpm::from_args();
  match lpm.cmd {
    enums::Command::Plugin(options) => exec("plugins", options, lpm_toml)?,
    enums::Command::Color(options) => exec("colors", options, lpm_toml)?,
    enums::Command::Font(options) => exec("fonts", options, lpm_toml)?,
  }
  Ok(())
}

fn exec(value: &'static str, option: enums::Options, lpm_toml: structs::LpmTOML) -> Result<()> {
  if !option.install.is_empty() {
    package::install(value, option.install)?;
  } else if !option.link.is_empty() {
    package::link(value, option.link, lpm_toml)?;
  } else if !option.unlink.is_empty() {
    package::unlink(value, option.unlink, lpm_toml)?;
  } else if option.list {
    package::list(value, lpm_toml, option.global)?;
    enums::Command::Plugin {
      install,
      link,
      unlink,
      list,
      global,
      ..
    } => {
      if !install.is_empty() {
        package::install("plugins", install)?;
      } else if !link.is_empty() {
        package::link("plugins", link, lpm_toml)?;
      } else if !unlink.is_empty() {
        package::unlink("plugins", unlink, lpm_toml)?;
      } else if list {
        package::list("plugins", lpm_toml, global)?;
      }
    }
    enums::Command::Color {
      install,
      link,
      unlink,
      list,
      global,
      ..
    } => {
      if !install.is_empty() {
        package::install("colors", install)?;
      } else if !link.is_empty() {
        package::link("colors", link, lpm_toml)?;
      } else if !unlink.is_empty() {
        package::unlink("colors", unlink, lpm_toml)?;
      } else if list {
        package::list("colors", lpm_toml, global)?;
      }
    }
    enums::Command::Font {
      install,
      link,
      unlink,
      list,
      global,
      ..
    } => {
      if !install.is_empty() {
        package::install("fonts", install)?;
      } else if !link.is_empty() {
        package::link("fonts", link, lpm_toml)?;
      } else if !unlink.is_empty() {
        package::unlink("fonts", unlink, lpm_toml)?;
      } else if list {
        package::list("fonts", lpm_toml, global)?;
      }
    }
  }
  Ok(())
}
