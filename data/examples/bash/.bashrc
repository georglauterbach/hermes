#! /usr/bin/env bash

# ██████╗  █████╗ ███████╗██╗  ██╗
# ██╔══██╗██╔══██╗██╔════╝██║  ██║
# ██████╔╝███████║███████╗███████║
# ██╔══██╗██╔══██║╚════██║██╔══██║
# ██████╔╝██║  ██║███████║██║  ██║          task  user-wide shell (Bash) initialization
# ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝    sourced by  (usually) interactive non-login shells

# shellcheck source=/dev/null
source "${HOME}/.config/bash/90-hermes.sh"

# shellcheck disable=SC2139
alias v="${EDITOR:-vi}"
# shellcheck disable=SC2139
alias sv="sudo ${EDITOR:-vi}"

alias zj='zellij'

alias btm='btm --config_location "${HOME}/.config/bottom/config.toml"'

if __is_command 'theme'; then
  alias td='theme dark'
  alias tl='theme light'
fi

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

alias gcs='git commit --signoff --gpg-sign'
function git() {
  case "${1:-}" in
    ( 'update' )
      command git fetch -apt
      command git pull
      ;;

    ( 'diff' )
      if __is_command fzf && __is_command delta; then
        command git diff --name-only | \
          fzf --ansi --preview "git diff --color=always -- {-1} | delta"
      else
        command git "${@}"
      fi
      ;;

    ( * )
      command git "${@}"
      ;;
  esac
}
