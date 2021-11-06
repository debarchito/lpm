## Lite-XL Package Manager (lpm)

**NOTE:** Under development. Therefore everything is very scattered; I'll organize some stuff in future commits. So, you are free to explore but expect breaking changes.

### Introduction

*lpm* is an **attempt** to create a package manager for the [Lite-XL](https://github.com/lite-xl/lite-xl) code editor. It's primary goal is to provide you with a simple cli with which you can manage everything starting from plugins, colors, fonts (and even update Lite-XL to latest release in the future). *lpm* was highly inspired by the [pnpm](https://github.com/pnpm/pnpm) package manager for [Node.js](https://nodejs.org). *lpm* downloads everything into a centralized store (.lpm-store) and uses *symlink* in UNIX(-like) while uses *junction* in Windows.

### Building *lpm* from source

Building *lpm* is as easy as it could be. You should use the Rust nightly toolchain (tested on 1.57.0-nightly+)
```bash
git clone https://github.com/debarchito/lpm
cd lpm
cargo build --release
```
**NOTE:** In UNIX(-like), *lpm* uses the `openssl` crate which provides **OpenSSL** bindings for Rust. To build this crate, you need a **C compiler**, **Perl**, and **make** alongside Rust.

### How to use?

Check [GUIDE.md](https://github.com/debarchito/lpm/blob/main/GUIDE.md). (GUIDE covers my plans; not everything is implemented just yet)

### LICENSE
MIT
