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
      git fetch -apt
      git pull
      ;;
    ( * )
      command git "${@}"
      ;;
  esac
}

function theme() {
  case "${1:-dark}" in
    ( 'dark' )
      gsettings set org.gnome.desktop.interface gtk-theme Adwaita-dark
      gsettings set org.gnome.desktop.interface color-scheme prefer-dark
      ;;

    ( 'light' )
      gsettings set org.gnome.desktop.interface gtk-theme Adwaita
      gsettings set org.gnome.desktop.interface color-scheme prefer-light
      ;;

    ( * )
      echo "'${1}' is not a valid option - use 'light' or 'dark'" >&2
      return 1
      ;;
  esac
}
