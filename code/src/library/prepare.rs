//! This module contains functionality to call _hermes_
//! again in a prepared environment and with proper arguments.

use ::std::env;

use ::anyhow::Context as _;
use ::clap::ValueEnum;
use ::std::io::Write as _;

use crate::cli;

/// The path to `/etc/os-release`
const ETC_OSRELEASE_STR: &str = "/etc/os-release";

/// Checks `/etc/environment`
fn check_etc_environment() -> ::anyhow::Result<String> {
    ::log::trace!("Checkling '{}'", ETC_OSRELEASE_STR);
    let etc_osrelease = ::std::path::Path::new(ETC_OSRELEASE_STR);
    ::anyhow::ensure!(etc_osrelease.exists(), "{ETC_OSRELEASE_STR} does not exist");
    ::anyhow::ensure!(etc_osrelease.is_file(), "{ETC_OSRELEASE_STR} is not a file");

    ::log::trace!("Checkling Ubuntu version");
    let etc_osrelease_contents = ::std::fs::read_to_string(ETC_OSRELEASE_STR)
        .context(format!("Could not read contents of {ETC_OSRELEASE_STR}"))?;
    let ubuntu_version_id = if let Some(capture) = ::regex::Regex::new(r#"VERSION_ID="(.*)""#)
        .context("BUG! Ubuntu version ID regex should be constructible")?
        .captures(&etc_osrelease_contents)
    {
        if let Some(capture_result) = capture.get(1) {
            capture_result.as_str()
        } else {
            ::anyhow::bail!("Could not match on 'VERSION_ID' in file '{ETC_OSRELEASE_STR}'");
        }
    } else {
        ::anyhow::bail!("Could not acquire 'VERSION_ID' in file '{ETC_OSRELEASE_STR}'");
    };

    ::log::info!("Ubuntu version ID is '{ubuntu_version_id}'");
    Ok(format!(
        "{}",
        cli::UbuntuVersion::from_str(ubuntu_version_id, true)
            .map_err(::std::io::Error::other)
            .context(
                "'UbuntuVersion::from_str' should always yield 'Ok()' with fallback if necessary"
            )?
    ))
}

/// Takes care of finding the path with which _hermes_ has been called
fn get_path_to_self() -> ::anyhow::Result<String> {
    let Some(hermes_binrary_path) = env::args().next() else {
        anyhow::bail!("Weird! On UNIX-like operating systems, the first argument to a program is always itself - but this was just violated. I can break rules too. Goodbye!");
    };

    let hermes_binrary_path = ::std::path::Path::new(&hermes_binrary_path);

    let hermes_binary_path = if hermes_binrary_path.is_absolute() {
        hermes_binrary_path
            .canonicalize()
            .context(format!("Could not canonicalize path '{hermes_binrary_path:?}' to myself - this is weird and should not happen"))?
    } else {
        // If the path is not absolute, then we have three options:
        //
        // 1. The binary path is relative to the current working directory and valid
        // 2. The binary is in $PATH and we can find it
        // 3. The path is somehow invalid
        let hermes_relative_path = ::std::env::current_dir()
            .context("Could not determine current working directory")?
            .join(hermes_binrary_path);
        if hermes_relative_path.exists() {
            hermes_relative_path
        } else {
            ::which::which(
                hermes_binrary_path
                    .file_name()
                    .context("Weird! Could not determine file name of myself")?,
            )
            .context("Could not find myself in $PATH - this is weird and should not happen")?
        }
        .canonicalize()
        .context("Could not canonicalize path to myself - this is weird and should not happen")?
    };

    ::log::trace!("Acquired file system path {hermes_binary_path:?} to myself",);

    Ok(hermes_binary_path
        .to_str()
        .context("Could not convert path to string")?
        .to_string())
}

/// Acquires information about the user that invoked _hermes_
///
/// We do not use a crate for this but the `id` command because in corporate
/// networks, users may not be local and crates like `users` cannot deal with this.
fn get_user_information() -> ::anyhow::Result<(String, u32, String, u32, String)> {
    let output = ::std::process::Command::new("id").arg("--user").output()?;
    if !output.status.success() {
        ::anyhow::bail!("Could not determine user ID (UID)");
    }
    let uid = std::str::from_utf8(&output.stdout)?.trim().parse::<u32>()?;

    let output = ::std::process::Command::new("id")
        .args(["--user", "--name"])
        .arg(uid.to_string())
        .output()?;
    if !output.status.success() {
        ::anyhow::bail!("Could not determine user name from UID {uid}");
    }
    let user_name = std::str::from_utf8(&output.stdout)?.trim().to_string();
    ::log::info!("Current user name is '{user_name}' with UID '{uid}'");

    let output = ::std::process::Command::new("id").arg("--group").output()?;
    if !output.status.success() {
        ::anyhow::bail!("Could not determine group ID (GID)");
    }
    let gid = std::str::from_utf8(&output.stdout)?.trim().parse::<u32>()?;

    let output = ::std::process::Command::new("id")
        .args(["--group", "--name"])
        .arg(uid.to_string())
        .output()?;
    if !output.status.success() {
        ::anyhow::bail!("Could not determine group name from UID {uid} and GID {gid}");
    }
    let group_name = std::str::from_utf8(&output.stdout)?.trim().to_string();
    ::log::info!("Current user's group name is '{group_name}' with GID '{gid}'");

    let home_dir =
        ::std::env::var("HOME").context("Required environment variable 'HOME' is not set")?;
    ::log::info!("Current user's home directory is '{home_dir}'");

    Ok((user_name, uid, group_name, gid, home_dir))
}

/// Checking 'sudo' and building command to invoke itself again
fn get_invocation_command(uid: u32) -> ::anyhow::Result<::std::process::Command> {
    let command = if uid == 0 {
        log::debug!("UID is 0 - not using 'sudo'");
        let mut command = ::std::process::Command::new("env");
        command.args(["--ignore-environment", "-"]);
        command
    } else {
        log::debug!("UID is not 0 - checking for 'sudo'");
        if ::which::which("sudo").is_err() {
            anyhow::bail!(
            "Cannot run commands with 'sudo' (not installed or not in PATH?), and calling user does not have UID 0"
        );
        }

        let mut command = ::std::process::Command::new("sudo");
        command.args(["env", "--ignore-environment", "-"]);
        command
    };
    Ok(command)
}

/// Acquire HTTP proxies if they are set
fn get_http_proxies() -> (String, String, String) {
    let http_proxy = env::var("http_proxy").unwrap_or_default();
    let http_secure_proxy = env::var("https_proxy").unwrap_or_default();
    let no_proxy = env::var("no_proxy").unwrap_or_default();

    if !http_proxy.is_empty() {
        ::log::info!("Using proxy http_proxy={http_proxy}");
    }

    if !http_secure_proxy.is_empty() {
        ::log::info!("Using proxy https_proxy={http_secure_proxy}");
    }

    if !http_secure_proxy.is_empty() {
        ::log::info!("Using proxy no_proxy={no_proxy}");
    }

    (http_proxy, http_secure_proxy, no_proxy)
}

/// _hermes_ invokes itself again with correct paramters and other permissions.
/// This function does exactly that.
///
/// #### Errors
///
/// In case the correct environment could not be loaded, we use [`anyhow::bail!`]
/// to return such an error early. If performing the actual work failed, we return
/// [`Ok`] with a value of `false`. If everything worked, we return [`Ok`] with a
/// value of `true`.
pub fn call_again(arguments: &crate::cli::Arguments) -> anyhow::Result<bool> {
    ::log::info!(
        "Preparing environment and arguments to call myself again - this is expected and correct"
    );

    // ? Checking PATH
    let environment_variable_path =
        env::var("PATH").context("Required environment variable 'PATH' is not set")?;

    let ubuntu_version_id = check_etc_environment()?;
    let path_to_self = get_path_to_self()?;

    // ? Acquiring user name, group name, UID, GID, and home directory
    let (user_name, uid, user_primary_group_name, gid, user_home_dir) = get_user_information()?;

    let mut command = get_invocation_command(uid)?;

    // ? Noting GUI options
    ::log::info!(
        "GUI will {}be installed",
        if arguments.gui { "" } else { "not " }
    );

    // ? Noting APT options
    ::log::info!(
        "APT sources will {}be changed",
        if arguments.change_apt_sources {
            ""
        } else {
            "not "
        }
    );

    // ? Asking for confirmation if not supressed
    if arguments.non_interactive {
        ::log::debug!("Skipping confirmation prompts because '--non-interactive' was supplied");
    } else {
        let mut user_input = String::new();
        print!("\nProceed? [Y/n] ");
        ::std::io::stdout()
            .flush()
            .context("Weird! Could not flush stdout, which should be possible")?;
        ::std::io::stdin()
            .read_line(&mut user_input)
            .context("Weird! Could not read line, which should be possible")?;
        let user_input = user_input.trim().to_lowercase();
        println!();
        ::log::trace!("Input was: '{user_input}'");

        match user_input.as_str() {
            "" | "y" | "ye" | "yes" => (),
            _ => {
                ::log::warn!("Setup interrupted - not proceeding");
                return Ok(true);
            }
        }
    }

    let (http_proxy, http_secure_proxy, no_proxy) = get_http_proxies();
    let env_lang = env::var("LANG").unwrap_or_else(|_| String::from("C"));
    let env_lc_all = env::var("LC_ALL").unwrap_or_else(|_| env_lang.clone());

    // ? Finally, calling itself again
    ::log::debug!("Calling myself again with correct environment");
    if command
        .args([
            format!("PATH={environment_variable_path}"),
            format!("USER={user_name}"),
            format!("GROUP={user_primary_group_name}"),
            format!("HOME={user_home_dir}"),
            format!("UID={uid}"),
            format!("GID={gid}"),
            format!("LANG={env_lang}"),
            format!("LC_ALL={env_lc_all}"),
            format!("UBUNTU_VERSION_ID={ubuntu_version_id}"),
            String::from("DEBIAN_FRONTEND=noninteractive"),
            String::from("DEBCONF_NONINTERACTIVE_SEEN=true"),
            format!("http_proxy={http_proxy}"),
            format!("https_proxy={http_secure_proxy}"),
            format!("no_proxy={no_proxy}"),
            path_to_self,
            String::from("--assume-correct-invocation"),
        ])
        .args(env::args().skip(1))
        .stdout(::std::process::Stdio::inherit())
        .stderr(::std::process::Stdio::inherit())
        .stdin(::std::process::Stdio::inherit())
        .output()
        .context("Calling myself again did not yield proper output")?
        .status
        .success()
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Contains functions that work with the environment that _hermes_ is called
/// in again by itself.
pub mod environment {
    /// Get the user name of the user who invoked _hermes_
    ///
    /// #### Panics
    ///
    /// This function assumes _hermes_ has been called again already
    /// and hence that the environment variable `USER` is set correctly.
    #[must_use]
    pub fn user() -> String {
        ::std::env::var("USER").expect("Could not determine home directory")
    }

    /// Get the group name of the user who invoked _hermes_
    ///
    /// #### Panics
    ///
    /// This function assumes _hermes_ has been called again already
    /// and hence that the environment variable `GROUP` is set correctly.
    #[must_use]
    pub fn group() -> String {
        ::std::env::var("GROUP").expect("Could not determine home directory")
    }

    /// Get the home directory of the user who invoked _hermes_
    ///
    /// #### Panics
    ///
    /// This function assumes _hermes_ has been called again already
    /// and hence that the environment variable `HOME` is set correctly.
    #[must_use]
    pub fn home() -> ::std::path::PathBuf {
        ::std::path::PathBuf::from(
            ::std::env::var("HOME").expect("Could not determine home directory"),
        )
    }

    /// Get the home directory name of the user who invoked _hermes_
    /// as a [`String`]
    ///
    /// #### Panics
    ///
    /// Panics when [`home`] panics.
    #[must_use]
    pub fn home_str() -> String {
        home().to_string_lossy().to_string()
    }

    /// Adds arbitrary directories to the directory obtained by
    /// calling [`home`]
    ///
    /// #### Panics
    ///
    /// Panics when [`home`] panics.
    #[must_use]
    pub fn home_and(join: impl AsRef<str>) -> String {
        format!("{}", home().join(join.as_ref()).to_string_lossy())
    }

    /// Returns the directory `${HOME}/.local/bin`
    ///
    /// #### Panics
    ///
    /// Panics when [`home`] panics.
    #[must_use]
    pub fn home_local_bin() -> String {
        home_and(".local/bin")
    }

    /// Get the user ID of the user who invoked _hermes_
    ///
    /// #### Panics
    ///
    /// This function assumes _hermes_ has been called again already
    /// and hence that the environment variable `UID` is set correctly.
    #[must_use]
    pub fn uid() -> u32 {
        ::std::env::var("UID")
            .expect("Could not determine UID")
            .parse()
            .expect("Could not parse UID")
    }

    /// Get the group ID of the user who invoked _hermes_
    ///
    /// #### Panics
    ///
    /// This function assumes _hermes_ has been called again already
    /// and hence that the environment variable `GID` is set correctly.
    #[must_use]
    pub fn gid() -> u32 {
        ::std::env::var("GID")
            .expect("Could not determine GID")
            .parse()
            .expect("Could not parse GID")
    }

    /// Get the Ubuntu version ID [`String`] of the currently
    /// installed version of Ubuntu
    ///
    /// #### Panics
    ///
    /// This function assumes _hermes_ has been called again already
    /// and hence that the environment variable `UBUNTU_VERSION_ID`
    /// is set correctly.
    #[must_use]
    pub fn ubuntu_version_id() -> super::super::cli::UbuntuVersion {
        use clap::ValueEnum as _;
        super::super::cli::UbuntuVersion::from_str(
            &::std::env::var("UBUNTU_VERSION_ID").expect("Could not determine Ubuntu version ID"),
            false,
        )
        .expect("Could not parse Ubuntu version ID")
    }
}
