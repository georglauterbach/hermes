# add this to ${HOME}/.profile to execute Sway
# without a login manager

# shellcheck disable=SC2292

if [ "$(tty)" = "/dev/tty1" ] ; then
  export QT_QPA_PLATFORM=wayland
  export QT_WAYLAND_DISABLE_WINDOWDECORATION=1

  export MOZ_ENABLE_WAYLAND=1
  export MOZ_WEBRENDER=1

  export XDG_SESSION_TYPE=wayland
  export XDG_CURRENT_DESKTOP=sway

  export ELECTRON_OZONE_PLATFORM_HINT=auto

  export WLR_RENDERER=vulkan
  exec sway
fi
