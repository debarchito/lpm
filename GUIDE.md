## Lite-XL Package Manager (lpm) Guide

### Subcommands

*lpm* has **4** subcommands.
- plugin, p
- color, c
- font, f
- pull, P

**NOTE:** All the sub-commands take *flag values* only. All the flags are listed below.

### Independent Flags

- `--install`

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| True        | True     | -i    |

Installs plugins/colors/fonts in store if not installed already.

```bash
# Examples
lpm plugin --install plugin-1 plugin-2 plugin-3 ...
lpm p -i plugin-1 plugin-2 plugin-3 ...
```

<br>

- `--add`

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| True        | True     | -a    |

Installs plugins/colors/fonts in store if not installed already and links them to Lite-XL's data directory.

```bash
# Examples
lpm plugin --add plugin-1 plugin-2 plugin-3 ...
lpm p -a plugin-1 plugin-2 plugin-3 ...
```

<br>

- `--link`

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| True        | True     | -L    |

Links plugins/colors/fonts to Lite-XL's data directory.

```bash
# Examples
lpm plugin --link plugin-1 plugin-2 plugin-3 ...
lpm p -L plugin-1 plugin-2 plugin-3 ...
```

<br>

- `--remove`

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| True        | True     | -r    |

Unlinks plugins/colors/fonts from Lite-XL's data directory (if linked) and uninstalls them from store.

```bash
# Examples
lpm plugin --remove plugin-1 plugin-2 plugin-3 ...
lpm p -r plugin-1 plugin-2 plugin-3 ...
```

<br>

- `--unlink`

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| True        | True     | -U    |

Unlinks plugins/colors/fonts from Lite-XL's data directory.

```bash
# Examples
lpm plugin --unlink plugin-1 plugin-2 plugin-3 ...
lpm p -U plugin-1 plugin-2 plugin-3 ...
```

<br>

- `--list`

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| False       | False    | -l    |

List all the plugins/colors/fonts installed in Lite-XL's data directory (by default).

```bash
# Examples
lpm plugin --list
lpm p -l
```

### Dependent Flags

- `--force`

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| False       | False    | -f    |

When passed with either `--install` or `--add` flags, forcefully installs the plugins/colors/fonts even if they are already installed and re-links them in case of `--add`.

```bash
# Examples
lpm plugin --add plugin-1 plugin-2 plugin-3 ... --force
lpm p -a plugin-1 plugin-2 plugin-3 ... -f
lpm p -f -a plugin-1 plugin-2 plugin-3 ...
```

<br>

- `--global`

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| False       | False    | -g    |

When passed with `--list` flag, points to store instead of Lite-XL's data directory.

```bash
# Examples
lpm plugin --list --global
lpm p -l -g
lpm p -g -l
```

### Cloning Git Repositories

- `--git`

| Takes Value | Multiple           | Alias |
| ----------- | ------------------ | ----- |
| True        | Partially True     | -G    |

**NOTE:**: This feature needs the *git* binary to be available via PATH! <br>
**NOTE:** "Partially True" here means: It does take multiple value inputs but only spawns a single *git* session to handle it i.e. all the arguments are directly passed to the `clone` subcommand. Therefore, all the functionality of *git*'s `clone` subcommand is available out of the box e.g. branch, ssh, shallow clone etc.

*lpm* allows cloning git repositories if the `config.git` option in `lpm.toml` (~/.lpm-store/lpm.toml) is set to `true` (enabled by default). *lpm* will also look for the `.lpm` file in the resulting directory for dependencies, build scripts etc. which *lpm* will start installing/executing.

```bash
# Examples
lpm plugin --git https://github.com/username/project.git
lpm p -G @git:github.com:username/project.git
```

Since all the arguments are directly passed to *git*'s `clone` subcommand, to rename the clone, you can just do:


```bash
# Examples
lpm p -G https://github.com/username/project.git modified-project-name
```

### Decentralized Lua Plugins/Colors and TrueType Fonts

*lpm* allows installation of single-file/raw plugins/colors/fonts if the `config.decentralize` option in `lpm.toml` (~/.lpm-store/lpm.toml) configuration file is set to true (disabled by default). *lpm* will install plugin/colors as `<name>/init.lua` instead of `<name>.lua` while fonts as `<name>/init.ttf` instead of `<name>.ttf` (Lite-XL only supports TrueType fonts as of now but may support OpenType in future).

```bash
# Examples
lpm plugin --decentralize https://somewebsite.com/plugin.lua ...
lpm p -D https://somewebsite.com/plugin.lua ...
lpm f -D https://somewebsite.com/font.ttf ...
```

You can also rename the resulting directory. Just add a name after the url followed by a space and with no `.lua` extension (for plugins/colors only); *lpm* will identify that as a rename request. The rules apply to fonts as well, just the name shouldn't end with `.ttf` instead of `.lua`.

```bash
# Examples
lpm p -D https://somewebsite.com/plugin.lua another-plugin-name ...
lpm f -D https://somewebsite.com/font.ttf another-font-name ...
```

### Pulling latest Lite-XL release

You can pull the latest Lite-XL release from github using *lpm*. For this to work, the `preserve.root` option must point to Lite-XL's extraction location. This will be automatically added to your *lpm.toml* if the `lite-xl` binary is available via PATH; otherwise, you need to manually configure it. *lpm* knows that you have many custom configurations in your `init.lua` file. So, *lpm* will create a copy of your `init.lua` and keep it in the preserve directory (~/.lpm-store/.preserve/). Preservation is configurable via *lpm.toml*. Just add your files/directories like this:

```toml
# lpm.toml
[preserve]
root = "..." # Lite-XL's extraction location
items = [
  "data/core/init.lua", # relative to preserve.root
  "directory",
  "..."
]
```

After all the necessary precautions are taken, you can pull and install the new release by doing:

```bash
lpm pull --lite-xl
```

The `pull` subcommand has no shorthand neither the `--lite-xl` flag has. After installation is complete, *lpm* will go ahead and apply the preserves.

### Updating lpm

Updating *lpm* is very easy. You can just pull the latest release just like Lite-XL:

```bash
lpm pull --lpm
```