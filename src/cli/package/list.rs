use super::term;
use crate::config::structs;
use anyhow::{Context, Result};
use std::{fs::read_dir, process};

pub fn list(r#type: &str, lpm_toml: structs::LpmTOML, global: bool) -> Result<()> {
  term::pretty_print(&format!("\n[?] List of {}: \n", r#type), 1)?;
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
