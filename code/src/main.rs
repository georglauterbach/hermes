//! _hermes_ binary part that uses [`lib.rs`](./lib.rs).

use ::anyhow::Context as _;

/// _hermes_' entrypoint.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let arguments = <hermes::cli::Arguments as clap::Parser>::parse();
    hermes::logger::Logger::initialize(arguments.verbosity.log_level())?;
    ::log::trace!("Dumping CLI arguments: \n{arguments:#?}");

    if let Err(error) = if arguments.assume_correct_invocation {
        hermes::work::run(arguments).await
    } else {
        let success = hermes::prepare::call_again(&arguments)
            .context("Initial conditions could not be met")?;
        if success {
            ::log::info!("Finished without errors");
            Ok(())
        } else {
            std::process::exit(1);
        }
    } {
        let mut chain = error.chain().rev();
        log::error!("{}", chain.next().unwrap());

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
