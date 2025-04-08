//! TODO

/// TODO
#[::tracing::instrument(skip_all, name = "ip")]
pub(super) async fn install(install: bool) -> ::anyhow::Result<()> {
    if !install {
        ::tracing::info!("Skipping package installation");
        return Ok(());
    }

    match std::env::var("DISTRIBUTION_ID")
        .unwrap_or_default()
        .as_str()
    {
        "ubuntu" | "debian" => ubuntu_debian::install().await,
        _ => {
            tracing::info!("Distribution unknown - skipping package installation");
            Ok(())
        }
    }
}

mod ubuntu_debian {
    /// Configures the system with APT, which boils down to
    ///
    /// 1. updating package signatures (version-specific)
    /// 2. installing packages
    pub(super) async fn install() -> ::anyhow::Result<()> {
        ::tracing::info!(target: "work", "Installing packages (IP)");

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
            ::anyhow::bail!("Could not update package signatures");
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
            .stdout(::std::process::Stdio::null())
            .stderr(::std::process::Stdio::inherit())
            .output()
            .await?
            .status
            .success()
        {
            ::anyhow::bail!("Could not install base packages");
        }

        Ok(())
    }
}
