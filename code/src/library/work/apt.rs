//! This module contains functions that perform various
//! operations with APT, such as updating package signatures
//! or installing packages.

use super::{
    super::{data, prepare::environment},
    configuration_files, GITHUB_RAW_URI,
};

/// Change APT sources in `/etc/apt/sources.list.d/` if requested
/// by the user
async fn set_up_new_apt_sources(
    ubuntu: &dyn data::versioned::UbuntuVersion,
    change_apt_sources: bool,
    gui: bool,
) -> ::anyhow::Result<()> {
    let mut errors = vec![];

    if change_apt_sources {
        ::log::debug!("Changing APT sources");
        if let Err(error) = configuration_files::download_and_place_configuration_files(
            ubuntu.apt_index(),
            true,
            format!(
                "{GITHUB_RAW_URI}/data/versioned/{}",
                environment::ubuntu_version_id()
            ),
            "changing APT sources",
        )
        .await
        {
            ::log::warn!("Changing APT sources failed");
            errors.push(error);
        };
    }

    if gui {
        ::log::debug!("Changing GUI APT sources");
        if let Err(error) = configuration_files::download_and_place_configuration_files(
            ubuntu.gui_apt_index(),
            true,
            format!(
                "{GITHUB_RAW_URI}/data/versioned/{}",
                environment::ubuntu_version_id()
            ),
            "updating GUI APT sources",
        )
        .await
        {
            ::log::warn!("Updating GUI APT sources failed");
            errors.push(error);
        };
    }

    super::super::evaluate_errors_vector!(errors, "Changing APT sources failed")
}

/// Prepare APT so that it is in a usable state
async fn prepare_apt() -> ::anyhow::Result<()> {
    ::log::debug!("Updating APT package signatures");
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

    //::log::debug!("Upgrading APT packages");
    //if !::async_std::process::Command::new("apt-get")
    //    .args(["--yes", "upgrade"])
    //    .stdout(::std::process::Stdio::null())
    //    .stderr(::std::process::Stdio::inherit())
    //    .output()
    //    .await?
    //    .status
    //    .success()
    //{
    //    ::anyhow::bail!("Could not upgrade packages with APT");
    //}

    //::log::debug!("Auto-removing unnecessary packages");
    //if !::async_std::process::Command::new("apt-get")
    //    .args(["--yes", "autoremove"])
    //    .stdout(::std::process::Stdio::null())
    //    .stderr(::std::process::Stdio::inherit())
    //    .output()
    //    .await?
    //    .status
    //    .success()
    //{
    //    ::anyhow::bail!("Could not update packages with APT");
    //}

    Ok(())
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
    ::log::info!("Configuring system with APT (CSWA)");
    let ubuntu = data::versioned::get_version_information();

    set_up_new_apt_sources(ubuntu, change_apt_sources, gui).await?;
    prepare_apt().await?;

    ::log::debug!("Installing base packages");
    if !::async_std::process::Command::new("apt-get")
        .args([
            "--yes",
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
        ::log::debug!("Installing GUI packages");
        if !::async_std::process::Command::new("apt-get")
            .args(["--yes", "install"])
            .args(ubuntu.gui_packages())
            .stdout(::std::process::Stdio::null())
            .stderr(::std::process::Stdio::inherit())
            .output()
            .await?
            .status
            .success()
        {
            ::anyhow::bail!("Could not install GUI packages");
        }
    }

    Ok(())
}
