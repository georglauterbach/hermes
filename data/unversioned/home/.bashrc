#! /usr/bin/env bash

# ██████╗  █████╗ ███████╗██╗  ██╗    ███████╗
# ██╔══██╗██╔══██╗██╔════╝██║  ██║    ██╔════╝
# ██████╔╝███████║███████╗███████║    ███████╗
# ██╔══██╗██╔══██║╚════██║██╔══██║    ╚════██║
# ██████╔╝██║  ██║███████║██║  ██║    ███████║
# ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝    ╚══════╝

# version       3.0.0
# executed by   Bash for non-login shells
# task          shell (Bash) initialization

# ! Do not edit this file.
# ! Use ${HOME}/.config/bash/{20-custom_early,99-custom_late}.sh instead.

function __bash_setup() {
  local SETUP_FILE_PREFIX="${HOME}/.config/bash"

  function __load_script() {
    local SCRIPT_FILE=${1:?Path to script to load is required}
    # shellcheck source=/dev/null
    [[ -e ${SCRIPT_FILE} ]] && [[ -r ${SCRIPT_FILE} ]] && source "${SCRIPT_FILE}"
    return 0
  }

  function __load_helper() {
    local HELPER_SCRIPT_FILE="${SETUP_FILE_PREFIX}/${1}"
    __load_script "${HELPER_SCRIPT_FILE}"
  }

  __load_helper '00-base.sh'

  # if not running interactively, don't do anything
  [[ ${-} == *i* ]] || return 0

  __load_helper '10-misc.sh'
  __load_script "${HERMES_CUSTOM_EARLY_SCRIPT:-${SETUP_FILE_PREFIX}/20-custom_early.sh}"

  __evaluates_to_true HERMES_LOAD_EXTRA_PROGRAMS    'false' && __load_helper '30-extra_programs.sh'
  __evaluates_to_true HERMES_LOAD_OVERRIDE_COMMANDS 'false' && __load_helper '40-override_commands.sh'
  __evaluates_to_true HERMES_LOAD_GLOBAL_ALIASES    'false' && __load_helper '80-global_aliases.sh'

  __load_script "${HERMES_CUSTOM_LATE_SCRIPT:-${SETUP_FILE_PREFIX}/99-custom_late.sh}"

  [[ -v BLE_VERSION ]] && ble-attach
}

__bash_setup "${@}"
unset __bash_setup __load_script __load_helper
