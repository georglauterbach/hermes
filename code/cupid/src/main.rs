//! A support crate for `hermes` that packs the
//! archive that `hermes` uses for its (offline)
//! installation

/// The entrypoint of this build script
#[::tokio::main(flavor = "multi_thread")]
async fn main() {
    if let Err(error) = async {
        ::tokio::try_join!(
            ::cupid::programs::process(),
            ::cupid::symlink_configuration_directory()
        )?;
        ::cupid::create_archive().await
    }
    .await
    {
        eprintln!("{error:?}");
        ::std::process::exit(1);
    }
}
