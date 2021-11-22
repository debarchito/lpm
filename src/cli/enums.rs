use structopt::StructOpt;

//Install Package and link them

#[derive(Debug, StructOpt)]
pub enum Command {
  #[structopt(alias = "p")]
  /// Handle Lite-XL plugins. Alias "p"
  Plugin {
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install plugins in store
    install: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install plugins in store and link them with Lite-XL's plugins directory
    add: Vec<String>,
    #[structopt(long, short)]
    /// Force downloads
    force: bool,
    #[structopt(long, short = "L", takes_value = true, multiple = true)]
    /// Link plugins with Lite-XL's plugins directory
    link: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Unlink plugins from Lite-XL's plugins directory and uninstall them from store
    remove: Vec<String>,
    #[structopt(long, short = "U", takes_value = true, multiple = true)]
    /// Unlink plugins from Lite-XL's plugins directory
    unlink: Vec<String>,
    #[structopt(long, short)]
    /// List all plugins installed in store
    list: bool,
    #[structopt(long, short)]
    /// Point to the centralized store instead
    global: bool,
  },
  #[structopt(alias = "c")]
  /// Handle Lite-XL colors. Alias "c"
  Color {
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install plugins in store
    install: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install colors in store and link them with Lite-XL's colors directory
    add: Vec<String>,
    #[structopt(long, short)]
    /// Force downloads
    force: bool,
    #[structopt(long, short = "L", takes_value = true, multiple = true)]
    /// Link colors with Lite-XL's colors directory
    link: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Unlink colors from Lite-XL's colors directory and uninstall them from store
    remove: Vec<String>,
    #[structopt(long, short = "U", takes_value = true, multiple = true)]
    /// Unlink colors from Lite-XL's colors directory
    unlink: Vec<String>,
    #[structopt(long, short)]
    /// List all colors installed in store
    list: bool,
    #[structopt(long, short)]
    /// Point to the centralized store instead
    global: bool,
  },
  #[structopt(alias = "f")]
  /// Handle Lite-XL fonts. Alias "f"
  Font {
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install fonts in store
    install: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Install fonts in store and link them with Lite-XL's fonts directory
    add: Vec<String>,
    #[structopt(long, short)]
    /// Force downloads
    force: bool,
    #[structopt(long, short = "L", takes_value = true, multiple = true)]
    /// Link fonts with Lite-XL's fonts directory
    link: Vec<String>,
    #[structopt(long, short, takes_value = true, multiple = true)]
    /// Unlink fonts from Lite-XL's fonts directory and uninstall them from store
    remove: Vec<String>,
    #[structopt(long, short = "U", takes_value = true, multiple = true)]
    /// Unlink fonts from Lite-XL's fonts directory
    unlink: Vec<String>,
    #[structopt(long, short)]
    /// List all fonts installed in store
    list: bool,
    #[structopt(long, short)]
    /// Point to the centralized store instead
    global: bool,
  },
}
