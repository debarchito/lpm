## Lite-XL Package Manager (lpm) &nbsp; | &nbsp; GUIDE

### Sub-commands

*lpm* has **3** sub-commands.
- plugin, p
- color, c
- font, f

**NOTE:** All the sub-commands take *flag-values* only. All the flags are listed below.

### Flags

**NOTE:** Dependent flags are flags that depend on other flags.

- `--install`

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| True        | True     | -i    |

Installs plugins/colors/fonts in the centralized store (~/.lpm-store) if not installed already.

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

Installs plugins/colors/fonts in the centralized store (~/.lpm-store) if not installed already and links them to *Lite-XL config directory* (~/.config/lite-xl).

```bash
# Examples
lpm plugin --add plugin-1 plugin-2 plugin-3 ...
lpm p -a plugin-1 plugin-2 plugin-3 ...
```
<br>

- `--force` (Dependent Flag)

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| False       | False    | -f    |

When passed with either `--install` or `--add` flag, forcefully installs the plugins/colors/fonts even if they are already installed.

```bash
# Examples
lpm plugin --add plugin-1 plugin-2 plugin-3 ... --force
lpm p -a plugin-1 plugin-2 plugin-3 ... -f
```
<br>

- `--link`

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| True        | True     | -L    |

Links plugins/colors/fonts from the centralized store to *Lite-XL config directory* (~/.config/lite-xl).

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

Unlinks plugins/colors/fonts from *Lite-XL config directory* (~/.config/lite-xl) and uninstalls from the centralized store (~/.lpm-store).

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

Unlinks plugins/colors/fonts from *Lite-XL config directory* (~/.config/lite-xl).

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

List all the plugins/colors/fonts in **Lite-XL config directory (~/.config/lite-xl)

```bash
# Examples
lpm plugin --list
lpm p -l
```

- `--global` (Dependent Flag)

| Takes Value | Multiple | Alias |
| ----------- | -------- | ----- |
| False       | False    | -g    |

Tells *lpm* to point to the centralized store (~/.lpm-store) instead of pointing to the *Lite-XL config directory* (~/.config/lite-xl). It is used with the `--list` flag.

```bash
# Examples
lpm plugin --list --global
lpm p -l -g
```
