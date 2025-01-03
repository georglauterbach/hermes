//! Contains information about unversioned
//! configuration files.

use super::{ConfigurationFileIndex, FileOverride};

/// The actual list of configuration files. The list
/// contains tuples. Each tuple contains
///
/// 1. the remote part (a part of the request URL), and
/// 2. the non-canonical path on the local file system.
pub const INDEX: ConfigurationFileIndex = &[
    // Bash
    ("home/.bashrc", "~/.bashrc", FileOverride::Yes),
    (
        "home/.config/bash/00-base.sh",
        "~/.config/bash/00-base.sh",
        FileOverride::Yes,
    ),
    (
        "home/.config/bash/20-custom_early.sh",
        "~/.config/bash/20-custom_early.sh",
        FileOverride::No,
    ),
    (
        "home/.config/bash/40-misc.sh",
        "~/.config/bash/40-misc.sh",
        FileOverride::Yes,
    ),
    (
        "home/.config/bash/50-init_extra_programs.sh",
        "~/.config/bash/50-init_extra_programs.sh",
        FileOverride::Yes,
    ),
    (
        "home/.config/bash/60-override_commands.sh",
        "~/.config/bash/60-override_commands.sh",
        FileOverride::Yes,
    ),
    (
        "home/.config/bash/80-global_aliases.sh",
        "~/.config/bash/80-global_aliases.sh",
        FileOverride::Yes,
    ),
    (
        "home/.config/bash/99-custom_late.sh",
        "~/.config/bash/99-custom_late.sh",
        FileOverride::No,
    ),
    (
        "home/.config/blesh/init.sh",
        "~/.config/blesh/init.sh",
        FileOverride::No,
    ),
    (
        "home/.config/nvim/init.lua",
        "~/.config/nvim/init.lua",
        FileOverride::No,
    ),
    (
        "home/.config/starship/starship.toml",
        "~/.config/starship/starship.toml",
        FileOverride::No,
    ),
];
