#! /usr/bin/env bash

set -eE -u -o pipefail
shopt -s inherit_errexit

sudo apt-get --yes install git make gawk

git clone --recursive --depth 1 --shallow-submodules https://github.com/akinomyoga/ble.sh.git "${HOME}/.ble.sh"
make -C "${HOME}/.ble.sh" install PREFIX=~/.local

echo -e "\nYou may now add 'source ~/.local/share/blesh/ble.sh' to you Bash setup"
