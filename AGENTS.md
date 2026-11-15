# hermes - Agent Guide

This project delivers programs and configuration for the command line.

## Repository Layout

|Path|Purpose|
|:-|:-|
|`.devcontainer/`|[Development Container](https://containers.dev/) with the complete environment|
|`.github/`|GitHub-related content like CI/CD, issues, etc.|
|`cspell.config.yaml`|Spell-check configuration|
|`README.md`|Project description|
|`code/`|contains the source code|
|`code/Cargo.toml`|Rust package manifest|
|`code/cupid/`|the helper that collects all of hermes' programs and bundles them|
|`code/src/`|hermes' actual source code|
|`data`|contains all configuration file and notes|
|`data/home/.config/`|all programs' configuration files that hermes ships|
|`data/examples/`|custom example configurations that are not shipped with hermes|
|`data/manuals/`|written manuals for all kinds of work on the command line|
|`data/scripts/`|common useful scripts that I need from time to time|

## Programming Language

hermes is written in Rust.

### Version Information

The file [`code/rust-toolchain.toml`](./code/rust-toolchain.toml) contains information about the used toolchain version, targets, profiles, etc. Additionally, [`code/Cargo.toml`](./code/Cargo.toml) contains the used Rust edition.

### Build

The following commands show how to build hermes in release mode (omit the `--release` flag for debug mode):

```console
$ cd code
$ cargo run --package cupid --release
$ cargo build --release
```

### Lint & Style

The project is linted with clippy. The lint rules are very strict. Run the linter with

```console
$ cd code
$ cargo clippy --workspace --quiet --all-features -- -D warnings
$ cargo fmt --all -- --check
$ cargo doc --workspace --quiet --no-deps --document-private-items
```

The project also uses [EditorConfig](https://editorconfig.org/) for general-purpose style-enforcement.
