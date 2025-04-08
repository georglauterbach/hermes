//! Contains CLI parameter definition using [`clap`]
//! and helper structures tied to CLI input.

use ::std::path::Display;

/// The Linux distribution we're running on
#[derive(Debug)]
pub enum Distribution {
    /// Arch Linux
    Arch,
    /// Debian GNU/Linux
    Debian,
    /// Fedora
    Fedora,
    /// Ubuntu
    Ubuntu,
    /// Distribution not recognized
    Unknown,
}

impl From<&str> for Distribution {
    fn from(input: &str) -> Self {
        match input {
            "arch" => Self::Arch,
            "debian" => Self::Debian,
            "fedora" => Self::Fedora,
            "ubuntu" => Self::Ubuntu,
            _ => Self::Unknown,
        }
    }
}

impl ::std::fmt::Display for Distribution {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let name = match self {
            Self::Arch => "Arch",
            Self::Debian => "Debian",
            Self::Fedora => "Fedora",
            Self::Ubuntu => "Ubuntu",
            Self::Unknown => "unknown",
        };

        write!(formatter, "{name}")
    }
}

/// Workspace member that eases working with `unCORE`.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, clap::Parser, Clone)]
#[command(
  bin_name=clap::crate_name!(),
  author=clap::crate_authors!(),
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

    /// Install GUI programs
    #[clap(short, long, default_value_t = false)]
    pub gui: bool,

    /// Install new APT sources
    #[clap(short, long, default_value_t = false)]
    pub change_apt_sources: bool,

    /// Update hermes itself
    #[clap(short, long, default_value_t = false)]
    pub update: bool,

    /// Used when calling _hermes_ again in the correct environment.
    /// Indicates whether _hermes_ was called again.
    #[clap(long, hide = true, default_value_t = false)]
    pub assume_correct_invocation: bool,
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
            ).with(tracing_subscriber::fmt::layer())
            .init();
    }
}
