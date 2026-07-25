#! /usr/bin/env bash

export ZELLIJ_AUTO_ATTACH_GIT=true
export ZELLIJ_AUTO_RUN=true
export ZELLIJ_AUTO_EXIT=false

if command -v zellij &>/dev/null &&  [[ -z ${ZELLIJ:-} ]]; then
  # auto-attach to a session named after the current repository
  if [[ ${ZELLIJ_AUTO_ATTACH_GIT:-false} == true ]] && git status &>/dev/null; then
    zellij attach -c "$(basename "${PWD}")"
    __ZELLIJ_CAN_AUTO_EXIT=true
  elif [[ ${ZELLIJ_AUTO_RUN:-false} == true ]]; then
    zellij
    __ZELLIJ_CAN_AUTO_EXIT=true
  fi

  __RET=${?}
  if  [[ ${__ZELLIJ_CAN_AUTO_EXIT:-false} == true ]] && [[ ${ZELLIJ_AUTO_EXIT:-false} == true ]]; then
    exit "${__RET}"
  fi
  unset __RET __ZELLIJ_CAN_AUTO_EXIT
fi
