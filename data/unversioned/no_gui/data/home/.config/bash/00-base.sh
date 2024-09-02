#! /usr/bin/env bash

# version       1.1.0
# sourced by    ${HOME}/.bashrc or manually
# task          this file should be usable even when not
#               running interactively to provide very basic
#               functionality and initialization

function __hermes__is_bash_function() {
  [[ $(type -t "${1:?Name of type to check is required}" || :) == 'function' ]]
}
export -f __hermes__is_bash_function

function __hermes__command_exists() {
  command -v "${1:?Command name is required}" &>/dev/null
}
export -f __hermes__command_exists

function __hermes__execute_real_command() {
  local COMMAND DIR FULL_COMMAND
  COMMAND=${1:?Command name required}
  shift 1

  for DIR in ${PATH//:/ }; do
    FULL_COMMAND="${DIR}/${COMMAND}"
    [[ -x ${FULL_COMMAND} ]] && { ${FULL_COMMAND} "${@}" ; return ${?} ; }
  done

  echo "Command '${COMMAND}' not found" >&2
  return 1
}
export -f __hermes__execute_real_command

# shellcheck disable=SC2120
function __hermes__declare_helpers() {
  local FUNCTIONS=('__hermes__do_as_root' '__hermes__command_exists' '__hermes__is_bash_function' '__hermes__execute_real_command')
  [[ ${#} -gt 0 ]] && FUNCTIONS+=("${@}")
  declare -f "${FUNCTIONS[@]}"
}
export -f __hermes__declare_helpers

function __hermes__do_as_root() {
  local SU_COMMAND=${SU_COMMAND:-}

  if [[ -n ${SU_COMMAND} ]]; then
    :
  elif __hermes__command_exists 'doas'; then
    SU_COMMAND='doas'
  elif __hermes__command_exists 'sudo'; then
    SU_COMMAND='sudo'
  else
    echo 'Could not find program to execute command as root'
    return 1
  fi

  if __hermes__is_bash_function "${1:?Command is required}"; then
    ${SU_COMMAND} bash -c "$(__hermes__declare_helpers || :) ; ${*}"
  else
    ${SU_COMMAND} "${@}"
  fi
}
export -f __hermes__do_as_root

function setup_path() {
  local ADDITIONAL_PATH_ENTRIES=(
    "${HOME}/bin"
    "${HOME}/.local/bin"
    "${HOME}/.atuin/bin"
    "${HOME}/.fzf/bin"
  )

  for ADDITIONAL_PATH_ENTRY in "${ADDITIONAL_PATH_ENTRIES[@]}"; do
    if [[ -d ${ADDITIONAL_PATH_ENTRY} ]] && [[ ${PATH} != *${ADDITIONAL_PATH_ENTRY}* ]]; then
      export PATH="${ADDITIONAL_PATH_ENTRY}${PATH:+:${PATH}}"
    fi
  done

  local ADDITIONAL_SOURCE_PATHS=(
    "${HOME}/.cargo/env"
    "${HOME}/.atuin/env"
  )

  for ADDITIONAL_SOURCE_PATH in "${ADDITIONAL_SOURCE_PATHS[@]}"; do
    if [[ -e ${ADDITIONAL_SOURCE_PATH} ]] && [[ -r ${ADDITIONAL_SOURCE_PATH} ]]; then
      # shellcheck source=/dev/null
      source "${ADDITIONAL_SOURCE_PATH}"
    fi
  done

  return 0
}

setup_path
unset setup_path
