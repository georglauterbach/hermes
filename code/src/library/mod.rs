//! This private module hosts modules that perform work in _hermes_.

pub mod cli;
mod data;
mod fs;
pub mod logger;
pub mod prepare;
pub mod work;

/// Given a vector of [`anyhow::Error`]s, this macro evaluates the vector and returns an [`anyhow::Result`].
/// It returns [`Ok`] of the vector is empty, and [`Err`] with the errors and contexts sorted correctly.
/// It will also log the message given the second argument.
macro_rules! evaluate_errors_vector {
    ($errors:expr, $message:expr) => {{
      use ::anyhow::Context as _;

      if $errors.is_empty() {
        Ok(())
    } else {
        let mut final_error = Err($errors.pop().context(
            "BUG! Popping an error should always be possible because we checked the size before",
        )?);
        for error in $errors {
            final_error = final_error.context(error);
        }

        final_error.context("Could not acquire all additonal programs from GitHub successfully")
    }
    }};
}

use evaluate_errors_vector;
