use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Commands {
  #[structopt(alias = "p")]
  /// Handle Lite-XL plugins. Alias "p"
  Plugin(Options),
  #[structopt(alias = "c")]
  /// Handle Lite-XL colors. Alias "c"
  Color(Options),
  #[structopt(alias = "f")]
  /// Handle Lite-XL fonts. Alias "f"
  Font(Options),
}

#[derive(Debug, StructOpt)]
pub struct Options {
  #[structopt(long, short, takes_value = true, multiple = true)]
  /// Install package in store
  pub install: Vec<String>,
  #[structopt(long, short, takes_value = true, multiple = true)]
  /// Install package in store and link them with Lite-XL
  pub add: Vec<String>,
  #[structopt(long, short)]
  /// Force downloads
  pub force: bool,
  #[structopt(long, short = "L", takes_value = true, multiple = true)]
  /// Link package with Lite-XL
  pub link: Vec<String>,
  #[structopt(long, short, takes_value = true, multiple = true)]
  /// Unlink package from Lite-XL and uninstall them from store
  pub remove: Vec<String>,
  #[structopt(long, short = "U", takes_value = true, multiple = true)]
  /// Unlink package from Lite-XL
  pub unlink: Vec<String>,
  #[structopt(long, short)]
  /// List packages installed in store
  pub list: bool,
  #[structopt(long, short)]
  /// Point to the centralized store instead
  pub global: bool,
  #[structopt(long, short = "G", takes_value = true, multiple = true)]
  /// Clone Git repositories
  pub git: Vec<String>,
  #[structopt(long, short = "D", takes_value = true, multiple = true)]
  /// Install decentralized Lua plugins/colors and TrueType fonts
  pub decentralize: Vec<String>,
}
