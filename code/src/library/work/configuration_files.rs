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
    let mut request_handler = ::tokio::task::JoinSet::new();
    let mut success = true;
    let mut final_error = anyhow::anyhow!("Finished {log_prefix} with errors");

    for (remote_part, local_path, overwrite) in index {
        ::log::debug!("{log_prefix}: handling configuration file path '{local_path}' now");
        let log_prefix = format!("{log_prefix}: {local_path}: ");

        let local_path = local_path.replace('~', &environment::home_str());
        let canonical_local_path = match path::absolute(path::Path::new(&local_path)) {
            Ok(path) => path,
            Err(error) => {
                success = false;
                final_error = final_error.context(format!("{error}"));
                continue;
            }
        };

        if canonical_local_path.exists() && *overwrite == data::FileOverride::Yes {
            ::log::debug!("{log_prefix}file exists and shall not be overridden");
            continue;
        }

        request_handler.spawn(download_and_place_configuration_file(
            format!("{base_uri}/{remote_part}"),
            canonical_local_path,
            place_as_root,
        ));
    }

    while let Some(result) = request_handler.join_next().await {
        match result {
            Ok(actual_result) => match actual_result {
                Ok(()) => (),
                Err(error) => {
                    let message =
                        format!("Something went wrong placing a configuration file: {error}");
                    ::log::warn!("{message}");
                    final_error = error.context(message);
                    success = false;
                }
            },
            Err(error) => {
                let message = format!(
                    "Could not join an async handle (this should not have happened): {error}"
                );
                ::log::warn!("{message}");
                final_error = final_error.context(message);
                success = false;
            }
        }
    }

    if success {
        Ok(())
    } else {
        Err(final_error)
    }
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
            ::log::info!("Finished PUCF successfully");
            Ok(())
        }
        Err(error) => {
            Err(::anyhow::anyhow!("Finished PUCF with errors")).context(format!("{error}"))
        }
    }
}

/// This function takes care of placing all versioned configuration files
/// onto the local file system.
pub(super) async fn setup_up_versioned_configuration_files(gui: bool) -> ::anyhow::Result<()> {
    ::log::info!("Placing versioned configuration files (PVCF)");
    let mut contexts: Vec<String> = vec![];

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
            contexts.push(format!("{error}"));
        }

        log::debug!("To change the bookmarks in file explorers, edit ~/.config/user-firs.dirs, ~/.config/gtk-3.0/bookmarks, and /etc/xdg/user-dirs.defaults");
    }

    if contexts.is_empty() {
        ::log::info!("Finished PVCF successfully");
        Ok(())
    } else {
        ::log::warn!("Finished PVCF with errors");
        let mut error = ::anyhow::anyhow!("Finished PVCF with errors");
        for context in contexts {
            error = error.context(context);
        }
        Err(error)
    }
}
