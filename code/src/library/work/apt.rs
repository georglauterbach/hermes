//! This module contains functions that perform various
//! operations with APT, such as updating package signatures
//! or installing packages.

use super::{
    super::{data, prepare::environment},
    configuration_files, GITHUB_RAW_URI,
};

/// Change APT sources in `/etc/apt/sources.list.d/` if requested
/// by the user
#[::tracing::instrument(skip_all, name = "cswa")]
async fn set_up_new_apt_sources(
    ubuntu: &dyn data::versioned::UbuntuVersion,
    change_apt_sources: bool,
    gui: bool,
) -> ::anyhow::Result<()> {
    ::tracing::info!(target: "work", "Configuring system with APT (CSWA)");
    let mut errors = vec![];

    if change_apt_sources {
        ::tracing::debug!("Changing APT sources");
        if let Err(error) = configuration_files::download_and_place_configuration_files(
            ubuntu.apt_index(),
            format!(
                "{GITHUB_RAW_URI}/data/versioned/{}/apt",
                environment::ubuntu_version_id()
            ),
        )
        .await
        {
            ::tracing::warn!("Changing APT sources failed");
            errors.push(error);
        };
    }

    if gui {
        ::tracing::debug!("Changing GUI APT sources");
        if let Err(error) = configuration_files::download_and_place_configuration_files(
            ubuntu.gui_apt_index(),
            format!(
                "{GITHUB_RAW_URI}/data/versioned/{}/apt_gui",
                environment::ubuntu_version_id()
            ),
        )
        .await
        {
            ::tracing::warn!("Updating GUI APT sources failed");
            errors.push(error);
        };
    }

    super::super::evaluate_errors_vector!(errors, "Changing APT sources failed")
}

/// Configures the system with APT, which boils down to
///
/// 1. updating APT sources if requested,
/// 2. updating package signatures (version-specific),
/// 3. upgrading packages,
/// 4. autoremoving unused packages,
/// 5. installing packages (version-specific).
pub(super) async fn configure_system_with_apt(
    change_apt_sources: bool,
    gui: bool,
) -> ::anyhow::Result<()> {
    ::tracing::info!(target: "work", "Configuring system with APT (CSWA)");
    let ubuntu = data::versioned::get_version_information();

    set_up_new_apt_sources(ubuntu, change_apt_sources, gui).await?;

    ::tracing::debug!("Updating APT package signatures");
    if !::async_std::process::Command::new("apt-get")
        .args(["--yes", "update"])
        .stdout(::std::process::Stdio::null())
        .stderr(::std::process::Stdio::inherit())
        .output()
        .await?
        .status
        .success()
    {
        ::anyhow::bail!("Could not update packages with APT");
    }

    ::tracing::debug!("Installing base packages");
    if !::async_std::process::Command::new("apt-get")
        .args([
            "--yes",
            "--no-install-recommends",
            "install",
            "apt-utils",
            "bash-completion",
            "ca-certificates",
            "curl",
            "gawk",
            "git",
            "locales",
            "tar",
            "wget",
            "xz-utils",
        ])
        .args(ubuntu.base_packages())
        .stdout(::std::process::Stdio::null())
        .stderr(::std::process::Stdio::inherit())
        .output()
        .await?
        .status
        .success()
    {
        ::anyhow::bail!("Could not install base packages");
    }

    if gui {
        ::tracing::debug!("Installing GUI packages");
        if !::async_std::process::Command::new("apt-get")
            .args(["--yes", "install", "--no-install-recommends"])
            .args(ubuntu.gui_packages().0)
            .stdout(::std::process::Stdio::null())
            .stderr(::std::process::Stdio::inherit())
            .output()
            .await?
            .status
            .success()
        {
            ::anyhow::bail!("Could not install GUI packages");
        }

        ::tracing::debug!("Removed unwanted GUI packages");
        if !::async_std::process::Command::new("apt-get")
            .args(["--yes", "remove"])
            .args(ubuntu.gui_packages().1)
            .stdout(::std::process::Stdio::null())
            .stderr(::std::process::Stdio::inherit())
            .output()
            .await?
            .status
            .success()
        {
            ::anyhow::bail!("Could not remove unwanted GUI packages");
        }
    }

    ::tracing::debug!("Auto-removing unnecessary packages");
    if !::async_std::process::Command::new("apt-get")
        .args(["--yes", "autoremove"])
        .stdout(::std::process::Stdio::null())
        .stderr(::std::process::Stdio::inherit())
        .output()
        .await?
        .status
        .success()
    {
        ::anyhow::bail!("Could not remove orphaned packages with APT");
    }

    Ok(())
}
