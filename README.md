# 🌃 Shade

[![Run cargo batch.](https://github.com/EnabledFish/Shade/actions/workflows/CargoBatch.yml/badge.svg)](https://github.com/EnabledFish/Shade/actions/workflows/CargoBatch.yml)

Shade is an operating system written for the communication.

# 📃 Experiencing

### Building Requirements

1. System: `Windows`, `Linux` or `macOS`.
2. Memory: `1GB` for free.
3. Pieces of software: `Rust`(`Cargo`&`Rustup`) and `Qemu`.

### Cloning Repo

This will create a folder `./Shade` for you.

```shell
git clone https://github.com/EnabledFish/Shade.git
cd Shade
```

### Installing Nightly Rust

If you already have the nightly version of rust, you can skip this and override the toolchain only.

Before overriding the toolchain of rust, make sure you are in the root directory of this repo.

```shell
rustup install nightly
rustup override add nightly
```

## Run Batch File

Shade provides a cross-platform building tool which you can use to construct the image of shade easily.

You can use the [batch](./Batch.shb) command to build and run Shade like this:

```shell
cargo batch
```

If you want to build Shade only without running, trying `cargo batch -r false`.

# ❤️ Contributors

I am always waiting for your prs, truly.

1. [EnabledFish](https://github.com/EnabledFish)
