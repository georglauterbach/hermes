# _hermes_

Delivers setup and configuration for your CLI like a god. But actually a glorified tar-decompressor.

## About

_hermes_ places [selected programs and configuration files](#programs) for your user. The configuration is mostly unopinionated, non-intrusive, and tries to enhance the out-of-the-box experience of your CLI. _hermes_ is built for `x86_64` and `aarch64`.

## Usage

To **download** the latest version of _hermes_, run the following commands:

```bash
function download_hermes_latest() {
  local HERMES_LOCATION="${HOME}/.local/bin/hermes"
  local HERMES_RELEASE_URI_BASE='https://github.com/georglauterbach/hermes/releases'
  local HERMES_VERSION

  HERMES_VERSION=$(curl --silent --show-error --fail --location --write-out '%{url_effective}' --output /dev/null \
    "${HERMES_RELEASE_URI_BASE}/latest" | sed 's|.*/||')

  mkdir --parents "$(dirname "${HERMES_LOCATION}")"
  curl --silent --show-error --fail --location --output "${HERMES_LOCATION}" \
    "${HERMES_RELEASE_URI_BASE}/download/${HERMES_VERSION}/hermes-${HERMES_VERSION}-$(uname -m)-unknown-linux-musl"

  chmod +x "${HERMES_LOCATION}"
}

# download hermes
download_hermes_latest

# execute hermes
"${HOME}/.local/bin/hermes"
```

## Additional Setup

### Bash

1. **To enable _hermes_**: add `source "${HOME}/.config/bash/90-hermes.sh"` to (the end of) your `${HOME}/.bashrc`
2. **To modify _hermes_**: edit the file [`${HOME}/.config/bash/91-hermes_settings.sh`](./data/config/bash/91-hermes_settings.sh)

### Programs

_hermes_ installs additional programs into `${HOME}/.local/bin/`. These programs include:

- [_Atuin_](https://github.com/atuinsh/atuin)
  - "magical" shell history using SQLite rather than a file
  - enabled with `HERMES_INIT_ATUIN`
  - `CTRL+e` (or `up-arrow` when `HERMES_CONFIG_ATUIN_DISABLE_UP_ARROW=false`) brings up the history
  - setting `HERMES_CONFIG_ATUIN_DB_FILE` changes the database file
- [_bat_](https://github.com/sharkdp/bat)
  - `cat` with syntax highlighting and git integration
  - enabled with `HERMES_INIT_BAT`, override `cat` with `HERMES_OVERRIDE_CAT_WITH_BAT`
- [_bottom_](https://github.com/ClementTsang/bottom)
  - cross-platform graphical process/system monitor and `<X>top` replacement
- [_ble.sh_](https://github.com/akinomyoga/ble.sh)
  - command line editor written in pure Bash which replaces the default GNU Readline
  - enabled with `HERMES_INIT_BLE_SH`
- [_delta_](https://github.com/dandavison/delta)
  - syntax-highlighting pager for `git`, `diff`, `grep`, and `blame` output
  - override `diff` with `HERMES_OVERRIDE_DIFF_WITH_DELTA`
- [_dust_](https://github.com/bootandy/dust)
  - a more intuitive version of `du`
- [_dysk_](https://github.com/Canop/dysk)
  - get information on filesystems, like `df`, but better
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
- [_just_](https://github.com/casey/just)
  - just a command runner
- [_ripgrep_](https://github.com/BurntSushi/ripgrep)
  - fast, modern alternative to `grep`
  - override `grep` with `HERMES_OVERRIDE_GREP_WITH_RIPGREP`
- [_starship_](https://github.com/starship/starship)
  - minimal, blazing-fast, and infinitely customizable prompt for any shell
  - enabled with `HERMES_INIT_STARSHIP`
- [_yazi_](https://github.com/sxyazi/yazi)
  - blazing fast terminal file manager
  - set/override `y` with `HERMES_OVERRIDE_Y_WITH_YAZI`
  - for optional extensions, take a look at [the installation documentation](https://yazi-rs.github.io/docs/installation)
- [_zoxide_](https://github.com/ajeetdsouza/zoxide)
  - smarter cd command
  - enabled with `HERMES_INIT_ZOXIDE`, override `cd` with `HERMES_OVERRIDE_CD_WITH_ZOXIDE`
- [_zellij_](https://github.com/zellij-org/zellij)
  - terminal workspace with batteries included

The following statically-compiled programs are currently only available on `x86_64`:

- [_git_](https://github.com/darkvertex/static-git)
  - a distributed version control software system that is capable of managing versions of source code or data
- [_Neovim_](https://github.com/neovim/neovim/blob/master/BUILD.md#build-static-binary-linux)
  - modern, fast and feature-rich editor

### Supplementary Setup Scripts

You can find additional setup scripts that aid in setting up machines under the [`data/scripts/` directory](./data/scripts/).

### Examples

You can find setup examples in the [`data/examples/`](./data/examples/) directory. A custom GUI setup can be found there too.
