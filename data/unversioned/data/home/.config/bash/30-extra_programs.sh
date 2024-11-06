#! /usr/bin/env bash

# version       1.0.0
# sourced by    ${HOME}/.bashrc
# task          setup up miscellaneous programs when they are installed

function __hermes__setup_ble() {
  local BLE_SOURCE="${HOME}/.local/share/blesh/ble.sh"
  if [[ -e ${BLE_SOURCE} ]]; then
    local BLE_CONFIG_FILE="${HOME}/.config/bash/ble_config.sh"
    if [[ -e ${BLE_CONFIG_FILE} ]]; then
      # shellcheck source=/dev/null
      source "${BLE_SOURCE}" --attach=none --rcfile "${BLE_CONFIG_FILE}"
    else
      # shellcheck source=/dev/null
      source "${BLE_SOURCE}" --attach=none
    fi
  fi
}

function __hermes__setup_fzf() {
  if __is_command 'fzf'; then
    # shellcheck source=/dev/null
    source "${HOME}/.fzf/shell/completion.bash" 2>/dev/null
    [[ -v BLE_VERSION ]] && ble-import --delay 'integration/fzf-completion'

    # shellcheck source=/dev/null
    source "${HOME}/.fzf/shell/key-bindings.bash"
    [[ -v BLE_VERSION ]] && ble-import --delay 'integration/fzf-key-bindings'
  fi
}

function __hermes__setup_history() {
  # Using Atuin only works with Bash preexec.sh or ble.sh. When ble.sh has
  # been initialized before, Atuin will hook into ble.sh. One must not enable
  # Atuin, though, if ble.sh is not configured (because this may break other
  # programs, like StarShip).
  #
  # ref: https://docs.atuin.sh/guide/installation/#installing-the-shell-plugin
  if __is_command 'atuin' && [[ -n ${BLE_VERSION} ]]; then
    eval "$(atuin init bash --disable-up-arrow --disable-ctrl-r  || :)"
    bind -x '"\C-e": __atuin_history'
  else
    shopt -s histappend
    export HISTCONTROL='ignoreboth'
    export HISTSIZE=10000
    export HISTFILESIZE=10000
  fi
}

function __hermes__setup_bat() {
  local BAT_NAME='batcat' # use 'bat' on older distributions
  if __is_command "${BAT_NAME}"; then
    export MANPAGER="bash -c 'col -bx | ${BAT_NAME} -l man --style=plain --theme=gruvbox-dark'"
    export MANROFFOPT='-c'
    # `PAGER` is set in `10-misc.sh`
    # shellcheck disable=SC2154
    export BAT_PAGER=${PAGER}
    # make sure `PAGER` is set before this alias is defined
    # shellcheck disable=SC2139
    alias less="${BAT_NAME} --paging=always --color=always --style=plain --theme=gruvbox-dark"
  fi
}

function __hermes__setup_zoxide() {
  if __is_command 'zoxide'; then
    eval "$(zoxide init bash || :)"
    alias cd='z'
    [[ -v BLE_VERSION ]] && ble-import -f 'integration/zoxide'
  fi
}

function __hermes__setup_starship() {
  if __is_command 'starship'; then
    export STARSHIP_CONFIG="${HOME}/.config/bash/starship.toml"
    if [[ ! -f ${STARSHIP_CONFIG} ]] || [[ ! -r ${STARSHIP_CONFIG} ]]; then
      unset STARSHIP_CONFIG
    fi

    eval "$(starship init bash || :)"
  fi
}

# The order of initialization is important: the setup for ble has to run
# before fzf and Atuin
for __FUNCTION in 'ble' 'fzf' 'history' 'bat' 'zoxide' 'starship'; do
  "__hermes__setup_${__FUNCTION}" || :
  unset "__hermes__setup_${__FUNCTION}"
done
