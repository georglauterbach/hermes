#! /usr/bin/env bash

# shellcheck source=/dev/null

# export ZELLIJ_AUTO_ATTACH_GIT=true
# export ZELLIJ_AUTO_RUN=true
#
# if command -v zellij &>/dev/null && [[ -z ${ZELLIJ:-} ]]; then
#   if [[ ${ZELLIJ_AUTO_ATTACH_GIT:-false} == true ]] && git status &>/dev/null; then
#     exec zellij attach -c "git! $(basename "${PWD}")"
#   elif [[ ${ZELLIJ_AUTO_RUN:-false} == true ]]; then
#     exec zellij
#   fi
# fi

source "${HOME}/.config/bash/90-hermes.sh"
[[ -r ${HOME}/.config/bash/95-custom.sh ]] && source "${HOME}/.config/bash/95-custom.sh"
