//! This module contains functions that take
//! care of managing configuration files.

use super::{
    super::{data, prepare::environment},
    GITHUB_RAW_URI,
};
use ::std::path;

use ::anyhow::Context as _;

/// This function takes care of placing all unversioned configuration files
/// onto the local file system.
#[::tracing::instrument(skip_all, name = "pcf")]
pub(super) async fn place() -> ::anyhow::Result<()> {
    ::tracing::info!(target: "work", "Placing configuration files");

    let mut join_set = ::tokio::task::JoinSet::new();
    let mut errors = vec![];

    for (remote_part, local_path, overwrite) in super::super::data::INDEX {
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
            format!("{GITHUB_RAW_URI}/data/base/{remote_part}"),
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
                ::tracing::warn!("Could not join an async handle (bug?): {error}");
                errors.push(::anyhow::anyhow!(error));
            }
        }
    }

    super::super::evaluate_errors_vector!(errors, "Placing configuration files failed")
}
