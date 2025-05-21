#! /usr/bin/env bash

# - rofi (patched for Wayland)
#   https://github.com/georglauterbach/hermes/releases/tag/rofi-v1.7.7%2Bwayland
# - sway-audio-idle-inhibit (optional)
#   https://github.com/ErikReider/SwayAudioIdleInhibit
# - gpu-usage-waybar (optional)
#   https://github.com/PolpOnline/gpu-usage-waybar

set -eE -u -o pipefail
shopt -s inherit_errexit

if [[ ${EUID} -ne 0 ]]; then
  echo "ERROR: This script needs to run with superuser privileges" >&2
  exit 1
fi

source /etc/os-release || { echo "ERROR: Could not source '/etc/os-release'" ; exit 1 ; }

case "${ID}" in
  ( 'ubuntu' )
    readonly PACKAGES=(
      sway sway-backgrounds swayidle swaylock sway-notification-center xwayland
      libvulkan1 mesa-vulkan-drivers vulkan-tools
      xdg-desktop-portal xdg-desktop-portal-gtk xdg-desktop-portal-wlr
      waybar
      alacritty
      evince grimshot
      fonts-font-awesome
      pipewire pipewire-audio-client-libraries pulseaudio-utils rtkit wireplumber
      polkitd pkexec
      librsvg2-2
    )

    apt-get --yes install --no-install-recommends --no-install-suggests "${PACKAGES[@]}"

    rm --force /etc/systemd/user/graphical-session.target.wants/{waybar,swaync}.service
    apt-get --yes purge wmenu
    systemctl --user disable swaync.service waybar.service || :
    systemctl --user enable --now pipewire-pulse.service
    ;;

  ( * )
    echo "ERROR: Distribution '${ID}' is not supported"
    exit 1
    ;;
esac
