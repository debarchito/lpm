use std::collections::HashMap;

use crate::cli::colorize;
use anyhow::{Context, Result};
use reqwest::blocking as request;
use termcolor::{ColorChoice, StandardStream};

pub fn verify(r#type: &str, packages: Vec<String>) -> Result<HashMap<String, String>> {
  println!();
  let mut stdout = colorize::Colorize {
    stream: StandardStream::stdout(ColorChoice::Always),
  };
  stdout.success()?;
  println!("[?] Registry look-up in progress. This is the only blocking task. So, stay put...");
  stdout.reset()?;
  let mut filtered_packages: HashMap<String, String> = HashMap::new();
  for package in packages.into_iter() {
    if let Some(mut first_letter) = package.chars().next() {
      first_letter = first_letter.to_ascii_lowercase();
      let body = request::get(format!(
        "https://debarchito.github.io/lpm-routes/{}/{}/{}.toml",
        r#type, first_letter, package
      ))
      .context("Couldn't reach the registry. Probably some network issue.")?
      .text()
      .context("Couldn't parse text from incoming request.")?
      .to_string()
      .replace('\n', "\n")
      .replace('\r', "\r");
      if body == "404\n" {
        stdout.failure()?;
        println!(
          "[!] \"{}\" doesn't exist in registry. Therfore, skipping it.",
          package
        );
        stdout.reset()?;
      } else {
        filtered_packages.insert(package, body);
      }
    } else { //None
    };
  }
  Ok(filtered_packages)
}
