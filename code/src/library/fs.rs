//! Contains functions associated with the file system, i.e.
//! changing permissions, placing files on the FS, etc.

use ::anyhow::Context as _;

/// Crate the parent directory of a file or directory.
pub async fn create_parent_dir(directory: &str) -> ::anyhow::Result<&::async_std::path::Path> {
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

pub mod download {
    //! This module contains functions to download files from
    //! the internet and and place them on the local file system.

    use super::super::fs;

    use ::anyhow::Context as _;
    use ::async_std::io::WriteExt as _;

    /// Downloads a file asynchronously and returns its contents as [`::bytes::Bytes`].
    #[::tracing::instrument(skip_all)]
    pub async fn download(
        uri: impl AsRef<str> + Send + ::std::fmt::Debug,
    ) -> ::anyhow::Result<::bytes::Bytes> {
        let uri = uri.as_ref();

        ::tracing::trace!("Downloading from '{uri}'");
        ::reqwest::get(uri)
            .await
            .context("BUG! reqwest encountered an error that prevented the start of the download")?
            .error_for_status()?
            .bytes()
            .await
            .context(format!("Could not get request contents of URL '{uri}'"))
    }

    /// Uses [`download`] to download a file and writes it to a local path.
    pub async fn download_and_place(
        uri: String,
        local_path: impl AsRef<str>,
    ) -> ::anyhow::Result<()> {
        let response = download(&uri).await?;
        let local_path = local_path.as_ref();

        ::tracing::trace!("Placing file '{local_path}'");
        fs::create_parent_dir(local_path).await?;
        ::async_std::fs::File::create(local_path)
            .await
            .context(format!("Could not create file '{local_path}'"))?
            .write_all(&response[..])
            .await
            .context(format!(
                "Could not write data from request to file {local_path}"
            ))?;

        Ok(())
    }
}

pub mod extract {
    use std::collections;

    use ::anyhow::Context as _;
    use ::async_std::stream::StreamExt as _;

    /// Extracts files from an TAR archive and places them in the local
    /// file system. Paths to extract are given in `entry_path_mappings` key set,
    /// and their corresponding local paths are in the value of the map.
    async fn extract_from_tar_archive<R>(
        mut archive: ::tokio_tar::Archive<R>,
        mut entry_path_mappings: collections::HashMap<String, String>,
    ) -> anyhow::Result<()>
    where
        R: ::tokio::io::AsyncRead + Unpin + Send,
    {
        let mut entries = archive
            .entries()
            .context("Could not turn archive into iterator over entries")?;
        while let Some(entry) = entries.next().await {
            let mut entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    ::tracing::warn!("Could not get entry from archive: {error}");
                    continue;
                }
            };

            let entry_path_str = match entry.path() {
                Ok(path) => path.to_string_lossy().to_string(),
                Err(error) => {
                    ::tracing::warn!("Could get acquire path of entry '{error}'");
                    continue;
                }
            };

            if let Some(local_path) = entry_path_mappings.remove(&entry_path_str) {
                super::create_parent_dir(&local_path).await?;
                ::tracing::trace!("Unpacking {entry_path_str} from archive to {local_path}");
                if let Err(error) = entry.unpack(&local_path).await {
                    ::tracing::warn!(
                        "Could not unpack entry '{entry_path_str}' to '{local_path}': {error}"
                    );
                    continue;
                }

                if entry_path_mappings.is_empty() {
                    break;
                }
            }
        }

        if !entry_path_mappings.is_empty() {
            ::tracing::warn!(
                "Not all desired entries from the archive were unpacked: {:?}",
                entry_path_mappings.keys()
            );
        }

        Ok(())
    }

    /// Extracts files from an ZIP archive and places them in the local
    /// file system. Paths to extract are given in `entry_path_mappings` key set,
    /// and their corresponding local paths are in the value of the map.
    async fn extract_from_zip_archive(
        archive: ::bytes::Bytes,
        entry_path_mappings: collections::HashMap<String, String>,
    ) -> anyhow::Result<()> {
        use std::io::Read as _;
        use std::os::unix::fs::PermissionsExt as _;

        tokio::task::spawn( async move {
        let mut decoder_archive =
            ::zip::ZipArchive::new(std::io::Cursor::new(&archive[..]))
            .context("Could not build ZIP archive reader - ZIP malformed?")?;

        for (filename_in_archive, path_on_fs) in &entry_path_mappings {
            let mut zip_file = decoder_archive
                .by_name(filename_in_archive)
                .context(format!("File '{filename_in_archive}' could not be found"))?;

            let mut content = Vec::with_capacity(4096);
            zip_file
                .read_to_end(&mut content)
                .context(format!("Could not read file '{filename_in_archive}'"))?;

            super::create_parent_dir(path_on_fs).await?;
            std::fs::write(path_on_fs, content).context(format!("Could not write file '{filename_in_archive}' from archive to file system location '{path_on_fs}'"))?;

            std::fs::set_permissions(path_on_fs, std::fs::Permissions::from_mode(zip_file.unix_mode().unwrap_or(0o755))).context(format!("Could not set correct permissions for file '{path_on_fs}'"))?;
        }

        Ok(())
    })
    .await
    .context("Unzipping archive did not succeed")?
    }

    /// Download a TAR or ZIP archive and extracts it
    pub async fn download_and_extract(
        uri: String,
        entry_path_mappings: collections::HashMap<String, String>,
    ) -> ::anyhow::Result<()> {
        let response = super::download::download(&uri).await?;
        let uri_extension = std::path::Path::new(&uri)
            .extension()
            .ok_or_else(|| anyhow::Error::msg("Could not identify archive format"))?;

        if uri_extension.eq_ignore_ascii_case("gz") {
            let gz_decoder = ::async_compression::tokio::bufread::GzipDecoder::new(&response[..]);
            extract_from_tar_archive(::tokio_tar::Archive::new(gz_decoder), entry_path_mappings)
                .await
        } else if uri_extension.eq_ignore_ascii_case("xz") {
            let xz_decoder = ::async_compression::tokio::bufread::XzDecoder::new(&response[..]);
            extract_from_tar_archive(::tokio_tar::Archive::new(xz_decoder), entry_path_mappings)
                .await
        } else if uri_extension.eq_ignore_ascii_case("zip") {
            extract_from_zip_archive(response, entry_path_mappings).await
        } else {
            anyhow::bail!("Unknown archive format for URI '{uri}'");
        }
    }
}
