mod config;
mod execution;
pub mod features;

pub use self::{config::Config, execution::TaskKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to get cargo metadata")]
    Metadata(#[from] cargo_metadata::Error),
    #[error(transparent)]
    InvalidFeature(#[from] features::MissingFeature),
    #[error("{}", message)]
    Io {
        message: &'static str,
        #[source]
        source: std::io::Error,
    },
    #[error("child process exited with {}", _0)]
    Fail(std::process::ExitStatus),
}

#[cfg(test)]
mod tests {}
