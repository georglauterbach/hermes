//! A glorified tar-decompressor

use ::async_std::stream::StreamExt as _;

/// The arguments `hermes` takes
#[derive(Debug, ::clap::Parser)]
#[command(
    bin_name=::clap::crate_name!(),
    author=::clap::crate_authors!(),
    about=::clap::crate_description!(),
    long_about=::clap::crate_description!(),
    version=::clap::crate_version!(),
    propagate_version=true
)]
struct Arguments {
    /// Define the log verbosity
    #[clap(flatten)]
    verbosity: ::clap_verbosity_flag::Verbosity<::clap_verbosity_flag::InfoLevel>,
    /// Whether to overwrite all files when unpacking
    #[clap(short, long)]
    force: bool,
}

impl Arguments {
    /// Initializes the [`::tracing_subscriber`] based on the verbosity level.
    pub fn init_tracing(&self) {
        ::tracing_subscriber::fmt()
            .with_max_level(self.verbosity)
            .with_target(false)
            .init();
    }
}

/// The `.tar.xz` archive created by `cupid`
#[cfg(target_arch = "x86_64")]
const ARCHIVE: &[u8] = include_bytes!("../../.assets/x86_64/archive.tar.xz");
#[cfg(target_arch = "aarch64")]
const ARCHIVE: &[u8] = include_bytes!("../../.assets/aarch64/archive.tar.xz");

/// Log a message and terminate `hermes`
fn log_and_exit_with_error(message: impl AsRef<str>) -> ! {
    tracing::error!("{}", message.as_ref());
    ::std::process::exit(1);
}

/// _hermes_' entrypoint
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let arguments = <Arguments as ::clap::Parser>::parse();
    arguments.init_tracing();

    ::tracing::info!("Starting hermes {}", ::clap::crate_version!());

    let Some(home_directory) = ::std::env::home_dir() else {
        log_and_exit_with_error("Could not locate home directory");
    };

    let buffer_reader = ::tokio::io::BufReader::new(ARCHIVE);
    let mut decoder = ::async_compression::tokio::bufread::XzDecoder::new(buffer_reader);
    let mut archive = ::tokio_tar::Archive::new(&mut decoder);

    if arguments.force {
        if let Err(error) = archive.unpack(home_directory).await {
            log_and_exit_with_error(format!(
                "Failed to unpack complete archive forcefully: {error}"
            ));
        }
    } else {
        let Ok(mut entries) = archive.entries() else {
            log_and_exit_with_error("Could not turn archive into iterator over entries");
        };

        while let Some(entry) = entries.next().await {
            let mut entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    log_and_exit_with_error(format!("Could not get entry from archive: {error}"));
                }
            };

            let entry_path_str = match entry.path() {
                Ok(path) => path.to_string_lossy().to_string(),
                Err(error) => {
                    log_and_exit_with_error(format!("Could get acquire path of entry: '{error}'"));
                }
            };

            let local_path = home_directory.join(&entry_path_str);

            if local_path.exists() && !local_path.is_dir() {
                ::tracing::info!("Not overwriting '{}'", local_path.display());
                continue;
            }

            if let Some(parent) = local_path.parent()
                && let Err(error) = ::std::fs::create_dir_all(parent)
            {
                log_and_exit_with_error(format!(
                    "Could not create parent directory for new file '{error}'"
                ));
            }

            if let Err(error) = entry.unpack(&local_path).await {
                log_and_exit_with_error(format!(
                    "Could not unpack entry '{entry_path_str}' to '{}': {error}",
                    local_path.display()
                ));
            }
        }
    }

    ::tracing::info!("Finished");
}
