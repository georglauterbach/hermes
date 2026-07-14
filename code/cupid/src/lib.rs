// Copyright (c) 2024 - PRESENT Georg Lauterbach
// MIT License

//! The library part of `cupid`

use ::anyhow::Context as _;

pub mod arguments {
    //! Argument parsing

    /// Information about the architecture that we
    /// pack `hermes`'s archive for
    #[derive(Debug, Clone, Copy, PartialEq, Eq, ::clap::ValueEnum)]
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
        /// Do not create the final archive file
        #[clap(long)]
        pub no_archive: bool,
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
        .join("home")
        .join(".config");

    if !archive_directory.exists() {
        ::tokio::fs::create_dir_all(archive_directory)
            .await
            .context("Could not create archive directory")?;
    }

    if config_directory.exists() {
        ::std::fs::remove_file(&config_directory)
            .with_context(|| format!("Could not delete {}", config_directory.display()))?;
    }

    ::tokio::fs::symlink(&existing_config_directory, &config_directory)
        .await
        .with_context(|| {
            format!(
                "Could not symlink '{}' -> '{}'",
                config_directory.display(),
                existing_config_directory.display()
            )
        })?;

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
        let mut builder = ::tokio_tar::Builder::new(Vec::with_capacity(1_000_000 * 50));
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
        join_set.spawn(bat(architecture));
        join_set.spawn(btop(architecture));
        join_set.spawn(delta(architecture));
        join_set.spawn(dust(architecture));
        join_set.spawn(dysk(architecture));
        join_set.spawn(eza(architecture));
        join_set.spawn(fd(architecture));
        join_set.spawn(flyline(architecture));
        join_set.spawn(fzf(architecture));
        join_set.spawn(gitui(architecture));
        join_set.spawn(jaq(architecture));
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
        /// Not compressed
        Uncompressed,
        /// A `.tar.gz` archive
        TarGz,
        /// A `.zip` archive
        Zip,
    }

    impl ::std::fmt::Display for ArchiveType {
        fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            let ending = match self {
                Self::Uncompressed => "",
                Self::TarGz => ".tar.gz",
                Self::Zip => ".zip",
            };
            write!(formatter, "{ending}")
        }
    }

    /// Which entries from a program's archive to unpack
    #[derive(Debug)]
    enum Entries<'a> {
        /// Unpack only specific entries
        Specific(::std::collections::HashMap<String, String>),
        /// Unpack all entries
        All(&'a str, String),
    }

    /// A structure that groups properties of a program
    #[derive(Debug)]
    struct Program<'a, 'b, 'c> {
        /// The program name
        name: &'a str,
        /// The program version
        version: &'b str,
        /// The [`ArchiveType`] that the program is packaged in
        download_archive_type: ArchiveType,
        /// The URI from which to download the archive that
        /// contains the program and its associated data
        download_uri: String,
        /// The entries inside the archive to copy into the archive
        archive_entries: Entries<'c>,
    }

    impl<'a, 'b, 'c> Program<'a, 'b, 'c> {
        /// Create a new instance of [`Program`]
        #[must_use]
        pub const fn new(
            name: &'a str,
            version: &'b str,
            download_archive_type: ArchiveType,
            download_uri: String,
            archive_entries: Entries<'c>,
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
                ArchiveType::Uncompressed => {
                    use ::tokio::io::AsyncWriteExt as _;

                    match tokio::fs::create_dir_all(&directory_extracted).await {
                        Ok(()) => {
                            tokio::fs::OpenOptions::new()
                                .create(true)
                                .truncate(true)
                                .write(true)
                                .mode(0o755)
                                .open(directory_extracted.join(self.name))
                                .await
                                .unwrap()
                                .write_all(&archive)
                                .await
                        }
                        error => error,
                    }
                }
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
                }
                ArchiveType::Zip => ::zip::ZipArchive::new(std::io::Cursor::new(&archive[..]))
                    .context("Could not build ZIP archive reader - ZIP malformed?")?
                    .extract(&directory_extracted)
                    .map_err(::std::io::Error::other),
            }
            .with_context(|| format!("Could not unpack {} archive", self.download_archive_type))
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
                    .with_context(|| {
                        format!("Could not create directory to put '{}' in", to.display())
                    })?;

                if to.exists() {
                    tokio::fs::remove_file(to).await.with_context(|| {
                        format!("Could not remove existing symbolic link '{}'", to.display())
                    })?;
                }

                ::tokio::fs::symlink(&from, &to).await.with_context(|| {
                    format!(
                        "Could not create symbolic link from archive entry '{}' to '{}'",
                        from.display(),
                        to.display()
                    )
                })?;

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

    /// Download a separate completion script for Bash
    async fn download_completion_script(
        name: &str,
        version: &str,
        architecture: Architecture,
        uri: String,
    ) -> ::anyhow::Result<()> {
        let name_completion = format!("{name}-completion");
        let archive_type = ArchiveType::Uncompressed;

        Program::new(
            &name_completion,
            version,
            archive_type,
            uri,
            Entries::All(&name_completion, bash_completion(name)),
        )
        .process(architecture)
        .await
    }

    /// A helper to easily compute `.local/bin/` + `and`
    fn local_bin(and: &str) -> String {
        format!(".local/bin/{and}")
    }

    /// A helper to easily compute `.local/lib/` + `and`
    fn local_lib(and: &str) -> String {
        format!(".local/lib/{and}")
    }

    /// A helper to easily compute
    /// `.local/share/bash-completion/completions/` + `and` + `.bash`
    fn bash_completion(and: &str) -> String {
        format!(".local/share/bash-completion/completions/{and}.bash")
    }

    /// <https://github.com/sharkdp/bat>
    async fn bat(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "bat";
        let version = "0.26.1";
        let file = format!("{name}-v{version}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/sharkdp/bat/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/{name}"), local_bin(name));
        entries.insert(
            format!("{file}/autocomplete/{name}.bash"),
            bash_completion(name),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/aristocratos/btop>
    async fn btop(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "btop";
        let version = "1.4.7";
        let file = format!("{name}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/aristocratos/btop/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert("./btop/bin/btop".to_owned(), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/dandavison/delta>
    async fn delta(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "delta";
        let version = "0.19.2";
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
            .await?;

        download_completion_script(name, version, architecture, format!("https://raw.githubusercontent.com/dandavison/delta/refs/tags/{version}/etc/completion/completion.bash")).await
    }

    /// <https://github.com/bootandy/dust>
    async fn dust(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "dust";
        let version = "1.2.4";
        let file = format!("{name}-v{version}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/bootandy/dust/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/{name}"), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await?;

        download_completion_script(name, version, architecture, format!("https://raw.githubusercontent.com/bootandy/dust/refs/tags/v{version}/completions/dust.bash")).await
    }

    /// <https://github.com/Canop/dysk>
    async fn dysk(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "dysk";
        let version = "3.6.1";
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
            bash_completion("dysk"),
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
        let base_uri = "https://github.com/eza-community/eza/releases/download";
        let uri = format!("{base_uri}/v{version}/{file}{archive_type}");

        let mut entries = collections::HashMap::new();
        entries.insert(format!("./{name}"), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await?;

        let uri = format!("{base_uri}/v{version}/completions-{version}{archive_type}");
        let mut entries = collections::HashMap::new();
        entries.insert(
            format!("./target/completions-{version}/{name}"),
            bash_completion(name),
        );

        Program::new(
            "eza-completion",
            version,
            archive_type,
            uri,
            Entries::Specific(entries),
        )
        .process(architecture)
        .await
    }

    /// <https://github.com/sharkdp/fd>
    async fn fd(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "fd";
        let version = "10.4.2";
        let file = format!("{name}-v{version}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/sharkdp/fd/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/{name}"), local_bin(name));
        entries.insert(
            format!("{file}/autocomplete/{name}.bash"),
            bash_completion(name),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/>
    async fn flyline(architecture: Architecture) -> ::anyhow::Result<()> {
        // cSpell: ignore libflyline
        let name = "flyline";
        let version = "1.2.3";
        let file = format!("libflyline-v{version}-{architecture}-unknown-linux-gnu");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/HalFrgrd/flyline/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(
            format!("libflyline.so.{version}"),
            local_lib("libflyline.so"),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/junegunn/fzf>
    async fn fzf(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "fzf";
        let version = "0.73.1";
        let file = match architecture {
            Architecture::X86_64 => format!("{name}-{version}-linux_amd64"),
            Architecture::Aarch64 => format!("{name}-{version}-linux_arm64"),
        };
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/junegunn/fzf/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(name.to_owned(), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await?;

        download_completion_script(
            name,
            version,
            architecture,
            format!(
                "https://raw.githubusercontent.com/junegunn/fzf/refs/tags/v{version}/shell/completion.bash"
            ),
        ).await
    }

    /// <https://github.com/extrawurst/gitui>
    async fn gitui(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "gitui";
        let version = "0.28.1";
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

    /// <https://github.com/01mf02/jaq>
    async fn jaq(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "jaq";
        let version = "3.1.0";
        let file = match architecture {
            Architecture::X86_64 => format!("{name}-{architecture}-unknown-linux-musl"),
            Architecture::Aarch64 => format!("{name}-{architecture}-unknown-linux-gnu"),
        };
        let archive_type = ArchiveType::Uncompressed;
        let uri = format!("https://github.com/01mf02/jaq/releases/download/v{version}/{file}");

        Program::new(
            name,
            version,
            archive_type,
            uri,
            Entries::All(name, local_bin(name)),
        )
        .process(architecture)
        .await
    }

    /// <https://github.com/casey/just>
    async fn just(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "just";
        let version = "1.55.1";
        let file = format!("{name}-{version}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/casey/just/releases/download/{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(name.to_owned(), local_bin(name));
        entries.insert(format!("completions/{name}.bash"), bash_completion(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/BurntSushi/ripgrep>
    async fn ripgrep(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "rg";
        let version = "15.1.0";
        let file = format!(
            "ripgrep-{version}-{architecture}-unknown-linux-{}",
            architecture.link_library()
        );
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/BurntSushi/ripgrep/releases/download/{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/{name}"), local_bin(name));
        entries.insert(
            format!("{file}/complete/{name}.bash"),
            bash_completion(name),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/starship/starship>
    async fn starship(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "starship";
        let version = "1.26.0";
        let file = format!("{name}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/starship/starship/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(name.to_owned(), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/sxyazi/yazi>
    async fn yazi(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "yazi";
        let version = "26.5.6";
        let file = format!("{name}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::Zip;
        let uri = format!(
            "https://github.com/sxyazi/yazi/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(format!("{file}/ya"), local_bin("ya"));
        entries.insert(format!("{file}/{name}"), local_bin(name));
        entries.insert(format!("{file}/completions/ya.bash"), bash_completion("ya"));
        entries.insert(
            format!("{file}/completions/{name}.bash"),
            bash_completion(name),
        );

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/zellij-org/zellij>
    async fn zellij(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "zellij";
        let version = "0.44.3";
        let file = format!("{name}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/zellij-org/zellij/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(name.to_owned(), local_bin(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }

    /// <https://github.com/ajeetdsouza/zoxide>
    async fn zoxide(architecture: Architecture) -> ::anyhow::Result<()> {
        let name = "zoxide";
        let version = "0.9.9";
        let file = format!("{name}-{version}-{architecture}-unknown-linux-musl");
        let archive_type = ArchiveType::TarGz;
        let uri = format!(
            "https://github.com/ajeetdsouza/zoxide/releases/download/v{version}/{file}{archive_type}"
        );

        let mut entries = collections::HashMap::new();
        entries.insert(name.to_owned(), local_bin(name));
        entries.insert(format!("completions/{name}.bash"), bash_completion(name));

        Program::new(name, version, archive_type, uri, Entries::Specific(entries))
            .process(architecture)
            .await
    }
}
