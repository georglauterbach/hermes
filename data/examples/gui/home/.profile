# add this to ${HOME}/.profile to execute Sway
# without a login manager

# shellcheck disable=SC2292

# https://www.reddit.com/r/swaywm/comments/lv2336/launch_sway_without_a_display_manager_on_login/
if [ -z "${DISPLAY}" ] && [ "$(tty)" = "/dev/tty1" ] ; then
    export QT_QPA_PLATFORM=wayland
    export QT_WAYLAND_DISABLE_WINDOWDECORATION=1

    export MOZ_ENABLE_WAYLAND=1
    export MOZ_WEBRENDER=1

    export XDG_SESSION_TYPE=wayland
    export XDG_CURRENT_DESKTOP=sway

    export ELECTRON_OZONE_PLATFORM_HINT=auto

    export WLR_RENDERER=vulkan
    exec dbus-run-session -- sway >/tmp/sway.stdout.log 2>/tmp/sway.stderr.log
fi
