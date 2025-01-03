//! Definitions for Ubuntu 22.04.
//!
//! Ubuntu 22.04 consists only of unversioned data,
//! and does not actually install anything else.

use super::super::{ConfigurationFileIndex, PackageIndex};

/// No specific version of Ubuntu was detected, so we fall back to this.
pub struct Fallback;

impl super::UbuntuVersion for Fallback {
    fn base_packages(&self) -> PackageIndex {
        &[
            "gnupg2",
            "openssh-client",
            "pinentry-curses",
            "vim",
            "xz-utils",
        ]
    }

    fn apt_index(&self) -> ConfigurationFileIndex {
        &[]
    }

    fn gui_apt_index(&self) -> ConfigurationFileIndex {
        &[]
    }

    fn gui_configuration_index(&self) -> ConfigurationFileIndex {
        &[]
    }

    fn gui_packages(&self) -> PackageIndex {
        &[]
    }

    fn gui_packages_removal(&self) -> PackageIndex {
        &[]
    }
}
