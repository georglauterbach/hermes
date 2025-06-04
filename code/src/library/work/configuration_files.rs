//! This module contains functions that take
//! care of managing configuration files.

use super::{
    super::{data, prepare::environment},
    GITHUB_RAW_URI,
};
use ::std::path;

/// This function takes care of placing all unversioned configuration files
/// onto the local file system.
#[::tracing::instrument(name = "configuration files", skip_all)]
pub(super) async fn place() -> ::anyhow::Result<()> {
    ::tracing::info!("Placing files");

    let mut join_set = ::tokio::task::JoinSet::new();
    let mut results = vec![];

    for (remote_part, local_path, overwrite) in super::super::data::INDEX {
        ::tracing::debug!("handling configuration file path '{local_path}' now");
        let local_path = local_path.replace('~', &environment::home_str());
        let canonical_local_path = match path::absolute(path::Path::new(&local_path)) {
            Ok(path) => path,
            Err(error) => {
                results.push(Err(::anyhow::anyhow!(error)));
                continue;
            }
        };

        if canonical_local_path.exists() && *overwrite == data::FileOverride::No {
            ::tracing::debug!("file {canonical_local_path:?} exists and shall not be overridden");
            continue;
        }

        let canonical_local_path_string = canonical_local_path
            .into_os_string()
            .to_string_lossy()
            .to_string();

        join_set.spawn(super::super::fs::download::download_and_place(
            format!("{GITHUB_RAW_URI}/data/core/{remote_part}"),
            canonical_local_path_string,
        ));
    }

    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(actual_result) => results.push(actual_result),
            Err(error) => {
                ::tracing::warn!("Could not join an async handle (bug?): {error}");
            }
        }
    }

    super::super::evaluate_results(results)
}
