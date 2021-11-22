#![allow(stable_features)]
#![feature(is_symlink)]

use anyhow::Result;
mod cli;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
  cli::init(config::init()?).await?;
  Ok(())
}
