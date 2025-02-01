//! Contains information about unversioned
//! configuration files.

use super::{ConfigurationFileIndex, FileOverride};

/// The actual list of configuration files. The list
/// contains tuples. Each tuple contains
///
/// 1. the remote part (a part of the request URL), and
/// 2. the non-canonical path on the local file system.
pub const INDEX: ConfigurationFileIndex = &[
    // Starship
    (
        "home/.config/starship/starship.toml",
        "~/.config/starship/starship.toml",
        FileOverride::No,
    ),
    // Bash
    ("home/.bashrc", "~/.bashrc", FileOverride::Yes),
    (
        "home/.config/bash/20-custom_early.sh",
        "~/.config/bash/20-custom_early.sh",
        FileOverride::No,
    ),
    (
        "home/.config/bash/50-hermes.sh",
        "~/.config/bash/40-misc.sh",
        FileOverride::Yes,
    ),
    (
        "home/.config/bash/99-custom_late.sh",
        "~/.config/bash/99-custom_late.sh",
        FileOverride::No,
    ),
    // bat
    (
        "home/.config/bat/themes/everforest-light.tmTheme",
        "~/.config/bat/themes/everforest-light.tmTheme",
        FileOverride::No,
    ),
    (
        "home/.config/bat/themes/gruvbox-material-dark.tmTheme",
        "~/.config/bat/themes/gruvbox-material-dark.tmTheme",
        FileOverride::No,
    ),
    // ble.sh
    (
        "home/.config/blesh/init.sh",
        "~/.config/blesh/init.sh",
        FileOverride::No,
    ),
    // NeoVim
    (
        "home/.config/nvim/init.lua",
        "~/.config/nvim/init.lua",
        FileOverride::No,
    ),
    // Zellij
    (
        "home/.config/zellij/config.kdl",
        "~/.config/zellij/config.kdl",
        FileOverride::No,
    ),
];
