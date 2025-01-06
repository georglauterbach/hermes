#! /usr/bin/env bash

# sourced by  ${HOME}/.bashrc
#       task  configure universal Bash aliases

# ! Do not edit this file - use ${HOME}/.config/bash/{20-custom_early,99-custom_late}.sh instead.

__evaluates_to_true HERMES_LOAD_GLOBAL_ALIASES || return 0

alias lsa='ls -a'

alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'
alias .....='cd ../../../..'
alias ......='cd ../../../../..'
alias .......='cd ../../../../../..'
