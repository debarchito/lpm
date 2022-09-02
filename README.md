## Lite-XL Package Manager (lpm)

**NOTE:** This implementation has been deprecated, but there are talks going on about re-implementing it. But, lpm isn't dead. There are many variables to consider while making a package manager. Hop on the [Lite-XL Discord Server](https://discord.gg/RWzqC3nx7K) and ping `@Andew` for any questions or advices.

### Introduction

*lpm* is an **attempt** to create a package manager for the [Lite-XL](https://github.com/lite-xl/lite-xl) code editor. It's primary goal is to provide a simple CLI to manage everything starting from plugins, colors, fonts (and even update Lite-XL to latest release in the future). *lpm* was highly inspired by the [pnpm](https://github.com/pnpm/pnpm) package manager for [Node.js](https://nodejs.org). *lpm* downloads everything into a centralized store and links it to Lite-XL's data directory via *symlink* in UNIX(-like) and *junction* in Windows.

### Building *lpm* from source

Building *lpm* is very easy. You just need the Rust nightly toolchain (tested on 1.57.0-nightly+) and you can run:
```bash
git clone https://github.com/debarchito/lpm
cd lpm
cargo build --release
```
**NOTE:** In UNIX(-like), *lpm* uses the `openssl` crate which provides **OpenSSL** bindings for Rust. To build this crate, you need a **C compiler**, **Perl**, and **make** alongside Rust.<br>
**NOTE:** Release builds are optimized for speed and efficiency (see Cargo.toml). Therefore, it may take quite a bit of time to compile.

### How to use?

Check [GUIDE.md](https://github.com/debarchito/lpm/blob/main/GUIDE.md). (Covers all my plans as of now. Not everything is implemented just yet)

### LICENSE
MIT
