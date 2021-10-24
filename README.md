## Lite-XL Package Manager (lpm) (Under Development)

*lpm* is an **attempt** to create a package manager for the [Lite-XL](https://github.com/lite-xl/lite-xl) code editor. It's primary goal is to provide you with a simple cli with which you can manage everything starting from plugins, colors, fonts (and even update Lite-XL to latest release in the future). *lpm* was highly inspired by the [pnpm](https://github.com/pnpm/pnpm) package manager for [Node.js](https://nodejs.org). *lpm* downloads everything into a centralized store (.lpm-store) and uses *symlink* in UNIX while uses *junction* in Windows for distribution powering features like linking and unlinking plugins.

### Building lpm from source

Building *lpm* is as easy as it could be. You should use the Rust nightly toolchain as `Cargo.toml` contains some nightly-specific configurations. If you want to use the stable toolchain you need to dig into `profile.release`.
```bash
git clone https://github.com/debarchito/lpm
cd lpm
cargo build --release
```
### Running lpm

**NOTE 1:** *lpm* needs **git** in PATH! <br>
**NOTE 2:** *lpm* generates a file named `lpm.toml` in the executable directory of *lpm*!

### LICENSE
MIT