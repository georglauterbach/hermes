#! /usr/bin/env bash

# cSpell: ignore editv libsecret

alias edit='fresh'
alias editv='codium --password-store=gnome-libsecret --enable-features=UseOzonePlatform --ozone-platform=wayland'

alias e='edit'
alias ev='editv'

if __is_command 'gitui'; then
  alias g='gitui'
else
  alias g='git diff'
fi
