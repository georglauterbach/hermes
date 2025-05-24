# shellcheck shell=sh disable=SC2155,SC2292

# add this to ${HOME}/.profile to execute Sway without a login manager
# ref: https://www.reddit.com/r/swaywm/comments/lv2336/launch_sway_without_a_display_manager_on_login/
if [ -z "${DISPLAY}" ] && [ "$(tty)" = "/dev/tty1" ] ; then
    export QT_QPA_PLATFORM=wayland
    export QT_WAYLAND_DISABLE_WINDOWDECORATION=1

    export MOZ_ENABLE_WAYLAND=1
    export MOZ_WEBRENDER=1

    export ELECTRON_OZONE_PLATFORM_HINT=auto

    export XDG_CONFIG_HOME="${HOME}/.config"
    export XDG_CURRENT_DESKTOP=sway # xdg-desktop-portal
    export XDG_RUNTIME_DIR="/run/user/$(id --user)"
    export XDG_SESSION_TYPE=wayland # xdg/systemd
    export XDG_SESSION_DESKTOP=sway # systemd

    export WLR_RENDERER=vulkan
    exec dbus-run-session -- sway >/tmp/sway.log 2>&1
fi
