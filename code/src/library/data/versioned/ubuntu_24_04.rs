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
            (
                "apt/sources.list",
                "/etc/apt/sources.list",
                FileOverride::Yes,
            ),
            (
                "apt/ubuntu.sources",
                "/etc/apt/sources.list.d/ubuntu.sources",
                FileOverride::Yes,
            ),
            // Custom PPAs
            (
                "apt/flatpak.sources",
                "/etc/apt/sources.list.d/flatpak.sources",
                FileOverride::Yes,
            ),
            (
                "apt/git.sources",
                "/etc/apt/sources.list.d/git.sources",
                FileOverride::Yes,
            ),
            (
                "apt/neovim.sources",
                "/etc/apt/sources.list.d/neovim.sources",
                FileOverride::Yes,
            ),
        ]
    }

    fn gui_apt_index(&self) -> ConfigurationFileIndex {
        &[
            (
                "gui/apt/alacritty.sources",
                "/etc/apt/sources.list.d/alacritty.sources",
                FileOverride::Yes,
            ),
            (
                "gui/apt/cryptomator.sources",
                "/etc/apt/sources.list.d/cryptomator.sources",
                FileOverride::Yes,
            ),
            (
                "gui/apt/regolith.sources",
                "/etc/apt/sources.list.d/regolith.sources",
                FileOverride::Yes,
            ),
            (
                "gui/apt/vscode.sources",
                "/etc/apt/sources.list.d/vscode.sources",
                FileOverride::No,
            ),
        ]
    }

    fn gui_configuration_index(&self) -> ConfigurationFileIndex {
        &[
            // Alacritty
            (
                "gui/home/.config/alacritty/alacritty.toml",
                "~/.config/alacritty/alacritty.toml",
                FileOverride::No,
            ),
            (
                "gui/home/.config/alacritty/10-general.toml",
                "~/.config/alacritty/10-general.toml",
                FileOverride::No,
            ),
            (
                "gui/home/.config/alacritty/20-font.toml",
                "~/.config/alacritty/20-font.toml",
                FileOverride::No,
            ),
            (
                "gui/home/.config/alacritty/30-colors.toml",
                "~/.config/alacritty/30-colors.toml",
                FileOverride::No,
            ),
            (
                "gui/home/.config/alacritty/40-bindings.toml",
                "~/.config/alacritty/40-bindings.toml",
                FileOverride::No,
            ),
        ]
    }

    fn gui_packages(&self) -> PackageIndex {
        &[
            "alacritty",
            "code",
            "regolith-desktop",
            "regolith-session-sway",
            "regolith-look-gruvbox",
            "regolith-wm-user-programs",
            "swaylock"
        ]
    }
}
