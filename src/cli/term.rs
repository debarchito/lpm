use super::colorize;
use anyhow::Result;
use termcolor::{ColorChoice, StandardStream};

pub fn pretty_print(message: &str, print_type: u8) -> Result<()> {
  let mut stdout = colorize::Colorize {
    stream: StandardStream::stdout(ColorChoice::Always),
  };
  match print_type {
    0 => stdout.failure()?,
    1 => stdout.success()?,
    _ => stdout.info()?,
  }
  println!("{}", message);
  stdout.reset().unwrap();
  Ok(())
}
