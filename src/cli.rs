use crate::config::structs;
use anyhow::Result;
use structopt::StructOpt;
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
  commands: enums::Commands,
}

pub async fn init(lpm_toml: structs::LpmTOML) -> Result<()> {
  let lpm = Lpm::from_args();
  match lpm.commands {
    enums::Commands::Plugin(options) => exec("plugins", options, lpm_toml)?,
    enums::Commands::Color(options) => exec("colors", options, lpm_toml)?,
    enums::Commands::Font(options) => exec("fonts", options, lpm_toml)?,
  }
  Ok(())
}

fn exec(value: &'static str, option: enums::Options, lpm_toml: structs::LpmTOML) -> Result<()> {
  if !option.link.is_empty() {
    package::link::link(value, option.link, lpm_toml)?;
  } else if !option.remove.is_empty() {
    package::remove::remove(value, option.remove, lpm_toml)?;
  } else if !option.unlink.is_empty() {
    package::unlink::unlink(value, option.unlink, lpm_toml)?;
  } else if !option.git.is_empty() {
    package::git::git(value, option.git, lpm_toml)?;
  } else if option.list {
    package::list::list(value, lpm_toml, option.global)?;
  }
  Ok(())
}
