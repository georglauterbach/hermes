//! This module contains functions to download files from
//! the internet and and place them on the local file system.

use super::super::fs;

use ::anyhow::Context as _;
use ::async_std::io::WriteExt as _;

/// Downloads a file asynchronously and returns its contents as [`::bytes::Bytes`].
#[::tracing::instrument(skip_all)]
pub(super) async fn download(
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
pub(super) async fn download_and_place(uri: String, local_path: String) -> ::anyhow::Result<()> {
    let response = download(&uri).await?;

    ::tracing::trace!("Placing file '{local_path}'");
    fs::create_parent_dir(&local_path).await?;
    ::async_std::fs::File::create(&local_path)
        .await
        .context(format!("Could not create file '{local_path}'"))?
        .write_all(&response[..])
        .await
        .context(format!(
            "Could not write data from request to file {local_path}"
        ))?;

    Ok(())
}

/// Download a single file and place it onto the file system.
#[::tracing::instrument]
pub(super) async fn download_and_place_configuration_file(
    request_uri: String,
    absolute_local_path: ::std::path::PathBuf,
) -> ::anyhow::Result<()> {
    download_and_place(
        request_uri,
        absolute_local_path.to_string_lossy().to_string(),
    )
    .await?;
    Ok(())
}
