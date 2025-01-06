#! /usr/bin/env bash

# sourced by  ${HOME}/.bashrc
#       task  overwrite well-known commands with modern alternatives

# shellcheck disable=SC2317

# ! Do not edit this file - use ${HOME}/.config/bash/{20-custom_early,99-custom_late}.sh instead.

# Overrides `cat` with `bat` but only if `bat` is available.
function __hermes__override_cat_with_bat() {
  __evaluates_to_true HERMES_OVERRIDE_CAT_WITH_BAT || return 0
  __is_command 'bat' && alias cat='bat --style=plain --paging=never'
}

# Overrides `cd` with `zoxide` but only if `zoxide` is available.
function __hermes__override_cd_with_zoxide() {
  __evaluates_to_true HERMES_OVERRIDE_CD_WITH_ZOXIDE || return 0
  __is_command 'zoxide' && alias cd='z'
}

# Overrides `find` with `fd` but only if `fd` is available.
function __hermes__override_find_with_fd() {
  __evaluates_to_true HERMES_OVERRIDE_FIND_WITH_FD || return 0
  __is_command 'fd' && alias find='fd'
}

# Overrides `grep` with `rg` but only if `rg` is available.
function __hermes__override_grep_with_ripgrep() {
  __evaluates_to_true HERMES_OVERRIDE_GREP_WITH_RIPGREP || return 0
  __is_command 'rg' && alias grep='rg'
}

# Overrides `ls` with `eza` but only if `eza` is available.
function __hermes__override_ls_with_eza() {
  __evaluates_to_true HERMES_OVERRIDE_LS_WITH_EZA || return 0
  __is_command 'eza' && alias ls='eza --header --long --binary --group --classify --extended --group-directories-first'
}

for __FUNCTION in 'cat_with_bat' 'cd_with_zoxide' 'find_with_fd' 'grep_with_ripgrep' 'ls_with_eza'; do
  "__hermes__override_${__FUNCTION}" || :
  unset "__hermes__override_${__FUNCTION}"
done
