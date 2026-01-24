#! /usr/bin/env bash

# ██████╗  █████╗ ███████╗██╗  ██╗
# ██╔══██╗██╔══██╗██╔════╝██║  ██║
# ██████╔╝███████║███████╗███████║
# ██╔══██╗██╔══██║╚════██║██╔══██║
# ██████╔╝██║  ██║███████║██║  ██║          task  user-wide shell (Bash) initialization
# ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝    sourced by  (usually) interactive non-login shells

# shellcheck source=/dev/null
source "${HOME}/.config/bash/90-hermes.sh"

alias btm='btm --config_location "${HOME}/.config/bottom/config.toml"'

if __is_command 'doas'; then
  complete -cf doas
  alias sudo='doas'
fi

if __is_command 'gitui'; then
  alias g='gitui'
elif __is_command 'lazygit'; then
  alias g='lazygit'
else
  alias g='git diff'
fi
