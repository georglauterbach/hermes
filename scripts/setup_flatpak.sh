#! /usr/bin/env -S bash -eE -u -o pipefail -O inherit_errexit

SUDO_CMD="add-apt-repository --yes --no-update ppa:flatpak/stable"
SUDO_CMD+=" && apt-get --yes update"
SUDO_CMD+=" && apt-get --yes install flatpak malcontent-gui" # steam-devices

sudo bash -c "${SUDO_CMD}"
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo

echo "Restart you machine for changes to take effect"
