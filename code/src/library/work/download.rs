//! This module contains functions to download files from
//! the internet and and place them on the local file system.

use ::anyhow::Context as _;
use ::async_std::io::WriteExt as _;

/// Downloads a file asynconously and returns its contents as [`::bytes::Bytes`].
pub(super) async fn download_file(uri: impl AsRef<str> + Send) -> ::anyhow::Result<::bytes::Bytes> {
    let uri = uri.as_ref();

    ::log::trace!("Downloading from '{uri}'");
    ::reqwest::get(uri)
        .await
        .context("BUG! reqwest encountered an error that prevented the start of the download")?
        .error_for_status()?
        .bytes()
        .await
        .context(format!("Could not get request contents of URL '{uri}'"))
}

/// Uses [`download_file`] to download a file and writes it to a local path.
pub(super) async fn download_and_place(uri: String, local_path: String) -> ::anyhow::Result<()> {
    let response = download_file(&uri).await?;

    ::log::trace!("Placing file '{local_path}'");
    super::super::fs::create_parent_dir(&local_path).await?;
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
