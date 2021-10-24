use anyhow::Result;
use std::process;
use structopt::StructOpt;

mod enums;
use crate::config::structs;
#[path = "package.rs"]
mod package;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Lite-XL Package Manager (lpm)",
  about = "A package manager for the Lite-XL text editor"
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
      uninstall,
      remove,
      unlink,
      list,
    } => {
      if list {
        package::list("plugins", lpm_toml)?;
      } else if !link.is_empty() {
        package::link("plugins", link, lpm_toml)?;
      } else if !unlink.is_empty() {
        package::unlink("plugins", unlink, lpm_toml)?;
      }
    }
    enums::Command::Color {
      install,
      add,
      link,
      uninstall,
      remove,
      unlink,
      list,
    } => {
      if list {
        package::list("colors", lpm_toml)?;
      } else if !link.is_empty() {
        package::link("colors", link, lpm_toml)?;
      } else if !unlink.is_empty() {
        package::unlink("colors", unlink, lpm_toml)?;
      }
    }
    enums::Command::Font {
      install,
      add,
      link,
      uninstall,
      remove,
      unlink,
      list,
    } => {
      if list {
        package::list("fonts", lpm_toml)?;
      } else if !link.is_empty() {
        package::link("fonts", link, lpm_toml)?;
      } else if !unlink.is_empty() {
        package::unlink("fonts", unlink, lpm_toml)?;
      }
    }
  }
  Ok(())
}
