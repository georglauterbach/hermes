#! /usr/bin/env bash

# version       0.3.0
# sourced by    ${HOME}/.bashrc
# task          set up functions required during setup

function __is_bash_function() {
  [[ $(type -t "${1:?Name of type to check is required}" || :) == 'function' ]]
}

if ! __is_bash_function '__command_exists'; then
  function __command_exists() {
    command -v "${1:?Command name is required}" &>/dev/null
  }

  readonly -f __command_exists
  export -f __command_exists
fi

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

function __hermes__declare_helpers() {
  local FUNCTIONS=('do_as_root' '__command_exists' '__is_bash_function' '__hermes__execute_real_command')
  [[ -n ${1:-} ]] && FUNCTIONS+=("${@}")
  declare -f "${FUNCTIONS[@]}"
}

function do_as_root() {
  local SU_COMMAND=${SU_COMMAND:-}

  if [[ -n ${SU_COMMAND} ]]; then
    :
  elif __command_exists 'doas'; then
    SU_COMMAND='doas'
  elif __command_exists 'sudo'; then
    SU_COMMAND='sudo'
  else
    echo 'Could not find program to execute command as root'
    return 1
  fi

  if __is_bash_function "${1:?Command is required}"; then
    ${SU_COMMAND} bash -c "$(__hermes__declare_helpers || :) ; ${*}"
  else
    ${SU_COMMAND} "${@}"
  fi
}
export -f do_as_root
