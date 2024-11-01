#! /usr/bin/env bash

# version       2.0.0
# sourced by    ${HOME}/.bashrc or manually
# task          this file should be usable even when not
#               running interactively to provide very basic
#               functionality and initialization

function __is_function() {
  [[ $(type -t "${1:?Name of type to check is required}" || :) == 'function' ]]
}; export -f __is_function

function __is_command() {
  command -v "${1:?Command name is required}" &>/dev/null
}; export -f __is_command

function __do_as_root() {
  local SU_COMMAND=${SU_COMMAND:-}

  if [[ -n ${SU_COMMAND} ]]; then
    :
  elif __is_command 'doas'; then
    SU_COMMAND='doas'
  elif __is_command 'sudo'; then
    SU_COMMAND='sudo'
  else
    echo 'Could not find program to execute command as root' >&2
    return 1
  fi

  if __is_function "${1:?Command is required}"; then
    ${SU_COMMAND} bash -c "$(declare -f '__do_as_root' '__is_command' '__is_function' || :) ; ${*}"
  else
    ${SU_COMMAND} "${@}"
  fi
}; export -f __do_as_root

function __hermes__setup_path() {
  local ADDITIONAL_PATH_ENTRIES=(
    "${HOME}/bin"
    "${HOME}/.local/bin"
    "${HOME}/.fzf/bin"
  )

  for ADDITIONAL_PATH_ENTRY in "${ADDITIONAL_PATH_ENTRIES[@]}"; do
    if [[ -d ${ADDITIONAL_PATH_ENTRY} ]] && [[ ${PATH} != *${ADDITIONAL_PATH_ENTRY}* ]]; then
      export PATH="${ADDITIONAL_PATH_ENTRY}${PATH+:${PATH}}"
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

__hermes__setup_path
unset __hermes__setup_path
