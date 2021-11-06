#![allow(stable_features)]
#![feature(is_symlink)]

use anyhow::Result;
mod cli;
mod config;

fn main() -> Result<()> {
  cli::init(config::init()?)?;
  Ok(())
}
