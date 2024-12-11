//! Contains functions associated with the file system, i.e.
//! changing permissions, placing files on the FS, etc.

use ::anyhow::Context as _;

/// Adjust the owner, group, and permissions of a file or directory.
pub fn adjust_permissions(
    path: &impl AsRef<::std::path::Path>,
    root: bool,
    mask: u32,
) -> ::anyhow::Result<()> {
    use ::std::os::unix::fs::PermissionsExt as _;
    use anyhow::Context as _;

    let (uid, gid) = if root {
        (0, 0)
    } else {
        (
            super::prepare::environment::uid(),
            super::prepare::environment::gid(),
        )
    };

    let path_str = path.as_ref().to_string_lossy();
    ::log::trace!("{path_str}: adjusting permissions");

    ::std::os::unix::fs::chown(path, Some(uid), Some(gid))
        .context(format!("{path_str}: could not change owner and group"))?;

    let mut permissions = path
        .as_ref()
        .metadata()
        .context(format!("{path_str}: could not access metadata"))?
        .permissions();
    permissions.set_mode(mask);
    ::std::fs::set_permissions(path, permissions)
        .context(format!("{path_str}: could not change permissions"))?;

    Ok(())
}

/// Crate the parent directory of a file or directory.
pub async fn create_parent_dir(
    directory: &String,
    create_as_root: bool,
) -> ::anyhow::Result<::async_std::path::PathBuf> {
    let Some(parent_dir) = ::async_std::path::Path::new(directory).parent() else {
        anyhow::bail!("Could not get parent directory of '{directory:?}'");
    };

    let parent_dir = parent_dir.to_owned();
    if !parent_dir.exists().await {
        ::async_std::fs::create_dir_all(&parent_dir)
            .await
            .context(format!(
                "Could not create parent directory '{parent_dir:?}'"
            ))?;

        adjust_permissions(&parent_dir, create_as_root, 0o755)?;
    }

    Ok(parent_dir)
}
