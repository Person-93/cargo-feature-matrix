mod config;
mod execution;
pub mod features;

pub use self::{config::Config, execution::TaskKind};
use self::{execution::Task, features::FeatureMatrix};
use cargo_metadata::{Metadata, MetadataCommand, Package};
use figment::{
  providers::{Format, Json},
  Figment,
};
use std::path::PathBuf;
use thiserror::Error;

pub fn run(
  command: String,
  args: Vec<String>,
  task: TaskKind,
  manifest_path: Option<PathBuf>,
  figment: Figment,
) -> Result<(), Error> {
  let mut cmd = MetadataCommand::new();
  if let Some(manifest_path) = &manifest_path {
    cmd.manifest_path(manifest_path);
  }
  let metadata = cmd.exec()?;

  let mut error = None;
  for package in get_workspace_members(&metadata) {
    let figment =
      if let Some(package_config) = package.metadata.get("feature-matrix") {
        figment
          .clone()
          .merge(Figment::from(Json::string(&package_config.to_string())))
      } else {
        figment.clone()
      };

    let config = Config::from(figment)?;

    let matrix = FeatureMatrix::new(package, &config);
    let task = Task::new(
      task,
      &command,
      manifest_path.as_deref(),
      &package.name,
      &args,
      matrix,
    );

    if let Err(err) = task.run() {
      error = Some(err);
    }
  }

  match error {
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
  #[error("{}", message)]
  Io {
    message: &'static str,
    #[source]
    source: std::io::Error,
  },
  #[error("child process exited with {}", _0)]
  Fail(std::process::ExitStatus),
  #[error("failed to get config from metadata")]
  Config(#[from] figment::Error),
}

#[cfg(test)]
mod tests {}
