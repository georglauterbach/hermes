//! This module contains the definitions of the functions that
//! perform the actual work, or call other subroutines that
//! perform work depending on the Ubuntu version.

mod apt;
mod configuration_files;
mod download;
mod programs;

/// The base part of URI that we fetch file from.
const GITHUB_RAW_URI: &str =
    "https://raw.githubusercontent.com/georglauterbach/hermes/refs/heads/main";

/// Does the actual work that _hermes_ is supposed to do.
///
/// #### Errors
///
/// If any of the function implementing the actual work fails, the error
/// is propagated. This is done for all functions, so that the context
/// is complete and the user is informed about all errors and issues.
pub async fn run(arguments: super::cli::Arguments) -> ::anyhow::Result<()> {
    ::log::debug!("I was now called correctly");

    ::log::trace!("Here is my environment:");
    if ::log::max_level() == ::log::Level::Trace {
        for (var_key, var_name) in ::std::env::vars() {
            println!("  {var_key}={var_name}");
        }
    }

    ::log::info!("Starting work now");

    let mut task_handler = ::tokio::task::JoinSet::new();
    ::log::debug!("Spawning tasks in async runtime");
    task_handler.spawn(apt::configure_system_with_apt(
        arguments.change_apt_sources,
        arguments.gui,
    ));
    task_handler.spawn(configuration_files::set_up_unversioned_configuration_files());
    task_handler.spawn(configuration_files::setup_up_versioned_configuration_files(
        arguments.gui,
    ));
    task_handler.spawn(programs::download_custom_programs());

    let mut errors = vec![];
    while let Some(handler) = task_handler.join_next().await {
        match handler {
            Ok(actual_result) => match actual_result {
                Ok(()) => (),
                Err(error) => {
                    errors.push(error);
                }
            },
            Err(error) => {
                errors.push(::anyhow::anyhow!(error));
            }
        }
    }

    super::evaluate_errors_vector!(errors, "Errors occured during execution")
}
