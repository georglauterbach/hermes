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
) -> ::anyhow::Result<()> {
    let mut join_set = ::tokio::task::JoinSet::new();
    let mut errors = vec![];

    for (remote_part, local_path, overwrite) in index {
        ::tracing::debug!("handling configuration file path '{local_path}' now");
        let local_path = local_path.replace('~', &environment::home_str());
        let canonical_local_path = match path::absolute(path::Path::new(&local_path)) {
            Ok(path) => path,
            Err(error) => {
                errors.push(::anyhow::anyhow!(error));
                continue;
            }
        };

        if canonical_local_path.exists() && *overwrite == data::FileOverride::No {
            ::tracing::debug!("file {canonical_local_path:?} exists and shall not be overridden");
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
#[::tracing::instrument(skip_all, name = "pucf")]
pub(super) async fn set_up_unversioned_configuration_files() -> ::anyhow::Result<()> {
    ::tracing::info!(target: "work", "Placing unversioned configuration files (PUCF)");

    let result = download_and_place_configuration_files(
        super::super::data::unversioned::INDEX,
        format!("{GITHUB_RAW_URI}/data/unversioned"),
    )
    .await;

    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(error).context("Finished PUCF with errors"),
    }
}
