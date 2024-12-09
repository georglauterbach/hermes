//! TODO

use super::{
    super::{data, fs, prepare::environment},
    GITHUB_RAW_URI,
};
use ::std::path;

use ::anyhow::Context as _;

/// Download a file and place it onto the file system.
pub async fn download_and_place_configuration_file(
    request_uri: String,
    absolute_local_path: path::PathBuf,
    place_as_root: bool,
) -> ::anyhow::Result<()> {
    super::download::download_and_place(
        request_uri,
        absolute_local_path.to_string_lossy().to_string(),
    )
    .await?;

    fs::adjust_permissions(
        &absolute_local_path,
        if place_as_root { 0 } else { environment::uid() },
        if place_as_root { 0 } else { environment::gid() },
        0o644,
    )?;

    Ok(())
}

/// TODO
pub(super) async fn download_and_place_configuration_files(
    index: data::ConfigurationFileIndex,
    place_as_root: bool,
    base_uri: String,
    log_prefix: &'static str,
) -> ::anyhow::Result<()> {
    let mut join_set = ::tokio::task::JoinSet::new();
    let mut errors = vec![];

    for (remote_part, local_path, overwrite) in index {
        ::log::debug!("{log_prefix}: handling configuration file path '{local_path}' now");
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
            ::log::debug!("{log_prefix}file exists and shall not be overridden");
            continue;
        }

        join_set.spawn(download_and_place_configuration_file(
            format!("{base_uri}/{remote_part}"),
            canonical_local_path,
            place_as_root,
        ));
    }

    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(actual_result) => match actual_result {
                Ok(()) => (),
                Err(error) => {
                    ::log::warn!("Something went wrong placing a configuration file: {error}");
                    errors.push(error);
                }
            },
            Err(error) => {
                ::log::warn!(
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
    ::log::info!("Placing unversioned configuration files (PUCF)");

    let result = download_and_place_configuration_files(
        super::super::data::unversioned::INDEX,
        false,
        format!("{GITHUB_RAW_URI}/data/unversioned"),
        "PUCF",
    )
    .await;

    match result {
        Ok(()) => {
            Ok(())
        }
        Err(error) => Err(error).context("Finished PUCF with errors"),
    }
}

/// This function takes care of placing all versioned configuration files
/// onto the local file system.
pub(super) async fn setup_up_versioned_configuration_files(gui: bool) -> ::anyhow::Result<()> {
    ::log::info!("Placing versioned configuration files (PVCF)");
    let mut errors = vec![];

    if gui {
        if let Err(error) = download_and_place_configuration_files(
            data::versioned::get_version_information().gui_configuration_index(),
            false,
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

        log::debug!("To change the bookmarks in file explorers, edit ~/.config/user-firs.dirs, ~/.config/gtk-3.0/bookmarks, and /etc/xdg/user-dirs.defaults");
    }

    super::super::evaluate_errors_vector!(errors, "Finished PVCF with errors")
}
