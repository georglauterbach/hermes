//! Contains CLI paramter definition using [`clap`]
//! and helper structures tied to CLI input.

/// Represents the version of Ubuntu that we are working with.
#[derive(Debug, Copy, Clone)]
pub enum UbuntuVersion {
    /// Fallback version, used when no other version could be detected
    Fallback,
    /// Ubuntu 24.04 LTS (Noble Numbat)
    Ubuntu24_04,
}

impl ::clap::ValueEnum for UbuntuVersion {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Fallback, Self::Ubuntu24_04]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Fallback => ::clap::builder::PossibleValue::new("fallback"),
            Self::Ubuntu24_04 => ::clap::builder::PossibleValue::new("24.04"),
        })
    }

    fn from_str(input: &str, _: bool) -> Result<Self, String> {
        #[allow(clippy::single_match_else)]
        match input {
            "24.04" => Ok(Self::Ubuntu24_04),
            _ => {
                ::tracing::debug!("Fallback chosen for Ubuntu {input}");
                Ok(Self::Fallback)
            }
        }
    }
}

impl ::std::fmt::Display for UbuntuVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Self::Fallback => write!(f, "fallback"),
            Self::Ubuntu24_04 => write!(f, "24.04"),
        }
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
