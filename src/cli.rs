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
    enums::Command::Plugin(options) => exec(options, lpm_toml)?,
    enums::Command::Color(options) => exec(options, lpm_toml)?,
    enums::Command::Font(options) => exec(options, lpm_toml)?,
  }
  Ok(())
}

fn exec(option: enums::Options, lpm_toml: structs::LpmTOML) -> Result<()> {
  if !option.install.is_empty() {
    package::install("fonts", option.install)?;
  } else if !option.link.is_empty() {
    package::link("fonts", option.link, lpm_toml)?;
  } else if !option.unlink.is_empty() {
    package::unlink("fonts", option.unlink, lpm_toml)?;
  } else if option.list {
    package::list("fonts", lpm_toml, option.global)?;
  }
  Ok(())
}
