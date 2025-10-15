//! A glorified tar-decompressor

/// The `.tar.xz` archive created by `cupid`
#[cfg(debug_assertions)]
const ARCHIVE: &[u8] = &[];

/// The `.tar.xz` archive created by `cupid`
#[cfg(all(not(debug_assertions), target_arch = "x86_64"))]
const ARCHIVE: &[u8] = include_bytes!("../../.assets/x86_64/archive.tar.xz");

/// The `.tar.xz` archive created by `cupid`
#[cfg(all(not(debug_assertions), target_arch = "aarch64"))]
const ARCHIVE: &[u8] = include_bytes!("../../.assets/aarch64/archive.tar.xz");

/// _hermes_' entrypoint
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Starting hermes {}", env!("CARGO_PKG_VERSION"));

    let Some(home_directory) = std::env::home_dir() else {
        eprintln!("Could not locate home directory");
        std::process::exit(1);
    };

    let mut decoder =
        ::async_compression::tokio::bufread::XzDecoder::new(::tokio::io::BufReader::new(ARCHIVE));
    let mut archive = ::tokio_tar::Archive::new(&mut decoder);

    if let Err(error) = archive.unpack(home_directory).await {
        eprintln!("Unpacking archive failed: {error}");
        std::process::exit(1);
    }
}
