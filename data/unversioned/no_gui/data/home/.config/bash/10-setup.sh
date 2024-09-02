#! /usr/bin/env bash

# version       0.5.0
# sourced by    ${HOME}/.bashrc
# task          provide Bash's main setup

function setup_shopt() {
  shopt -s checkwinsize globstar autocd
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

    if __hermes__command_exists 'kubectl'; then
      complete -o default -F __start_kubectl k
      alias k='kubectl'
    fi
  fi
}

function setup_basic_prompt() {
  export PROMPT_DIRTRIM=4

  # disable blinking cursor (e.g., in TMUX)
  printf '\033[2 q'

  if ! __hermes__command_exists 'starship'; then
    PS2=''   # continuation shell prompt
    PS4='> ' # `set -x` tracing prompt

    if [[ -v debian_chroot ]] && [[ -r /etc/debian_chroot ]]; then
      # shellcheck disable=SC2155
      export debian_chroot=$(</etc/debian_chroot)
    fi
  fi
}

for __FUNCTION in 'shopt' 'variables' 'completion' 'basic_prompt'; do
  "setup_${__FUNCTION}"
  unset "setup_${__FUNCTION}"
done
