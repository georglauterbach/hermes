#! /usr/bin/env bash

# version       1.2.0
# sourced by    ${HOME}/.bashrc
# task          provide miscellaneous main setup

function __hermes__setup_shopt() {
  shopt -s checkwinsize globstar autocd
}

function __hermes__setup_variables() {
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
}

function __hermes__setup_completion() {
  if ! shopt -oq posix; then
    if [[ -f /usr/share/bash-completion/bash_completion ]]; then
      # shellcheck source=/dev/null
      source /usr/share/bash-completion/bash_completion
    elif [[ -f /etc/bash_completion ]]; then
      # shellcheck source=/dev/null
      source /etc/bash_completion
    fi
  fi
}

function __hermes__setup_basic_prompt() {
  export PROMPT_DIRTRIM=4

  # disable blinking cursor (e.g., in TMUX)
  printf '\033[2 q'

  if ! __is_command 'starship'; then
    PS2=''   # continuation shell prompt
    PS4='> ' # `set -x` tracing prompt

    if [[ -v debian_chroot ]] && [[ -r /etc/debian_chroot ]]; then
      # shellcheck disable=SC2155
      export debian_chroot=$(</etc/debian_chroot)
    fi
  fi
}

for __FUNCTION in 'shopt' 'variables' 'completion' 'basic_prompt'; do
  "__hermes__setup_${__FUNCTION}" || :
  unset "__hermes__setup_${__FUNCTION}"
done
