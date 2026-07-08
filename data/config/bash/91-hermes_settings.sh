#! /usr/bin/env bash

# ! Customize hermes
#   Sourced by "${HOME}/.config/bash/90-hermes.sh"

# -----------------------------------------------
# ----  Program Initialization  -----------------
# -----------------------------------------------

export HERMES_INIT_BAT=${HERMES_INIT_BAT:-true}
export HERMES_INIT_FLYLINE=${HERMES_INIT_FLYLINE:-true}
export HERMES_INIT_FZF=${HERMES_INIT_FZF:-true}
export HERMES_INIT_STARSHIP=${HERMES_INIT_STARSHIP:-true}
export HERMES_INIT_ZOXIDE=${HERMES_INIT_ZOXIDE:-true}

# -----------------------------------------------
# ----  System Command Overrides  ---------------
# -----------------------------------------------

export HERMES_OVERRIDE_CAT_WITH_BAT=${HERMES_OVERRIDE_CAT_WITH_BAT:-true}
export HERMES_OVERRIDE_CD_WITH_ZOXIDE=${HERMES_OVERRIDE_CD_WITH_ZOXIDE:-true}
export HERMES_OVERRIDE_DIFF_WITH_DELTA=${HERMES_OVERRIDE_DIFF_WITH_DELTA:-true}
export HERMES_OVERRIDE_LESS_WITH_BAT=${HERMES_OVERRIDE_LESS_WITH_BAT:-true}
export HERMES_OVERRIDE_LS_WITH_EZA=${HERMES_OVERRIDE_LS_WITH_EZA:-true}
export HERMES_OVERRIDE_Y_WITH_YAZI=${HERMES_OVERRIDE_Y_WITH_YAZI:-true}

# -----------------------------------------------
# ----  Miscellaneous  --------------------------
# -----------------------------------------------

# This settings loads useful aliases that are very likely to not override anything.
export HERMES_LOAD_GLOBAL_ALIASES=${HERMES_LOAD_GLOBAL_ALIASES:-true}
