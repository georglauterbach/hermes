# _hermes_

_hermes_ installs [programs](#programs) and configuration files for the command line. The setup is non-intrusive (it does not overwrite existing files by default) and mostly unopinionated. _hermes_ is built for `x86_64` and `aarch64`.

## Usage

```bash
# Download the latest version of hermes and
# place hermes at ${HOME}/.local/bin/hermes
function download_hermes_latest() {
  local HERMES="${HOME}/.local/bin/hermes"
  local RELEASE_URI_BASE='https://github.com/georglauterbach/hermes/releases'
  local VERSION

  VERSION=$(curl --silent --show-error --fail --location \
    --write-out '%{url_effective}' --output /dev/null    \
    "${RELEASE_URI_BASE}/latest" | sed 's|.*/||')

  mkdir --parents "$(dirname "${HERMES}")"
  curl --silent --show-error --fail --location --output "${HERMES}" \
    "${RELEASE_URI_BASE}/download/${VERSION}/hermes-${VERSION}-$(uname -m)-unknown-linux-musl"

  chmod +x "${HERMES}"
}

download_hermes_latest      # download hermes
"${HOME}/.local/bin/hermes" # execute hermes
```

## Additional Setup

### Supplementary Setup Scripts

You can find setup scripts that aid in setting up machines in [`data/scripts/`](./data/scripts/). You might also want to run `bat cache --build` to initialize `bat`'s theme cache.

### Examples

You can find some personal configuration files in [`data/examples/`](./data/examples/).

### Bash

_hermes_'s command line setup focuses on Bash. Use [`source "${HOME}/.config/bash/90-hermes.sh"`](./data/config/bash/90-hermes.sh) in `${HOME}/.bashrc` to load the setup. To modify the setup, adjust [`${HOME}/.config/bash/91-hermes_settings.sh`](./data/config/bash/91-hermes_settings.sh).


### Programs

_hermes_ installs additional programs into `${HOME}/.local/bin/`.

- [_Atuin_](https://github.com/atuinsh/atuin)
  - "magical" shell history using SQLite rather than a file
  - enabled with `HERMES_INIT_ATUIN`
  - `CTRL+e` (or `up-arrow` when `HERMES_CONFIG_ATUIN_DISABLE_UP_ARROW=false`) brings up the history
  - setting `HERMES_CONFIG_ATUIN_DB_FILE` changes the database file
- [_bat_](https://github.com/sharkdp/bat)
  - `cat` with syntax highlighting and git integration
  - enabled with `HERMES_INIT_BAT`, override `cat` with `HERMES_OVERRIDE_CAT_WITH_BAT`
- [_ble.sh_](https://github.com/akinomyoga/ble.sh)
  - command line editor written in pure Bash which replaces the default GNU Readline
  - enabled with `HERMES_INIT_BLE_SH`
- [_btop_](https://github.com/aristocratos/btop)
  - a resource monitor
  - consider running `sudo setcap cap_perfmon=+ep "$(command -v btop)"` to set the [`perfmon` capability](https://github.com/torvalds/linux/blob/master/Documentation/admin-guide/perf-security.rst) for btop
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

The following programs are currently only available on `x86_64`:

- [_neovim_](https://github.com/neovim/neovim/blob/master/BUILD.md#build-static-binary-linux)
  - modern, fast and feature-rich editor
