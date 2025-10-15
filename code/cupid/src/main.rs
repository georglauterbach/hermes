//! A support crate for `hermes` that packs the
//! archive that `hermes` uses for its (offline)
//! installation

/// The entrypoint of this build script
#[::tokio::main(flavor = "multi_thread")]
async fn main() {
    let arguments = <::cupid::arguments::Arguments as ::clap::Parser>::parse();

    if let Err(error) = async {
        ::tokio::try_join!(
            ::cupid::programs::process(arguments.architecture),
            ::cupid::symlink_configuration_directory(arguments.architecture)
        )?;
        ::cupid::create_archive(arguments.architecture).await
    }
    .await
    {
        eprintln!("{error:?}");
        ::std::process::exit(1);
    }
}
