use anyhow::Result;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub fn link_from_store(src: PathBuf, dest: PathBuf) -> Result<()> {
  junction::create(src, dest)?;
  Ok(())
}
#[cfg(target_os = "windows")]
pub fn link_exists(link: &PathBuf) -> Result<bool> { Ok(junction::exists(link)?) }

#[cfg(not(target_os = "windows"))]
pub fn link_from_store(src: PathBuf, dest: PathBuf) -> Result<()> {
  std::os::unix::fs::symlink(src, dest)?;
  Ok(())
}
#[cfg(not(target_os = "windows"))]
pub fn link_exists(link: &Path) -> Result<bool> {
  Ok(std::fs::symlink_metadata(link)?.is_symlink())
}
