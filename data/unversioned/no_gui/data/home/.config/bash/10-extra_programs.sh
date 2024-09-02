#! /usr/bin/env bash

# version       0.3.0
# sourced by    ${HOME}/.bashrc
# task          setup up miscellaneous programs if they are installed

function setup_ble() {
  local BLE_SOURCE="${HOME}/.local/share/blesh/ble.sh"
  if [[ -e ${BLE_SOURCE} ]]; then
    local BLE_CONFIG_FILE="${HOME}/.config/bash/ble.sh"
    if [[ -e ${BLE_CONFIG_FILE} ]]; then
      # shellcheck source=/dev/null
      source "${BLE_SOURCE}" --attach=none --rcfile "${BLE_CONFIG_FILE}"
    else
      # shellcheck source=/dev/null
      source "${BLE_SOURCE}" --attach=none
    fi
  fi
}

function setup_fzf() {
  if __hermes__command_exists 'fzf'; then
    # shellcheck source=/dev/null
    source "${HOME}/.fzf/shell/completion.bash" 2>/dev/null
    [[ -v BLE_VERSION ]] && ble-import -d 'integration/fzf-completion'

    # shellcheck source=/dev/null
    source "${HOME}/.fzf/shell/key-bindings.bash"
    ble-import -d integration/fzf-key-bindings
  fi
}

function setup_history() {
  if __hermes__command_exists 'atuin'; then
    # Atuin automatically hooks into ble.sh if ble.sh has been initialized before!
    # https://docs.atuin.sh/guide/installation/#installing-the-shell-plugin
    eval "$(atuin init bash --disable-up-arrow --disable-ctrl-r  || :)"
    bind -x '"\C-e": __atuin_history'
  else
    shopt -s histappend
    export HISTCONTROL='ignoreboth'
    export HISTSIZE=10000
    export HISTFILESIZE=10000
  fi
}

function setup_rust() {
  __hermes__command_exists sccache && export RUSTC_WRAPPER='sccache'
}

function setup_bat() {
  local BAT_NAME='batcat' # use 'bat' on older distributions
  if __hermes__command_exists "${BAT_NAME}"; then
    export MANPAGER="bash -c 'col -bx | ${BAT_NAME} -l man --style=plain --theme=gruvbox-dark'"
    export MANROFFOPT='-c'
    # `PAGER` is set in `10-setup.sh`
    # shellcheck disable=SC2154
    export BAT_PAGER=${PAGER}
    # make sure `PAGER` is set before this alias is defined
    # shellcheck disable=SC2139
    alias less="${BAT_NAME} --style=plain --paging=always --color=always --theme=gruvbox-dark"
  fi
}

function setup_zoxide() {
  if __hermes__command_exists 'zoxide'; then
    eval "$(zoxide init bash || :)"
    alias cd='z'
    [[ -v BLE_VERSION ]] && ble-import -f 'integration/zoxide'
  fi
}

function setup_starship() {
  if __hermes__command_exists 'starship'; then
    STARSHIP_CONFIG="${HOME}/.config/bash/starship.toml"
    if [[ -f ${STARSHIP_CONFIG} ]] && [[ -r ${STARSHIP_CONFIG} ]]; then
      export STARSHIP_CONFIG
    else
      unset STARSHIP_CONFIG
    fi

    eval "$(starship init bash || :)"
  fi
}

# The order of initialization is important: the setup for ble has to run
# before fzf and Atuin
for __FUNCTION in 'ble' 'fzf' 'history' 'rust' 'bat' 'zoxide' 'starship'; do
  "setup_${__FUNCTION}"
  unset "setup_${__FUNCTION}"
done
