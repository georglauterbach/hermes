//! Contains CLI parameter definition using [`clap`]
//! and helper structures tied to CLI input.

/// The command to run
#[derive(Debug, ::clap::Subcommand)]
pub enum Command {
    /// Install configuration files, packages & programs
    Run {
        /// Whether to install a set of core packages
        #[clap(short, long, default_value_t = false)]
        install_packages: bool,
    },
    /// Update hermes itself (to the latest available version)
    Update,
}

/// The set of arguments parsed and evaluated by [`::clap`]
#[derive(Debug, clap::Parser)]
#[command(
  bin_name=clap::crate_name!(),
  author=clap::crate_authors!(),
  about=clap::crate_description!(),
  long_about=clap::crate_description!(),
  version=clap::crate_version!(),
  propagate_version=true)]
pub struct Arguments {
    /// Specify the verbosity
    #[clap(flatten)]
    pub verbosity: ::clap_verbosity_flag::Verbosity<::clap_verbosity_flag::InfoLevel>,

    /// Run in non-interactive mode (do not ask for confirmations)
    #[clap(short, long, default_value_t = false)]
    pub non_interactive: bool,

    /// Used when calling _hermes_ again in the correct environment.
    /// Indicates whether _hermes_ was called again.
    #[clap(long, hide = true, default_value_t = false)]
    pub assume_correct_invocation: bool,

    /// The command to run
    #[command(subcommand)]
    pub command: Command,
}

impl Arguments {
    /// Initializes the [`::tracing_subscriber`] based on the verbosity level.
    pub fn init_tracing(&self) {
        use ::tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

        ::tracing_subscriber::registry()
            .with(
              ::tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                  format!("polling=warn,reqwest=warn,reqwest=warn,hyper_util::client::legacy=warn,async_io=warn,async_std=warn,{}", self.verbosity).into()
              })
            ) .with(tracing_subscriber::fmt::layer().with_target(false))
            .init();
    }
}
