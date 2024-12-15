# _hermes_

Delivers setup and configuration for _Ubuntu_ like a god.

## About

_hermes_ configures _Ubuntu_ by installing various packages and (re-)placing configuration files. The setup is mostly unopinionated, non-intrusive, and tries to enhance the out-of-the-box experience of _Ubuntu_. Optionally, configuration of the GUI is supported. _hermes_ is built for `x86_64` (full support) and `aarch64` (only non-GUI).

## Usage

> [!NOTE]
> While optional, I recommend running `sudo apt-get --yes --quiet=2 update && sudo apt-get --yes --quiet=2 upgrade` before using _hermes_.

Run the following commands to acquire the latest version of _hermes_:

```console
$ HERMES_VERSION="$(curl -sSIL -w '%{url_effective}' -o /dev/null \
  "https://github.com/georglauterbach/hermes/releases/latest" | sed 's|.*/||')"
$ sudo curl --silent --show-error --fail --location --output /usr/local/bin/hermes \
  "https://github.com/georglauterbach/hermes/releases/download/${HERMES_VERSION}/hermes-${HERMES_VERSION}-$(uname -m)-unknown-linux-musl"
$ sudo chmod +x /usr/local/bin/hermes
$ hermes --help
```

To update _hermes_, run the following commands:

```console
$ hermes --update --non-interactive
```

> [!CAUTION]
> _hermes_ overwrites files like `.bashrc`. You should make a backup of your configuration files before running _hermes_. You can later re-introduce the code from your `.bashrc` into a new file \[[1](#bash)\].

## Optional Additional Setup

### Bash

The setup of Bash is performed by `${HOME}/.bashrc` and scripts in `${HOME}/.config/bash/`. These setup files can be found [here](data/unversioned/home/.config/).

If you want to modify the havior of _hermes_, take a look at `{HOME}/.config/bash/20-custom_early.sh`. This file contains variables that control the initialization of programs and their overrides as well as other configurations. The "[Programs](#programs)" section below referrs to these variables.

To add code that would normally go to `.bashrc`, edit `${HOME}/.config/bash/99-cutom_late.sh`. These two files are not overwritten by _hermes_ if you run _hermes_ again.

### Programs

_hermes_ installs additional programs into `${HOME}/.local/bin/`. These programs include:

- [_Atuin_](https://github.com/atuinsh/atuin)
  - "magical" shell history using SQLite rather than a file
  - enbled with `HERMES_INIT_ATUIN`
  - `CTRL+e` (or up-arrow with `HERMES_CONFIG_ATUIN_DISABLE_UP_ARROW`) brings up the history
  - setting `HERMES_CONFIG_ATUIN_DB_FILE` changes the database file
- [_bat_](https://github.com/sharkdp/bat)
  - `cat` with syntax highlighting and git integration
  - enbled with `HERMES_INIT_BAT`, override `cat` with `HERMES_OVERRIDE_CAT_WITH_BAT`
- [_ble.sh_](https://github.com/akinomyoga/ble.sh)
  - command line editor written in pure Bash which replaces the default GNU Readline
  - enabled with `HERMES_INIT_BLE_SH`
- [_eza_](https://github.com/eza-community/eza)
  - fast, modern alternative to `ls`
  - override `ls` with `HERMES_OVERRIDE_LS_WITH_EZA`
- [_fd_](https://github.com/sharkdp/fd)
  - fast, modern alternative to `find`
  - override `find` with `HERMES_OVERRIDE_FIND_WITH_FD`
- [_fzf_](https://github.com/junegunn/fzf)
  - general-purpose command-line fuzzy finder
  - enabled with `HERMES_INIT_FZF`
- [_gitui_](https://github.com/extrawurst/gitui)
  - a fast, modern TUI for `git`
- [_ripgrep_](https://github.com/BurntSushi/ripgrep)
  - fast, modern alternative to `grep`
  - override `grep` with `HERMES_OVERRIDE_GREP_WITH_RIPGREP`
- [_starship_](https://github.com/starship/starship)
  - minimal, blazing-fast, and infinitely customizable prompt for any shell
  - enbled with `HERMES_INIT_STARSHIP`
- [_zoxide_](https://github.com/ajeetdsouza/zoxide)
  - smarter cd command
  - enbled with `HERMES_INIT_ZOXIDE`, override `cd` with `HERMES_OVERRIDE_CD_WITH_ZOXIDE`

### Changing APT Sources

To change and add APT sources (including PPAs), run _hermes_ with the `--change-apt-sources` (or `-c`) flag. This option installs a new `ubuntu.sources` file in `/etc/apt/sources.list.d/` and adds additonal PPAs for `git`, `neovim`, and `flatpak`.

### GUI

> [!WARNING]
> Support for installing a GUI via [_Regolith Linux_](https://regolith-desktop.com/) is experimental. I am currently working on the Sway integration but lack Wayland capabilities because of an old graphics card of mine.

To set up a GUI, run _hermes_ with the `--gui` flag. This option installs new PPAs for _Regolith Linux_, [_Alacritty_](https://github.com/alacritty/alacritty), [_Cryptomator_](https://github.com/cryptomator/cryptomator), and [_Visual Studio Code_](https://github.com/microsoft/vscode). _hermes_ installs _Regolith Linux_, _Alacritty_ and _VS Code_, and provides a default configuratin for _Alacritty_.

### Supplementary Setup Scripts

You can find additional setup scripts that aid in setting up machines under the [`./misc/` directory](./misc/).

## Architecture

_hermes_ is written in the [Rust programming language](https://www.rust-lang.org/). As it performs a lot of network requests and disk I/O, it utilizes asynchronous programming with the [_tokio_](https://tokio.rs/) runtime. _hermes_ performs four main tasks:

1. Downloading and placing [unversioned configuration files](./data/unversioned/).
2. Downloading and placing [versioned configuration files](./data/versioned/).
3. Setting up APT (optionally) and installing additional programs with it.
4. Installing [additional programs](#programs) by downloading them from GitHub and unpacking them.

All of these tasks are performed asynchronously, which gives the illusion of parallelism (especially in the log). This makes _hermes_ extremely fast.

Most configuration is stored in the [`data/`](./code/src/library/data/) directory and is downloaded onto the target system.
