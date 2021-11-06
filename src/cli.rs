#![allow(unused_variables)]

use structopt::StructOpt;
use anyhow::Result;

use crate::config::structs;
mod enums;
mod package;
pub mod colorize;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Lite-XL Package Manager (lpm)",
  about = "A package manager for the Lite-XL code editor"
)]
struct LPM {
  #[structopt(subcommand)]
  cmd: enums::Command,
}

pub fn init(lpm_toml: structs::LpmTOML) -> Result<()> {
  let lpm = LPM::from_args();
  match lpm.cmd {
    enums::Command::Plugin {
      install,
      add,
      link,
      force,
      remove,
      unlink,
      list,
      global,
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
      add,
      link,
      force,
      remove,
      unlink,
      list,
      global,
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
      add,
      link,
      force,
      remove,
      unlink,
      list,
      global,
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
