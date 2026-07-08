# hermes - Agent Guide

This project delivers programs and configuration for the command line.

## Associated Projects

1. [`github.com/georglauterbach/desktop`](https://github.com/georglauterbach/desktop): Like hermes, but for the GUI
2. [`github.com/georglauterbach/evergruv`](https://github.com/georglauterbach/evergruv): My color scheme for everything
3. [`github.com/georglauterbach/linter`](https://github.com/georglauterbach/linter): A composite linter for all of my projects

## Repository Layout

| Path             | Purpose                                                           |
| :--------------- | :---------------------------------------------------------------- |
| `.devcontainer/` | [Development Container] with the complete environment             |
| `.github/`       | GitHub-related content like CI/CD, issues, etc.                   |
| `code/`          | contains all source code                                          |
| `code/cupid/`    | the helper that collects all of hermes' programs and bundles them |
| `code/src/`      | hermes' actual source code                                        |
| `data/config/`   | all programs' configuration files that hermes ships               |
| `data/examples/` | custom example configurations that are not shipped with hermes    |
| `data/manuals/`  | written manuals for all kinds of work on the command line         |
| `data/scripts/`  | common useful scripts that I need from time to time               |

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

The project also uses [EditorConfig] for general-purpose style-enforcement.

[//]: # (Links)

[Development Container]: https://containers.dev/
[EditorConfig]: https://editorconfig.org/
