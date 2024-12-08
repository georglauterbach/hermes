//! Contains all information about the different
//! versions of Ubuntu that are supported by this tool.

use super::super::cli;

mod fallback;
mod ubuntu_24_04;

/// A type can implement this trait to provide the necessary information about packages,
/// GUI-related configuration, etc. for a specific version of Ubuntu.
pub trait UbuntuVersion: Send + Sync {
    /// The base set of packages that is to be installed for this version of Ubuntu.
    /// Extends an already existing set of packages that will always be installed.
    fn base_packages(&self) -> super::PackageIndex;

    /// A list of extra APT source files that are to be installed for this version of Ubuntu.
    fn apt_index(&self) -> super::ConfigurationFileIndex;

    /// A list of extra APT source files for the GUI that are to be installed
    /// for this version of Ubuntu.
    fn gui_apt_index(&self) -> super::ConfigurationFileIndex;
    /// A list of extra configuration files for the GUI that are to be installed
    /// for the GUI for this version of Ubuntu.
    fn gui_configuration_index(&self) -> super::ConfigurationFileIndex;
    /// A list of packages that are to be installed for the GUI for this version of Ubuntu.
    fn gui_packages(&self) -> super::PackageIndex;
}

/// Return the
pub fn get_version_information() -> &'static dyn UbuntuVersion {
    match super::super::prepare::environment::ubuntu_version_id() {
        cli::UbuntuVersion::Fallback => &fallback::Fallback,
        cli::UbuntuVersion::Ubuntu24_04 => &ubuntu_24_04::Ubuntu24_04,
    }
}
