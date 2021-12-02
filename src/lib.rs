mod config;
mod execution;
pub mod features;

pub use self::{config::Config, execution::TaskKind};
use self::{execution::Task, features::FeatureMatrix};
use cargo_metadata::{Metadata, MetadataCommand, Package};
use itertools::Itertools;
use std::path::PathBuf;
use thiserror::Error;

pub fn run(
    command: String,
    args: Vec<String>,
    task: TaskKind,
    manifest_path: Option<PathBuf>,
    config: Config,
) -> Result<(), Error> {
    let mut cmd = MetadataCommand::new();
    if let Some(manifest_path) = manifest_path {
        cmd.manifest_path(manifest_path);
    }
    let metadata = cmd.exec()?;

    let matrices: Vec<_> = get_workspace_members(&metadata)
        .map(|package| {
            FeatureMatrix::new(package, &config)
                .map(|matrix| (&package.name, matrix))
        })
        .try_collect()?;

    let (_, errors): (Vec<_>, Vec<_>) = matrices
        .into_iter()
        .map(|(package_name, matrix)| {
            Task::new(task, &command, package_name, &args, matrix)
        })
        .map(Task::run)
        .partition_result();

    match errors.into_iter().next() {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

/// Gets a list of packages that are members of the workspace
fn get_workspace_members(
    metadata: &Metadata,
) -> impl Iterator<Item = &Package> + '_ {
    metadata
        .packages
        .iter()
        .filter(|package| metadata.workspace_members.contains(&package.id))
}

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
