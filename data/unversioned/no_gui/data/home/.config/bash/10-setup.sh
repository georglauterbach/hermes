#! /usr/bin/env bash

# version       0.5.0
# sourced by    ${HOME}/.bashrc
# task          provide Bash's main setup

function setup_misc() {
  shopt -s histappend checkwinsize globstar autocd
}

function setup_variables() {
  VISUAL='nano'
  __hermes__command_exists 'vi' && VISUAL='vi'
  __hermes__command_exists 'vim' && VISUAL='vim'
  __hermes__command_exists 'nvim' && VISUAL='nvim'

  EDITOR=${VISUAL}
  PAGER="$(command -v less) -R"
  GPG_TTY=$(tty)

  export VISUAL EDITOR PAGER GPG_TTY

  if ! __hermes__command_exists 'ble'; then
    export HISTCONTROL='ignoreboth'
    export HISTSIZE=10000
    export HISTFILESIZE=10000
  fi
}

function setup_completion() {
  if ! shopt -oq posix; then
    if [[ -f /usr/share/bash-completion/bash_completion ]]; then
      # shellcheck source=/dev/null
      source /usr/share/bash-completion/bash_completion
    elif [[ -f /etc/bash_completion ]]; then
      # shellcheck source=/dev/null
      source /etc/bash_completion
    fi

    if __hermes__command_exists 'doas'; then
      complete -cf doas
      alias sudo='doas'
    fi
  fi
}

function setup_prompt() {
  export PROMPT_DIRTRIM=4

  # disable blinking cursor (e.g., in TMUX)
  printf '\033[2 q'

  PS2=''  # continuation shell prompt
  PS4='> ' # `set -x` tracing prompt

  if ! __hermes__command_exists 'starship' && [[ -v debian_chroot ]] && [[ -r /etc/debian_chroot ]]; then
    # shellcheck disable=SC2155
    export debian_chroot=$(</etc/debian_chroot)
  fi
}

for __FUNCTION in 'misc' 'path' 'variables' 'completion' 'prompt'; do
  "setup_${__FUNCTION}"
  unset "setup_${__FUNCTION}"
done
