#! /usr/bin/env bash

# ██████╗  █████╗ ███████╗██╗  ██╗    ███████╗
# ██╔══██╗██╔══██╗██╔════╝██║  ██║    ██╔════╝
# ██████╔╝███████║███████╗███████║    ███████╗
# ██╔══██╗██╔══██║╚════██║██╔══██║    ╚════██║
# ██████╔╝██║  ██║███████║██║  ██║    ███████║
# ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝    ╚══════╝

# version       2.0.0
# executed by   Bash for non-login shells
# task          shell (Bash) initialization

# ! Do not edit this file.
# ! Use ${HOME}/.config/bash/99-custom.sh instead.

function bash_setup() {
  function load_helper() {
    local SETUP_FILE="${HOME}/.config/bash/${1}"
    # shellcheck source=/dev/null
    [[ -e ${SETUP_FILE} ]] && [[ -r ${SETUP_FILE} ]] && source "${SETUP_FILE}"
    return 0
  }

  load_helper '00-base.sh'

  # if not running interactively, don't do anything
  [[ ${-} != *i* ]] && return 0

  load_helper '10-misc.sh'
  load_helper '20-custom_early.sh'

  [[ ${HERMES_LOAD_EXTRA_PROGRAMS:-false} == 'true' ]] && load_helper '30-extra_programs.sh'
  [[ ${HERMES_LOAD_ALIASES:-false} == 'true' ]]        && load_helper '80-aliases.sh'
  [[ ${HERMES_LOAD_WRAPPER:-false} == 'true' ]]        && load_helper '90-wrapper.sh'

  load_helper '99-custom_late.sh'

  [[ -v BLE_VERSION ]] && ble-attach
}

bash_setup "${@}"
unset bash_setup
