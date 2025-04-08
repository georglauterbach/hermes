//! This module contains the logic to update _hermes_ itself.

use ::anyhow::Context as _;

/// Updates _hermes_ itself.
#[::tracing::instrument(name = "update", skip_all)]
pub(super) async fn run() -> ::anyhow::Result<()> {
    ::tracing::info!("Updating myself now");

    let url = ::reqwest::get("https://github.com/georglauterbach/hermes/releases/latest").await?;
    let url = url.url();
    let latest_version = url
        .as_str()
        .split('/')
        .last()
        .context("Could not acquire the latest version name")?;

    ::tracing::info!("The latest version is {latest_version}");

    let hermes_tmp_path = std::path::PathBuf::from("/tmp/hermes_updated");
    if hermes_tmp_path.exists() {
        ::std::fs::remove_file(hermes_tmp_path.clone()).context(format!(
            "Could not remove existing temporary file {hermes_tmp_path:?}"
        ))?;
    }
    let hermes_tmp_path = hermes_tmp_path.to_string_lossy().to_string();
    ::tracing::debug!("Placing new file at {hermes_tmp_path:?}");

    super::download::download_and_place(
      format!("https://github.com/georglauterbach/hermes/releases/download/{latest_version}/hermes-{latest_version}-{}-unknown-linux-musl", super::additional_programs::ARCHITECTURE),
      hermes_tmp_path.clone()
    )
    .await
    .context("Could not download latest version")?;

    let hermes_current_path = ::which::which("hermes").map_or_else(
        |_| String::from("/usr/local/bin/hermes"),
        |path| path.to_string_lossy().to_string(),
    );

    std::fs::rename(hermes_tmp_path.clone(), hermes_current_path.clone()).context(
        format!("Could not move new binary to current installation location (run 'sudo mv {hermes_tmp_path} {hermes_current_path}' yourself)"),
    )?;

    ::std::process::Command::new("chmod")
        .args(["+x", hermes_current_path.as_str()])
        .output()
        .context(format!(
            "Could not change permissions of binary file at '{hermes_current_path}'"
        ))?;

    Ok(())
}
