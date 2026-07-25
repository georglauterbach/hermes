#! /usr/bin/env bash

alias code='code --password-store=gnome-libsecret --enable-features=UseOzonePlatform --ozone-platform=wayland'

if __is_command 'doas'; then
  complete -cf doas
  alias sudo='doas'
fi

if __is_command 'gitui'; then
  alias g='gitui'
else
  alias g='git diff'
fi
