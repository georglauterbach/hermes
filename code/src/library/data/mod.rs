//! This module holds all data, that is: indices, package lists, etc.

pub mod unversioned;
pub mod versioned;

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
