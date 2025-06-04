//! This module contains the definitions of the functions that
//! perform the actual work, or call other subroutines that
//! perform work depending on the Ubuntu version.

mod additional_programs;
mod configuration_files;
mod download;
mod packages;
mod update;

use crate::prepare::environment;
use ::anyhow::Context as _;

/// The common base part of URI that we fetch file from.
const GITHUB_RAW_URI_COMMON: &str =
    "https://raw.githubusercontent.com/georglauterbach/hermes/refs/";

/// The base part of URI that we fetch file from.
#[cfg(debug_assertions)]
const GITHUB_RAW_URI: &str = ::const_format::concatcp!(GITHUB_RAW_URI_COMMON, "heads/main");

/// The base part of URI that we fetch file from.
#[cfg(not(debug_assertions))]
const GITHUB_RAW_URI: &str =
    ::const_format::concatcp!(GITHUB_RAW_URI_COMMON, "tags/v", env!("CARGO_PKG_VERSION"));

/// Does the actual work that _hermes_ is supposed to do.
///
/// #### Errors
///
/// If any of the function implementing the actual work fails, the error
/// is propagated. This is done for all functions, so that the context
/// is complete and the user is informed about all errors and issues.
pub async fn run(arguments: super::arguments::Arguments) -> ::anyhow::Result<()> {
    ::tracing::debug!("I was now called correctly");

    ::tracing::trace!("Here is my environment:");
    for (var_key, var_name) in ::std::env::vars() {
        ::tracing::trace!("  {var_key}={var_name}");
    }

    match arguments.command {
        super::arguments::Command::Run { install_packages } => {
            ::tracing::info!("Starting installation");

            let results = ::tokio::join!(
                configuration_files::place(),
                additional_programs::install(),
                packages::install(install_packages)
            );

            super::evaluate_results(<[Result<(), ::anyhow::Error>; 3]>::from(results))?;
            final_chown().context("Aborted permission adjustments early")
        }
        super::arguments::Command::Update => {
            ::tracing::info!("Starting self-update");
            update::run().await
        }
    }
}

/// Perform a final `chown` on the calling user's home directory to
/// adjust the permissions once, which is most effective.
fn final_chown() -> ::anyhow::Result<()> {
    fn chown(path: impl AsRef<std::path::Path>, uid: u32, gid: u32) -> ::anyhow::Result<()> {
        let path = path.as_ref();

        if path.exists() {
            std::os::unix::fs::chown(path, Some(uid), Some(gid))
                .context(format!("Could not change permissions of {path:?}"))
        } else {
            tracing::debug!("Path {path:?} does not exist - not adjusting permissions");
            Ok(())
        }
    }

    ::tracing::debug!("Adjusting permissions of touched directories and files");

    let uid = environment::uid();
    let gid = environment::gid();

    for subdirectory_non_recursive in [".cache", ".local", ".local/state"] {
        let path = environment::home_and(subdirectory_non_recursive);
        let path = std::path::Path::new(&path);
        if path.exists() {
            chown(path, uid, gid)?;
        }
    }

    for subdirectory_recursive in [".bashrc", ".config", ".local/bin", ".local/share"] {
        for file in ::walkdir::WalkDir::new(environment::home_and(subdirectory_recursive)) {
            match file {
                Ok(file) => chown(file.path(), uid, gid)?,
                Err(error) => {
                    tracing::warn!(
                        "Iterating over a file or directory in '{subdirectory_recursive}' not possible: {error}"
                    );
                }
            }
        }
    }

    Ok(())
}
