use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
  /// Handle Lite-XL plugins
  Plugin {
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install colors in store
    install: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install plugins in store and link them with Lite-XL's plugins directory
    add: Vec<String>,
    #[structopt(long, short = "L", takes_value = true, multiple = true)]
    /// Link plugins with Lite-XL's plugins directory
    link: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Uninstall colors from store
    uninstall: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Unlink plugins from Lite-XL's plugins directory and uninstall them from store
    remove: Vec<String>,
    #[structopt(long, short = "U", takes_value = true, multiple = true)]
    /// Unlink plugins from Lite-XL's plugins directory
    unlink: Vec<String>,
    #[structopt(long, short, multiple = true)]
    /// List all plugins installed in store
    list: bool,
  },
  /// Handle Lite-XL colors
  Color {
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install colors in store
    install: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install colors in store and link them with Lite-XL's colors directory
    add: Vec<String>,
    #[structopt(long, short = "L", takes_value = true, multiple = true)]
    /// Link colors with Lite-XL's colors directory
    link: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Uninstall colors from store
    uninstall: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Unlink colors from Lite-XL's colors directory and uninstall them from store
    remove: Vec<String>,
    #[structopt(long, short = "U", takes_value = true, multiple = true)]
    /// Unlink colors from Lite-XL's colors directory
    unlink: Vec<String>,
    #[structopt(long, short, multiple = true)]
    /// List all colors installed in store
    list: bool,
  },
  /// Handle Lite-XL fonts
  Font {
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install fonts in store
    install: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install fonts in store and link them with Lite-XL's fonts directory
    add: Vec<String>,
    #[structopt(long, short = "L", takes_value = true, multiple = true)]
    /// Link fonts with Lite-XL's fonts directory
    link: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Uninstall fonts from store
    uninstall: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Unlink fonts from Lite-XL's fonts directory and uninstall them from store
    remove: Vec<String>,
    #[structopt(long, short = "U", takes_value = true, multiple = true)]
    /// Unlink fonts from Lite-XL's fonts directory
    unlink: Vec<String>,
    #[structopt(long, short, multiple = true)]
    /// List all fonts installed in store
    list: bool,
  },
}
