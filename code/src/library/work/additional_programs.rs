//! This module handles installing additional programs from GitHub.

use super::super::{
    fs::{download, extract},
    prepare::environment,
};
use ::std::collections;

use ::anyhow::Context as _;

/// The architecture string for the amd64 (`x86_64`) architecture
#[cfg(target_arch = "x86_64")]
pub(super) const ARCHITECTURE: &str = "x86_64";
/// The library that is linked against by programs. Not all programs
/// support `musl`, especially on `aarch64`.
#[cfg(target_arch = "x86_64")]
const LINK_LIBRARY: &str = "musl";

/// The architecture string for the arm64 (`aarch64`) architecture
#[cfg(target_arch = "aarch64")]
pub(super) const ARCHITECTURE: &str = "aarch64";
/// The library that is linked against by programs. Not all programs
/// support `musl`, especially on `aarch64`.
#[cfg(target_arch = "aarch64")]
const LINK_LIBRARY: &str = "gnu";

/// Download custom programs (so that we can unpack them later if required)
///
/// This function is mainly an optimization. [`super::packages::install`]
/// runs much longer than the other functions that perform work. Hence, we can use our
/// time more efficiently if we already start the download of custom programs.
#[::tracing::instrument(name = "additional programs", skip_all)]
pub(super) async fn install() -> ::anyhow::Result<()> {
    ::tracing::info!("Installing");

    let results = ::tokio::join!(
        ::tokio::spawn(atuin()),
        ::tokio::spawn(bat()),
        ::tokio::spawn(bottom()),
        ::tokio::spawn(blesh()),
        ::tokio::spawn(delta()),
        ::tokio::spawn(dust()),
        ::tokio::spawn(dysk()),
        ::tokio::spawn(eza()),
        ::tokio::spawn(fd()),
        ::tokio::spawn(fzf()),
        ::tokio::spawn(gitui()),
        ::tokio::spawn(just()),
        ::tokio::spawn(ripgrep()),
        ::tokio::spawn(starship()),
        ::tokio::spawn(yazi()),
        ::tokio::spawn(zellij()),
        ::tokio::spawn(zoxide()),
    );

    let results = [
        results.0, results.1, results.2, results.3, results.4, results.5, results.6, results.7,
        results.8, results.9, results.10, results.11, results.12, results.13, results.14,
        results.15, results.16,
    ]
    .into_iter()
    .flat_map(|result| result.map_err(::anyhow::Error::from))
    .collect::<Vec<Result<(), ::anyhow::Error>>>();

    super::super::evaluate_results(results)
}

/// Returns the string `~/.local/share/bash-completion/completions/<completion_file_name>`
fn user_completions_dir(completion_file_name: impl AsRef<str>) -> String {
    format!(
        "{}/.local/share/bash-completion/completions/{}",
        environment::home_str(),
        completion_file_name.as_ref()
    )
}

/// Install `atuin` (<https://github.com/atuinsh/atuin>)
async fn atuin() -> ::anyhow::Result<()> {
    /// Version of `atuin` to install
    const ATUIN_VERSION: &str = "18.6.1";

    let file = format!("atuin-{ARCHITECTURE}-unknown-linux-musl");
    let uri = format!(
        "https://github.com/atuinsh/atuin/releases/download/v{ATUIN_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        format!("{file}/atuin"),
        format!("{}/atuin", environment::home_local_bin()),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `bat` (<https://github.com/sharkdp/bat>)
async fn bat() -> ::anyhow::Result<()> {
    /// Version of `bat` to install
    const BAT_VERSION: &str = "0.25.0";

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
        user_completions_dir("bat.bash"),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `bottom` (<https://github.com/ClementTsang/bottom>)
async fn bottom() -> ::anyhow::Result<()> {
    /// Version of `bat` to install
    const BOTTOM_VERSION: &str = "nightly";

    let file = format!("bottom_{ARCHITECTURE}-unknown-linux-musl");
    let uri = format!(
        "https://github.com/ClementTsang/bottom/releases/download/{BOTTOM_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("btm"),
        format!("{}/btm", environment::home_local_bin()),
    );
    entries.insert(
        String::from("completion/btm.bash"),
        user_completions_dir("btm.bash"),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `ble.sh` (<https://github.com/akinomyoga/ble.sh>)
async fn blesh() -> ::anyhow::Result<()> {
    let file = "ble-nightly";
    let uri =
        format!("https://github.com/akinomyoga/ble.sh/releases/download/nightly/{file}.tar.xz");

    let target_dir = format!("{}/.local/share", environment::home_str());
    let _ = ::async_std::fs::create_dir_all(&target_dir).await;
    let _ = ::async_std::fs::remove_dir_all(format!("/tmp/{file}")).await;
    let _ = ::async_std::fs::remove_dir_all(format!("{target_dir}/blesh")).await;

    // We download and unpack the archive to `${HOME}/.local/share`
    let response = download::download(uri).await?;
    let xz_decoder = ::async_compression::tokio::bufread::XzDecoder::new(&response[..]);
    let mut archive = ::tokio_tar::Archive::new(xz_decoder);

    archive
        .unpack(&target_dir)
        .await
        .context("Could not unpack ble.sh archive")?;

    ::async_std::fs::rename(
        format!("{target_dir}/{file}"),
        format!("{target_dir}/blesh"),
    )
    .await
    .context("Could not move unpacked ble.sh archive to final location")?;

    let _ =
        ::async_std::fs::remove_dir_all(format!("{}/.cache/blesh", environment::home_str())).await;
    Ok(())
}

/// Install `delta` (<https://github.com/dandavison/delta>)
async fn delta() -> ::anyhow::Result<()> {
    /// The version of `delta` to install
    const DELTA_VERSION: &str = "0.18.2";

    let file = format!("delta-{DELTA_VERSION}-{ARCHITECTURE}-unknown-linux-{LINK_LIBRARY}");
    let uri = format!(
        "https://github.com/dandavison/delta/releases/download/{DELTA_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        format!("{file}/delta"),
        format!("{}/delta", environment::home_local_bin()),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `dust` (<https://github.com/bootandy/dust>)
async fn dust() -> ::anyhow::Result<()> {
    /// The version of `dust` to install
    const DUST_VERSION: &str = "1.2.2";

    let file = format!("dust-v{DUST_VERSION}-{ARCHITECTURE}-unknown-linux-musl");
    let uri =
        format!("https://github.com/bootandy/dust/releases/download/v{DUST_VERSION}/{file}.tar.gz");

    let mut entries = collections::HashMap::new();
    entries.insert(
        format!("{file}/dust"),
        format!("{}/dust", environment::home_local_bin()),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `dysk` (<https://github.com/Canop/dysk>)
async fn dysk() -> ::anyhow::Result<()> {
    /// The version of `dysk` to install
    const DYSK_VERSION: &str = "2.10.1";

    let uri = format!(
        "https://github.com/Canop/dysk/releases/download/v{DYSK_VERSION}/dysk_{DYSK_VERSION}.zip"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        #[cfg(target_arch = "x86_64")]
        String::from("build/x86_64-unknown-linux-musl/dysk"),
        #[cfg(target_arch = "aarch64")]
        String::from("build/aarch64-unknown-linux-musl/dysk"),
        format!("{}/dysk", environment::home_local_bin()),
    );
    entries.insert(
        String::from("build/completion/dysk.bash"),
        user_completions_dir("dysk.bash"),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `eza` (<https://github.com/eza-community/eza>)
async fn eza() -> ::anyhow::Result<()> {
    /// The version of `eza` to install
    const EZA_VERSION: &str = "0.22.0";

    let file = format!("eza_{ARCHITECTURE}-unknown-linux-{LINK_LIBRARY}");
    let uri = format!(
        "https://github.com/eza-community/eza/releases/download/v{EZA_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("./eza"),
        format!("{}/eza", environment::home_local_bin()),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `fd` (<https://github.com/sharkdp/fd>)
async fn fd() -> ::anyhow::Result<()> {
    /// The version of `fd` to install
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
        user_completions_dir("fd.bash"),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `fzf` (<https://github.com/junegunn/fzf>)
async fn fzf() -> ::anyhow::Result<()> {
    /// Version of `fzf` to install
    const FZF_VERSION: &str = "0.63.0";

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

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `gitui` (<https://github.com/extrawurst/gitui>)
async fn gitui() -> ::anyhow::Result<()> {
    /// Version of `gitui` to install
    const GITUI_VERSION: &str = "0.27.0";

    let file = format!("gitui-linux-{ARCHITECTURE}.tar.gz");
    let uri =
        format!("https://github.com/extrawurst/gitui/releases/download/v{GITUI_VERSION}/{file}");

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("./gitui"),
        format!("{}/gitui", environment::home_local_bin()),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `just` (<https://github.com/casey/just>)
async fn just() -> ::anyhow::Result<()> {
    /// The version of `just` to install
    const JUST_VERSION: &str = "1.41.0";

    let file = format!("just-{JUST_VERSION}-{ARCHITECTURE}-unknown-linux-musl");
    let uri =
        format!("https://github.com/casey/just/releases/download/{JUST_VERSION}/{file}.tar.gz");

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("just"),
        format!("{}/just", environment::home_local_bin()),
    );
    entries.insert(
        String::from("completions/just.bash"),
        user_completions_dir("just.bash"),
    );

    extract::download_and_extract(uri, entries).await?;
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
        user_completions_dir("rg.bash"),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `starship` (<https://github.com/starship/starship>)
async fn starship() -> ::anyhow::Result<()> {
    /// Version of `starship` to install
    const STARSHIP_VERSION: &str = "1.23.0";

    let file = format!("starship-{ARCHITECTURE}-unknown-linux-musl");
    let uri = format!(
        "https://github.com/starship/starship/releases/download/v{STARSHIP_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("starship"),
        format!("{}/starship", environment::home_local_bin()),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `yazi` (<https://github.com/sxyazi/yazi>)
async fn yazi() -> ::anyhow::Result<()> {
    /// Version of `starship` to install
    const YAZI_VERSION: &str = "25.5.31";

    let file = format!("yazi-{ARCHITECTURE}-unknown-linux-musl");
    let uri =
        format!("https://github.com/sxyazi/yazi/releases/download/v{YAZI_VERSION}/{file}.zip");

    let mut entries = collections::HashMap::new();
    entries.insert(
        format!("{file}/ya"),
        format!("{}/ya", environment::home_local_bin()),
    );
    entries.insert(
        format!("{file}/yazi"),
        format!("{}/yazi", environment::home_local_bin()),
    );
    entries.insert(
        format!("{file}/completions/ya.bash"),
        user_completions_dir("ya.bash"),
    );
    entries.insert(
        format!("{file}/completions/yazi.bash"),
        user_completions_dir("yazi.bash"),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `zoxide` (<https://github.com/zellij-org/zellij>)
async fn zellij() -> ::anyhow::Result<()> {
    /// Version of `zoxide` to install
    const ZOXIDE_VERSION: &str = "0.42.2";

    let file = format!("zellij-{ARCHITECTURE}-unknown-linux-musl");
    let uri = format!(
        "https://github.com/zellij-org/zellij/releases/download/v{ZOXIDE_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("zellij"),
        format!("{}/zellij", environment::home_local_bin()),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}

/// Install `zoxide` (<https://github.com/ajeetdsouza/zoxide>)
async fn zoxide() -> ::anyhow::Result<()> {
    /// Version of `zoxide` to install
    const ZOXIDE_VERSION: &str = "0.9.8";

    let file = format!("zoxide-{ZOXIDE_VERSION}-{ARCHITECTURE}-unknown-linux-musl");
    let uri = format!(
        "https://github.com/ajeetdsouza/zoxide/releases/download/v{ZOXIDE_VERSION}/{file}.tar.gz"
    );

    let mut entries = collections::HashMap::new();
    entries.insert(
        String::from("zoxide"),
        format!("{}/zoxide", environment::home_local_bin()),
    );

    extract::download_and_extract(uri, entries).await?;
    Ok(())
}
