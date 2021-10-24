use anyhow::Result;
mod cli;
mod config;

fn main() -> Result<()> {
  cli::init(config::init()?)?;
  Ok(())
}
