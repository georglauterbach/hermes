#! /usr/bin/env bash

# ! Customize hermes
#   Sourced by "${HOME}/.config/bash/90-hermes.sh"

# -----------------------------------------------
# ----  Program Initialization  -----------------
# -----------------------------------------------

HERMES_INIT_BAT=${HERMES_INIT_BAT:-true}
HERMES_INIT_FLYLINE=${HERMES_INIT_FLYLINE:-true}
HERMES_INIT_FZF=${HERMES_INIT_FZF:-true}
HERMES_INIT_STARSHIP=${HERMES_INIT_STARSHIP:-true}
HERMES_INIT_STINKPOT=${HERMES_INIT_STINKPOT:-true}
HERMES_INIT_ZOXIDE=${HERMES_INIT_ZOXIDE:-true}

# -----------------------------------------------
# ----  System Command Overrides  ---------------
# -----------------------------------------------

HERMES_OVERRIDE_CAT_WITH_BAT=${HERMES_OVERRIDE_CAT_WITH_BAT:-true}
HERMES_OVERRIDE_CD_WITH_ZOXIDE=${HERMES_OVERRIDE_CD_WITH_ZOXIDE:-true}
HERMES_OVERRIDE_DIFF_WITH_DELTA=${HERMES_OVERRIDE_DIFF_WITH_DELTA:-true}
HERMES_OVERRIDE_LESS_WITH_BAT=${HERMES_OVERRIDE_LESS_WITH_BAT:-true}
HERMES_OVERRIDE_LS_WITH_EZA=${HERMES_OVERRIDE_LS_WITH_EZA:-true}
HERMES_OVERRIDE_Y_WITH_YAZI=${HERMES_OVERRIDE_Y_WITH_YAZI:-true}

# -----------------------------------------------
# ----  Theming  --------------------------------
# -----------------------------------------------

HERMES_ENABLE_THEMING=${HERMES_ENABLE_THEMING:-false}

HERMES_THEME_BAT=${HERMES_THEME_BAT:-true}
HERMES_THEME_BTOP=${HERMES_THEME_BTOP:-true}
HERMES_THEME_EZA=${HERMES_THEME_EZA:-true}
HERMES_THEME_FLYLINE=${HERMES_THEME_FLYLINE:-true}
HERMES_THEME_FZF=${HERMES_THEME_FZF:-true}
HERMES_THEME_GITUI=${HERMES_THEME_GITUI:-true}
HERMES_THEME_HERDR=${HERMES_THEME_HERDR:-true}

# -----------------------------------------------
# ----  Miscellaneous  --------------------------
# -----------------------------------------------

HERMES_ENABLE_ADDITIONAL_ALIASES=${HERMES_ENABLE_ADDITIONAL_ALIASES:-true}
HERMES_ENABLE_EXPORT_OF_ENVS=${HERMES_ENABLE_EXPORT_OF_ENVS:-false}
