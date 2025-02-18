#! /usr/bin/env -S bash -eE -u -o pipefail -O inherit_errexit

# ref: https://flatpak.org/setup/Ubuntu

set -eE -u -o pipefail
shopt -s inherit_errexit

if ! command -v flatpak &>/dev/null; then
  SUDO_CMD='export DEBIAN_FRONTEND=noninteractive ; export DEBCONF_NONINTERACTIVE_SEEN=true ;'
  SUDO_CMD+=' add-apt-repository --yes --no-update ppa:flatpak/stable'
  SUDO_CMD+=' && apt-get --yes update'
  SUDO_CMD+=' && apt-get --yes install flatpak malcontent-gui'
  sudo bash -c "${SUDO_CMD}"
fi

flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo

read -r -p 'Do you want to install additional extra programs? [y/N] ' REPONSE
if [[ ${REPONSE,,} =~ ^(y(es)?)$ ]]; then
  readonly EXTRA_PROGRAMS=(
    'com.bitwarden.desktop'
    'com.discordapp.Discord'
    'com.github.tchx84.Flatseal'
    'com.nextcloud.desktopclient.nextcloud'
    'com.valvesoftware.Steam'
    'eu.betterbird.Betterbird'
    'net.lutris.Lutris'
    'org.cryptomator.Cryptomator'
    'org.onlyoffice.desktopeditors'
    'org.mozilla.firefox'
  )

  flatpak install --noninteractive --or-update flathub "${EXTRA_PROGRAMS[@]}"
fi

echo "Restart you machine for changes to take effect"
