//! This module contains the definitions of the functions that
//! perform the actual work, or call other subroutines that
//! perform work depending on the Ubuntu version.

use crate::prepare::environment;

mod apt;
mod configuration_files;
mod download;
mod programs;
mod update;

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
    ::tracing::debug!("I was now called correctly");

    if arguments.update {
        return update::update_self().await;
    }

    ::tracing::trace!("Here is my environment:");
    for (var_key, var_name) in ::std::env::vars() {
        ::tracing::trace!("  {var_key}={var_name}");
    }

    ::tracing::info!(target: "work", "Starting actual work now");

    let mut task_handler = ::tokio::task::JoinSet::new();
    ::tracing::debug!("Spawning tasks in async runtime");
    task_handler.spawn(apt::configure_system_with_apt(
        arguments.change_apt_sources,
        arguments.gui,
    ));
    task_handler.spawn(configuration_files::set_up_unversioned_configuration_files());
    task_handler.spawn(programs::install_additional_programs());

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

    final_chown(&mut errors);

    super::evaluate_errors_vector!(errors, "Errors occured during execution")
}

/// Perform a final `chown` on the calling user's home directory to
/// adjust the permissions once, which is most effective.
fn final_chown(errors: &mut Vec<::anyhow::Error>) {
    ::tracing::debug!(
        "Running final 'chown' on users directory '{}'",
        environment::home_str()
    );

    let output = ::std::process::Command::new("chown")
        .arg("-R")
        .arg(format!("{}:{}", environment::user(), environment::group()))
        .arg(environment::home())
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                errors.push(::anyhow::anyhow!("Final 'chown' on user directory failed"));
            }
        }
        Err(error) => errors.push(
            ::anyhow::anyhow!(error)
                .context("Could not generate output from final 'chown' on user directory"),
        ),
    }
}
