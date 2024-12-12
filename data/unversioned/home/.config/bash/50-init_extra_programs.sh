#! /usr/bin/env bash

#    version  2.2.0
# sourced by  ${HOME}/.bashrc
#       task  initialize miscellaneous programs

# ! Do not edit this file - use ${HOME}/.config/bash/{20-custom_early,99-custom_late}.sh instead.

function __hermes__init_bat() {
  if ! __evaluates_to_true HERMES_INIT_BAT || ! __is_command 'bat'; then
    return 0
  fi

  # shellcheck disable=SC2154
  [[ -v PAGER ]] && export BAT_PAGER=${PAGER}

  export MANPAGER="bash -c 'col -bx | bat --language=man --style=plain --theme=gruvbox-dark'"
  export MANROFFOPT='-c'

  alias less="bat --paging=always --color=always --style=plain --theme=gruvbox-dark"
}

# ref: https://github.com/akinomyoga/blesh-contrib/blob/master/integration/fzf.md
function __hermes__init_fzf() {
  if ! __evaluates_to_true HERMES_INIT_FZF || ! __is_command 'fzf'; then
    return 0
  fi

  local FZF_COMPLETION="${HOME}/.config/fzf/completion.bash"
  if [[ -e ${FZF_COMPLETION} ]] && [[ -r ${FZF_COMPLETION} ]]; then
    if [[ -v BLE_VERSION ]]; then
      ble-import --delay 'integration/fzf-completion'
    else
      # shellcheck source=/dev/null
      source "${FZF_COMPLETION}" 2>/dev/null
    fi
  fi

  local FZF_KEY_BINDINGS="${HOME}/.config/fzf/key-bindings.bash"
  if [[ -e ${FZF_KEY_BINDINGS} ]] && [[ -r ${FZF_KEY_BINDINGS} ]]; then
    if [[ -v BLE_VERSION ]]; then
      ble-import --delay 'integration/fzf-key-bindings'
    else
      # shellcheck source=/dev/null
      source "${FZF_KEY_BINDINGS}"
    fi
  fi
}

function __hermes__init_zoxide() {
  if ! __evaluates_to_true HERMES_INIT_ZOXIDE || ! __is_command 'zoxide'; then
    return 0
  fi

  eval "$(zoxide init bash || :)"
  [[ -v BLE_VERSION ]] && ble-import --delay 'integration/zoxide'
}

for __FUNCTION in 'bat' 'fzf' 'zoxide'; do
  "__hermes__init_${__FUNCTION}" || :
  unset "__hermes__init_${__FUNCTION}"
done
