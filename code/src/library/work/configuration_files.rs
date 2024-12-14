//! This module contains functions that take
//! care of managing configuration files.

use super::{
    super::{data, prepare::environment},
    GITHUB_RAW_URI,
};
use ::std::path;

use ::anyhow::Context as _;

/// Given an `index`, iterates over the index asynchronously using
/// [`super::download::download_and_place`] to download and place
/// said configuration files.
pub(super) async fn download_and_place_configuration_files(
    index: data::ConfigurationFileIndex,
    base_uri: String,
    log_prefix: &'static str,
) -> ::anyhow::Result<()> {
    let mut join_set = ::tokio::task::JoinSet::new();
    let mut errors = vec![];

    for (remote_part, local_path, overwrite) in index {
        ::tracing::debug!("{log_prefix}: handling configuration file path '{local_path}' now");
        let log_prefix = format!("{log_prefix}: {local_path}: ");

        let local_path = local_path.replace('~', &environment::home_str());
        let canonical_local_path = match path::absolute(path::Path::new(&local_path)) {
            Ok(path) => path,
            Err(error) => {
                errors.push(::anyhow::anyhow!(error));
                continue;
            }
        };

        if canonical_local_path.exists() && *overwrite == data::FileOverride::No {
            ::tracing::debug!("{log_prefix}file exists and shall not be overridden");
            continue;
        }

        join_set.spawn(super::download::download_and_place_configuration_file(
            format!("{base_uri}/{remote_part}"),
            canonical_local_path,
        ));
    }

    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(actual_result) => match actual_result {
                Ok(()) => (),
                Err(error) => {
                    ::tracing::warn!("Something went wrong placing a configuration file: {error}");
                    errors.push(error);
                }
            },
            Err(error) => {
                ::tracing::warn!(
                    "Could not join an async handle (this should not have happened): {error}"
                );
                errors.push(::anyhow::anyhow!(error));
            }
        }
    }

    super::super::evaluate_errors_vector!(errors, "Placing configuration files from index failed")
}

/// This function takes care of placing all unversioned configuration files
/// onto the local file system.
pub(super) async fn set_up_unversioned_configuration_files() -> ::anyhow::Result<()> {
    ::tracing::info!(target: "work", "Placing unversioned configuration files (PUCF)");

    let result = download_and_place_configuration_files(
        super::super::data::unversioned::INDEX,
        format!("{GITHUB_RAW_URI}/data/unversioned"),
        "PUCF",
    )
    .await;

    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(error).context("Finished PUCF with errors"),
    }
}

/// This function takes care of placing all versioned configuration files
/// onto the local file system.
pub(super) async fn setup_up_versioned_configuration_files(gui: bool) -> ::anyhow::Result<()> {
    ::tracing::info!(target: "work", "Placing versioned configuration files (PVCF)");
    let mut errors = vec![];

    if gui {
        if let Err(error) = download_and_place_configuration_files(
            data::versioned::get_version_information().gui_configuration_index(),
            format!(
                "{GITHUB_RAW_URI}/data/versioned/{}",
                environment::ubuntu_version_id()
            ),
            "PVCF (GUI)",
        )
        .await
        {
            errors.push(error);
        }

        ::tracing::debug!("To change the bookmarks in file explorers, edit ~/.config/user-firs.dirs, ~/.config/gtk-3.0/bookmarks, and /etc/xdg/user-dirs.defaults");
    }

    super::super::evaluate_errors_vector!(errors, "Finished PVCF with errors")
}
