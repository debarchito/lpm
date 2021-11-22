use crate::cli::colorize;
use crate::config::structs;
use anyhow::{Context, Result};
use std::{fs::read_dir, process};
use termcolor::{ColorChoice, StandardStream};

pub fn list(r#type: &str, lpm_toml: structs::LpmTOML, global: bool) -> Result<()> {
  let mut stdout = colorize::Colorize {
    stream: StandardStream::stdout(ColorChoice::Always),
  };
  stdout.success()?;
  println!("\n[?] List of {}: \n", r#type);
  stdout.reset()?;
  let point_to = if global {
    lpm_toml.store.join(r#type)
  } else {
    lpm_toml.target.join(r#type)
  };
  let mut index = 0;
  for entry in read_dir(point_to)? {
    let path = entry?.path();
    if path.is_dir() {
      println!(
        "{}/",
        path
          .file_name()
          .with_context(|| format!("Failed to extract the filename for path: {:?}", path))?
          .to_str()
          .with_context(|| format!(
            "Failed to convert filename (osstr) to str for path: {:?}",
            path
          ))?
      );
      index += 1;
    }
  }
  if index == 0 {
    println!("No {} (T-T)", r#type);
  }
  process::exit(1);
}
