//! The library part of `cupid`

use ::anyhow::Context as _;

pub mod arguments {
    //! Argument parsing

    /// Information about the architecture that we
    /// pack `hermes`'s archive for
    #[derive(Debug, Clone, Copy, ::clap::ValueEnum)]
    pub enum Architecture {
        /// AMD64
        X86_64,
        /// ARM64
        Aarch64,
    }

    impl Architecture {
        /// The link-library name of a
        /// [target triple](https://mcyoung.xyz/2025/04/14/target-triples/)
        #[must_use]
        pub const fn link_library(&self) -> &'static str {
            match self {
                Self::X86_64 => "musl",
                Self::Aarch64 => "gnu",
            }
        }
    }

    impl ::std::fmt::Display for Architecture {
        fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::X86_64 => write!(formatter, "x86_64"),
                Self::Aarch64 => write!(formatter, "aarch64"),
            }
        }
    }

    /// Arguments accepted by `cupid`
    #[derive(Debug, clap::Parser)]
    #[command(
        bin_name=clap::crate_name!(),
        author=clap::crate_authors!(),
        about=clap::crate_description!(),
        long_about=clap::crate_description!(),
        version=clap::crate_version!(),
        propagate_version=true
    )]
    pub struct Arguments {
        /// The architecture to be used
        #[clap(short, long, default_value = "x86-64")]
        pub architecture: Architecture,
    }
}

/// Copy the project configuration into the archive that is packed for `hermes`
///
/// ### Errors
///
/// All encountered errors are immediately propagated.
pub async fn symlink_configuration_directory(
    architecture: arguments::Architecture,
) -> ::anyhow::Result<()> {
    let asset_directory = asset_base_directory(architecture);
    let archive_directory = archive_directory(architecture);
    let config_directory = archive_directory.join(".config");

    let existing_config_directory = asset_directory
        .parent()
        .context("Could not get parent of asset directory")?
        .parent()
        .context("Could not get repository root directory (parent parent of asset directory)")?
        .join("data")
        .join("config");

    if !archive_directory.exists() {
        ::tokio::fs::create_dir_all(archive_directory)
            .await
            .context("Could not create archive directory")?;
    }

    if config_directory.exists() {
        ::std::fs::remove_file(&config_directory)
            .context(format!("Could not delete {}", config_directory.display()))?;
    }

    ::tokio::fs::symlink(&existing_config_directory, &config_directory)
        .await
        .context(format!(
            "Could not symlink '{}' -> '{}'",
            config_directory.display(),
            existing_config_directory.display()
        ))?;

    Ok(())
}

/// Create the final `.tar.xz` archive
///
/// ### Errors
///
/// All encountered errors are immediately propagated.
pub async fn create_archive(architecture: arguments::Architecture) -> ::anyhow::Result<()> {
    {
        println!("Creating final archive");
        let mut builder = ::tokio_tar::Builder::new(Vec::with_capacity(1_000_000 * 40));
        builder.follow_symlinks(true);
        builder
            .append_dir_all("", archive_directory(architecture))
            .await
            .context("Appending archive directory to archive builder failed")?;

        let data = builder.into_inner().await?;
        let tar_reader = ::tokio::io::BufReader::new(&data[..]);
        let mut encoder = ::async_compression::tokio::bufread::XzEncoder::new(tar_reader);

        let mut out_file =
            ::tokio::fs::File::create(asset_base_directory(architecture).join("archive.tar.xz"))
                .await?;
        ::tokio::io::copy(&mut encoder, &mut out_file).await?;

        Ok::<(), ::tokio::io::Error>(())
    }
    .context("Could not build final archive for hermes")
}

/// Get the path to the `.asset/` directory in this repository
#[must_use]
pub fn asset_base_directory(architecture: arguments::Architecture) -> ::std::path::PathBuf {
    let cargo_manifest_directory = ::std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cargo_manifest_directory
        .parent()
        .unwrap_or(&cargo_manifest_directory)
        .parent()
        .unwrap_or(&cargo_manifest_directory)
        .join(".assets")
        .join(architecture.to_string())
}

/// [`asset_base_directory`] + `/archive`
#[must_use]
pub fn archive_directory(architecture: arguments::Architecture) -> ::std::path::PathBuf {
    asset_base_directory(architecture).join("archive")
}

pub mod programs {
    //! This module handles all programs and their associated data

    use crate::asset_base_directory;

    use super::arguments::Architecture;
    use ::std::collections;

    use ::anyhow::Context as _;

    /// Process all programs and their associated archived and files
    ///
    /// ### Errors
    ///
    /// All encountered errors are immediately propagated.
    pub async fn process(architecture: Architecture) -> ::anyhow::Result<()> {
        let mut join_set = ::tokio::task::JoinSet::new();
        join_set.spawn(atuin(architecture));
        join_set.spawn(bat(architecture));
        join_set.spawn(blesh(architecture));
        join_set.spawn(bottom(architecture));
        join_set.spawn(delta(architecture));
        join_set.spawn(dust(architecture));
        join_set.spawn(dysk(architecture));
        join_set.spawn(eza(architecture));
        join_set.spawn(fd(architecture));
        join_set.spawn(fzf(architecture));
        join_set.spawn(gitui(architecture));
        join_set.spawn(just(architecture));
        join_set.spawn(ripgrep(architecture));
        join_set.spawn(starship(architecture));
        join_set.spawn(yazi(architecture));
        join_set.spawn(zellij(architecture));
        join_set.spawn(zoxide(architecture));

        while let Some(result) = join_set.join_next().await {
            match result {
                Err(join_error) => {
                    return Err(join_error).context("Could not join program-processing handle");
                }
                Ok(Err(download_error)) => {
                    return Err(download_error)
                        .context("An error occurred processing a program download");
                }
                Ok(Ok(())) => (),
            }
        }

        Ok(())
    }

    /// The type of archive we download in [`Program`]
    #[derive(Debug, Clone, Copy)]
    enum ArchiveType {
        /// A `.tar.gz` archive
        TarGz,
        /// A `.tar.xz` archive
        TarXz,
        /// A `.zip` archive
        Zip,
    }

    impl ::std::fmt::Display for ArchiveType {
        fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            let ending = match self {
                Self::TarGz => ".tar.gz",
                Self::TarXz => ".tar.xz",
                Self::Zip => ".zip",
            };
            write!(formatter, "{ending}")
        }
    }

    /// Which entries from a program's archive to unpack
    #[derive(Debug)]
    enum Entries {
        /// Unpack only specific entries
        Specific(::std::collections::HashMap<String, String>),
        /// Unpack all entries
        All(&'static str, &'static str),
    }

    /// A structure that groups properties of a program
    #[derive(Debug)]
    struct Program {
        /// The program name
        name: &'static str,
        /// The program version
        version: &'static str,
        /// The [`ArchiveType`] that the program is packaged in
        download_archive_type: ArchiveType,
        /// The URI from which to download the archive that
        /// contains the program and its associated data
        download_uri: String,
        /// The entries inside the archive to copy into the archive
        archive_entries: Entries,
    }

    impl Program {
        /// Create a new instance of [`Program`]
        #[must_use]
        pub const fn new(
            name: &'static str,
            version: &'static str,
            download_archive_type: ArchiveType,
            download_uri: String,
            archive_entries: Entries,
        ) -> Self {
            Self {
                name,
                version,
                download_archive_type,
                download_uri,
                archive_entries,
            }
        }

        /// Process a [`Program`]
        ///
        /// 1. Download and / or read it
        /// 2. Extract the archive
        /// 3. Place the correct files for packing later
        ///
        /// ### Errors
        ///
        /// All encountered errors are immediately propagated.
        pub async fn process(self, architecture: Architecture) -> ::anyhow::Result<()> {
            let asset_directory = super::asset_base_directory(architecture);

            let archive = self
                .download_or_read(&asset_directory)
                .await
                .context("Could not download or read archive")?;

            let extracted_directory = self
                .extract(archive, &asset_directory)
                .await
                .context("Could not extract archive")?;

            self.symlink_files(&extracted_directory, architecture)
                .await
                .context("Could not place archive files for packing")
        }

        /// Download the archive described by a [`Program`] and / or read it
        async fn download_or_read(
            &self,
            asset_directory: &::std::path::Path,
        ) -> ::anyhow::Result<::bytes::Bytes> {
            let download_directory = asset_directory
                .join("downloads")
                .join(self.name)
                .join(self.version);

            if !download_directory.exists() {
                ::tokio::fs::create_dir_all(&download_directory)
                    .await
                    .context(format!(
                        "Could not create asset directory '{}'",
                        download_directory.display()
                    ))?;
            }

            let archive_file =
                download_directory.join(format!("{}{}", self.name, self.download_archive_type));

            let archive = if archive_file.exists() {
                println!("Reading already existing archive for '{}'", self.name);
                ::tokio::fs::read(archive_file)
                    .await
                    .context("Could not read archive")?
                    .into()
            } else {
                println!("Downloading and reading archive for '{}'", self.name);
                let response = ::reqwest::get(&self.download_uri)
                    .await
                    .context("Could not download archive")?;

                if !response.status().is_success() {
                    ::anyhow::bail!(
                        "Request to '{}' failed: {}",
                        self.download_uri,
                        response.status()
                    );
                }
                let response = response
                    .bytes()
                    .await
                    .context("Could not convert response into bytes")?;

                ::tokio::fs::write(archive_file, &response)
                    .await
                    .context("Could not write archive to disk")?;

                response
            };

            Ok(archive)
        }

        /// Extract a downloaded archive described by a [`Program`]
        async fn extract(
            &self,
            archive: ::bytes::Bytes,
            asset_directory: &::std::path::Path,
        ) -> ::anyhow::Result<::std::path::PathBuf> {
            let directory_extracted = asset_directory
                .join("extracted")
                .join(self.name)
                .join(self.version);

            if directory_extracted.exists() {
                println!("Archive for '{}' already unpacked", self.name);
                return Ok(directory_extracted);
            }

            println!("Unpacking archive for '{}'", self.name);

            match self.download_archive_type {
                ArchiveType::TarGz => {
                    let decoder =
                        ::async_compression::tokio::bufread::GzipDecoder::new(&archive[..]);
                    ::tokio_tar::ArchiveBuilder::new(decoder)
                        .set_preserve_permissions(true)
                        .set_preserve_mtime(true)
                        .set_unpack_xattrs(true)
                        .build()
                        .unpack(&directory_extracted)
                        .await
                        .context("Could not unpack .tar.gz archive")
                }
                ArchiveType::TarXz => {
                    let decoder = ::async_compression::tokio::bufread::XzDecoder::new(&archive[..]);
                    ::tokio_tar::ArchiveBuilder::new(decoder)
                        .set_preserve_permissions(true)
                        .set_preserve_mtime(true)
                        .set_unpack_xattrs(true)
                        .build()
                        .unpack(&directory_extracted)
                        .await
                        .context("Could not unpack .tar.xz archive")
                }
                ArchiveType::Zip => ::zip::ZipArchive::new(std::io::Cursor::new(&archive[..]))
                    .context("Could not build ZIP archive reader - ZIP malformed?")?
                    .extract(&directory_extracted)
                    .context("Could not unpack .zip archive"),
            }
            .map_err(|error| {
                if let Err(error) = ::std::fs::remove_dir_all(&directory_extracted)
                    .context("Could not clean up extracted directory after error")
                {
                    error
                } else {
                    error
                }
            })?;

            Ok(directory_extracted)
        }

        /// Symlink files into the archive directory that is later packaged
        async fn symlink_files(
            &self,
            extracted_directory: &::std::path::Path,
            architecture: Architecture,
        ) -> ::anyhow::Result<()> {
            /// Actually create the symbolic link
            async fn symlink(
                name: &str,
                from: &::std::path::Path,
                to: &::std::path::Path,
            ) -> ::anyhow::Result<()> {
                if !from.exists() {
                    ::anyhow::bail!(
                        "File '{}' from archive for '{}' does not exist",
                        from.display(),
                        name
                    );
                }

                to.parent()
                    .map(::std::fs::create_dir_all)
                    .transpose()
                    .context(format!(
                        "Could not create directory to put '{}' in",
                        to.display()
                    ))?;

                if to.exists() {
                    tokio::fs::remove_file(to)
                        .await
                        .context("Could not remove existing symbolic link")?;
                }

                ::tokio::fs::symlink(&from, &to).await.context(format!(
                    "Could not create symbolic link from archive entry '{}' to '{}'",
                    from.display(),
                    to.display()
                ))?;

                Ok(())
            }

            let archive_directory = super::archive_directory(architecture);
            println!("Symlinking files for {}", self.name);

            if !archive_directory.exists() {
                ::tokio::fs::create_dir_all(&archive_directory)
                    .await
                    .context(format!(
                        "Could not create archive directory '{}'",
                        archive_directory.display()
                    ))?;
            }

            match &self.archive_entries {
                Entries::All(from, to) => {
                    let from = extracted_directory.join(from);
                    let to = archive_directory.join(to);
                    symlink(self.name, &from, &to).await?;
                }
                Entries::Specific(entries) => {
                    for (from, to) in entries {
                        let from = extracted_directory.join(from);
                        let to = archive_directory.join(to);
                        symlink(self.name, &from, &to).await?;
                    }
                }
            }

            Ok(())
        }
    }

    /// A helper to easily compute `.local/bin/` + `and`
    fn local_bin(and: &str) -> String {
        format!(".local/bin/{and}")
    }

    /// A helper to easily compute
    /// `.local/share/bash-completion/completions/` + `and`
    fn bash_completion(and: &str) -> String {
        format!(".local/share/bash-completion/completions/{and}")
    }

    /// <https://github.com/atuinsh/atuin>
    async fn atuin(architecture: super::arguments::Architecture) -> ::anyhow::Result<()> {
        let name = "atuin";
        let version = "18.10.0";
        let file = format!("{name}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/atuinsh/atuin/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/{name}"), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/sharkdp/bat>
    async fn bat(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "bat";
        let version = "0.26.0";
        let file = format!("{name}-v{version}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/sharkdp/bat/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/{name}"), local_bin(name));
        entries.insert(
            format!("{file}/autocomplete/bat.bash"),
            bash_completion("bat.bash"),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/akinomyoga/ble.sh>
    async fn blesh(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "blesh";
        let version = "nightly";
        let file = "ble-nightly";
        let archive_type = ArchiveType::TarXz;
        let uri = format!(
            "https://github.com/akinomyoga/ble.sh/releases/download/{version}/{file}{archive_type}"
        );

        Program::new(
            name,
            version,
            archive_type,
            uri,
            Entries::All("ble-nightly", ".local/share/blesh"),
        )
        .process(architecture)
        .await?;

        let ble_base_path = asset_base_directory(architecture)
            .join("extracted")
            .join(name)
            .join(version)
            .join(file);
        let _ = ::std::fs::remove_dir_all(ble_base_path.join("cache.d"));
        let _ = ::std::fs::remove_dir_all(ble_base_path.join("contrib").join("airline"));
        let _ = ::std::fs::remove_dir_all(ble_base_path.join("doc"));
        let _ = ::std::fs::remove_dir_all(ble_base_path.join("licenses"));
        let _ = ::std::fs::remove_dir_all(ble_base_path.join("run"));

        Ok(())
    }

    /// <https://github.com/ClementTsang/bottom>
    async fn bottom(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "bottom";
        let version = "0.12.3";
        let file = format!("{name}_{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/ClementTsang/bottom/releases/download/{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert("btm".to_string(), local_bin("btm"));
        entries.insert(
            "completion/btm.bash".to_string(),
            bash_completion("btm.bash"),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/dandavison/delta>
    async fn delta(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "delta";
        let version = "0.18.2";
        let file = format!(
            "{name}-{version}-{architecture}-unknown-linux-{}",
            architecture.link_library()
        );
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/dandavison/delta/releases/download/{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/{name}"), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/bootandy/dust>
    async fn dust(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "dust";
        let version = "1.2.3";
        let file = format!("{name}-v{version}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/bootandy/dust/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/{name}"), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/Canop/dysk>
    async fn dysk(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "dysk";
        let version = "3.4.0";
        let archive_type = ArchiveType::Zip;
        let uri = format!(
            "https://github.com/Canop/dysk/releases/download/v{version}/dysk_{version}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(
            format!("build/{architecture}-unknown-linux-musl/dysk"),
            local_bin(name),
        );
        entries.insert(
            String::from("build/completion/dysk.bash"),
            bash_completion("dysk.bash"),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/eza-community/eza>
    async fn eza(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "eza";
        let version = "0.23.4";
        let file = format!(
            "{name}_{architecture}-unknown-linux-{}",
            architecture.link_library()
        );
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/eza-community/eza/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("./{name}"), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/sharkdp/fd>
    async fn fd(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "fd";
        let version = "10.3.0";
        let file = format!("{name}-v{version}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/sharkdp/fd/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/{name}"), local_bin(name));
        entries.insert(
            format!("{file}/autocomplete/fd.bash"),
            bash_completion("fd.bash"),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/junegunn/fzf>
    async fn fzf(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "fzf";
        let version = "0.66.1";
        let file = match architecture {
            Architecture::X86_64 => format!("{name}-{version}-linux_amd64"),
            Architecture::Aarch64 => format!("{name}-{version}-linux_arm64"),
        };
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/junegunn/fzf/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(name.to_string(), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/extrawurst/gitui>
    async fn gitui(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "gitui";
        let version = "0.27.0";
        let file = format!("{name}-linux-{architecture}");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/extrawurst/gitui/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("./{name}"), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/casey/just>
    async fn just(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "just";
        let version = "1.43.0";
        let file = format!("{name}-{version}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/casey/just/releases/download/{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(name.to_string(), local_bin(name));
        entries.insert(
            String::from("completions/just.bash"),
            bash_completion("just.bash"),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/BurntSushi/ripgrep>
    async fn ripgrep(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "ripgrep";
        let version = "15.1.0";
        let file = format!(
            "{name}-{version}-{architecture}-unknown-linux-{}",
            architecture.link_library()
        );
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/BurntSushi/ripgrep/releases/download/{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/rg"), local_bin("rg"));
        entries.insert(
            format!("{file}/complete/rg.bash"),
            bash_completion("rg.bash"),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/starship/starship>
    async fn starship(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "starship";
        let version = "1.24.0";
        let file = format!("{name}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/starship/starship/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(name.to_string(), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/sxyazi/yazi>
    async fn yazi(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "yazi";
        let version = "25.5.31";
        let file = format!("{name}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::Zip;
        let uri = format!(
            "https://github.com/sxyazi/yazi/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/ya"), local_bin("ya"));
        entries.insert(format!("{file}/{name}"), local_bin(name));
        entries.insert(
            format!("{file}/completions/ya.bash"),
            bash_completion("ya.bash"),
        );
        entries.insert(
            format!("{file}/completions/yazi.bash"),
            bash_completion("yazi.bash"),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/zellij-org/zellij>
    async fn zellij(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "zellij";
        let version = "0.43.1";
        let file = format!("{name}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/zellij-org/zellij/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(name.to_string(), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/ajeetdsouza/zoxide>
    async fn zoxide(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "zoxide";
        let version = "0.9.8";
        let file = format!("{name}-{version}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/ajeetdsouza/zoxide/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(name.to_string(), local_bin(name));
        entries.insert(
            "completions/zoxide.bash".to_string(),
            bash_completion("zoxide.bash"),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }
}
