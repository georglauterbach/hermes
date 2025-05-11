//! _hermes_ binary part that uses [`lib.rs`](./lib.rs).

use ::anyhow::Context as _;

/// _hermes_' entrypoint.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let arguments = <hermes::arguments::Arguments as clap::Parser>::parse();
    arguments.init_tracing();

    ::tracing::trace!("Dumping CLI arguments: \n{arguments:#?}");

    if let Err(error) = if arguments.assume_correct_invocation {
        Box::pin(hermes::work::run(arguments)).await
    } else {
        ::tracing::info!("This is hermes {}", env!("CARGO_PKG_VERSION"));
        match hermes::prepare::call_again(&arguments).context("Initial conditions could not be met")
        {
            Ok(true) => {
                ::tracing::info!("Finished without errors");
                Ok(())
            }
            Ok(false) => ::std::process::exit(1),
            Err(error) => Err(error),
        }
    } {
        let mut chain = error.chain();
        ::tracing::error!("{}", chain.next().unwrap());

        if chain.len() > 0 {
            println!("Caused by:");
            for (number, error) in chain.enumerate() {
                println!("    {number}: {error}");
            }
        }

        std::process::exit(1);
    };

    Ok(())
}
