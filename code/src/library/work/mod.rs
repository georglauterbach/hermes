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
pub async fn run(arguments: super::cli::Arguments) -> ::anyhow::Result<()> {
    ::tracing::debug!("I was now called correctly");

    ::tracing::trace!("Here is my environment:");
    for (var_key, var_name) in ::std::env::vars() {
        ::tracing::trace!("  {var_key}={var_name}");
    }

    match arguments.command {
        super::cli::Command::Run { install_packages } => {
            ::tracing::info!(target: "work", "Starting installation");

            let results = ::tokio::join!(
                configuration_files::place(),
                additional_programs::install(),
                packages::install(install_packages)
            );

            super::evaluate_results(<[Result<(), ::anyhow::Error>; 3]>::from(results))?;
            final_chown()
        }
        super::cli::Command::Update => {
            ::tracing::info!(target: "work", "Starting self-update");
            update::run().await
        }
    }
}

/// Perform a final `chown` on the calling user's home directory to
/// adjust the permissions once, which is most effective.
fn final_chown() -> ::anyhow::Result<()> {
    ::tracing::debug!("Running final 'chown'");

    let files_to_be_changed: Vec<String> = super::data::INDEX
        .iter()
        .map(|(_, local_path, _)| local_path.replace('~', &environment::home_str()))
        .collect();

    ::std::process::Command::new("chown")
        .arg("-R")
        .arg(format!("{}:{}", environment::user(), environment::group()))
        .args(files_to_be_changed)
        .arg(environment::home_str() + "/.local/bin")
        .output()
        .context("Could not generate output from final 'chown' on user directory")?
        .status
        .success()
        .then(|| Ok(()))
        .context("Final 'chown' on user directory failed")?
}
