use anyhow::Result;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub struct Colorize {
  pub stream: StandardStream,
}

impl Colorize {
  pub fn info(&mut self) -> Result<()> {
    self.stream.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
    Ok(())
  }
  pub fn success(&mut self) -> Result<()> {
    self.stream.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    Ok(())
  }
  pub fn failure(&mut self) -> Result<()> {
    self.stream.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    Ok(())
  }
  pub fn reset(&mut self) -> Result<()> {
    self.stream.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    Ok(())
  }
}
