//! TODO

use super::super::{fs, prepare::environment};
use ::std::collections;

use ::anyhow::Context as _;
use ::async_std::stream::StreamExt as _;

/// TODO
#[cfg(target_arch = "x86_64")]
const ARCHITECTURE: &str = "x86_64";

/// TODO
#[cfg(target_arch = "aarch64")]
const ARCHITECTURE: &str = "aarch64";

// use async_std::prelude::*;

/// Download custom programs (so that we can unpack them later if required)
///
/// This function is mainly an optimization. [`super::apt::configure_system_with_apt`]
/// runs much longer than the other functions that perform work. Hence, we can use our
/// time more efficiently if we already start the download of custom programs.
pub(super) async fn download_custom_programs() -> ::anyhow::Result<()> {
    ::async_std::fs::create_dir_all(environment::home_and(".local/bin")).await?;
    fs::adjust_permissions(
        &environment::home_and(".local"),
        environment::uid(),
        environment::gid(),
        0o644,
    )?;
    fs::adjust_permissions(
        &environment::home_and(".local/bin"),
        environment::uid(),
        environment::gid(),
        0o644,
    )?;

    let mut error_occured = false;
    let mut final_err = ::anyhow::anyhow!("Downloading custom programs finished with errors");
    let mut join_set = ::tokio::task::JoinSet::new();

    join_set.spawn(atuin());
    join_set.spawn(bat());
    join_set.spawn(blesh());
    join_set.spawn(eza());
    join_set.spawn(fd());
    join_set.spawn(fzf());
    join_set.spawn(ripgrep());
    join_set.spawn(starship());
    join_set.spawn(zoxide());

    while let Some(handler) = join_set.join_next().await {
        match handler {
            Ok(actual_result) => match actual_result {
                Ok(()) => (),
                Err(error) => {
                    final_err = final_err.context(format!("{error}"));
                    error_occured = true;
                }
            },
            Err(error) => {
                final_err = final_err.context(format!(
                    "Could not join an async handle (this should not have happened): {error}"
                ));
                error_occured = true;
            }
        }
    }

    if error_occured {
        Err(final_err)
    } else {
        Ok(())
    }
}

/// Recursively extracts files from an archive and places them in the local
/// file system. Paths to extract are given in `entry_path_mappings`' key set,
/// and their corresponding local paths are in the value of the map.
async fn extract_from_archive<R>(
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
                ::log::warn!("Could not get entry from archive: {error}");
                continue;
            }
        };

        let entry_path_str = match entry.path() {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(error) => {
                ::log::warn!("Could get acquire path of entry '{error}'");
                continue;
            }
        };

        if let Some(local_path) = entry_path_mappings.remove(&entry_path_str) {
            fs::create_parent_dir(&local_path).await?;
            ::log::trace!("Unpacking {entry_path_str} from archive to {local_path}");
            if let Err(error) = entry.unpack(&local_path).await {
                ::log::warn!(
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
        ::log::warn!(
            "Not all desired entries from the archive were unpacked: {:?}",
            entry_path_mappings.keys()
        );
    }

    Ok(())
}

/// Download an archive and extract it with [`extract_from_archive`].
async fn download_and_extract(
    uri: String,
    entry_path_mappings: collections::HashMap<String, String>,
) -> ::anyhow::Result<()> {
    let response = super::download::download_file(&uri).await?;

    if uri.ends_with(".tar.gz") {
        let gz_decoder = ::async_compression::tokio::bufread::GzipDecoder::new(&response[..]);
        extract_from_archive(::tokio_tar::Archive::new(gz_decoder), entry_path_mappings).await
    } else if uri.ends_with("tar.xz") {
        let xz_decoder = ::async_compression::tokio::bufread::XzDecoder::new(&response[..]);
        extract_from_archive(::tokio_tar::Archive::new(xz_decoder), entry_path_mappings).await
    } else {
        anyhow::bail!("Unknown archive format for URI '{uri}'");
    }
}

/// Install `atuin` (<https://github.com/atuinsh/atuin>)
async fn atuin() -> ::anyhow::Result<()> {
    /// Version of `atuin` to install
    const ATUIN_VERSION: &str = "18.3.0";
    let file = format!("atuin-{ARCHITECTURE}-unknown-linux-musl");
    let uri = format!(
        "https://github.com/atuinsh/atuin/releases/download/v{ATUIN_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        format!("{file}/atuin"),
        format!("{}/atuin", environment::home_local_bin()),
    );

    download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `bat` (<https://github.com/sharkdp/bat>)
async fn bat() -> ::anyhow::Result<()> {
    /// Version of `bat` to install
    const BAT_VERSION: &str = "0.24.0";
    let file = format!("bat-v{BAT_VERSION}-{ARCHITECTURE}-unknown-linux-musl");
    let uri =
        format!("https://github.com/sharkdp/bat/releases/download/v{BAT_VERSION}/{file}.tar.gz");

    let mut entries = collections::HashMap::new();
    entries.insert(
        format!("{file}/bat"),
        format!("{}/bat", environment::home_local_bin()),
    );
    entries.insert(
        format!("{file}/autocomplete/bat.bash"),
        String::from("/etc/bash_completion.d/bat.bash"),
    );

    download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `ble.sh` (<https://github.com/akinomyoga/ble.sh>)
async fn blesh() -> ::anyhow::Result<()> {
    /// Version of `blesh` to install
    const BLESH_VERSION: &str = "0.4.0-devel3";
    let file = format!("ble-{BLESH_VERSION}-2");
    let uri = format!(
        "https://github.com/akinomyoga/ble.sh/releases/download/v{BLESH_VERSION}/{file}.tar.xz"
    );

    let local_archive_path = format!("{}/.cache/{file}.tar.xz", environment::home_str());
    super::download::download_and_place(uri, local_archive_path.clone()).await?;
    ::log::debug!(
        "To install ble.sh, unpack the archive with 'tar xf {local_archive_path}' and install it by running 'make -C <UNPACKED DIR> install PREFIX=~/.local'"
    );
    Ok(())
}

/// Install `eza` (<https://github.com/eza-community/eza>)
async fn eza() -> ::anyhow::Result<()> {
    /// The version `eza` to install
    const EZA_VERSION: &str = "0.20.11";
    let file = format!("eza_{ARCHITECTURE}-unknown-linux-musl");
    let uri = format!(
        "https://github.com/eza-community/eza/releases/download/v{EZA_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("./eza"),
        format!("{}/eza", environment::home_local_bin()),
    );

    download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `fd` (<https://github.com/sharkdp/fd>)
async fn fd() -> ::anyhow::Result<()> {
    /// The version `fd` to install
    const FD_VERSION: &str = "10.2.0";
    let file = format!("fd-v{FD_VERSION}-{ARCHITECTURE}-unknown-linux-musl");
    let uri =
        format!("https://github.com/sharkdp/fd/releases/download/v{FD_VERSION}/{file}.tar.gz");

    let mut entries = collections::HashMap::new();
    entries.insert(
        format!("{file}/fd"),
        format!("{}/fd", environment::home_local_bin()),
    );
    entries.insert(
        format!("{file}/autocomplete/fd.bash"),
        String::from("/etc/bash_completion.d/fd.bash"),
    );

    download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `fzf` (<https://github.com/junegunn/fzf>)
async fn fzf() -> ::anyhow::Result<()> {
    /// Version of `fzf` to install
    const FZF_VERSION: &str = "0.56.3";
    #[cfg(target_arch = "x86_64")]
    let file = format!("fzf-{FZF_VERSION}-linux_amd64");
    #[cfg(target_arch = "aarch64")]
    let file = format!("fzf-{FZF_VERSION}-linux_arm64");
    let uri =
        format!("https://github.com/junegunn/fzf/releases/download/v{FZF_VERSION}/{file}.tar.gz");

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("fzf"),
        format!("{}/fzf", environment::home_local_bin()),
    );

    // TODO download additional config files
    download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `ripgrep` (<https://github.com/BurntSushi/ripgrep>)
async fn ripgrep() -> ::anyhow::Result<()> {
    /// Version of `ripgrep` to install
    const RIPGREP_VERSION: &str = "14.1.1";
    let file = format!("ripgrep-{RIPGREP_VERSION}-{ARCHITECTURE}-unknown-linux-musl");
    let uri = format!(
        "https://github.com/BurntSushi/ripgrep/releases/download/{RIPGREP_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        format!("{file}/rg"),
        format!("{}/rg", environment::home_local_bin()),
    );
    entries.insert(
        format!("{file}/complete/rg.bash"),
        String::from("/etc/bash_completion.d/rg.bash"),
    );

    download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `starship` (<https://github.com/starship/starship>)
async fn starship() -> ::anyhow::Result<()> {
    /// Version of `starship` to install
    const STARSHIP_VERSION: &str = "1.21.1";
    let file = format!("starship-{ARCHITECTURE}-unknown-linux-musl");
    let uri = format!(
        "https://github.com/starship/starship/releases/download/v{STARSHIP_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("starship"),
        format!("{}/starship", environment::home_local_bin()),
    );

    download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `zoxide` (<https://github.com/ajeetdsouza/zoxide>)
async fn zoxide() -> ::anyhow::Result<()> {
    /// Version of `zoxide` to install
    const ZOXIDE_VERSION: &str = "0.9.6";
    let file = format!("zoxide-{ZOXIDE_VERSION}-{ARCHITECTURE}-unknown-linux-musl");
    let uri = format!(
        "https://github.com/ajeetdsouza/zoxide/releases/download/v{ZOXIDE_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("zoxide"),
        format!("{}/zoxide", environment::home_local_bin()),
    );

    download_and_extract(uri, entries).await?;
    Ok(())
}
