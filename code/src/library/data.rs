//! This module holds all data, that is: indices, package lists, etc.

/// An index that can be iterated over, containing an URI-part this this project's
/// repository to download a file from, a path on the local file system (not
/// canonical or absolute), and whether to override the file if it exists already.
pub type ConfigurationFileIndex = &'static [(&'static str, &'static str, FileOverride)];

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
        "starship/starship.toml",
        "~/.config/starship/starship.toml",
        FileOverride::No,
    ),
    // Bash
    (
        "bash/90-hermes.sh",
        "~/.config/bash/90-hermes.sh",
        FileOverride::Yes,
    ),
    (
        "bash/91-hermes_settings.sh",
        "~/.config/bash/91-hermes_settings.sh",
        FileOverride::Yes,
    ),
    // bat
    (
        "bat/themes/everforest-light.tmTheme",
        "~/.config/bat/themes/everforest-light.tmTheme",
        FileOverride::No,
    ),
    (
        "bat/themes/gruvbox-material-dark.tmTheme",
        "~/.config/bat/themes/gruvbox-material-dark.tmTheme",
        FileOverride::No,
    ),
    // ble.sh
    (
        "blesh/init.sh",
        "~/.config/blesh/init.sh",
        FileOverride::No,
    ),
    // NeoVim
    (
        "nvim/init.lua",
        "~/.config/nvim/init.lua",
        FileOverride::No,
    ),
    // Zellij
    (
        "zellij/config.kdl",
        "~/.config/zellij/config.kdl",
        FileOverride::No,
    ),
];
