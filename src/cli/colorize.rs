use anyhow::{Context, Result};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

const CONTEXT: &str = "Caused at \"$PROJECTDIR/src/cli/colorize.rs\" by the \"termcolor\" crate";

pub struct Colorize {
  pub stream: StandardStream,
}

impl Colorize {
  pub fn success(&mut self) -> Result<()> {
    self
      .stream
      .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
      .context(CONTEXT)?;
    Ok(())
  }
  pub fn failure(&mut self) -> Result<()> {
    self
      .stream
      .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
      .context(CONTEXT)?;
    Ok(())
  }
  pub fn reset(&mut self) -> Result<()> {
    self
      .stream
      .set_color(ColorSpec::new().set_fg(Some(Color::White)))
      .context(CONTEXT)?;
    Ok(())
  }
}
