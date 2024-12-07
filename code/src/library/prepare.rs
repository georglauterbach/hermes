//! This module contains functionality to call _hermes_
//! again in a prepared environment and with proper arguments.

use ::std::env;

use ::anyhow::Context as _;
use ::clap::ValueEnum;
use ::std::io::Write as _;
use ::users::os::unix::UserExt as _;

use crate::cli;

/// The path to `/etc/os-release`
const ETC_OSRELEASE_STR: &str = "/etc/os-release";

/// TODO
fn check_etc_environment() -> ::anyhow::Result<String> {
    // ? Checking '/etc/os-release'
    ::log::trace!("Checkling '{}'", ETC_OSRELEASE_STR);
    let etc_osrelease = ::std::path::Path::new(ETC_OSRELEASE_STR);
    ::anyhow::ensure!(
        etc_osrelease.exists(),
        "{} does not exist",
        ETC_OSRELEASE_STR
    );
    ::anyhow::ensure!(
        etc_osrelease.is_file(),
        "{} is not a file",
        ETC_OSRELEASE_STR
    );

    // ? Checking Ubuntu version in '/etc/os-release'
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
        cli::UbuntuVersion::from_str(ubuntu_version_id, true).expect(
            "'UbuntuVersion::from_str' should always yield 'Ok()' with fallback if necessary"
        )
    ))
}

/// TODO this does not yet work when the binary is in $PATH (not the whole file path is provided)
fn get_path_to_self() -> ::anyhow::Result<String> {
    // ? Acquire path to itself
    let Some(hermes_binrary_path) = env::args().next() else {
        anyhow::bail!("Weird! On UNIX-like operating systems, the first argument to a program is always itself - but this was just violated. I can break rules too. Goodbye!");
    };
    let hermes_binrary_path = ::std::path::Path::new(&hermes_binrary_path)
        .canonicalize()
        .context(format!("Could not canonicalize path '{hermes_binrary_path}' to myself - this is weird and should not happen"))?;
    ::log::trace!(
        "Acquired file system path '{}' to myself",
        hermes_binrary_path.to_string_lossy()
    );

    Ok(hermes_binrary_path
        .to_str()
        .context("Could not convert path to string")?
        .to_string())
}

/// TODO
fn get_user_information() -> ::anyhow::Result<(String, u32, String, u32, String)> {
    let uid = users::get_current_uid();
    let user = users::get_user_by_uid(uid)
        .context("ould not determine user name for current UID '{user_uid}'")?;
    let user_name = user.name().to_str().context("Weird! Could not convert user name to UTF-8 string - hermes only works with UTF-8 strings")?.to_string();
    ::log::info!("Current user name is '{user_name}' with UID '{uid}'");

    let gid = user.primary_group_id();
    let user_primary_group_name = users::get_group_by_gid(gid).context(
        "Could not determine group name for current user '{user_name}' with GID '{user_gid}'",
    )?;
    let user_primary_group_name = user_primary_group_name.name().to_str().context("Weird! Could not convert group name to UTF-8 string - hermes only works with UTF-8 strings")?.to_string();
    ::log::info!("Current user's group name is '{user_primary_group_name}' with UID '{gid}'");

    let user_home_dir = user.home_dir().to_str().context("Weird! Could not convert home directory path to UTF-8 string - hermes only works with UTF-8 strings")?.to_string();
    ::log::info!("Current user's home directory is '{user_home_dir}'");

    Ok((user_name, uid, user_primary_group_name, gid, user_home_dir))
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
        if ::std::process::Command::new("sudo")
            .arg("--version")
            .stdout(::std::process::Stdio::null())
            .output()
            .is_err()
        {
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
            .context("Weird! Could not flush ::stdout, which should be possible")?;
        ::std::io::stdin()
            .read_line(&mut user_input)
            .context("Weird! Could not read line, which should be possible")?;
        user_input = user_input.replace('\n', "");
        user_input = user_input.replace('\r', "");
        println!();
        ::log::trace!("Input was: '{user_input}'");

        match user_input.to_lowercase().as_str() {
            "" | "y" | "ye" | "yes" => (),
            _ => {
                ::log::warn!("Setup interrupted - not proceeding");
                return Ok(true);
            }
        }
    }

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
            format!("UBUNTU_VERSION_ID={ubuntu_version_id}"),
            String::from("DEBIAN_FRONTEND=noninteractive"),
            String::from("DEBCONF_NONINTERACTIVE_SEEN=true"),
            format!("http_proxy={}", env::var("http_proxy").unwrap_or_default()),
            format!(
                "https_proxy={}",
                env::var("https_proxy").unwrap_or_default()
            ),
            format!("no_proxy={}", env::var("no_proxy").unwrap_or_default()),
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

/// TODO
pub mod environment {
    /// TODO
    ///
    /// #### Panics
    #[must_use]
    pub fn user() -> String {
        ::std::env::var("USER").expect("Could not determine home directory")
    }

    /// TODO
    ///
    /// #### Panics
    #[must_use]
    pub fn group() -> String {
        ::std::env::var("GROUP").expect("Could not determine home directory")
    }

    /// TODO
    ///
    /// #### Panics
    #[must_use]
    pub fn home() -> ::std::path::PathBuf {
        ::std::path::PathBuf::from(
            ::std::env::var("HOME").expect("Could not determine home directory"),
        )
    }

    /// TODO
    ///
    /// #### Panics
    #[must_use]
    pub fn home_str() -> String {
        home().to_string_lossy().to_string()
    }

    /// TODO
    ///
    /// #### Panics
    #[must_use]
    pub fn home_and(join: impl AsRef<str>) -> String {
        format!("{}", home().join(join.as_ref()).to_string_lossy())
    }

    /// TODO
    ///
    /// #### Panics
    #[must_use]
    pub fn home_local_bin() -> String {
        home_and(".local/bin")
    }

    /// TODO
    ///
    /// #### Panics
    #[must_use]
    pub fn uid() -> u32 {
        ::std::env::var("UID")
            .expect("Could not determine UID")
            .parse()
            .expect("Could not parse UID")
    }

    /// TODO
    ///
    /// #### Panics
    #[must_use]
    pub fn gid() -> u32 {
        ::std::env::var("GID")
            .expect("Could not determine GID")
            .parse()
            .expect("Could not parse GID")
    }

    /// TODO
    ///
    /// #### Panics
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
