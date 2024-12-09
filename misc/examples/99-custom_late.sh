#! /usr/bin/env bash

# version       0.1.0
# sourced by    ${HOME}/.bashrc
# task          user-customizable shell setup

# ! You can edit this file to change the behavior of your shell
# ! after _hermes_ finished loading.

if __is_command 'doas'; then
  complete -cf doas
  alias sudo='doas'
fi

if __is_command 'kubectl'; then
  complete -o default -F __start_kubectl k
  alias k='kubectl'
fi

alias gcs='git commit'
alias shutn='shutdown now'

if __is_command 'gitui'; then
  alias g='gitui'
elif __is_command 'lazygit'; then
  alias g='lazygit'
else
  alias g='git diff'
fi

if __is_command 'btop'; then
  alias htop='btop'
fi

if __is_command 'zellij'; then
  alias tmux='zellij'
fi

function git() {
  case "${1:-}" in
    ( 'update' )
      git fetch --all --tags --prune
      git pull
      git submodule update --recursive
      ;;
    ( * ) command git "${@}" ;;
  esac
}
