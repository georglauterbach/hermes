#! /usr/bin/env bash

# shellcheck source=/dev/null
source "${HOME}/.config/bash/90-hermes.sh"

if __is_command 'doas'; then
  complete -cf doas
  alias sudo='doas'
fi

if __is_command 'gitui'; then
  alias g='gitui'
else
  alias g='git diff'
fi
