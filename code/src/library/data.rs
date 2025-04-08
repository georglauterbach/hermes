//! This module holds all data, that is: indices, package lists, etc.

/// An index that can be iterated over, containing an URI-part this this project's
/// repository to download a file from, a path on the local file system (not
/// canonical or absolute), and whether to override the file if it exists already.
pub type ConfigurationFileIndex = &'static [(&'static str, &'static str, FileOverride)];

/// A list of packages that are to be installed.
pub type PackageIndex = &'static [&'static str];

/// Indicates whether to override an existing file on the local file system
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileOverride {
    /// Override
    No,
    /// Do not override
    Yes,
}

impl From<FileOverride> for bool {
    fn from(value: FileOverride) -> Self {
        match value {
            FileOverride::No => false,
            FileOverride::Yes => true,
        }
    }
}

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
        "~/.config/bash/50-hermes.sh",
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
