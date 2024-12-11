//! This module handling installing additonal programs fro GitHub.

use super::super::{fs, prepare::environment};
use ::std::collections;

use ::anyhow::Context as _;
use ::async_std::stream::StreamExt as _;

/// The architecture string for the amd64 (`x86_64`) architecture
#[cfg(target_arch = "x86_64")]
const ARCHITECTURE: &str = "x86_64";
/// The library that is linked against by programs. Not all programs
/// support `musl`, especially on `aarch64`.
#[cfg(target_arch = "x86_64")]
const LINK_LIBRARY: &str = "musl";

/// The architecture string for the arm64 (`aarch64`) architecture
#[cfg(target_arch = "aarch64")]
const ARCHITECTURE: &str = "aarch64";
/// The library that is linked against by programs. Not all programs
/// support `musl`, especially on `aarch64`.
#[cfg(target_arch = "aarch64")]
const LINK_LIBRARY: &str = "gnu";

// use async_std::prelude::*;

/// Download custom programs (so that we can unpack them later if required)
///
/// This function is mainly an optimization. [`super::apt::configure_system_with_apt`]
/// runs much longer than the other functions that perform work. Hence, we can use our
/// time more efficiently if we already start the download of custom programs.
pub(super) async fn download_custom_programs() -> ::anyhow::Result<()> {
    let mut join_set = ::tokio::task::JoinSet::new();
    let mut errors = vec![];

    join_set.spawn(atuin());
    join_set.spawn(bat());
    join_set.spawn(blesh());
    join_set.spawn(eza());
    join_set.spawn(fd());
    join_set.spawn(fzf());
    join_set.spawn(gitui());
    join_set.spawn(ripgrep());
    join_set.spawn(starship());
    join_set.spawn(zoxide());

    while let Some(handler) = join_set.join_next().await {
        match handler {
            Ok(actual_result) => match actual_result {
                Ok(()) => (),
                Err(error) => errors.push(error),
            },
            Err(error) => errors.push(::anyhow::anyhow!(error)),
        }
    }

    super::super::evaluate_errors_vector!(
        errors,
        "Could not acquire all additonal programs from GitHub successfully"
    )
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
    let file = format!("bat-v{BAT_VERSION}-{ARCHITECTURE}-unknown-linux-{LINK_LIBRARY}");
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
    let file = format!("ble-{BLESH_VERSION}");
    let uri = format!(
        "https://github.com/akinomyoga/ble.sh/releases/download/v{BLESH_VERSION}/{file}-2.tar.xz"
    );

    let target_dir = format!("{}/.local/share", environment::home_str());
    ::async_std::fs::create_dir_all(&target_dir)
        .await
        .context(format!(
            "Could not create ble.sh target directory '{target_dir}'"
        ))?;
    let _ = ::async_std::fs::remove_dir_all(format!("{target_dir}/{file}")).await;
    let _ = ::async_std::fs::remove_dir_all(format!("{target_dir}/blesh")).await;

    // We download and unpacl the archive to `${HOME}/.local/share`
    let response = super::download::download_file(uri).await?;
    let xz_decoder = ::async_compression::tokio::bufread::XzDecoder::new(&response[..]);
    let mut archive = ::tokio_tar::Archive::new(xz_decoder);

    archive
        .unpack(format!("{target_dir}"))
        .await
        .context("Could not unpack ble.sh archive")?;

    ::async_std::fs::rename(format!("/tmp/{file}"), format!("{target_dir}/blesh"))
        .await
        .context("Could not move unpackaed ble.sh archive to final location")?;

    Ok(())
}

/// Install `eza` (<https://github.com/eza-community/eza>)
async fn eza() -> ::anyhow::Result<()> {
    /// The version `eza` to install
    const EZA_VERSION: &str = "0.20.11";
    let file = format!("eza_{ARCHITECTURE}-unknown-linux-{LINK_LIBRARY}");
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

    download_and_extract(uri, entries).await?;

    for (uri, additional_file) in [
        (
            "https://raw.githubusercontent.com/junegunn/fzf/refs/heads/master/shell",
            "completion.bash",
        ),
        (
            "https://raw.githubusercontent.com/junegunn/fzf/refs/heads/master/shell",
            "key-bindings.bash",
        ),
    ] {
        let local_path = format!(
            "{}/.config/bash/fzf/{additional_file}",
            environment::home_str()
        );
        super::download::download_and_place_configuration_file(
            format!("{uri}/{additional_file}"),
            ::std::path::PathBuf::from(local_path.clone()),
        )
        .await?;
    }

    Ok(())
}

/// Install `gitui` (<https://github.com/extrawurst/gitui>)
async fn gitui() -> ::anyhow::Result<()> {
    /// Version of `gitui` to install
    const GITUI_VERSION: &str = "0.26.3";
    let file = format!("gitui-linux-{ARCHITECTURE}.tar.gz");
    let uri =
        format!("https://github.com/extrawurst/gitui/releases/download/v{GITUI_VERSION}/{file}");

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("./gitui"),
        format!("{}/gitui", environment::home_local_bin()),
    );

    download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `ripgrep` (<https://github.com/BurntSushi/ripgrep>)
async fn ripgrep() -> ::anyhow::Result<()> {
    /// Version of `ripgrep` to install
    const RIPGREP_VERSION: &str = "14.1.1";
    let file = format!("ripgrep-{RIPGREP_VERSION}-{ARCHITECTURE}-unknown-linux-{LINK_LIBRARY}");
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
