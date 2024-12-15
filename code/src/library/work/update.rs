//! This module contains the logic to update _hermes_ itself.

use ::anyhow::Context as _;

/// Updates _hermes_ itself.
pub(super) async fn update_self() -> ::anyhow::Result<()> {
    ::tracing::info!(target: "update", "Updating myself now");

    let url = ::reqwest::get("https://github.com/georglauterbach/hermes/releases/latest").await?;
    let url = url.url();
    let latest_version = url
        .as_str()
        .split('/')
        .last()
        .context("Could not acquire the latest version name")?;

    ::tracing::info!(target: "update", "The latest version is {latest_version}");

    let hermes_path = ::which::which("hermes").map_or_else(
        |_| String::from("/home/ubuntu/hermes"),
        |path| path.to_string_lossy().to_string(),
    );

    ::tracing::debug!("Placing new file at {hermes_path}");

    super::download::download_and_place(
      format!("https://github.com/georglauterbach/hermes/releases/download/{latest_version}/hermes-{latest_version}-{}-unknown-linux-musl", super::programs::ARCHITECTURE),
      hermes_path.clone()
    )
    .await
    .context("Could not download latest version")?;

    ::std::process::Command::new("chmod")
        .args(["+x", hermes_path.as_str()])
        .output()
        .context("Could not change permissions of new binary file")?;

    Ok(())
}
