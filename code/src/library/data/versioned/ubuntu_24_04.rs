//! Definitions for Ubuntu 24.04.

use super::super::{ConfigurationFileIndex, FileOverride, PackageIndex};

/// Ubuntu 24.04 LTS (Noble Numbat)
pub struct Ubuntu24_04;

impl super::UbuntuVersion for Ubuntu24_04 {
    fn base_packages(&self) -> PackageIndex {
        &[
            "btop",
            "gnupg2",
            "make",
            "neovim",
            "openssh-client",
            "pinentry-curses",
        ]
    }

    fn apt_index(&self) -> ConfigurationFileIndex {
        &[
            // Default APT sources
            ("sources.list", "/etc/apt/sources.list", FileOverride::Yes),
            (
                "ubuntu.sources",
                "/etc/apt/sources.list.d/ubuntu.sources",
                FileOverride::Yes,
            ),
            // Custom PPAs
            (
                "git.sources",
                "/etc/apt/sources.list.d/git.sources",
                FileOverride::Yes,
            ),
            (
                "neovim.sources",
                "/etc/apt/sources.list.d/neovim.sources",
                FileOverride::Yes,
            ),
        ]
    }

    fn gui_apt_index(&self) -> ConfigurationFileIndex {
        &[
            (
                "alacritty.sources",
                "/etc/apt/sources.list.d/alacritty.sources",
                FileOverride::Yes,
            ),
            (
                "regolith.sources",
                "/etc/apt/sources.list.d/regolith.sources",
                FileOverride::Yes,
            ),
            (
                "vscode.sources",
                "/etc/apt/sources.list.d/vscode.sources",
                FileOverride::No,
            ),
        ]
    }

    fn gui_packages(&self) -> (PackageIndex, PackageIndex) {
        (
            &[
                "alacritty",
                "code",
                "dex",
                "grimshot",
                "regolith-desktop",
                "regolith-session-sway",
                "regolith-look-gruvbox",
                "regolith-sway-audio-idle-inhibit",
                "regolith-sway-background",
                "regolith-sway-clamshell",
                "regolith-sway-dbus-activation",
                "regolith-sway-default-style",
                "regolith-sway-gaps",
                "regolith-sway-grimshot",
                "regolith-sway-kbd-layout",
                "regolith-sway-media-keys",
                "regolith-sway-next-workspace",
                "regolith-sway-polkit",
                "regolith-sway-root-config",
                "regolith-sway-screensharing",
                "regolith-sway-session",
                "regolith-sway-unclutter",
                "xdg-desktop-portal-regolith-wayland-config",
                "rofi",
                "swaylock",
                "sway-notification-center",
            ],
            &[
                "foot",
                "i3status",
                "i3status-rs",
                "ilia",
                "regolith-powerd",
                "regolith-rofication",
                "regolith-sway-gtklock",
                "zutty",
            ],
        )
    }
}
