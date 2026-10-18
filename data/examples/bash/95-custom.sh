#! /usr/bin/env bash

# cSpell: ignore vedit libsecret

alias edit='fresh'
alias vedit='codium --password-store=gnome-libsecret --enable-features=UseOzonePlatform --ozone-platform=wayland'

alias e='edit'
alias ve='vedit'

if __is_command 'gitui'; then
  alias g='gitui'
else
  alias g='git diff'
fi
