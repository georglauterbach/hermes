//! Contains functions associated with the file system, i.e.
//! changing permissions, placing files on the FS, etc.

use ::anyhow::Context as _;

/// Crate the parent directory of a file or directory.
pub async fn create_parent_dir(directory: &String) -> ::anyhow::Result<&::async_std::path::Path> {
    let Some(parent_dir) = ::async_std::path::Path::new(directory).parent() else {
        anyhow::bail!("Could not get parent directory of '{directory}'");
    };

    if !parent_dir.exists().await {
        ::async_std::fs::create_dir_all(parent_dir)
            .await
            .context(format!(
                "Could not create parent directory '{parent_dir:?}'"
            ))?;
    }

    Ok(parent_dir)
}
