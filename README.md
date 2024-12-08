# _hermes_

Delivers setup and configuration for _Ubuntu_ like a god.

## About

_hermes_ configures _Ubuntu_ by installing various packages and (re-)placing configuration files. The setup is mostly unopinionated, it is non-intrusive, and tries to enhance the out-of-the-box experience of _Ubuntu_. Optionally, configuration of the GUI is supported. _hermes_ is built for `x86_64` (full support) and `aarch64` (only non-GUI).

## Usage

Go to [the latest release](https://github.com/georglauterbach/hermes/releases/latest) and note the version number. Then run

```console
$ HERMES_VERSION=<VERSION>
$ sudo curl --silent --show-error --fail --location --output /usr/local/bin/hermes \
  "https://github.com/georglauterbach/hermes/releases/download/v${HERMES_VERSION}/hermes-v${HERMES_VERSION}-$(uname -m)-unknown-linux-musl"
$ chmod +x /usr/local/bin/hermes
$ hermes --help
```

> [!CAUTION]
> _hermes_ overwrites files like `.bashrc`. You should make a backup of your configuration files before running _hermes_. You can later re-introduce the code from your `.bashrc` into a new file \[[1](#bash)\].

## Optional Additional Setup

### Programs

_hermes_ installs additional programs. To take a look which ones, visit [`programs.rs`](code/src/library/work/programs.rs). The programs are installed to `${HOME}/.local/bin/`.

### Bash

The setup of Bash is performed by `${HOME}/.bashrc` and scripts loaded by this file in `${HOME}/.config/bash/`. The associated setup files can be found [here](data/unversioned/home/.config/bash). If you want to modify the havior of _hermes_, take a look at `{HOME}/.config/bash/20-custom_early.sh`. To add code that would normally go to `.bashrc`, edit `${HOME}/.config/bash/99-cutom_late.sh`. These two files are not overwritten by _hermes_ if you run _hermes_ again.

### Updating APT

To change and add APT sources (including PPAs), run _hermes_ with the `--change-apt-sources` (or `-c`) flag. This option installs a new `ubuntu.sources` file in `/etc/apt/sources.list.d/` and adds additonal PPAs for `git`, `neovim`, and `flatpak`.

### GUI

> [!WARNING]
> Support for installing a GUI via [_Regolith Linux_](https://regolith-desktop.com/) is experimental. I am currently working on the Sway integration but lack Wayland capabilities because of an old graphics card of mine.

To set up a GUI, run _hermes_ with the `--gui` flag. This option installs new PPAs for _Regolith Linux_, [_Alacritty_](https://github.com/alacritty/alacritty), [_Cryptomator_](https://github.com/cryptomator/cryptomator), and [_Visual Studio Code_](https://github.com/microsoft/vscode). It installs _Regolith Linux_, _Alacritty_ and _VS Code_, and provides a default configuratin for _Alacritty_.

### Supplementary Setup Scripts

Under the [`./misc/` directory](./misc/), you can find additional setup scripts that aid in setting up machines.

## Architecture

TODO
