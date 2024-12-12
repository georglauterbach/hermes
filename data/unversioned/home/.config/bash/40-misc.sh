#! /usr/bin/env bash

#    version  2.0.0
# sourced by  ${HOME}/.bashrc
#       task  miscellaneous setup

# ! Do not edit this file - use ${HOME}/.config/bash/{20-custom_early,99-custom_late}.sh instead.

function __hermes__setup_misc() {
  # shell options
  shopt -s checkwinsize globstar autocd

  # miscellaneous environment variables
  VISUAL='nano'
  if   __is_command 'nvim'; then VISUAL='nvim'
  elif __is_command 'vim' ; then VISUAL='vim'
  elif __is_command 'vi'  ; then VISUAL='vi'
  fi

  EDITOR=${VISUAL}
  PAGER="$(command -v less) -R"
  GPG_TTY=$(tty)
  HERMES_LOADED=true

  export VISUAL EDITOR PAGER GPG_TTY HERMES_LOADED

  # completion
  if ! shopt -oq posix; then
    if [[ -f /etc/profile.d/bash_completion.sh ]]; then
      # shellcheck source=/dev/null
      source /etc/profile.d/bash_completion.sh
    elif [[ -f /etc/bash_completion ]]; then
      # shellcheck source=/dev/null
      source /etc/bash_completion
    fi
  fi
}

function __hermes__setup_prompt() {
  if ! __evaluates_to_true HERMES_INIT_STARSHIP || ! __is_command 'starship'; then
    export PROMPT_DIRTRIM=4
    export PS2=''   # continuation shell prompt
    export PS4='> ' # `set -x` tracing prompt

    # shellcheck disable=SC2155
    [[ ! -v debian_chroot ]] && [[ -r /etc/debian_chroot ]] && export debian_chroot=$(</etc/debian_chroot)
  fi

  export STARSHIP_CONFIG
  [[ -v STARSHIP_CONFIG ]] || STARSHIP_CONFIG="${HOME}/.config/starship/starship.toml"
  eval "$(starship init bash || :)"
}

function __hermes__setup_history() {
  function __hermes__init_default_history() {
    export HISTFILE=${HISTFILE:-${HOME}/.bash_history}
    export HISTCONTROL=${HISTCONTROL:-ignoreboth}
    export HISTSIZE=${HISTSIZE:-10000}
    export HISTFILESIZE=${HISTFILESIZE:-10000}

    shopt -s histappend
    mkdir -p "$(dirname "${HISTFILE}")"
    touch "${HISTFILE}"
    [[ -v BLE_VERSION ]] && bleopt history_share=yes
  }

  if ! __evaluates_to_true HERMES_INIT_ATUIN || ! __is_command 'atuin'; then
    __hermes__init_default_history
    return 0
  fi

  # Using Atuin only works with Bash preexec.sh or ble.sh. When ble.sh has been initialized before,
  # Atuin will hook into ble.sh. One must not enable Atuin, though, if ble.sh is not configured
  # (because this may break other programs, like StarShip).
  #
  # ref: https://docs.atuin.sh/guide/installation/#installing-the-shell-plugin
  if [[ ! -v BLE_VERSION ]]; then
    echo 'hermes: loading Atuin without ble.sh does not work (falling back to normal history)' >&2
    __hermes__init_default_history
    return 0
  fi

  shopt -u histappend
  export HISTFILE=/dev/null
  unset HISTCONTROL HISTSIZE HISTFILESIZE
  eval "$(atuin init bash --disable-up-arrow --disable-ctrl-r  || :)"
  bind -x '"\C-e": __atuin_history' # CTRL+e will bring up Atuin
}

# The setup of ble.sh should be done as early as possible and outside a function.
#
# ref: https://github.com/akinomyoga/ble.sh?tab=readme-ov-file#13-set-up-bashrc
HERMES_BLE_SOURCE="${HOME}/.local/share/blesh/ble.sh"
if __evaluates_to_true HERMES_INIT_BLE_SH && [[ -e ${HERMES_BLE_SOURCE} ]]; then
  # shellcheck source=/dev/null
  source "${HERMES_BLE_SOURCE}" --attach=none
fi
unset HERMES_BLE_SOURCE

for __FUNCTION in 'misc' 'prompt' 'history'; do
  "__hermes__setup_${__FUNCTION}" || :
  unset "__hermes__setup_${__FUNCTION}"
done
