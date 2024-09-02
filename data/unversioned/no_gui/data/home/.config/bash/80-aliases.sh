#! /usr/bin/env bash

# version       0.3.0
# sourced by    ${HOME}/.bashrc
# task          configure Bash aliases

alias l='ls'
alias ll='lsa'
alias lsa='ls -a'

alias gcs='git commit'

# `EDITOR` is defined in `10-setup.sh`
# shellcheck disable=SC2139,SC2154
alias v="${EDITOR}"
# shellcheck disable=SC2139
alias sv="sudo ${EDITOR}"

alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'
alias .....='cd ../../../..'
alias ......='cd ../../../../..'
alias .......='cd ../../../../../..'

if __hermes__command_exists 'gitui'; then
  alias g='gitui'
elif __hermes__command_exists 'lazygit'; then
  alias g='lazygit'
else
  alias g='git diff'
fi

if __hermes__command_exists 'btop'; then
  alias htop='btop'
fi

if __hermes__command_exists 'zellij'; then
  alias tmux='zellij'
fi
