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
export HERMES_INIT_ZELLIJ=${HERMES_INIT_ZELLIJ:-true}
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
# ----  Colors  ---------------------------------
# -----------------------------------------------

export HERMES_COLOR_BAT=${HERMES_COLOR_BAT:-true}
export HERMES_COLOR_BTOP=${HERMES_COLOR_BTOP:-true}
export HERMES_COLOR_EZA=${HERMES_COLOR_EZA:-true}
export HERMES_COLOR_FLYLINE=${HERMES_COLOR_FLYLINE:-true}
export HERMES_COLOR_FZF=${HERMES_COLOR_FZF:-true}
export HERMES_COLOR_GITUI=${HERMES_COLOR_GITUI:-true}

# -----------------------------------------------
# ----  Miscellaneous  --------------------------
# -----------------------------------------------

# Loads additional aliases
export HERMES_LOAD_ADDITIONAL_ALIASES=${HERMES_LOAD_ADDITIONAL_ALIASES:-true}

# Whether to automatically start Zellij
# 1. When a shell is started in a git repository (with the directory name
#    being a part of the session name)
export HERMES_ZELLIJ_AUTO_ATTACH_GIT=${HERMES_ZELLIJ_AUTO_ATTACH_GIT:-false}
# 2. When a shell is started in general
export HERMES_ZELLIJ_AUTO_RUN=${HERMES_ZELLIJ_AUTO_RUN:-false}
