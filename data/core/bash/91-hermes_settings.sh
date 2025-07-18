#! /usr/bin/env bash

#          task  customize hermes
#    sourced by  ${HOME}/.config/bash/90-hermes.sh

# -----------------------------------------------
# ----  Extra Program Initialization  -----------
# -----------------------------------------------

# These variables control whether additional programs ought to be initialized.

export HERMES_INIT_ATUIN=${HERMES_INIT_ATUIN:-false}
export HERMES_INIT_BAT=${HERMES_INIT_BAT:-false}
export HERMES_INIT_BLE_SH=${HERMES_INIT_BLE_SH:-false}
export HERMES_INIT_FZF=${HERMES_INIT_FZF:-false}
export HERMES_INIT_STARSHIP=${HERMES_INIT_STARSHIP:-false}
export HERMES_INIT_ZOXIDE=${HERMES_INIT_ZOXIDE:-false}

# -----------------------------------------------
# ----  System Command Overrides  ---------------
# -----------------------------------------------

# These variables control whether you want to have default commands (like `ls`)
# overridden by another, more advanced command (like `eza`).
#
# Initialization of additional programs previously is independent
# of overriding system commands.

export HERMES_OVERRIDE_CAT_WITH_BAT=${HERMES_OVERRIDE_CAT_WITH_BAT:-false}
export HERMES_OVERRIDE_CD_WITH_ZOXIDE=${HERMES_OVERRIDE_CD_WITH_ZOXIDE:-false}
export HERMES_OVERRIDE_DIFF_WITH_DELTA=${HERMES_OVERRIDE_DIFF_WITH_DELTA:-false}
export HERMES_OVERRIDE_FIND_WITH_FD=${HERMES_OVERRIDE_FIND_WITH_FD:-false}
export HERMES_OVERRIDE_GREP_WITH_RIPGREP=${HERMES_OVERRIDE_GREP_WITH_RIPGREP:-false}
export HERMES_OVERRIDE_LESS_WITH_BAT=${HERMES_OVERRIDE_LESS_WITH_BAT:-false}
export HERMES_OVERRIDE_LS_WITH_EZA=${HERMES_OVERRIDE_LS_WITH_EZA:-false}
export HERMES_OVERRIDE_Y_WITH_YAZI=${HERMES_OVERRIDE_Y_WITH_YAZI:-false}

# -----------------------------------------------
# ----  Miscellaneous  --------------------------
# -----------------------------------------------

# This settings loads useful aliases that are very likely to not override anything.
export HERMES_LOAD_GLOBAL_ALIASES=${HERMES_LOAD_GLOBAL_ALIASES:-false}

# -----------------------------------------------
# ----  Individual Program Configurations  ------
# -----------------------------------------------

# Configures the location of Atuin's SQLite database file
export HERMES_CONFIG_ATUIN_DB_FILE=${HERMES_CONFIG_ATUIN_DB_FILE:-}
# Enable a fallback history file even when Atuin is enabled
export HERMES_CONFIG_ATUIN_WITH_HISTFILE=${HERMES_CONFIG_ATUIN_WITH_HISTFILE:-/dev/null}
# Controls whether "`CTRL+f` to open the history" should be disabled for Atuin; if
# this option is disabled, `CTRL+e` is used
export HERMES_CONFIG_ATUIN_DISABLE_CTRL_F=${HERMES_CONFIG_ATUIN_DISABLE_CTRL_F:-true}
# Controls whether "up-arrow to open the history" should be disabled for Atuin
export HERMES_CONFIG_ATUIN_DISABLE_UP_ARROW=${HERMES_CONFIG_ATUIN_DISABLE_UP_ARROW:-false}
