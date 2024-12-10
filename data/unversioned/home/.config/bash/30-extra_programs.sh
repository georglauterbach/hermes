#! /usr/bin/env bash

# version       2.1.0
# sourced by    ${HOME}/.bashrc
# task          initialize miscellaneous programs

# ! Do not edit this file.
# ! Use ${HOME}/.config/bash/{20-custom_early,99-custom_late}.sh instead.

# ! Has to be loaded first
function __hermes__init_ble() {
  __evaluates_to_true HERMES_LOAD_EXTRA_PROGRAMS_BLE_SH 'true' || return 0
  local BLE_SOURCE="${HOME}/.local/share/blesh/ble.sh"
  if [[ -e ${BLE_SOURCE} ]]; then
    local BLE_CONFIG_FILE="${HOME}/.config/bash/ble.sh/config.sh"
    if [[ -e ${BLE_CONFIG_FILE} ]]; then
      # shellcheck source=/dev/null
      source "${BLE_SOURCE}" --attach=none --rcfile "${BLE_CONFIG_FILE}"
    else
      # shellcheck source=/dev/null
      source "${BLE_SOURCE}" --attach=none
    fi
  fi
}

function __hermes__init_atuin() {
  # Using Atuin only works with Bash preexec.sh or ble.sh. When ble.sh has
  # been initialized before, Atuin will hook into ble.sh. One must not enable
  # Atuin, though, if ble.sh is not configured (because this may break other
  # programs, like StarShip).
  #
  # ref: https://docs.atuin.sh/guide/installation/#installing-the-shell-plugin
  if __evaluates_to_true HERMES_LOAD_EXTRA_PROGRAMS_ATUIN 'true' \
  && __is_command 'atuin'                                        \
  && [[ ! -v BLE_VERSION ]]; then
    echo 'hermes: loading Atuin without ble.sh does not work (falling back to normal history)' >&2
  fi

  if __evaluates_to_true HERMES_LOAD_EXTRA_PROGRAMS_ATUIN 'true' \
  && __is_command 'atuin'                                        \
  && [[ -v BLE_VERSION ]]; then
    eval "$(atuin init bash --disable-up-arrow --disable-ctrl-r  || :)"
    bind -x '"\C-e": __atuin_history' # CTRL+e will bring up Atuin
  else
    shopt -s histappend
    export HISTCONTROL='ignoreboth'
    export HISTSIZE=10000
    export HISTFILESIZE=10000
  fi
}

function __hermes__init_bat() {
  __evaluates_to_true HERMES_LOAD_EXTRA_PROGRAMS_BAT 'true' || return 0

  local BAT_CMD_NAME='bat'
  __is_command 'bat' || BAT_CMD_NAME='batcat'

  if __is_command "${BAT_CMD_NAME}"; then
    export MANPAGER="bash -c 'col -bx | ${BAT_CMD_NAME} -l man --style=plain --theme=gruvbox-dark'"
    export MANROFFOPT='-c'

    # `PAGER` is set in `10-misc.sh`
    if [[ -v PAGER ]]; then
      # shellcheck disable=SC2154
      export BAT_PAGER=${PAGER}
      # shellcheck disable=SC2139
      alias less="${BAT_CMD_NAME} --paging=always --color=always --style=plain --theme=gruvbox-dark"
    fi
  fi
}

# ref: https://github.com/akinomyoga/blesh-contrib/blob/master/integration/fzf.md
function __hermes__init_fzf() {
  __evaluates_to_true HERMES_LOAD_EXTRA_PROGRAMS_FZF 'true' || return 0
  if __is_command 'fzf'; then
    local FZF_COMPLETION="${HOME}/.config/bash/fzf/completion.bash"
    if [[ -e ${FZF_COMPLETION} ]]; then
      if [[ -v BLE_VERSION ]]; then
        ble-import --delay 'integration/fzf-completion'
      else
        # shellcheck source=/dev/null
        source "${FZF_COMPLETION}" 2>/dev/null
      fi
    fi

    local FZF_KEY_BINDINGS="${HOME}/.config/bash/fzf/key-bindings.bash"
    if [[ -e ${FZF_KEY_BINDINGS} ]]; then
      if [[ -v BLE_VERSION ]]; then
        ble-import --delay 'integration/fzf-key-bindings'
      else
        # shellcheck source=/dev/null
        source "${FZF_KEY_BINDINGS}"
      fi
    fi
  fi
}

function __hermes__init_starship() {
  __evaluates_to_true HERMES_LOAD_EXTRA_PROGRAMS_STARSHIP 'true' || return 0
  if __is_command 'starship'; then
    if [[ ! -v STARSHIP_CONFIG ]]; then
      STARSHIP_CONFIG="${HOME}/.config/bash/starship/starship.toml"
    fi

    if [[ ! -f ${STARSHIP_CONFIG} ]] || [[ ! -r ${STARSHIP_CONFIG} ]]; then
      echo "hermes: Starship configuration file '${STARSHIP_CONFIG}' does exist or is not readable" >&2
      unset STARSHIP_CONFIG
    else
      export STARSHIP_CONFIG
    fi

    eval "$(starship init bash || :)"
  fi
}

function __hermes__init_zoxide() {
  __evaluates_to_true HERMES_LOAD_EXTRA_PROGRAMS_ZOXIDE 'true' || return 0
  if __is_command 'zoxide'; then
    eval "$(zoxide init bash || :)"
    [[ -v BLE_VERSION ]] && ble-import -f 'integration/zoxide'
  fi
}

# ! The order of initialization is important: The
# ! setup for ble has to run before fzf and Atuin.
for __FUNCTION in 'ble' 'atuin' 'bat' 'fzf' 'starship' 'zoxide'; do
  "__hermes__init_${__FUNCTION}" || :
  unset "__hermes__init_${__FUNCTION}"
done
