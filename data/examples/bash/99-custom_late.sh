#! /usr/bin/env bash

# sourced by    ${HOME}/.bashrc
# task          user-customizable shell setup

# ! You can edit this file to change the behavior of your shell
# ! after _hermes_ finished loading.

# shellcheck disable=SC2139
alias v="${EDITOR:-vi}"
# shellcheck disable=SC2139
alias sv="sudo ${EDITOR:-vi}"

alias zj='zellij'

if __is_command 'doas'; then
  complete -cf doas
  alias sudo='doas'
fi

if __is_command 'kubectl'; then
  complete -o default -F __start_kubectl k
  alias k='kubectl'
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
