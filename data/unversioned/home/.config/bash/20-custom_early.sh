#! /usr/bin/env bash

# version       1.0.0
# sourced by    ${HOME}/.bashrc
# task          user-customizable loading of _hermes_

# ! You can edit this file to change the behavior of _hermes_.

# The first variable is a "global" settings about
# whether to load all extra programs. The following
# varaibles control the laod of individual programs.
#
# HERMES_LOAD_EXTRA_PROGRAMS must be `true` in order
# to be able to load any extra programs!
export HERMES_LOAD_EXTRA_PROGRAMS=false
export HERMES_LOAD_EXTRA_PROGRAMS_ATUIN=false
export HERMES_LOAD_EXTRA_PROGRAMS_BAT=true
export HERMES_LOAD_EXTRA_PROGRAMS_BLE_SH=true
export HERMES_LOAD_EXTRA_PROGRAMS_FZF=true
export HERMES_LOAD_EXTRA_PROGRAMS_STARSHIP=true
export HERMES_LOAD_EXTRA_PROGRAMS_ZOXIDE=true

# These variables control whether you want to have default
# commands (like `ls`) overridden by another, more advanced
# command (like `eza`).
#
# Initialization of advanced commands previously is independent
# of overriding system commands: Maximum flexibility.
export HERMES_LOAD_OVERRIDE_COMMANDS=true
export HERMES_OVERRIDE_APT_WITH_NALA=false
export HERMES_OVERRIDE_CAT_WITH_BAT=false
export HERMES_OVERRIDE_CD_WITH_ZOXIDE=false
export HERMES_OVERRIDE_FIND_WITH_FD=false
export HERMES_OVERRIDE_GREP_WITH_RIPGREP=false
export HERMES_OVERRIDE_LS_WITH_EZA=false

# This settings loads useful aliases that are very likely to
# not override anything.
export HERMES_LOAD_GLOBAL_ALIASES=true
