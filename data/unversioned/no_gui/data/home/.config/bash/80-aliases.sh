#! /usr/bin/env bash

# version       0.4.0
# sourced by    ${HOME}/.bashrc
# task          configure Bash aliases

alias l='ls'
alias ll='lsa'
alias lsa='ls -a'

# `EDITOR` is defined in `10-setup.sh`
# shellcheck disable=SC2139,SC2154
alias v="${EDITOR:-vi}"
# shellcheck disable=SC2139
alias sv="sudo ${EDITOR:-vi}"

alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'
alias .....='cd ../../../..'
alias ......='cd ../../../../..'
alias .......='cd ../../../../../..'
