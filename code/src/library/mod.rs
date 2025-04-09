//! This private module hosts modules that perform work in _hermes_.

pub mod cli;
mod data;
mod fs;
pub mod prepare;
pub mod work;

/// Evaluates a list of `Result<(), ::anyhow::Error>` and
/// returns an [`Err`] if any element in the list is an
/// [`Err`].
///
/// All errors are recorded by using [`::anyhow::Context`].
pub fn evaluate_results(
    results: impl IntoIterator<Item = Result<(), ::anyhow::Error>>,
) -> Result<(), anyhow::Error> {
    let mut errors = vec![];

    let _: Vec<_> = results
        .into_iter()
        .filter_map(|result| result.map_err(|error| errors.push(error)).ok())
        .collect();

    let Some(mut error) = errors.pop() else {
        return Ok(());
    };

    for context in errors {
        error = error.context(context);
    }

    Err(error)
}
