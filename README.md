# hermes

_hermes_ installs [programs](#programs) and configuration files for the command line. The setup is non-intrusive (it does not overwrite existing files by default) and mostly unopinionated. _hermes_ is built for `x86_64` and `aarch64`.

## Usage

> [!tip]
>
> You do not need to manually save the `hermes_download_latest` function once you ran and source hermes because hermes exports it for you.

```bash
# Download the latest version of hermes and
# place hermes at ${HOME}/.local/bin/hermes
function hermes_download_latest() {
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

hermes_download_latest      # download hermes
"${HOME}/.local/bin/hermes" # execute hermes
```

## Additional Setup

### Supplementary Setup Scripts

You can find setup scripts that aid in setting up machines in [`data/scripts/`](./data/scripts/). You might also want to run `bat cache --build` to initialize `bat`'s theme cache.

### Examples

You can find some personal configuration files in [`data/examples/`](./data/examples/).

### Bash

#### Local

[`data/examples/bash/.bashrc`](./data/examples/bash/.bashrc) provides an example of what your `~/.bashrc` could look like with hermes. If you want to change the behavior of hermes' shell setup, update [`~/.config/bash/91-hermes_settings.sh`](./data/home/.config/bash/91-hermes_settings.sh).

hermes defines two types of functions: Those that start with `hermes_` can and should be used by you; those that start with `__hermes__` are for internal use and must not be used by you.

> [!important]
>
> To use [_flyline_](https://github.com/HalFrgrd/flyline), you need to manually update the symbolic link in `~/.local/lib/` after the initial installation and after updates! Navigate to `~/.local/lib/` and run `ln -sf libflyline.so.<VERSION> libflyline.so`.

#### SSH & Containers

hermes works flawlessly on remote hosts. The setup is the same for local and remote host, except for theming. If you want proper theming, you should

1. Enable theming locally with `HERMES_ENABLE_THEMING=true`
2. Add `SendEnv HERMES_THEME_VARIANT` to the desired host in `~/.ssh/config`
3. Enable theming on the remote (with `HERMES_ENABLE_THEMING=true`)
4. Adjust `/etc/ssh/sshd_config` on the remote to contain `AcceptEnv ... HERMES_*`
5. Restart `sshd` with `systemctl restart sshd` on the remote
6. Reconnect to the remote and run `hermes_switch_theme`

### Programs

> [!tip]
>
> If you are looking for more awesome terminal programs, head to [Terminal Trove](https://terminaltrove.com).

_hermes_ installs additional programs into `${HOME}/.local/bin/`.

- [_bat_](https://github.com/sharkdp/bat)
  - `cat` with syntax highlighting and git integration
  - enabled with `HERMES_INIT_BAT`, override `cat` with `HERMES_OVERRIDE_CAT_WITH_BAT`
  - enable coloring with `HERMES_OVERRIDE_COLORS_BAT`
  - create the theme cache with `bat cache --clear ; bat cache --build`
- [_btop_](https://github.com/aristocratos/btop)
  - a resource monitor
  - enable coloring with `HERMES_OVERRIDE_COLORS_BTOP`
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
  - override colors for punctuation with `HERMES_OVERRIDE_COLORS_EZA`
- [_fd_](https://github.com/sharkdp/fd)
  - fast, modern alternative to `find`
  - override `find` with `HERMES_OVERRIDE_FIND_WITH_FD`
- [_flyline_](https://github.com/HalFrgrd/flyline)
  - a Bash plugin to replace readline for a modern line editing experience
  - enabled with `HERMES_INIT_FLYLINE`
  - enable coloring with `HERMES_OVERRIDE_COLORS_FLYLINE`
- [_fzf_](https://github.com/junegunn/fzf)
  - general-purpose command-line fuzzy finder
  - enabled with `HERMES_INIT_FZF`
  - enable coloring with `HERMES_OVERRIDE_COLORS_FZF`
- [_gitui_](https://github.com/extrawurst/gitui)
  - a fast, modern TUI for `git`
  - enable coloring with `HERMES_OVERRIDE_COLORS_GITUI`
- [_jaq_](https://github.com/01mf02/jaq)
  - a `jq` clone focussed on correctness, speed, and simplicity
  - override `jq` with `HERMES_OVERRIDE_JQ_WITH_JAQ`
- [_just_](https://github.com/casey/just)
  - just a command runner
- [_ripgrep_](https://github.com/BurntSushi/ripgrep)
  - fast, modern alternative to `grep`
  - override `grep` with `HERMES_OVERRIDE_GREP_WITH_RIPGREP`
- [_starship_](https://github.com/starship/starship)
  - minimal, blazing-fast, and infinitely customizable prompt for any shell
  - enabled with `HERMES_INIT_STARSHIP`
  - generate completion by running `starship completions bash >"${XDG_DATA_HOME:-"${HOME}/.local/share"}/bash-completion/completions/starship.bash"`
- [_stinkpot_](https://github.com/georglauterbach/stinkpot-rs)
  - SQLite-backed shell-history searcher
  - enable with `HERMES_INIT_STINKPOT`
- [_yazi_](https://github.com/sxyazi/yazi)
  - blazing fast terminal file manager
  - set/override `y` with `HERMES_OVERRIDE_Y_WITH_YAZI`
  - for optional extensions, take a look at [the installation documentation](https://yazi-rs.github.io/docs/installation)
- [_zoxide_](https://github.com/ajeetdsouza/zoxide)
  - smarter cd command
  - enabled with `HERMES_INIT_ZOXIDE`, override `cd` with `HERMES_OVERRIDE_CD_WITH_ZOXIDE`
